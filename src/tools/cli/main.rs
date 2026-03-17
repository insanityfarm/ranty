#![allow(clippy::single_component_path_imports)]

use atty::Stream;
use clap::{App, Arg};
use codemap::CodeMap;
use codemap_diagnostic::{ColorConfig, Diagnostic, Emitter, Level, SpanLabel, SpanStyle};
use colored::*;
use compiler::Severity;
use ctrlc;
use exitcode::{self, ExitCode};
use rand::Rng;
use ranty::compiler::{CompilerMessage, Problem, Reporter};
use ranty::runtime::VM;
use ranty::*;
use std::io::{self, Read, Write};
use std::ops::Deref;
use std::process;
use std::sync::mpsc;
use std::{path::Path, time::Instant};

struct RantyCliOptions {
    no_debug: bool,
    no_warn: bool,
    bench_mode: bool,
    seed: Option<u64>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum LaunchMode {
    Eval,
    File,
    Stdin,
    Repl,
}

enum ProgramSource {
    Inline(String),
    Stdin(String),
    FilePath(String),
}

macro_rules! log_error {
  ($fmt:expr $(, $arg:expr),*) => {
    eprintln!("{}: {}", "error".bright_red().bold(), format!($fmt $(, $arg)*))
  }
}

fn main() {
    let version_long = format!("{} [{}]", BUILD_VERSION, embedded_triple::get());

    let arg_matches = App::new("Ranty CLI")
    .version(BUILD_VERSION)
    .about("Command-line interface for Ranty")
    .long_version(version_long.as_str())
    .arg(Arg::with_name("seed")
      .help("Specifies the initial 64-bit hex seed (1 to 16 hex digits, optional 0x prefix)")
      .short("s")
      .long("seed")
      .value_name("SEED")
    )
    .arg(Arg::with_name("eval")
      .help("Runs an inline program string; takes precedence over FILE and stdin")
      .short("e")
      .long("eval")
      .value_name("PROGRAM_STRING")
    )
    .arg(Arg::with_name("bench-mode")
      .help("Enables benchmarking")
      .short("b")
      .long("bench-mode")
    )
    .arg(Arg::with_name("no-warnings")
      .help("Disables compiler warnings")
      .short("W")
      .long("no-warnings")
    )
    .arg(Arg::with_name("no-debug")
      .help("Disable emitting debug symbols (may improve performance)")
      .short("D")
      .long("no-debug")
    )
    .arg(Arg::with_name("FILE")
      .help("Runs a Ranty file if --eval is not used")
      .index(1)
    )
    .get_matches();

    // Signal handling
    let (sig_tx, sig_rx) = mpsc::channel::<()>();
    ctrlc::set_handler(move || {
        sig_tx.send(()).unwrap();
    })
    .expect("failed to create signal handler");

    std::thread::spawn(move || {
        if sig_rx.recv().is_ok() {
            process::exit(exitcode::OK)
        }
    });

    let seed = match arg_matches.value_of("seed") {
        Some(raw_seed) => match parse_seed_arg(raw_seed) {
            Ok(seed) => Some(seed),
            Err(err) => {
                log_error!("{}", err);
                process::exit(exitcode::USAGE);
            }
        },
        None => None,
    };

    let opts = RantyCliOptions {
        bench_mode: arg_matches.is_present("bench-mode"),
        no_debug: arg_matches.is_present("no-debug"),
        no_warn: arg_matches.is_present("no-warnings"),
        seed,
    };

    let in_str = arg_matches.value_of("eval");
    let in_file = arg_matches.value_of("FILE");
    let stdin_is_tty = atty::is(Stream::Stdin);

    let mut ranty = Ranty::with_options(RantyOptions {
        use_stdlib: true,
        debug_mode: !opts.no_debug,
        top_level_defs_are_globals: false,
        seed: opts.seed.unwrap_or_else(|| rand::thread_rng().gen()),
    });

    register_cli_globals(&mut ranty);

    match select_launch_mode(in_str, in_file, stdin_is_tty) {
        LaunchMode::Eval => {
            let code = run_ranty(
                &mut ranty,
                ProgramSource::Inline(in_str.unwrap().to_owned()),
                &opts,
                false,
            );
            process::exit(code);
        }
        LaunchMode::File => {
            let path = in_file.unwrap();
            if !Path::new(path).exists() {
                log_error!("file not found: {}", path);
                process::exit(exitcode::NOINPUT);
            }
            let code = run_ranty(
                &mut ranty,
                ProgramSource::FilePath(path.to_owned()),
                &opts,
                false,
            );
            process::exit(code);
        }
        LaunchMode::Stdin => {
            let mut buf = vec![];
            if let Err(err) = io::stdin().read_to_end(&mut buf) {
                log_error!("failed to read from stdin: {}", err);
                process::exit(exitcode::SOFTWARE);
            }
            let source = String::from_utf8_lossy(&buf).into_owned();
            let code = run_ranty(&mut ranty, ProgramSource::Stdin(source), &opts, false);
            process::exit(code);
        }
        LaunchMode::Repl => {}
    }

    repl(&mut ranty, &opts);
}

fn parse_seed_arg(raw: &str) -> Result<u64, String> {
    let trimmed = raw.trim();
    let digits = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
        .unwrap_or(trimmed);

    if digits.is_empty() || digits.len() > 16 || !digits.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(format!(
            "invalid seed '{}'; expected 1 to 16 hexadecimal digits with an optional 0x prefix",
            raw
        ));
    }

