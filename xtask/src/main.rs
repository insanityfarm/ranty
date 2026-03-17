use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::ffi::OsStr;
use std::fs;
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
        _ => bail!("usage: cargo xtask docs <build|verify>"),
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
    out.push_str("| Symbol | Category | Call Form | Summary | Canonical Location |\n");
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

fn build_cli_binary(repo: &Path) -> Result<()> {
    run_command(
        repo,
        "cargo",
        &["build", "--quiet", "--features", "cli", "--bin", "ranty"],
    )
}

fn run_cli_eval(cli: &Path, code: &str) -> Result<String> {
    let output = Command::new(cli)
        .arg("--seed")
        .arg("1")
        .arg("--eval")
        .arg(code)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .with_context(|| format!("failed to run {}", cli.display()))?;

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
        if entry.call_form.trim().is_empty() {
            bail!("stdlib symbol `{name}` has an empty call form");
        }
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
