#![allow(dead_code)]

#[cfg(feature = "cli")]
use regex::Regex;
#[cfg(feature = "cli")]
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

use ranty::compiler::{CompilerError, CompilerMessage};
use ranty::runtime::RuntimeResult;
use ranty::{Ranty, RantyOptions, RantyProgram, RantyValue};

static NEXT_TEMP_ID: AtomicUsize = AtomicUsize::new(0);

pub fn test_ranty() -> Ranty {
    Ranty::with_options(RantyOptions {
        debug_mode: true,
        ..Default::default()
    })
}

pub fn compile(source: &str) -> Result<RantyProgram, CompilerError> {
    test_ranty().compile_quiet(source)
}

pub fn compile_with_reporter(
    source: &str,
) -> (Result<RantyProgram, CompilerError>, Vec<CompilerMessage>) {
    let ranty = test_ranty();
    let mut reporter = vec![];
    let result = ranty.compile(source, &mut reporter);
    (result, reporter)
}

pub fn run(source: &str) -> RuntimeResult<RantyValue> {
    let mut ranty = test_ranty();
    let program = ranty
        .compile_quiet(source)
        .expect("failed to compile program");
    ranty.run(&program)
}

pub fn run_str(source: &str) -> String {
    run(source).expect("failed to run program").to_string()
}

pub struct TempWorkspace {
    root: PathBuf,
}

impl TempWorkspace {
    pub fn new() -> Self {
        let mut root = env::temp_dir();
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock error")
            .as_nanos();
        let seq = NEXT_TEMP_ID.fetch_add(1, Ordering::Relaxed);
        root.push(format!("ranty-tests-{nonce}-{seq}"));
        fs::create_dir_all(&root).expect("failed to create temporary workspace");
        Self { root }
    }

    pub fn path(&self) -> &Path {
        &self.root
    }

    pub fn write(&self, rel_path: &str, contents: &str) -> PathBuf {
        let path = self.root.join(rel_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("failed to create parent directory");
        }
        fs::write(&path, contents).expect("failed to write test file");
        path
    }
}

impl Drop for TempWorkspace {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

pub struct EnvVarGuard {
    key: String,
    value: Option<std::ffi::OsString>,
}

impl EnvVarGuard {
    pub fn set(key: &str, value: impl AsRef<std::ffi::OsStr>) -> Self {
        let prev = env::var_os(key);
        env::set_var(key, value);
        Self {
            key: key.to_owned(),
            value: prev,
        }
    }
}

impl Drop for EnvVarGuard {
    fn drop(&mut self) {
        if let Some(value) = &self.value {
            env::set_var(&self.key, value);
        } else {
            env::remove_var(&self.key);
        }
    }
}

#[cfg(feature = "cli")]
#[derive(Debug, Deserialize)]
pub struct FixtureCorpus {
    pub cases: Vec<FixtureCase>,
}

#[cfg(feature = "cli")]
#[derive(Debug, Deserialize)]
pub struct FixtureCase {
    pub file: String,
    pub status: i32,
    pub stdout: String,
    pub stderr: String,
}

#[cfg(feature = "cli")]
#[derive(Debug, Deserialize)]
pub struct CliCorpus {
    pub cases: Vec<CliCase>,
}

#[cfg(feature = "cli")]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CliCase {
    pub name: String,
    pub kind: String,
    #[serde(default)]
    pub args: Vec<String>,
    pub stdin: Option<String>,
    pub expected_status: Option<i32>,
    pub expected_stdout: Option<String>,
    #[serde(default)]
    pub stdout_includes: Vec<String>,
    #[serde(default)]
    pub stderr_includes: Vec<String>,
    #[serde(default)]
    pub stderr_excludes: Vec<String>,
    #[serde(default)]
    pub files: std::collections::BTreeMap<String, String>,
    pub first: Option<CliLeg>,
    pub second: Option<CliLeg>,
}

#[cfg(feature = "cli")]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CliLeg {
    pub args: Vec<String>,
    pub stdin: Option<String>,
    pub expected_status: i32,
    #[serde(default)]
    pub stderr_includes: Vec<String>,
    #[serde(default)]
    pub stderr_excludes: Vec<String>,
}

#[cfg(feature = "cli")]
#[derive(Debug, Deserialize)]
pub struct FuzzCorpus {
    pub cases: Vec<FuzzCase>,
}

#[cfg(feature = "cli")]
#[derive(Debug, Deserialize)]
pub struct FuzzCase {
    pub label: String,
    pub kind: String,
    pub source: String,
    pub seed: Option<String>,
    pub status: i32,
    pub stdout: String,
    pub stderr: String,
}

#[cfg(feature = "cli")]
#[derive(Debug)]
pub struct CliOutput {
    pub status: i32,
    pub stdout: String,
    pub stderr: String,
}