    u64::from_str_radix(digits, 16).map_err(|_| {
        format!(
            "invalid seed '{}'; expected 1 to 16 hexadecimal digits with an optional 0x prefix",
            raw
        )
    })
}

fn select_launch_mode(eval: Option<&str>, file: Option<&str>, stdin_is_tty: bool) -> LaunchMode {
    if eval.is_some() {
        LaunchMode::Eval
    } else if file.is_some() {
        LaunchMode::File
    } else if !stdin_is_tty {
        LaunchMode::Stdin
    } else {
        LaunchMode::Repl
    }
}

fn repl(ranty: &mut Ranty, opts: &RantyCliOptions) {
    println!(
        "{}",
        format!("Ranty {}", ranty::BUILD_VERSION).white()
    );
    println!(
        "{}",
        "Write an expression and press Enter to run it."
            .truecolor(148, 148, 148)
            .italic()
    );
    println!(
        "{}\n",
        "More info: [credits], [copyright]"
            .truecolor(148, 148, 148)
            .italic()
    );
    loop {
        print!("{} ", ">>".cyan());
        io::stdout().flush().unwrap();
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                run_ranty(
                    ranty,
                    ProgramSource::Stdin(input.trim_end_matches(&['\r', '\n']).to_owned()),
                    opts,
                    true,
                );
            }
            Err(_) => log_error!("failed to read input"),
        }
    }
}

struct CliReporter {
    is_repl: bool,
    problems: Vec<CompilerMessage>,
}

impl CliReporter {
    fn new(is_repl: bool) -> Self {
        Self {
            is_repl,
            problems: Default::default(),
        }
    }
}

impl Reporter for CliReporter {
    fn report(&mut self, msg: CompilerMessage) {
        if self.is_repl && msg.is_warning() {
            match &msg.info() {
                // Since we share program-global variables between lines in the REPL,
                // it makes sense to ignore these types of warnings.
                Problem::UnusedVariable(_) | Problem::UnusedFunction(_) => return,
                _ => {}
            }
        }
        self.problems.push(msg);
    }
}

impl Deref for CliReporter {
    type Target = Vec<CompilerMessage>;

    fn deref(&self) -> &Self::Target {
        &self.problems
    }
}

fn register_cli_globals(ranty: &mut Ranty) {
    // Add [credits] function
    ranty.set_global_const(
        "credits",
        RantyValue::from_func(|vm: &mut VM, _: ()| {
            vm.cur_frame_mut().render_and_reset_output();
            vm.cur_frame_mut().write(include_str!("./_credits.txt"));
            Ok(())
        }),
    );

    // Add [copyright] function
    ranty.set_global_const(
        "copyright",
        RantyValue::from_func(|vm: &mut VM, _: ()| {
            vm.cur_frame_mut().render_and_reset_output();
            vm.cur_frame_mut().write(include_str!("./_copyright.txt"));
            Ok(())
        }),
    );
}

