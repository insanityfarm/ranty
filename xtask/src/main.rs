mod parity;

use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::{bail, Context, Result};
use mdbook::MDBook;
use ranty::Ranty;
use regex::Regex;
use walkdir::WalkDir;

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    match (
        args.next().as_deref(),
        args.next().as_deref(),
        args.next().as_deref(),
    ) {
        (Some("docs"), Some("build"), None) => docs_build(),
        (Some("docs"), Some("verify"), None) => docs_verify(),
        (Some("parity"), Some("build"), None) => parity::parity_build(),
        (Some("parity"), Some("verify"), None) => parity::parity_verify(),
        _ => bail!("usage: cargo xtask <docs|parity> <build|verify>"),
    }
}

fn docs_build() -> Result<()> {
    let repo = repo_root();
    generate_docs_artifacts(&repo)?;
    build_book(&repo)?;
    Ok(())
}

fn docs_verify() -> Result<()> {
    let repo = repo_root();
    generate_docs_artifacts(&repo)?;
    build_book(&repo)?;
    check_links(&repo.join("docs"))?;
    check_examples(&repo)?;
    check_docs_code_blocks(&repo)?;
    check_ranty_highlighting(&repo)?;
    check_tutorial_code_pairs(&repo)?;
    check_forbidden_strings(&repo)?;
    check_chapter_coverage(&repo)?;
    check_stdlib_docs(&repo)?;
    check_ci_drift(&repo)?;
    Ok(())
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask should live under the repo root")
        .to_path_buf()
}

fn build_book(repo: &Path) -> Result<()> {
    let docs_dir = repo.join("docs");
    if docs_dir.exists() {
        fs::remove_dir_all(&docs_dir)
            .with_context(|| format!("failed to remove {}", docs_dir.display()))?;
    }
    let book = MDBook::load(repo).context("failed to load mdBook configuration")?;
    book.build().context("mdBook build failed")?;
    let generated_dir = docs_dir.join("generated");
    if generated_dir.exists() && fs::read_dir(&generated_dir)?.next().is_none() {
        fs::remove_dir(&generated_dir)
            .with_context(|| format!("failed to remove empty {}", generated_dir.display()))?;
    }
    Ok(())
}

fn generate_docs_artifacts(repo: &Path) -> Result<()> {
    let generated_dir = repo.join("docs-src/generated");
    fs::create_dir_all(&generated_dir)
        .with_context(|| format!("failed to create {}", generated_dir.display()))?;

    write_generated_file(
        &generated_dir.join("compiler-messages.md"),
        &generate_compiler_messages_markdown(&repo.join("src/compiler/message.rs"))?,
    )?;
    write_generated_file(
        &generated_dir.join("runtime-errors.md"),
        &generate_runtime_errors_markdown(&repo.join("src/runtime/error.rs"))?,
    )?;
    write_generated_file(
        &generated_dir.join("stdlib-inventory.md"),
        &generate_stdlib_inventory_markdown(repo)?,
    )?;

    Ok(())
}

fn write_generated_file(path: &Path, contents: &str) -> Result<()> {
    fs::write(path, contents).with_context(|| format!("failed to write {}", path.display()))
}

