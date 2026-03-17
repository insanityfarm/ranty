#![cfg(feature = "cli")]

mod common;

use std::io::Write;
use std::process::{Command, Output, Stdio};

use common::TempWorkspace;

fn cli_bin() -> &'static str {
    env!("CARGO_BIN_EXE_ranty")
}

fn run_cli(args: &[&str], stdin: Option<&str>) -> Output {
    let mut command = Command::new(cli_bin());
    command.args(args);
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    if stdin.is_some() {
        command.stdin(Stdio::piped());
    } else {
        command.stdin(Stdio::null());
    }

    let mut child = command.spawn().expect("failed to spawn CLI");
    if let Some(stdin) = stdin {
        child
            .stdin
            .as_mut()
            .expect("stdin should be piped")
            .write_all(stdin.as_bytes())
            .expect("failed to write stdin");
    }

    child.wait_with_output().expect("failed to wait for CLI")
}

fn stdout_text(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n")
}

fn stderr_text(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n")
}

#[test]
fn cli_help_mentions_ranty_and_supported_flags() {
    let output = run_cli(&["--help"], None);
    let stdout = stdout_text(&output);

    assert_eq!(output.status.code(), Some(0));
    assert!(stdout.contains("Command-line interface for Ranty"));
    assert!(stdout.contains("--bench-mode"));
    assert!(stdout.contains("--no-debug"));
}

#[test]
fn cli_version_reports_the_current_build_version() {
    let output = run_cli(&["--version"], None);
    let stdout = stdout_text(&output);

    assert_eq!(output.status.code(), Some(0));
    assert!(stdout.contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn cli_runs_a_file() {
    let workspace = TempWorkspace::new();
    let program = workspace.write("hello.ranty", r#""hello from file""#);

    let output = run_cli(&[program.to_str().unwrap()], None);

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(stdout_text(&output), "hello from file\n");
}

#[test]
fn eval_takes_precedence_over_file_input() {
    let workspace = TempWorkspace::new();
    let bad_program = workspace.write("bad.ranty", "@if : {}");

    let output = run_cli(
        &["--eval", r#""from eval""#, bad_program.to_str().unwrap()],
        None,
    );

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(stdout_text(&output), "from eval\n");
}

#[test]
fn cli_runs_piped_stdin_when_no_other_source_is_selected() {
    let output = run_cli(&[], Some(r#""from stdin""#));

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(stdout_text(&output), "from stdin\n");
}

#[test]
fn cli_uses_deterministic_hex_seeds() {
    let first = run_cli(
        &["--seed", "0xdeadbeef", "--eval", "[rand:0;1000000]"],
        None,
    );
    let second = run_cli(&["--seed", "deadbeef", "--eval", "[rand:0;1000000]"], None);

    assert_eq!(first.status.code(), Some(0));
    assert_eq!(second.status.code(), Some(0));
    assert_eq!(stdout_text(&first), stdout_text(&second));
}

#[test]
fn cli_rejects_invalid_seed_values() {
    let output = run_cli(&["--seed", "xyz", "--eval", "test"], None);

    assert_eq!(output.status.code(), Some(64));
    assert!(stderr_text(&output).contains("invalid seed"));
}

#[test]
fn cli_can_suppress_warnings() {
    let warned = run_cli(&["--eval", "<$unused = 1>ok"], None);
    let silenced = run_cli(&["--no-warnings", "--eval", "<$unused = 1>ok"], None);

    assert_eq!(warned.status.code(), Some(0));
    assert_eq!(silenced.status.code(), Some(0));
    assert!(stderr_text(&warned).contains("defined but never used"));
    assert!(!stderr_text(&silenced).contains("defined but never used"));
}

#[test]
fn cli_accepts_no_debug_flag() {
    let output = run_cli(&["--no-debug", "--eval", r#""no debug""#], None);

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(stdout_text(&output), "no debug\n");
}

#[test]
fn cli_bench_mode_reports_compile_and_execution_timing() {
    let output = run_cli(
        &["--bench-mode", "--seed", "1", "--eval", r#""bench""#],
        None,
    );
    let stderr = stderr_text(&output);

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(stdout_text(&output), "bench\n");
    assert!(stderr.contains("Compiled"));
    assert!(stderr.contains("Executed"));
}

#[test]
fn cli_returns_dataerr_for_compile_failures() {
    let output = run_cli(&["--eval", "@require"], None);

    assert_eq!(output.status.code(), Some(65));
    assert!(stderr_text(&output).contains("Compile failed"));
}

#[test]
fn cli_returns_software_for_runtime_failures() {
    let output = run_cli(&["--eval", r#"[error: "boom"]"#], None);

    assert_eq!(output.status.code(), Some(70));
    assert!(stderr_text(&output).contains("Runtime error"));
}