#[cfg(feature = "cli")]
pub fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[cfg(feature = "cli")]
pub fn sources_root() -> PathBuf {
    repo_root().join("tests").join("sources")
}

#[cfg(feature = "cli")]
pub fn corpus_root() -> PathBuf {
    repo_root().join("tests").join("corpus")
}

#[cfg(feature = "cli")]
pub fn load_fixture_corpus() -> FixtureCorpus {
    read_json(corpus_root().join("fixtures.json"))
}

#[cfg(feature = "cli")]
pub fn load_cli_corpus() -> CliCorpus {
    read_json(corpus_root().join("cli.json"))
}

#[cfg(feature = "cli")]
pub fn load_fuzz_corpus() -> FuzzCorpus {
    read_json(corpus_root().join("fuzz.json"))
}

#[cfg(feature = "cli")]
pub fn cli_case_named<'a>(corpus: &'a CliCorpus, name: &str) -> &'a CliCase {
    corpus
        .cases
        .iter()
        .find(|case| case.name == name)
        .unwrap_or_else(|| panic!("missing CLI corpus case '{name}'"))
}

#[cfg(feature = "cli")]
pub fn run_cli(args: &[String], stdin: Option<&str>, cwd: &Path) -> CliOutput {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let mut command = Command::new(cli_bin());
    command.args(args);
    command.current_dir(cwd);
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

    let output = child.wait_with_output().expect("failed to wait for CLI");
    CliOutput {
        status: output.status.code().unwrap_or(1),
        stdout: normalize_stdout(&String::from_utf8_lossy(&output.stdout)),
        stderr: strip_ansi(&normalize_newlines(&String::from_utf8_lossy(
            &output.stderr,
        ))),
    }
}

#[cfg(feature = "cli")]
pub fn assert_cli_case(case: &CliCase) {
    match case.kind.as_str() {
        "simple" => {
            let output = run_cli(&case.args, case.stdin.as_deref(), &repo_root());
            assert_cli_output(
                output,
                case.expected_status,
                case.expected_stdout.as_deref(),
                &case.stdout_includes,
                &case.stderr_includes,
                &case.stderr_excludes,
            );
        }
        "workspace" => {
            let workspace = TempWorkspace::new();
            for (relative_path, contents) in &case.files {
                workspace.write(relative_path, contents);
            }
            let args = replace_workspace_tokens(&case.args, workspace.path());
            let output = run_cli(&args, case.stdin.as_deref(), workspace.path());
            assert_cli_output(
                output,
                case.expected_status,
                case.expected_stdout.as_deref(),
                &case.stdout_includes,
                &case.stderr_includes,
                &case.stderr_excludes,
            );
        }
        "paired-simple" => {
            let first = case
                .first
                .as_ref()
                .expect("paired-simple case missing first leg");
            let second = case
                .second
                .as_ref()
                .expect("paired-simple case missing second leg");

            let first_output = run_cli(&first.args, first.stdin.as_deref(), &repo_root());
            let second_output = run_cli(&second.args, second.stdin.as_deref(), &repo_root());

            assert_eq!(
                first_output.status, first.expected_status,
                "CLI case '{}' first leg exit mismatch",
                case.name
            );
            assert_eq!(
                second_output.status, second.expected_status,
                "CLI case '{}' second leg exit mismatch",
                case.name
            );
            for expected in &first.stderr_includes {
                assert!(
                    first_output.stderr.contains(expected),
                    "CLI case '{}' first leg stderr missing '{}'",
                    case.name,
                    expected
                );
            }
            for forbidden in &first.stderr_excludes {
                assert!(
                    !first_output.stderr.contains(forbidden),
                    "CLI case '{}' first leg stderr unexpectedly contained '{}'",
                    case.name,
                    forbidden
                );
            }
            for expected in &second.stderr_includes {
                assert!(
                    second_output.stderr.contains(expected),
                    "CLI case '{}' second leg stderr missing '{}'",
                    case.name,
                    expected
                );
            }
            for forbidden in &second.stderr_excludes {
                assert!(
                    !second_output.stderr.contains(forbidden),
                    "CLI case '{}' second leg stderr unexpectedly contained '{}'",
                    case.name,
                    forbidden
                );
            }
            assert_eq!(
                first_output.stdout, second_output.stdout,
                "CLI case '{}' expected matching stdout across paired runs",
                case.name
            );
        }
        other => panic!("unsupported CLI corpus case kind '{other}'"),
    }
}