fn generate_compiler_messages_markdown(path: &Path) -> Result<String> {
    let text =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    let problem_impl = extract_between(&text, "impl Problem {", "\nimpl Display for Problem {")?;
    let message_body = extract_between(
        problem_impl,
        "fn message(&self) -> String {",
        "fn inline_message(&self) -> Option<String> {",
    )?;
    let code_re = Regex::new(r#"Self::([A-Za-z0-9_]+)(?:\([^)]*\))?\s*=>\s*rcode!\((\d+)\),"#)?;
    let msg_re =
        Regex::new(r#"Self::([A-Za-z0-9_]+)(?:\([^)]*\))?\s*=>\s*rmsg!\("([^"]*(?:\\.[^"]*)*)""#)?;

    let mut codes = BTreeMap::new();
    for caps in code_re.captures_iter(&text) {
        codes.insert(caps[1].to_owned(), format!("R{}", &caps[2]));
    }

    let mut messages = BTreeMap::new();
    for caps in msg_re.captures_iter(message_body) {
        messages.insert(caps[1].to_owned(), caps[2].replace("\\\"", "\""));
    }

    let mut rows = vec![];
    for (variant, code) in codes {
        let message = messages
            .get(&variant)
            .cloned()
            .unwrap_or_else(|| "(message template unavailable)".to_owned());
        let severity = match code[1..].parse::<u32>() {
            Ok(value) if (1000..2000).contains(&value) => "warning",
            _ => "error",
        };
        rows.push((code, severity.to_owned(), message));
    }
    rows.sort_by(|a, b| a.0.cmp(&b.0));

    let mut out = String::new();
    out.push_str("## Compiler Messages\n\n");
    out.push_str("| Code | Severity | Message Template |\n");
    out.push_str("| --- | --- | --- |\n");
    for (code, severity, message) in rows {
        out.push_str(&format!(
            "| `{}` | {} | `{}` |\n",
            code,
            severity,
            markdown_escape_code(&message),
        ));
    }
    Ok(out)
}

fn generate_runtime_errors_markdown(path: &Path) -> Result<String> {
    let text =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    let enum_body = extract_between(
        &text,
        "pub enum RuntimeErrorType {",
        "\nimpl RuntimeErrorType {",
    )?;
    let variant_doc_re = Regex::new(
        r#"(?ms)^\s*/// (?P<summary>[^\n]+?)\.?\s*\n(?:\s*///[^\n]*\n)*?\s*(?P<variant>[A-Za-z0-9_]+)(?:\([^)]*\))?,"#,
    )?;
    let mut docs = BTreeMap::<String, String>::new();
    for caps in variant_doc_re.captures_iter(enum_body) {
        docs.insert(
            caps["variant"].to_owned(),
            caps["summary"].trim().trim_end_matches('.').to_owned(),
        );
    }

    let id_re = Regex::new(r#"Self::([A-Za-z0-9_]+)(?:\([^)]*\))?\s*=>\s*"([A-Z_]+)""#)?;
    let mut rows = vec![];
    for caps in id_re.captures_iter(&text) {
        let variant = caps[1].to_owned();
        let id = caps[2].to_owned();
        let summary = docs
            .get(&variant)
            .cloned()
            .unwrap_or_else(|| "Runtime failure category".to_owned());
        rows.push((id, summary));
    }
    rows.sort_by(|a, b| a.0.cmp(&b.0));
    rows.dedup_by(|a, b| a.0 == b.0);

    let mut out = String::new();
    out.push_str("## Runtime Error Categories\n\n");
    out.push_str("| Category | Summary |\n");
    out.push_str("| --- | --- |\n");
    for (id, summary) in rows {
        out.push_str(&format!(
            "| `{}` | {}. |\n",
            id,
            summary.trim_end_matches('.')
        ));
    }
    Ok(out)
}

fn generate_stdlib_inventory_markdown(repo: &Path) -> Result<String> {
    let exports = exported_symbols();
    let docs = parse_stdlib_docs(&repo.join("docs-src"))?;

    let mut out = String::new();
    out.push_str("## Stdlib Inventory\n\n");
    out.push_str("| Symbol | Category | Usage | Summary | Canonical Location |\n");
    out.push_str("| --- | --- | --- | --- | --- |\n");

    for name in exports {
        let entry = docs
            .get(&name)
            .with_context(|| format!("missing stdlib docs metadata for symbol `{name}`"))?;
        out.push_str(&format!(
            "| `{}` | {} | `{}` | {} | [{}]({}) |\n",
            name,
            markdown_table_escape(&entry.category),
            markdown_escape_code(entry.call_form.as_str()),
            markdown_table_escape(&entry.summary),
            markdown_table_escape(&entry.location),
            markdown_table_escape(&entry.location),
        ));
    }

    Ok(out)
}

fn check_links(docs_root: &Path) -> Result<()> {
    let link_re = Regex::new(r#"(?:href|src)="([^"]+)""#)?;
    let mut failures = vec![];

    for path in html_files(docs_root) {
        let text = fs::read_to_string(&path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        for caps in link_re.captures_iter(&text) {
            let target = &caps[1];
            if target.starts_with("http://")
                || target.starts_with("https://")
                || target.starts_with("mailto:")
                || target.starts_with("data:")
                || target.starts_with("javascript:")
                || target.starts_with('#')
            {
                continue;
            }

            let clean = target.split('#').next().unwrap().split('?').next().unwrap();
            if clean.is_empty() {
                continue;
            }
            let resolved = path.parent().unwrap().join(clean);
            if !resolved.exists() {
                failures.push(format!(
                    "{} -> {}",
                    path.strip_prefix(docs_root).unwrap().display(),
                    target
                ));
            }
        }
    }

    if failures.is_empty() {
        return Ok(());
    }

    bail!("broken documentation links:\n{}", failures.join("\n"))
}

fn check_examples(repo: &Path) -> Result<()> {
    let examples = parse_examples(&repo.join("docs-src"))?;
    if examples.is_empty() {
        return Ok(());
    }

    build_cli_binary(repo)?;
    let cli = cli_binary_path(repo);
    let mut failures = vec![];

    for example in examples {
        let output = run_cli_eval(&cli, &example.code)
            .with_context(|| format!("failed to execute example in {}", example.path.display()));
        match output {
            Ok(actual) if actual == example.expected => {}
            Ok(actual) => failures.push(format!(
                "{}: expected {:?}, got {:?}",
                example.path.display(),
                example.expected,
                actual
            )),
            Err(err) => failures.push(format!("{}: {err:#}", example.path.display())),
        }
    }

    if failures.is_empty() {
        return Ok(());
    }

    bail!("documentation example failures:\n{}", failures.join("\n"))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AuditedFenceKind {
    Ranty,
    RantyExample,
    Sh,
    Rust,
    Text,
    TextExpected,
}

fn check_docs_code_blocks(repo: &Path) -> Result<()> {
    let docs_src = repo.join("docs-src");
    let markdown_paths = markdown_files(&docs_src);
    if markdown_paths.is_empty() {
        return Ok(());
    }

    build_cli_binary(repo)?;
    let cli = cli_binary_path(repo);
    let mut failures = vec![];

    for path in markdown_paths {
        check_markdown_code_blocks(repo, &cli, &path, &mut failures)?;
    }

    if failures.is_empty() {
        return Ok(());
    }

    bail!(
        "documentation code block failures:\n{}",
        failures.join("\n\n")
    )
}

fn check_markdown_code_blocks(
    repo: &Path,
    cli: &Path,
    path: &Path,
    failures: &mut Vec<String>,
) -> Result<()> {
    let text =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    let lines: Vec<_> = text.lines().collect();
    let mut index = 0usize;

    while index < lines.len() {
        let line = lines[index].trim();
        let Some(kind) = audited_fence_kind(line) else {
            index += 1;
            continue;
        };

        let (body, next_index) = read_fenced_block(&lines, index)?;
        let start_line = index + 1;

        match kind {
            AuditedFenceKind::RantyExample => {
                match read_following_output_block(&lines, next_index) {
                    Ok(Some((expected, output_end, _))) => {
                        match run_cli_eval_in_dir(cli, &body, repo) {
                            Ok(actual) if matches_expected_output(&expected, &actual) => {}
                            Ok(actual) => failures.push(format!(
                                "{}:{}: expected {:?}, got {:?}",
                                path.display(),
                                start_line,
                                expected,
                                actual
                            )),
                            Err(err) => failures.push(format!(
                                "{}:{}: failed to execute ranty example: {err:#}",
                                path.display(),
                                start_line
                            )),
                        }
                        index = output_end;
                    }
                    Ok(None) => {
                        failures.push(format!(
                            "{}:{}: `ranty example` block is not followed by a text output block",
                            path.display(),
                            start_line
                        ));
                        index = next_index;
                    }
                    Err(err) => {
                        failures.push(format!(
                            "{}:{}: {err:#}",
                            path.display(),
                            start_line
                        ));
                        index = next_index;
                    }
                }
            }
            AuditedFenceKind::Ranty => {
                let paired_output = read_following_output_block(&lines, next_index)?;
                if let Some((expected, output_end, _)) = paired_output {
                    match run_cli_capture_in_dir(cli, &body, repo) {
                        Ok(outcome) if matches_expected_output(&expected, &outcome.text) => {}
                        Ok(outcome) => failures.push(format!(
                            "{}:{}: expected {:?}, got {:?}",
                            path.display(),
                            start_line,
                            expected,
                            outcome.text
                        )),
                        Err(err) => failures.push(format!(
                            "{}:{}: failed to execute ranty block: {err:#}",
                            path.display(),
                            start_line
                        )),
                    }
                    index = output_end;
                    continue;
                }

                let expectation_checks = parse_comment_expectation_checks(&body, start_line);
                if !expectation_checks.is_empty() {
                    for check in expectation_checks {
                        match run_cli_eval_in_dir(cli, &check.source, repo) {
                            Ok(actual) if actual == check.expected => {}
                            Ok(actual) => failures.push(format!(
                                "{}:{}: expected {:?}, got {:?}",
                                path.display(),
                                check.line,
                                check.expected,
                                actual
                            )),
                            Err(err) => failures.push(format!(
                                "{}:{}: failed to execute ranty probe: {err:#}",
                                path.display(),
                                check.line
                            )),
                        }
                    }
                    index = next_index;
                    continue;
                }

                index = next_index;
            }
            AuditedFenceKind::Sh => {
                let paired_output = read_following_output_block(&lines, next_index)?;
                if let Some((expected, output_end, _)) = paired_output {
                    match run_shell_block(repo, cli, &body) {
                        Ok(actual) if actual == expected => {}
                        Ok(actual) => failures.push(format!(
                            "{}:{}: expected shell output {:?}, got {:?}",
                            path.display(),
                            start_line,
                            expected,
                            actual
                        )),
                        Err(err) => failures.push(format!(
                            "{}:{}: failed to execute shell block: {err:#}",
                            path.display(),
                            start_line
                        )),
                    }
                    index = output_end;
                    continue;
                }

                if let Err(err) = check_shell_syntax(&body) {
                    failures.push(format!(
                        "{}:{}: shell example is not syntactically valid: {err:#}",
                        path.display(),
                        start_line
                    ));
                }
                index = next_index;
            }
            AuditedFenceKind::Rust => {
                if let Err(err) = check_rust_snippet(repo, &body) {
                    failures.push(format!(
                        "{}:{}: rust example failed to compile: {err:#}",
                        path.display(),
                        start_line
                    ));
                }
                index = next_index;
            }
            AuditedFenceKind::Text | AuditedFenceKind::TextExpected => {
                index = next_index;
            }
        }
    }

    Ok(())
}

#[derive(Debug)]
struct CommentExpectationCheck {
    line: usize,
    source: String,
    expected: String,
}

fn parse_comment_expectation_checks(body: &str, start_line: usize) -> Vec<CommentExpectationCheck> {
    #[derive(Clone)]
    struct Probe {
        body_index: usize,
        line: usize,
        code: String,
        expected: String,
    }

    let lines: Vec<_> = body.lines().collect();
    let mut probes = vec![];
    let mut pending_probe_candidate: Option<usize> = None;

    for (index, raw_line) in lines.iter().enumerate() {
        let line_no = start_line + 1 + index;

        if let Some(expected) = comment_only_expectation(raw_line) {
            if let Some(candidate_index) = pending_probe_candidate.take() {
                probes.push(Probe {
                    body_index: candidate_index,
                    line: start_line + 1 + candidate_index,
                    code: lines[candidate_index].to_owned(),
                    expected: expected.to_owned(),
                });
            }
            continue;
        }

        if let Some((code, expected)) = inline_expectation(raw_line) {
            probes.push(Probe {
                body_index: index,
                line: line_no,
                code: code.to_owned(),
                expected: expected.to_owned(),
            });
            pending_probe_candidate = None;
            continue;
        }

        if has_error_annotation(raw_line) {
            pending_probe_candidate = None;
            continue;
        }

        if raw_line.trim().is_empty() || raw_line.trim_start().starts_with('#') {
            pending_probe_candidate = None;
            continue;
        }

        pending_probe_candidate = Some(index);
    }

    let probe_lookup: BTreeMap<usize, Probe> = probes
        .iter()
        .cloned()
        .map(|probe| (probe.body_index, probe))
        .collect();

    probes
        .into_iter()
        .map(|probe| {
            let source = lines
                .iter()
                .enumerate()
                .filter_map(|(index, raw_line)| {
                    if comment_only_expectation(raw_line).is_some() || has_error_annotation(raw_line)
                    {
                        return None;
                    }

                    match probe_lookup.get(&index) {
                        Some(active_probe) if index == probe.body_index => {
                            Some(active_probe.code.clone())
                        }
                        Some(_) => None,
                        None => Some((*raw_line).to_owned()),
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");

            CommentExpectationCheck {
                line: probe.line,
                source,
                expected: probe.expected,
            }
        })
        .collect()
}

fn comment_only_expectation(line: &str) -> Option<&str> {
    line.trim_start()
        .strip_prefix("# ->")
        .map(|expected| expected.trim_start())
}

fn inline_expectation(line: &str) -> Option<(&str, &str)> {
    let marker_index = line.find("# ->")?;
    let code = line[..marker_index].trim_end();
    if code.is_empty() {
        return None;
    }
    let expected = line[marker_index + "# ->".len()..].trim_start();
    Some((code, expected))
}

fn has_error_annotation(line: &str) -> bool {
    let Some(comment_index) = line.find('#') else {
        return false;
    };
    let annotation = line[comment_index + 1..].trim_start().to_ascii_lowercase();
    annotation.starts_with("error") || annotation.starts_with("runtime error")
}

fn audited_fence_kind(line: &str) -> Option<AuditedFenceKind> {
    if !line.starts_with("```") {
        return None;
    }

    match line.trim_start_matches("```").trim() {
        "ranty" => Some(AuditedFenceKind::Ranty),
        "ranty example" => Some(AuditedFenceKind::RantyExample),
        "sh" => Some(AuditedFenceKind::Sh),
        "rust" => Some(AuditedFenceKind::Rust),
        "text" => Some(AuditedFenceKind::Text),
        "text expected" => Some(AuditedFenceKind::TextExpected),
        _ => None,
    }
}

fn read_following_output_block(
    lines: &[&str],
    next_index: usize,
) -> Result<Option<(String, usize, AuditedFenceKind)>> {
    let output_index = skip_tutorial_pair_gap(lines, next_index);
    if output_index >= lines.len() {
        return Ok(None);
    }

    let Some(kind) = audited_fence_kind(lines[output_index].trim()) else {
        return Ok(None);
    };
    if !matches!(kind, AuditedFenceKind::Text | AuditedFenceKind::TextExpected) {
        return Ok(None);
    }

    let (expected, output_end) = read_fenced_block(lines, output_index)?;
    Ok(Some((expected, output_end, kind)))
}

fn check_shell_syntax(script: &str) -> Result<()> {
    let status = Command::new("sh")
        .arg("-n")
        .arg("-c")
        .arg(script)
        .status()
        .context("failed to invoke sh for syntax checking")?;

    if status.success() {
        Ok(())
    } else {
        bail!("shell exited with {status}")
    }
}

fn run_shell_block(repo: &Path, cli: &Path, script: &str) -> Result<String> {
    let mut path_entries = vec![cli
        .parent()
        .with_context(|| format!("failed to locate parent dir for {}", cli.display()))?
        .display()
        .to_string()];
    if let Some(existing_path) = env::var_os("PATH") {
        path_entries.push(existing_path.to_string_lossy().into_owned());
    }

    let output = Command::new("sh")
        .arg("-c")
        .arg(script)
        .current_dir(repo)
        .env("PATH", path_entries.join(":"))
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .context("failed to execute shell block")?;

    if !output.status.success() {
        bail!(
            "{}",
            String::from_utf8_lossy(&output.stderr).trim().to_owned()
        );
    }

    Ok(String::from_utf8_lossy(&output.stdout)
        .trim_end_matches('\n')
        .replace("\r\n", "\n"))
}

fn check_rust_snippet(repo: &Path, snippet: &str) -> Result<()> {
    let temp = tempfile::tempdir().context("failed to create temp dir for rust snippet")?;
    fs::write(
        temp.path().join("Cargo.toml"),
        format!(
            r#"[package]
name = "doc-snippet-check"
version = "0.1.0"
edition = "2021"
publish = false

[dependencies]
ranty = {{ path = "{}" }}
"#,
            repo.display()
        ),
    )
    .context("failed to write Cargo.toml for rust snippet")?;

    let src_dir = temp.path().join("src");
    fs::create_dir_all(&src_dir).context("failed to create src dir for rust snippet")?;
    fs::write(
        src_dir.join("main.rs"),
        format!(
            "fn main() -> Result<(), Box<dyn std::error::Error>> {{\n{}\n}}\n",
            indent_block(&normalize_rust_doc_snippet(snippet), "    ")
        ),
    )
    .context("failed to write main.rs for rust snippet")?;

    run_command(temp.path(), "cargo", &["check", "--quiet", "--offline"])
}

fn indent_block(text: &str, indent: &str) -> String {
    text.lines()
        .map(|line| format!("{indent}{line}"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn normalize_rust_doc_snippet(snippet: &str) -> String {
    snippet
        .lines()
        .map(|line| {
            line.strip_prefix("# ")
                .or_else(|| line.strip_prefix('#'))
                .unwrap_or(line)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn check_tutorial_code_pairs(repo: &Path) -> Result<()> {
    let mut paths = markdown_files(&repo.join("docs-src/getting-started/tutorial"));
    let landing = repo.join("docs-src/getting-started/tutorial.md");
    if landing.exists() {
        paths.push(landing);
    }
    paths.sort();

    let mut failures = vec![];

    for path in paths {
        let text = fs::read_to_string(&path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        let lines: Vec<_> = text.lines().collect();
        let mut index = 0usize;

        while index < lines.len() {
            let line = lines[index].trim();
            let Some(kind) = tutorial_block_kind(line) else {
                index += 1;
                continue;
            };

            let (_, next_index) = read_fenced_block(&lines, index)?;
            match kind {
                TutorialBlockKind::RantyExample => {
                    let output_index = skip_tutorial_pair_gap(&lines, next_index);
                    if output_index >= lines.len()
                        || !is_expected_text_start(lines[output_index].trim())
                    {
                        failures.push(format!(
                            "{}:{}: tutorial `ranty example` block is not followed by `text expected`",
                            path.display(),
                            index + 1
                        ));
                        index = next_index;
                        continue;
                    }
                    let (_, output_end) = read_fenced_block(&lines, output_index)?;
                    index = output_end;
                }
                TutorialBlockKind::Ranty | TutorialBlockKind::Sh => {
                    let output_index = skip_tutorial_pair_gap(&lines, next_index);
                    if output_index >= lines.len()
                        || !is_plain_text_start(lines[output_index].trim())
                    {
                        failures.push(format!(
                            "{}:{}: tutorial `{}` block is not followed by `text`",
                            path.display(),
                            index + 1,
                            match kind {
                                TutorialBlockKind::Ranty => "ranty",
                                TutorialBlockKind::Sh => "sh",
                                _ => unreachable!(),
                            }
                        ));
                        index = next_index;
                        continue;
                    }
                    let (_, output_end) = read_fenced_block(&lines, output_index)?;
                    index = output_end;
                }
                TutorialBlockKind::Text | TutorialBlockKind::TextExpected => {
                    failures.push(format!(
                        "{}:{}: tutorial output block does not follow an input block",
                        path.display(),
                        index + 1
                    ));
                    index = next_index;
                }
            }
        }
    }

    if failures.is_empty() {
        return Ok(());
    }

    bail!("tutorial code block pairing failures:\n{}", failures.join("\n"))
}

struct HighlightExpectation {
    class_name: &'static str,
    text: &'static str,
}

struct RantyHighlightCase {
    name: &'static str,
    code: &'static str,
    expectations: Vec<HighlightExpectation>,
    absent_spans: Vec<&'static str>,
}

fn check_ranty_highlighting(repo: &Path) -> Result<()> {
    let mut failures = vec![];

    for case in ranty_highlight_cases() {
        let html = run_ranty_highlight(repo, case.code)
            .with_context(|| format!("failed to highlight case `{}`", case.name))?;

        for expectation in &case.expectations {
            let expected_span = format!(
                r#"<span class="hljs-{}">{}</span>"#,
                expectation.class_name,
                escape_html(expectation.text),
            );
            if !html.contains(&expected_span) {
                failures.push(format!(
                    "{}: missing {} span for {:?}\nhtml: {}",
                    case.name, expectation.class_name, expectation.text, html
                ));
            }
        }

        for unexpected in &case.absent_spans {
            if html.contains(unexpected) {
                failures.push(format!(
                    "{}: found unexpected highlighted fragment {:?}\nhtml: {}",
                    case.name, unexpected, html
                ));
            }
        }
    }

    if failures.is_empty() {
        return Ok(());
    }

    bail!("ranty syntax highlighting failures:\n{}", failures.join("\n\n"))
}

fn build_cli_binary(repo: &Path) -> Result<()> {
    run_command(
        repo,
        "cargo",
        &["build", "--quiet", "--features", "cli", "--bin", "ranty"],
    )
}

fn run_cli_eval(cli: &Path, code: &str) -> Result<String> {
    let cwd = env::current_dir().context("failed to read current working directory")?;
    run_cli_eval_in_dir(cli, code, &cwd)
}

fn run_cli_eval_in_dir(cli: &Path, code: &str, cwd: &Path) -> Result<String> {
    let outcome = run_cli_capture_in_dir(cli, code, cwd)?;
    if outcome.success {
        Ok(outcome.text)
    } else {
        bail!("{}", outcome.text)
    }
}

struct CliRunOutcome {
    success: bool,
    text: String,
}

fn run_cli_capture_in_dir(cli: &Path, code: &str, cwd: &Path) -> Result<CliRunOutcome> {
    let output = Command::new(cli)
        .arg("--seed")
        .arg("1")
        .arg("--eval")
        .arg(code)
        .current_dir(cwd)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .with_context(|| format!("failed to run {}", cli.display()))?;

    let text = if output.status.success() {
        String::from_utf8_lossy(&output.stdout)
            .replace("\r\n", "\n")
    } else {
        String::from_utf8_lossy(&output.stderr)
            .replace("\r\n", "\n")
    };

    Ok(CliRunOutcome {
        success: output.status.success(),
        text: normalize_cli_output(&text),
    })
}

fn normalize_cli_output(text: &str) -> String {
    Regex::new(r"\x1B\[[0-9;]*[A-Za-z]")
        .expect("ansi regex should compile")
        .replace_all(text, "")
        .trim_matches('\n')
        .to_string()
}

fn matches_expected_output(expected: &str, actual: &str) -> bool {
    if expected == actual {
        return true;
    }
    if !expected.contains("...") {
        return false;
    }

    let pattern = format!("(?s)^{}$", regex::escape(expected).replace("\\.\\.\\.", ".*"));
    Regex::new(&pattern)
        .map(|re| re.is_match(actual))
        .unwrap_or(false)
}

fn check_forbidden_strings(repo: &Path) -> Result<()> {
    let mut failures = vec![];
    let forbidden = [
        ("docs-src", Regex::new(r"\b(?:TODO|FIXME|HACK|WIP)\b")?),
        ("docs-src", Regex::new(r"\bRant 4\.x\b")?),
        (
            "docs-src",
            Regex::new(r"\b(?:pre-release|unstable|beta)\b")?,
        ),
    ];

    for (root, pattern) in forbidden {
        for path in markdown_files(&repo.join(root)) {
            let text = fs::read_to_string(&path)
                .with_context(|| format!("failed to read {}", path.display()))?;
            if pattern.is_match(&text) {
                failures.push(format!("{} matches {}", path.display(), pattern.as_str()));
            }
        }
    }

    let summary = fs::read_to_string(repo.join("docs-src/SUMMARY.md"))?;
    if summary.contains("proposals/") {
        failures.push("docs-src/SUMMARY.md still exposes proposal pages".to_owned());
    }

    if repo.join(".travis.yml").exists() {
        failures.push(".travis.yml should be removed".to_owned());
    }

    if !failures.is_empty() {
        bail!("forbidden docs residue detected:\n{}", failures.join("\n"));
    }

    Ok(())
}

fn check_chapter_coverage(repo: &Path) -> Result<()> {
    let summary = fs::read_to_string(repo.join("docs-src/SUMMARY.md"))?;
    for row in MIGRATION_ROWS {
        if !repo.join(row.destination).exists() {
            bail!(
                "migration target missing for {} -> {}",
                row.legacy,
                row.destination
            );
        }
    }

    for required in REQUIRED_DOCS {
        if !summary.contains(required.summary_entry) {
            bail!("SUMMARY.md is missing chapter {}", required.summary_entry);
        }
        if !repo.join(required.path).exists() {
            bail!("required chapter missing: {}", required.path);
        }
    }

    let docs_root = repo.join("docs");
    if docs_root.join("proposals").exists() {
        bail!("generated docs still expose proposal output");
    }

    Ok(())
}

fn check_stdlib_docs(repo: &Path) -> Result<()> {
    let exports = exported_symbols();
    let docs = parse_stdlib_docs(&repo.join("docs-src"))?;

    let exported: BTreeSet<_> = exports.iter().cloned().collect();
    let documented: BTreeSet<_> = docs.keys().cloned().collect();

    let missing: Vec<_> = exported.difference(&documented).cloned().collect();
    let extra: Vec<_> = documented.difference(&exported).cloned().collect();

    if !missing.is_empty() {
        bail!("undocumented stdlib symbols: {}", missing.join(", "));
    }
    if !extra.is_empty() {
        bail!(
            "documented-but-unexported stdlib symbols: {}",
            extra.join(", ")
        );
    }

    for name in exports {
        let entry = docs.get(&name).unwrap();
        validate_stdlib_entry(&name, entry)?;
        if looks_placeholder(&entry.summary) {
            bail!("stdlib symbol `{name}` has a placeholder summary");
        }
    }

    Ok(())
}

fn check_ci_drift(repo: &Path) -> Result<()> {
    if env::var_os("CI").is_none() {
        return Ok(());
    }

    let status = Command::new("git")
        .arg("diff")
        .arg("--exit-code")
        .arg("--")
        .arg("docs")
        .arg("docs-src/generated")
        .current_dir(repo)
        .status()
        .context("failed to run git diff for generated-doc drift")?;

    if status.success() {
        Ok(())
    } else {
        bail!("generated docs are out of date; run `cargo xtask docs build` and commit the results")
    }
}

fn run_command(repo: &Path, program: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(program)
        .args(args)
        .current_dir(repo)
        .status()
        .with_context(|| format!("failed to run {program} {}", args.join(" ")))?;

    if status.success() {
        Ok(())
    } else {
        bail!("{program} {} exited with {status}", args.join(" "))
    }
}

fn html_files(root: &Path) -> Vec<PathBuf> {
    WalkDir::new(root)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| entry.path().extension() == Some(OsStr::new("html")))
        .map(|entry| entry.into_path())
        .collect()
}

fn markdown_files(root: &Path) -> Vec<PathBuf> {
    WalkDir::new(root)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| entry.path().extension() == Some(OsStr::new("md")))
        .map(|entry| entry.into_path())
        .collect()
}

fn exported_symbols() -> Vec<String> {
    let ranty = Ranty::new();
    let mut names: Vec<_> = ranty.global_names().map(str::to_owned).collect();
    names.sort();
    names
}

#[derive(Debug)]
struct StdlibDoc {
    category: String,
    call_form: String,
    summary: String,
    location: String,
}

fn stdlib_usage_head(call_form: &str) -> Result<&str> {
    let trimmed = call_form.trim();
    let inner = trimmed
        .strip_prefix('[')
        .and_then(|s| s.strip_suffix(']'))
        .with_context(|| format!("usage is not a bracket form: {trimmed}"))?;
    Ok(inner.split(':').next().unwrap_or(inner).trim())
}

fn validate_stdlib_entry(name: &str, entry: &StdlibDoc) -> Result<()> {
    if entry.call_form.trim().is_empty() {
        bail!("stdlib symbol `{name}` has an empty call form");
    }
    if entry.call_form.trim() == name {
        return Ok(());
    }
    if entry.call_form.trim_start().starts_with("[%") {
        bail!(
            "stdlib symbol `{name}` uses invalid `%`-prefixed signature notation: {}",
            entry.call_form
        );
    }
    let usage_head = stdlib_usage_head(&entry.call_form)
        .with_context(|| format!("unable to parse stdlib usage for symbol `{name}`"))?;
    if usage_head != name {
        bail!(
            "stdlib symbol `{name}` has mismatched usage signature `{}`",
            entry.call_form
        );
    }
    Ok(())
}

fn parse_stdlib_docs(docs_src: &Path) -> Result<BTreeMap<String, StdlibDoc>> {
    let mut docs = BTreeMap::new();

    for path in markdown_files(&docs_src.join("stdlib")) {
        let rel = path
            .strip_prefix(docs_src)
            .unwrap()
            .to_string_lossy()
            .replace('\\', "/");
        let category = category_name_for(&rel)
            .unwrap_or("Standard Library")
            .to_owned();
        let text = fs::read_to_string(&path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        let sections = split_sections(&text);

        for section in sections {
            let Section {
                heading,
                first_rant_block,
                paragraphs,
            } = section;
            let name = heading.trim().to_owned();
            let is_constants_page = rel.ends_with("constants.md");
            let call_form = if is_constants_page {
                name.clone()
            } else {
                first_rant_block.unwrap_or_default()
            };
            let summary = if is_constants_page {
                paragraphs
                    .iter()
                    .find(|paragraph| !looks_like_type_annotation(paragraph))
                    .cloned()
                    .or_else(|| paragraphs.first().cloned())
                    .unwrap_or_default()
            } else {
                paragraphs.first().cloned().unwrap_or_default()
            };
            docs.insert(
                name.clone(),
                StdlibDoc {
                    category: category.clone(),
                    call_form: normalize_whitespace(&call_form),
                    summary: normalize_whitespace(&summary),
                    location: format!("{}#{}", rel, anchorize(&name)),
                },
            );
        }
    }

    Ok(docs)
}

struct Section<'a> {
    heading: &'a str,
    first_rant_block: Option<String>,
    paragraphs: Vec<String>,
}

fn split_sections(text: &str) -> Vec<Section<'_>> {
    let mut sections = vec![];
    let mut current_heading: Option<&str> = None;
    let mut current_lines: Vec<&str> = vec![];
    let mut in_fence = false;

    for line in text.lines() {
        if line.trim_start().starts_with("```") {
            in_fence = !in_fence;
        }
        if !in_fence {
            if let Some(heading) = line.strip_prefix("## ") {
                if let Some(previous_heading) = current_heading.take() {
                    sections.push(section_from_lines(previous_heading, &current_lines));
                }
                current_heading = Some(heading.trim());
                current_lines.clear();
                continue;
            }
        }
        if current_heading.is_some() {
            current_lines.push(line);
        }
    }

    if let Some(heading) = current_heading.take() {
        sections.push(section_from_lines(heading, &current_lines));
    }

    sections
}

fn section_from_lines<'a>(heading: &'a str, lines: &[&'a str]) -> Section<'a> {
    let mut first_rant_block = None;
    let mut in_rant_block = false;
    let mut ranty_block = String::new();
    let mut paragraph_lines = vec![];
    let mut paragraphs = vec![];

    for line in lines {
        let trimmed = line.trim();
        if trimmed.starts_with("```") {
            if in_rant_block {
                in_rant_block = false;
                if first_rant_block.is_none() {
                    first_rant_block = Some(ranty_block.trim().to_owned());
                }
                ranty_block.clear();
                continue;
            }

            let info = trimmed.trim_start_matches("```").trim();
            if info.starts_with("ranty") {
                in_rant_block = true;
            }
            continue;
        }

        if in_rant_block {
            ranty_block.push_str(line);
            ranty_block.push('\n');
            continue;
        }

        if trimmed.is_empty() {
            if !paragraph_lines.is_empty() {
                paragraphs.push(paragraph_lines.join(" "));
                paragraph_lines.clear();
            }
            continue;
        }

        if trimmed.starts_with('#')
            || trimmed.starts_with("**")
            || trimmed.starts_with("{{")
            || trimmed.starts_with('|')
            || trimmed.starts_with("&rarr;")
            || trimmed.starts_with("###")
            || trimmed.starts_with("```")
            || trimmed == "### Parameters"
            || trimmed == "### Example"
            || trimmed == "### Examples"
            || trimmed == "### Remarks"
            || trimmed == "### Errors"
        {
            continue;
        }

        if trimmed.starts_with('*') || trimmed.starts_with('-') {
            continue;
        }

        paragraph_lines.push(trimmed);
    }

    if !paragraph_lines.is_empty() {
        paragraphs.push(paragraph_lines.join(" "));
    }

    Section {
        heading,
        first_rant_block,
        paragraphs,
    }
}

#[derive(Debug)]
struct Example {
    path: PathBuf,
    code: String,
    expected: String,
}

fn parse_examples(docs_src: &Path) -> Result<Vec<Example>> {
    let mut examples = vec![];
    for path in markdown_files(docs_src) {
        let text = fs::read_to_string(&path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        let lines: Vec<_> = text.lines().collect();
        let mut index = 0usize;

        while index < lines.len() {
            let line = lines[index].trim();
            if !is_rant_example_start(line) {
                index += 1;
                continue;
            }
            let (code, next_index) = read_fenced_block(&lines, index)?;
            index = next_index;
            while index < lines.len() && lines[index].trim().is_empty() {
                index += 1;
            }
            if index < lines.len() && is_output_label_line(lines[index].trim()) {
                index += 1;
                while index < lines.len() && lines[index].trim().is_empty() {
                    index += 1;
                }
            }
            if index >= lines.len() || !is_expected_text_start(lines[index].trim()) {
                bail!(
                    "example block in {} is not followed by a `text expected` block",
                    path.display()
                );
            }
            let (expected, next_index) = read_fenced_block(&lines, index)?;
            index = next_index;
            examples.push(Example {
                path: path.clone(),
                code,
                expected,
            });
        }
    }
    Ok(examples)
}

fn is_rant_example_start(line: &str) -> bool {
    line.starts_with("```") && line.contains("ranty") && line.contains("example")
}

fn is_expected_text_start(line: &str) -> bool {
    line.starts_with("```") && line.contains("text") && line.contains("expected")
}

fn is_plain_text_start(line: &str) -> bool {
    line.starts_with("```") && line.trim_start_matches("```").trim() == "text"
}

fn is_output_label_line(line: &str) -> bool {
    let normalized = line
        .trim()
        .trim_matches(|c| c == '*' || c == '_')
        .trim();
    matches!(normalized, "Output" | "Output:")
}

#[derive(Clone, Copy)]
enum TutorialBlockKind {
    RantyExample,
    Ranty,
    Sh,
    TextExpected,
    Text,
}

fn tutorial_block_kind(line: &str) -> Option<TutorialBlockKind> {
    if !line.starts_with("```") {
        return None;
    }
    let info = line.trim_start_matches("```").trim();
    match info {
        "ranty example" => Some(TutorialBlockKind::RantyExample),
        "ranty" => Some(TutorialBlockKind::Ranty),
        "sh" => Some(TutorialBlockKind::Sh),
        "text expected" => Some(TutorialBlockKind::TextExpected),
        "text" => Some(TutorialBlockKind::Text),
        _ => None,
    }
}

fn skip_tutorial_pair_gap(lines: &[&str], mut index: usize) -> usize {
    while index < lines.len() && lines[index].trim().is_empty() {
        index += 1;
    }
    if index < lines.len() && is_tutorial_output_label_line(lines[index].trim()) {
        index += 1;
        while index < lines.len() && lines[index].trim().is_empty() {
            index += 1;
        }
    }
    index
}

fn is_tutorial_output_label_line(line: &str) -> bool {
    let normalized = line
        .trim()
        .trim_matches(|c| c == '*' || c == '_')
        .trim();
    matches!(
        normalized,
        "Output" | "Output:" | "What happened" | "What happened:"
    )
}

fn read_fenced_block(lines: &[&str], start: usize) -> Result<(String, usize)> {
    let mut body = vec![];
    let mut index = start + 1;
    while index < lines.len() {
        if lines[index].trim_start().starts_with("```") {
            return Ok((body.join("\n"), index + 1));
        }
        body.push(lines[index]);
        index += 1;
    }
    bail!("unclosed fenced block starting on line {}", start + 1)
}

fn cli_binary_path(repo: &Path) -> PathBuf {
    let exe_suffix = env::consts::EXE_SUFFIX;
    repo.join("target")
        .join("debug")
        .join(format!("ranty{exe_suffix}"))
}

fn category_name_for(rel_path: &str) -> Option<&'static str> {
    Some(match rel_path {
        "stdlib/general.md" => "General",
        "stdlib/control-flow.md" => "Attributes & Control Flow",
        "stdlib/collections.md" => "Collections",
        "stdlib/generators.md" => "Generators",
        "stdlib/formatting.md" => "Formatting",
        "stdlib/strings.md" => "Strings",
        "stdlib/boolean.md" => "Boolean",
        "stdlib/comparison.md" => "Comparison",
        "stdlib/math.md" => "Math",
        "stdlib/conversion.md" => "Conversion",
        "stdlib/verification.md" => "Verification",
        "stdlib/assertion.md" => "Assertion",
        "stdlib/constants.md" => "Constants",
        _ => return None,
    })
}

fn normalize_whitespace(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn extract_between<'a>(text: &'a str, start: &str, end: &str) -> Result<&'a str> {
    let start_index = text
        .find(start)
        .with_context(|| format!("failed to find marker `{start}`"))?;
    let body_start = start_index + start.len();
    let remainder = &text[body_start..];
    let end_index = remainder
        .find(end)
        .with_context(|| format!("failed to find marker `{end}`"))?;
    Ok(&remainder[..end_index])
}

fn looks_like_type_annotation(summary: &str) -> bool {
    let trimmed = summary.trim();
    let bare = trimmed
        .strip_prefix('`')
        .and_then(|rest| rest.strip_suffix('`'))
        .unwrap_or(trimmed);
    matches!(
        bare,
        "string"
            | "int"
            | "float"
            | "bool"
            | "list"
            | "tuple"
            | "map"
            | "range"
            | "function"
            | "selector"
            | "nothing"
    )
}

fn anchorize(name: &str) -> String {
    let mut out = String::new();
    for ch in name.chars() {
        match ch {
            'A'..='Z' => out.push(ch.to_ascii_lowercase()),
            'a'..='z' | '0'..='9' | '-' | '_' => out.push(ch),
            ' ' => out.push('-'),
            _ => {}
        }
    }
    out
}

fn markdown_escape_code(text: &str) -> Cow<'_, str> {
    if text.contains('`') {
        Cow::Owned(text.replace('`', "\\`"))
    } else {
        Cow::Borrowed(text)
    }
}

fn markdown_table_escape(text: &str) -> String {
    normalize_whitespace(text).replace('|', "\\|")
}

fn escape_html(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

fn run_ranty_highlight(repo: &Path, code: &str) -> Result<String> {
    let script = r#"
const fs = require("fs");
const hljs = require(process.argv[1]);
const code = fs.readFileSync(0, "utf8");
const result = hljs.highlight("ranty", code, true);
process.stdout.write(result.value);
"#;

    let mut child = Command::new("node")
        .arg("-e")
        .arg(script)
        .arg(repo.join("theme/highlight.js"))
        .current_dir(repo)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("failed to spawn node for ranty highlighting")?;

    child
        .stdin
        .take()
        .context("failed to open node stdin")?
        .write_all(code.as_bytes())
        .context("failed to send highlight input to node")?;

    let output = child
        .wait_with_output()
        .context("failed to read node highlight output")?;

    if !output.status.success() {
        bail!(
            "{}",
            String::from_utf8_lossy(&output.stderr).trim().to_owned()
        );
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

fn ranty_highlight_cases() -> Vec<RantyHighlightCase> {
    vec![
        RantyHighlightCase {
            name: "comments stay opaque",
            code: r#"# $vendor @if"#,
            expectations: vec![HighlightExpectation {
                class_name: "comment",
                text: "# $vendor @if",
            }],
            absent_spans: vec![
                r#"<span class="hljs-variable">$vendor</span>"#,
                r#"<span class="hljs-keyword">@if</span>"#,
            ],
        },
        RantyHighlightCase {
            name: "strings stay opaque",
            code: r#""Quote ""$vendor""""#,
            expectations: vec![HighlightExpectation {
                class_name: "string",
                text: r#""Quote ""$vendor""""#,
            }],
            absent_spans: vec![r#"<span class="hljs-variable">$vendor</span>"#],
        },
        RantyHighlightCase {
            name: "escapes and numbers",
            code: r#"\n \x41 \u0041 \U00000041 \U(41) 12 3.5 9E2"#,
            expectations: vec![
                HighlightExpectation {
                    class_name: "symbol",
                    text: r#"\n"#,
                },
                HighlightExpectation {
                    class_name: "symbol",
                    text: r#"\x41"#,
                },
                HighlightExpectation {
                    class_name: "symbol",
                    text: r#"\u0041"#,
                },
                HighlightExpectation {
                    class_name: "symbol",
                    text: r#"\U00000041"#,
                },
                HighlightExpectation {
                    class_name: "symbol",
                    text: r#"\U(41)"#,
                },
                HighlightExpectation {
                    class_name: "number",
                    text: "12",
                },
                HighlightExpectation {
                    class_name: "number",
                    text: "3.5",
                },
                HighlightExpectation {
                    class_name: "number",
                    text: "9E2",
                },
            ],
            absent_spans: vec![],
        },
        RantyHighlightCase {
            name: "keywords and literals",
            code: r#"<> @true @false @if @return @eq @text @lazy"#,
            expectations: vec![
                HighlightExpectation {
                    class_name: "literal",
                    text: "<>",
                },
                HighlightExpectation {
                    class_name: "literal",
                    text: "@true",
                },
                HighlightExpectation {
                    class_name: "literal",
                    text: "@false",
                },
                HighlightExpectation {
                    class_name: "keyword",
                    text: "@if",
                },
                HighlightExpectation {
                    class_name: "keyword",
                    text: "@return",
                },
                HighlightExpectation {
                    class_name: "keyword",
                    text: "@eq",
                },
                HighlightExpectation {
                    class_name: "keyword",
                    text: "@text",
                },
                HighlightExpectation {
                    class_name: "keyword",
                    text: "@lazy",
                },
            ],
            absent_spans: vec![],
        },
        RantyHighlightCase {
            name: "sigils and scoped identifiers",
            code: r#"<$foo = 1><%bar = 2><$/baz = 3>[$^next] { <^^foo> }"#,
            expectations: vec![
                HighlightExpectation {
                    class_name: "variable",
                    text: "$foo",
                },
                HighlightExpectation {
                    class_name: "variable",
                    text: "%bar",
                },
                HighlightExpectation {
                    class_name: "variable",
                    text: "$/baz",
                },
                HighlightExpectation {
                    class_name: "variable",
                    text: "$^next",
                },
                HighlightExpectation {
                    class_name: "operator",
                    text: "=",
                },
                HighlightExpectation {
                    class_name: "punctuation",
                    text: "<",
                },
                HighlightExpectation {
                    class_name: "punctuation",
                    text: ">",
                },
            ],
            absent_spans: vec![],
        },
        RantyHighlightCase {
            name: "call heads params and access paths",
            code: r#"[$greet: @text name; title ? "vendor"] { [kit/opening-line: Juniper] <stall/name> [step] [upper: hush] }"#,
            expectations: vec![
                HighlightExpectation {
                    class_name: "variable",
                    text: "$greet",
                },
                HighlightExpectation {
                    class_name: "params",
                    text: "name",
                },
                HighlightExpectation {
                    class_name: "params",
                    text: "title",
                },
                HighlightExpectation {
                    class_name: "title",
                    text: "kit",
                },
                HighlightExpectation {
                    class_name: "title",
                    text: "opening-line",
                },
                HighlightExpectation {
                    class_name: "title",
                    text: "step",
                },
                HighlightExpectation {
                    class_name: "title",
                    text: "upper",
                },
                HighlightExpectation {
                    class_name: "attr",
                    text: "name",
                },
            ],
            absent_spans: vec![],
        },
        RantyHighlightCase {
            name: "operators punctuation and temporal labels",
            code: r#"<> < > [ ] { } ( ) : ; :: .. + - * ** / % & | ^ = += -= *= **= /= %= &= |= ^= ?= |> [] ` ~ *pair*"#,
            expectations: vec![
                HighlightExpectation {
                    class_name: "punctuation",
                    text: "<",
                },
                HighlightExpectation {
                    class_name: "punctuation",
                    text: ">",
                },
                HighlightExpectation {
                    class_name: "punctuation",
                    text: "[",
                },
                HighlightExpectation {
                    class_name: "punctuation",
                    text: "]",
                },
                HighlightExpectation {
                    class_name: "punctuation",
                    text: "{",
                },
                HighlightExpectation {
                    class_name: "punctuation",
                    text: "}",
                },
                HighlightExpectation {
                    class_name: "punctuation",
                    text: "(",
                },
                HighlightExpectation {
                    class_name: "punctuation",
                    text: ")",
                },
                HighlightExpectation {
                    class_name: "punctuation",
                    text: ":",
                },
                HighlightExpectation {
                    class_name: "punctuation",
                    text: ";",
                },
                HighlightExpectation {
                    class_name: "punctuation",
                    text: "::",
                },
                HighlightExpectation {
                    class_name: "operator",
                    text: "..",
                },
                HighlightExpectation {
                    class_name: "operator",
                    text: "+",
                },
                HighlightExpectation {
                    class_name: "operator",
                    text: "-",
                },
                HighlightExpectation {
                    class_name: "operator",
                    text: "*",
                },
                HighlightExpectation {
                    class_name: "operator",
                    text: "**",
                },
                HighlightExpectation {
                    class_name: "operator",
                    text: "/",
                },
                HighlightExpectation {
                    class_name: "operator",
                    text: "%",
                },
                HighlightExpectation {
                    class_name: "operator",
                    text: "&",
                },
                HighlightExpectation {
                    class_name: "operator",
                    text: "|",
                },
                HighlightExpectation {
                    class_name: "operator",
                    text: "^",
                },
                HighlightExpectation {
                    class_name: "operator",
                    text: "=",
                },
                HighlightExpectation {
                    class_name: "operator",
                    text: "+=",
                },
                HighlightExpectation {
                    class_name: "operator",
                    text: "-=",
                },
                HighlightExpectation {
                    class_name: "operator",
                    text: "*=",
                },
                HighlightExpectation {
                    class_name: "operator",
                    text: "**=",
                },
                HighlightExpectation {
                    class_name: "operator",
                    text: "/=",
                },
                HighlightExpectation {
                    class_name: "operator",
                    text: "%=",
                },
                HighlightExpectation {
                    class_name: "operator",
                    text: "&=",
                },
                HighlightExpectation {
                    class_name: "operator",
                    text: "|=",
                },
                HighlightExpectation {
                    class_name: "operator",
                    text: "^=",
                },
                HighlightExpectation {
                    class_name: "operator",
                    text: "?=",
                },
                HighlightExpectation {
                    class_name: "operator",
                    text: "|>",
                },
                HighlightExpectation {
                    class_name: "operator",
                    text: "[]",
                },
                HighlightExpectation {
                    class_name: "operator",
                    text: "`",
                },
                HighlightExpectation {
                    class_name: "operator",
                    text: "~",
                },
                HighlightExpectation {
                    class_name: "symbol",
                    text: "*pair*",
                },
            ],
            absent_spans: vec![],
        },
    ]
}

fn looks_placeholder(summary: &str) -> bool {
    let lower = summary.trim().to_ascii_lowercase();
    lower.is_empty()
        || looks_like_type_annotation(summary)
        || lower.contains("todo")
        || lower.contains("tbd")
        || lower.contains("placeholder")
        || lower.contains("work-in-progress")
}

#[allow(dead_code)]
struct MigrationRow {
    legacy: &'static str,
    destination: &'static str,
    reference: &'static str,
    wayback: &'static str,
    notes: &'static str,
}

const MIGRATION_ROWS: &[MigrationRow] = &[
    MigrationRow {
        legacy: "docs/index.html",
        destination: "docs-src/intro.md",
        reference: "`docs-src/intro.md`",
        wayback: "`/`",
        notes: "Legacy landing page now resolves through the mdBook intro.",
    },
    MigrationRow {
        legacy: "docs/intro.html",
        destination: "docs-src/intro.md",
        reference: "`docs-src/intro.md`",
        wayback: "`/intro.html`",
        notes: "Merged with the current stable intro text.",
    },
    MigrationRow {
        legacy: "docs/getting-started.html",
        destination: "docs-src/getting-started.md",
        reference: "repo-local synthesis",
        wayback: "n/a",
        notes: "Preserves the repo-specific stable quickstart page.",
    },
    MigrationRow {
        legacy: "docs/language.html",
        destination: "docs-src/language.md",
        reference: "`docs-src/language.md`",
        wayback: "`/language.html`",
        notes: "Top-level language overview.",
    },
    MigrationRow {
        legacy: "docs/language/text.html",
        destination: "docs-src/language/text.md",
        reference: "`docs-src/language/text.md`",
        wayback: "`/language/text.html`",
        notes: "Merged with current hinting/sinking examples.",
    },
    MigrationRow {
        legacy: "docs/language/blocks.html",
        destination: "docs-src/language/blocks.md",
        reference: "`docs-src/language/blocks.md`",
        wayback: "`/language/blocks.html`",
        notes: "Expanded with protected-block subchapter.",
    },
    MigrationRow {
        legacy: "docs/language/functions.html",
        destination: "docs-src/language/functions.md",
        reference: "`docs-src/language/functions.md`",
        wayback: "`/language/functions.html`",
        notes:
            "Re-expanded into subchapters for lambdas, optionals, variadics, spreading, and piping.",
    },
    MigrationRow {
        legacy: "docs/language/data-types.html",
        destination: "docs-src/language/data-types.md",
        reference: "`docs-src/language/data-types.md`",
        wayback: "`/language/data-types.html`",
        notes: "Backed by per-type subpages in the mdBook tree.",
    },
    MigrationRow {
        legacy: "docs/language/accessors.html",
        destination: "docs-src/language/accessors.md",
        reference: "`docs-src/language/accessors.md`",
        wayback: "`/language/accessors.html`",
        notes: "Backed by access-path, fallback, descoping, and anonymous-accessor subpages.",
    },
    MigrationRow {
        legacy: "docs/language/keywords.html",
        destination: "docs-src/language/keywords.md",
        reference: "`docs-src/language/keywords.md`",
        wayback: "`/language/keywords.html`",
        notes: "Keyword index page.",
    },
    MigrationRow {
        legacy: "docs/language/keywords/require.html",
        destination: "docs-src/language/keywords/require.md",
        reference: "`docs-src/language/keywords/require.md`",
        wayback: "`/language/keywords/require.html`",
        notes: "Cross-checked with module resolver behavior.",
    },
    MigrationRow {
        legacy: "docs/language/keywords/return.html",
        destination: "docs-src/language/keywords/return.md",
        reference: "`docs-src/language/keywords/return.md`",
        wayback: "`/language/keywords/return.html`",
        notes: "Retained from upstream book.",
    },
    MigrationRow {
        legacy: "docs/language/keywords/continue.html",
        destination: "docs-src/language/keywords/continue.md",
        reference: "repo-local synthesis",
        wayback: "`/language/keywords/continue.html`",
        notes: "Replaced the upstream placeholder with shipped 4.0 semantics.",
    },
    MigrationRow {
        legacy: "docs/language/keywords/break.html",
        destination: "docs-src/language/keywords/break.md",
        reference: "repo-local synthesis",
        wayback: "`/language/keywords/break.html`",
        notes: "Replaced the upstream placeholder with shipped 4.0 semantics.",
    },
    MigrationRow {
        legacy: "docs/language/keywords/weight.html",
        destination: "docs-src/language/keywords/weight.md",
        reference: "`docs-src/language/keywords/weight.md`",
        wayback: "`/language/keywords/weight.html`",
        notes: "Weight semantics remain upstream-based.",
    },
    MigrationRow {
        legacy: "docs/language/keywords/text.html",
        destination: "docs-src/language/keywords/text.md",
        reference: "`docs-src/language/keywords/text.md`",
        wayback: "`/language/keywords/text.html`",
        notes: "Auto-hinting reference preserved.",
    },
    MigrationRow {
        legacy: "docs/language/operators.html",
        destination: "docs-src/language/operators.md",
        reference: "`docs-src/language/operators.md`",
        wayback: "`/language/operators.html`",
        notes: "Operator reference.",
    },
    MigrationRow {
        legacy: "docs/language/conditional-expressions.html",
        destination: "docs-src/language/conditional-expressions.md",
        reference: "`docs-src/language/conditional-expressions.md`",
        wayback: "`/language/conditional-expressions.html`",
        notes: "Merged with current stable truthiness table and short-circuit notes.",
    },
    MigrationRow {
        legacy: "docs/language/output-modifiers.html",
        destination: "docs-src/language/output-modifiers.md",
        reference: "`docs-src/language/output-modifiers.md`",
        wayback: "`/language/output-modifiers.html`",
        notes: "Merged with repo-local `@edit` behavior and placement rules.",
    },
    MigrationRow {
        legacy: "docs/runtime.html",
        destination: "docs-src/runtime.md",
        reference: "`docs-src/runtime.md`",
        wayback: "`/runtime.html`",
        notes: "Runtime overview refreshed for stable 4.0.",
    },
    MigrationRow {
        legacy: "docs/runtime/attributes.html",
        destination: "docs-src/runtime/attributes.md",
        reference: "`docs-src/runtime/attributes.md`",
        wayback: "`/runtime/attributes.html`",
        notes: "Attribute reference.",
    },
    MigrationRow {
        legacy: "docs/runtime/formatters.html",
        destination: "docs-src/runtime/formatters.md",
        reference: "`docs-src/runtime/formatters.md`",
        wayback: "`/runtime/formatters.html`",
        notes: "Expanded with case, number, and whitespace subpages.",
    },
    MigrationRow {
        legacy: "docs/modules.html",
        destination: "docs-src/modules.md",
        reference: "`docs-src/modules.md`",
        wayback: "`/modules.html`",
        notes: "Merged with the repo-local stable module-system clarifications.",
    },
    MigrationRow {
        legacy: "docs/cli.html",
        destination: "docs-src/cli.md",
        reference: "repo-local synthesis",
        wayback: "n/a",
        notes: "Preserves the repo-local CLI and REPL chapter.",
    },
    MigrationRow {
        legacy: "docs/stdlib.html",
        destination: "docs-src/stdlib.md",
        reference: "`docs-src/stdlib.md`",
        wayback: "`/stdlib.html`",
        notes: "Now links to the generated stdlib inventory and category chapters.",
    },
    MigrationRow {
        legacy: "docs/stdlib/general.html",
        destination: "docs-src/stdlib/general.md",
        reference: "`docs-src/stdlib/general.md`",
        wayback: "`/stdlib/general.html`",
        notes: "Retained as the canonical general-functions chapter.",
    },
    MigrationRow {
        legacy: "docs/stdlib/control-flow.html",
        destination: "docs-src/stdlib/control-flow.md",
        reference: "`docs-src/stdlib/control-flow.md`",
        wayback: "`/stdlib/control-flow.html`",
        notes: "Reconciled old attribute helper names with the shipped export surface.",
    },
    MigrationRow {
        legacy: "docs/stdlib/collections.html",
        destination: "docs-src/stdlib/collections.md",
        reference: "`docs-src/stdlib/collections.md`",
        wayback: "`/stdlib/collections.html`",
        notes: "Collections reference with missing entries restored.",
    },
    MigrationRow {
        legacy: "docs/stdlib/generators.html",
        destination: "docs-src/stdlib/generators.md",
        reference: "`docs-src/stdlib/generators.md`",
        wayback: "`/stdlib/generators.html`",
        notes: "Generator reference with missing entries restored.",
    },
    MigrationRow {
        legacy: "docs/stdlib/formatting.html",
        destination: "docs-src/stdlib/formatting.md",
        reference: "`docs-src/stdlib/formatting.md`",
        wayback: "`/stdlib/formatting.html`",
        notes: "Formatting reference with `ws-fmt` and current numeric formatter surface.",
    },
    MigrationRow {
        legacy: "docs/stdlib/strings.html",
        destination: "docs-src/stdlib/strings.md",
        reference: "`docs-src/stdlib/strings.md`",
        wayback: "`/stdlib/strings.html`",
        notes: "Strings reference with current `indent`, `trim`, and replacement helpers.",
    },
    MigrationRow {
        legacy: "docs/stdlib/boolean.html",
        destination: "docs-src/stdlib/boolean.md",
        reference: "`docs-src/stdlib/boolean.md`",
        wayback: "`/stdlib/boolean.html`",
        notes: "Boolean reference.",
    },
    MigrationRow {
        legacy: "docs/stdlib/comparison.html",
        destination: "docs-src/stdlib/comparison.md",
        reference: "`docs-src/stdlib/comparison.md`",
        wayback: "`/stdlib/comparison.html`",
        notes: "Comparison reference.",
    },
    MigrationRow {
        legacy: "docs/stdlib/math.html",
        destination: "docs-src/stdlib/math.md",
        reference: "`docs-src/stdlib/math.md`",
        wayback: "`/stdlib/math.html`",
        notes: "Math reference with `clamp` restored.",
    },
    MigrationRow {
        legacy: "docs/stdlib/conversion.html",
        destination: "docs-src/stdlib/conversion.md",
        reference: "`docs-src/stdlib/conversion.md`",
        wayback: "`/stdlib/conversion.html`",
        notes: "Conversion reference with `to-bool` restored.",
    },
    MigrationRow {
        legacy: "docs/stdlib/verification.html",
        destination: "docs-src/stdlib/verification.md",
        reference: "`docs-src/stdlib/verification.md`",
        wayback: "`/stdlib/verification.html`",
        notes: "Verification reference with `is`, `is-factor`, and current surface.",
    },
    MigrationRow {
        legacy: "docs/stdlib/assertion.html",
        destination: "docs-src/stdlib/assertion.md",
        reference: "`docs-src/stdlib/assertion.md`",
        wayback: "`/stdlib/assertion.html`",
        notes: "Assertion reference.",
    },
    MigrationRow {
        legacy: "docs/stdlib/constants.html",
        destination: "docs-src/stdlib/constants.md",
        reference: "`docs-src/stdlib/constants.md`",
        wayback: "`/stdlib/constants.html`",
        notes: "Constants reference.",
    },
    MigrationRow {
        legacy: "docs/compiler-messages.html",
        destination: "docs-src/compiler-messages.md",
        reference: "generated from Rust source",
        wayback: "`/compiler-messages.html`",
        notes: "Compiler/runtime diagnostics are regenerated during every docs build.",
    },
    MigrationRow {
        legacy: "docs/glossary.html",
        destination: "docs-src/glossary.md",
        reference: "`docs-src/glossary.md`",
        wayback: "`/glossary.html`",
        notes: "Glossary retained.",
    },
];

struct RequiredDoc {
    summary_entry: &'static str,
    path: &'static str,
}

const REQUIRED_DOCS: &[RequiredDoc] = &[
    RequiredDoc {
        summary_entry: "Getting Started",
        path: "docs-src/getting-started.md",
    },
    RequiredDoc {
        summary_entry: "CLI / REPL",
        path: "docs-src/cli.md",
    },
    RequiredDoc {
        summary_entry: "Modules",
        path: "docs-src/modules.md",
    },
    RequiredDoc {
        summary_entry: "Diagnostics",
        path: "docs-src/compiler-messages.md",
    },
    RequiredDoc {
        summary_entry: "Comparison of Rant 3 and Ranty",
        path: "docs-src/rant-3-vs-ranty.md",
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn parse_examples_from(markdown: &str) -> Result<Vec<Example>> {
        let dir = tempdir()?;
        fs::write(dir.path().join("page.md"), markdown)?;
        parse_examples(dir.path())
    }

    #[test]
    fn parse_examples_accepts_adjacent_expected_block() {
        let examples = parse_examples_from(
            r#"
```ranty example
hello
```

```text expected
hello
```
"#,
        )
        .expect("adjacent expected block should parse");

        assert_eq!(examples.len(), 1);
        assert_eq!(examples[0].code, "hello");
        assert_eq!(examples[0].expected, "hello");
    }

    #[test]
    fn parse_examples_accepts_output_label_between_blocks() {
        let examples = parse_examples_from(
            r#"
```ranty example
hello
```

**Output**

```text expected
hello
```
"#,
        )
        .expect("output label should be allowed");

        assert_eq!(examples.len(), 1);
        assert_eq!(examples[0].code, "hello");
        assert_eq!(examples[0].expected, "hello");
    }

    #[test]
    fn parse_examples_rejects_non_label_paragraph_between_blocks() {
        let err = parse_examples_from(
            r#"
```ranty example
hello
```

Not output.

```text expected
hello
```
"#,
        )
        .expect_err("non-label paragraph should fail");

        assert!(err
            .to_string()
            .contains("is not followed by a `text expected` block"));
    }

    fn check_tutorial_pairs_from(markdown: &str) -> Result<()> {
        let dir = tempdir()?;
        let tutorial_dir = dir.path().join("docs-src/getting-started/tutorial");
        fs::create_dir_all(&tutorial_dir)?;
        fs::write(tutorial_dir.join("page.md"), markdown)?;
        check_tutorial_code_pairs(dir.path())
    }

    #[test]
    fn tutorial_pairs_accept_ranty_and_text_blocks() {
        check_tutorial_pairs_from(
            r#"
**Wrong attempt**

```ranty
hello
```

**What happened**

```text
hello
```
"#,
        )
        .expect("ordinary tutorial pairs should parse");
    }

    #[test]
    fn tutorial_pairs_reject_lone_ranty_block() {
        let err = check_tutorial_pairs_from(
            r#"
```ranty
hello
```
"#,
        )
        .expect_err("lone tutorial ranty block should fail");

        assert!(err.to_string().contains("is not followed by `text`"));
    }

    #[test]
    fn tutorial_pairs_reject_output_without_input() {
        let err = check_tutorial_pairs_from(
            r#"
```text
hello
```
"#,
        )
        .expect_err("stray tutorial text block should fail");

        assert!(err
            .to_string()
            .contains("tutorial output block does not follow an input block"));
    }

    #[test]
    fn ranty_highlighting_covers_audited_token_families() {
        check_ranty_highlighting(&repo_root()).expect("ranty highlighting should cover audited syntax");
    }

    #[test]
    fn stdlib_validation_rejects_percent_prefixed_usage() {
        let entry = StdlibDoc {
            category: "General".into(),
            call_form: "[%len: value]".into(),
            summary: "Prints a length.".into(),
            location: "stdlib/general.md#len".into(),
        };

        let err = validate_stdlib_entry("len", &entry)
            .expect_err("percent-prefixed stdlib usage should fail");

        assert!(err
            .to_string()
            .contains("uses invalid `%`-prefixed signature notation"));
    }

    #[test]
    fn stdlib_validation_rejects_heading_signature_mismatches() {
        let entry = StdlibDoc {
            category: "Collections".into(),
            call_form: "[augment-self: dst-map; src-map]".into(),
            summary: "Prints the result.".into(),
            location: "stdlib/collections.md#augment-thru".into(),
        };

        let err = validate_stdlib_entry("augment-thru", &entry)
            .expect_err("mismatched stdlib usage should fail");

        assert!(err
            .to_string()
            .contains("has mismatched usage signature"));
    }
}