fn run_ranty(
    ctx: &mut Ranty,
    source: ProgramSource,
    opts: &RantyCliOptions,
    is_repl: bool,
) -> ExitCode {
    ctx.options_mut().top_level_defs_are_globals = is_repl;

    let show_stats = opts.bench_mode;
    let start_time = Instant::now();
    let mut problems = CliReporter::new(is_repl);

    let compile_result = match &source {
        ProgramSource::Inline(source) => ctx.compile_named(source, &mut problems, "cmdline"),
        ProgramSource::Stdin(source) => ctx.compile_named(source, &mut problems, "stdin"),
        ProgramSource::FilePath(path) => ctx.compile_file(path, &mut problems),
    };

    let parse_time = start_time.elapsed();

    let code = match &source {
        ProgramSource::Inline(s) => s.to_owned(),
        ProgramSource::Stdin(s) => s.to_owned(),
        ProgramSource::FilePath(path) => {
            std::fs::read_to_string(path).expect("can't open file for error reporting")
        }
    };

    let mut codemap = CodeMap::new();

    let file_span = codemap
        .add_file(
            match &source {
                ProgramSource::Inline(_) => "(cmdline)",
                ProgramSource::Stdin(_) => "(stdin)",
                ProgramSource::FilePath(path) => path,
            }
            .to_owned(),
            code,
        )
        .span;

    let mut emitter = Emitter::stderr(ColorConfig::Always, Some(&codemap));

    // Print errors/warnings
    for msg in problems.iter() {
        if opts.no_warn && msg.is_warning() {
            continue;
        }

        let d = Diagnostic {
            level: match msg.severity() {
                Severity::Warning => Level::Warning,
                Severity::Error => Level::Error,
            },
            message: msg.message(),
            code: Some(msg.code().to_owned()),
            spans: if let Some(pos) = &msg.pos() {
                let span = pos.span();
                let label = SpanLabel {
                    span: file_span.subspan(span.start as u64, span.end as u64),
                    label: msg.inline_message(),
                    style: SpanStyle::Primary,
                };
                vec![label]
            } else {
                vec![]
            },
        };
        eprintln!(); // extra line to separate code from errors
        emitter.emit(&[d]);
    }

    let errc = problems.iter().filter(|msg| msg.is_error()).count();

    // Make sure it compiled successfully
    match &compile_result {
        Ok(_) => {
            if show_stats {
                eprintln!("{} in {:?}", "Compiled".bright_green().bold(), parse_time)
            }
        }
        Err(_) => {
            eprintln!(
                "\n{}\n",
                format!(
                    "{} ({} {} found)",
                    "Compile failed".bright_red(),
                    errc,
                    if errc == 1 { "error" } else { "errors" }
                )
                .bold()
            );
            return exitcode::DATAERR;
        }
    }

    // Run it
    let program = compile_result.unwrap();
    let seed = opts.seed.unwrap_or_else(|| rand::thread_rng().gen());
    ctx.set_seed(seed);
    let start_time = Instant::now();
    let run_result = ctx.run(&program).map(|output| output.to_string());
    let run_time = start_time.elapsed();

    // Display results
    match run_result {
        Ok(output) => {
            if !output.is_empty() {
                println!("{}", output);
            }
            if show_stats {
                eprintln!(
                    "{} in {:?} (seed = {:016x})",
                    "Executed".bright_green().bold(),
                    run_time,
                    seed
                );
            }
            exitcode::OK
        }
        Err(err) => {
            eprintln!(
                "{}: {}\n\nstack trace:\n{}",
                "Runtime error".bright_red().bold(),
                &err,
                &err.stack_trace.as_deref().unwrap_or("(no trace available)")
            );
            if show_stats {
                eprintln!(
                    "{} in {:?} (seed = {:016x})",
                    "Crashed".bright_red().bold(),
                    run_time,
                    seed
                );
            }
            exitcode::SOFTWARE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_seed_arg, select_launch_mode, LaunchMode};

    #[test]
    fn launch_mode_precedence_is_eval_then_file_then_stdin_then_repl() {
        assert_eq!(
            select_launch_mode(Some("print"), Some("script.ranty"), false),
            LaunchMode::Eval
        );
        assert_eq!(
            select_launch_mode(None, Some("script.ranty"), false),
            LaunchMode::File
        );
        assert_eq!(select_launch_mode(None, None, false), LaunchMode::Stdin);
        assert_eq!(select_launch_mode(None, None, true), LaunchMode::Repl);
    }

    #[test]
    fn seed_parser_accepts_hex_with_or_without_prefix() {
        assert_eq!(parse_seed_arg("deadbeef").unwrap(), 0xdeadbeef);
        assert_eq!(parse_seed_arg("0xDEADBEEF").unwrap(), 0xdeadbeef);
        assert_eq!(parse_seed_arg("0").unwrap(), 0);
        assert_eq!(parse_seed_arg("ffffffffffffffff").unwrap(), u64::MAX);
    }

    #[test]
    fn seed_parser_rejects_invalid_values() {
        assert!(parse_seed_arg("").is_err());
        assert!(parse_seed_arg("xyz").is_err());
        assert!(parse_seed_arg("0x").is_err());
        assert!(parse_seed_arg("1234567890abcdef0").is_err());
    }
}