#[cfg(feature = "cli")]
pub fn discover_executable_fixture_files() -> Vec<String> {
    fn visit(root: &Path, dir: &Path, out: &mut Vec<String>) {
        let entries = fs::read_dir(dir).expect("failed to read sources directory");
        for entry in entries {
            let entry = entry.expect("failed to read fixture entry");
            let path = entry.path();
            let name = path
                .file_name()
                .and_then(|name| name.to_str())
                .expect("invalid fixture path");
            if path.is_dir() {
                if name != "tutorial" {
                    visit(root, &path, out);
                }
                continue;
            }
            if name.ends_with(".ranty") {
                out.push(
                    path.strip_prefix(root)
                        .expect("fixture path should be under repo root")
                        .to_string_lossy()
                        .replace('\\', "/"),
                );
            }
        }
    }

    let root = repo_root();
    let mut files = Vec::new();
    visit(&root, &sources_root(), &mut files);
    files.sort();
    files
}

#[cfg(feature = "cli")]
fn cli_bin() -> &'static str {
    env!("CARGO_BIN_EXE_ranty")
}

#[cfg(feature = "cli")]
fn assert_cli_output(
    output: CliOutput,
    expected_status: Option<i32>,
    expected_stdout: Option<&str>,
    stdout_includes: &[String],
    stderr_includes: &[String],
    stderr_excludes: &[String],
) {
    if let Some(expected_status) = expected_status {
        assert_eq!(output.status, expected_status, "CLI exit mismatch");
    }
    if let Some(expected_stdout) = expected_stdout {
        assert_eq!(output.stdout, expected_stdout, "CLI stdout mismatch");
    }
    for expected in stdout_includes {
        assert!(
            output.stdout.contains(expected),
            "CLI stdout missing '{expected}'"
        );
    }
    for expected in stderr_includes {
        assert!(
            output.stderr.contains(expected),
            "CLI stderr missing '{expected}'"
        );
    }
    for forbidden in stderr_excludes {
        assert!(
            !output.stderr.contains(forbidden),
            "CLI stderr unexpectedly contained '{forbidden}'"
        );
    }
}

#[cfg(feature = "cli")]
fn replace_workspace_tokens(args: &[String], workspace: &Path) -> Vec<String> {
    let workspace = workspace.to_string_lossy();
    args.iter()
        .map(|arg| arg.replace("$WORKSPACE", workspace.as_ref()))
        .collect()
}

#[cfg(feature = "cli")]
fn read_json<T: serde::de::DeserializeOwned>(path: PathBuf) -> T {
    let source = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read '{}': {error}", path.display()));
    serde_json::from_str(&source)
        .unwrap_or_else(|error| panic!("failed to parse '{}': {error}", path.display()))
}

#[cfg(feature = "cli")]
fn normalize_newlines(text: &str) -> String {
    text.replace("\r\n", "\n")
}

#[cfg(feature = "cli")]
fn normalize_stdout(text: &str) -> String {
    let normalized = normalize_newlines(text);
    let function_pattern = Regex::new(r"\[function\(0x[0-9a-fA-F]+\)\]").unwrap();
    function_pattern
        .replace_all(&normalized, "[function(...)]")
        .into_owned()
}

#[cfg(feature = "cli")]
fn strip_ansi(text: &str) -> String {
    let ansi_pattern = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    ansi_pattern.replace_all(text, "").into_owned()
}

#[cfg(feature = "cli")]
pub fn relevant_stderr(text: &str, status: i32) -> String {
    let normalized = strip_ansi(&normalize_newlines(text));
    let clean = normalized.trim();
    if clean.is_empty() {
        return String::new();
    }

    if status == 0 {
        return clean
            .lines()
            .filter(|line| {
                let trimmed = line.trim();
                !trimmed.is_empty()
                    && !trimmed.starts_with("warning[")
                    && !trimmed.starts_with("-->")
                    && !trimmed.starts_with('|')
                    && !Regex::new(r"^\d+\s+\|").unwrap().is_match(trimmed)
            })
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_owned();
    }

    if let Some(runtime_match) = Regex::new(r"Runtime error:[^\n]*")
        .unwrap()
        .find(&normalized)
    {
        return runtime_match.as_str().to_owned();
    }

    if let Some(first_diagnostic) = normalized.lines().map(|line| line.trim()).find(|line| {
        Regex::new(r"^error(?:\[[^\]]+\])?:")
            .unwrap()
            .is_match(line)
    }) {
        return Regex::new(r"^error(?:\[[^\]]+\])?:\s*")
            .unwrap()
            .replace(first_diagnostic, "")
            .into_owned();
    }

    let compile_matches = Regex::new(r"Compile failed[^\n]*")
        .unwrap()
        .find_iter(&normalized)
        .map(|m| m.as_str().to_owned())
        .collect::<Vec<_>>();
    if let Some(summary) = compile_matches.last() {
        return Regex::new(r"^Compile failed:\s*")
            .unwrap()
            .replace(summary, "")
            .into_owned();
    }

    normalized
        .lines()
        .find(|line| !line.trim().is_empty())
        .unwrap_or("")
        .to_owned()
}
