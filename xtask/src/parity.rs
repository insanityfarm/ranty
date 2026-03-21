use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{bail, Context, Result};
use glob::glob;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use walkdir::WalkDir;

const PARITY_ROOT: &str = "parity/ranty-js";
const CONTRACT_PATH: &str = "parity/ranty-js/contract.json";
const COMPONENTS_PATH: &str = "parity/ranty-js/components.yaml";
const OUTPUT_TESTS_ROOT: &str = "parity/ranty-js/tests";
const OUTPUT_CORPUS_ROOT: &str = "parity/ranty-js/tests/corpus";
const OUTPUT_SOURCES_ROOT: &str = "parity/ranty-js/tests/sources";
const SOURCE_CORPUS_ROOT: &str = "tests/corpus";
const SOURCE_SOURCES_ROOT: &str = "tests/sources";

pub fn parity_build() -> Result<()> {
    let repo = super::repo_root();
    let source = load_components(&repo)?;
    let generated = generate_contract(&repo, &source)?;
    write_outputs(&repo, &generated)?;
    Ok(())
}

pub fn parity_verify() -> Result<()> {
    let repo = super::repo_root();
    let source = load_components(&repo)?;
    let generated = generate_contract(&repo, &source)?;
    verify_outputs(&repo, &generated)
}

#[derive(Debug, Deserialize)]
struct ComponentSource {
    version: u32,
    components: Vec<ComponentDefinition>,
}

#[derive(Debug, Deserialize)]
struct ComponentDefinition {
    id: String,
    title: String,
    summary: String,
    scope: String,
    rust_paths: Vec<String>,
    js_targets: Vec<String>,
    depends_on: Vec<String>,
}

#[derive(Debug, Serialize)]
struct ContractDocument {
    schema_version: u32,
    generated_by: &'static str,
    source_repo: &'static str,
    source_commit: String,
    build_version: String,
    language_version: String,
    contract_root: &'static str,
    concepts: Vec<ConceptEntry>,
    module_resolution: ModuleResolutionContract,
    supported_file_extensions: Vec<String>,
    stdlib_symbols: Vec<String>,
    components: Vec<ComponentContract>,
}

#[derive(Debug, Serialize)]
struct ConceptEntry {
    name: &'static str,
    kind: &'static str,
    summary: &'static str,
}

#[derive(Debug, Serialize)]
struct ModuleResolutionContract {
    resolver_type: &'static str,
    cache_global: &'static str,
    environment_variables: Vec<&'static str>,
    search_order: Vec<&'static str>,
}

#[derive(Debug, Serialize)]
struct ComponentContract {
    id: String,
    title: String,
    summary: String,
    scope: String,
    depends_on: Vec<String>,
    rust_paths: Vec<String>,
    matched_files: Vec<String>,
    js_targets: Vec<String>,
    signature: String,
}

#[derive(Debug)]
struct GeneratedContract {
    contract_json: String,
    copied_files: BTreeMap<String, Vec<u8>>,
}

fn load_components(repo: &Path) -> Result<ComponentSource> {
    let source = fs::read_to_string(repo.join(COMPONENTS_PATH))
        .with_context(|| format!("failed to read {}", repo.join(COMPONENTS_PATH).display()))?;
    let parsed: ComponentSource =
        serde_yaml::from_str(&source).context("failed to parse parity component source")?;
    validate_components(&parsed)?;
    Ok(parsed)
}

fn validate_components(source: &ComponentSource) -> Result<()> {
    if source.version != 1 {
        bail!(
            "unsupported parity component source version {}; expected 1",
            source.version
        );
    }

    let mut ids = HashSet::new();
    for component in &source.components {
        if component.id.trim().is_empty() {
            bail!("parity component id cannot be empty");
        }
        if !ids.insert(component.id.clone()) {
            bail!("duplicate parity component id `{}`", component.id);
        }
        if component.rust_paths.is_empty() {
            bail!("parity component `{}` must define rust_paths", component.id);
        }
        if component.js_targets.is_empty() {
            bail!("parity component `{}` must define js_targets", component.id);
        }
    }

    for component in &source.components {
        for dependency in &component.depends_on {
            if !ids.contains(dependency) {
                bail!(
                    "parity component `{}` references unknown dependency `{}`",
                    component.id,
                    dependency
                );
            }
        }
    }

    Ok(())
}

fn generate_contract(repo: &Path, source: &ComponentSource) -> Result<GeneratedContract> {
    let mut covered_files = BTreeSet::new();
    let mut components = Vec::with_capacity(source.components.len());

    for component in &source.components {
        let matched_files = resolve_component_files(repo, component)?;
        if matched_files.is_empty() {
            bail!(
                "parity component `{}` did not match any Rust files",
                component.id
            );
        }
        for path in &matched_files {
            covered_files.insert(path.clone());
        }

        components.push(ComponentContract {
            id: component.id.clone(),
            title: component.title.clone(),
            summary: component.summary.clone(),
            scope: component.scope.clone(),
            depends_on: component.depends_on.clone(),
            rust_paths: component.rust_paths.clone(),
            matched_files: matched_files.clone(),
            js_targets: component.js_targets.clone(),
            signature: signature_for_files(repo, &matched_files)?,
        });
    }

    let expected_covered = collect_parity_surface_files(repo)?;
    let missing: Vec<_> = expected_covered
        .difference(&covered_files)
        .cloned()
        .collect();
    if !missing.is_empty() {
        bail!(
            "parity component coverage is incomplete:\n{}",
            missing
                .into_iter()
                .map(|path| format!("- {path}"))
                .collect::<Vec<_>>()
                .join("\n")
        );
    }

    let contract = ContractDocument {
        schema_version: 1,
        generated_by: "cargo xtask parity build",
        source_repo: "https://github.com/insanityfarm/ranty",
        source_commit: git_head_commit(repo)?,
        build_version: ranty::BUILD_VERSION.to_owned(),
        language_version: ranty::RANTY_LANG_VERSION.to_owned(),
        contract_root: PARITY_ROOT,
        concepts: vec![
            ConceptEntry {
                name: "Ranty",
                kind: "context",
                summary: "Execution context that owns options, globals, RNG, module resolution, and program execution.",
            },
            ConceptEntry {
                name: "RantyOptions",
                kind: "configuration",
                summary: "Constructor and runtime options for stdlib use, debug mode, top-level globals, seed, and allocation threshold.",
            },
            ConceptEntry {
                name: "RantyProgram",
                kind: "compiled-program",
                summary: "Compiled program root plus identifying metadata such as source path and display name.",
            },
            ConceptEntry {
                name: "ModuleResolver",
                kind: "integration-contract",
                summary: "Host-provided module resolution contract used by @require and module caching.",
            },
            ConceptEntry {
                name: "DataSource",
                kind: "integration-contract",
                summary: "Host-provided data source contract used by stdlib data-source functions.",
            },
        ],
        module_resolution: ModuleResolutionContract {
            resolver_type: "DefaultModuleResolver",
            cache_global: "__MODULES",
            environment_variables: vec!["RANTY_MODULES_PATH"],
            search_order: vec![
                "dependant program directory",
                "local modules path or current working directory",
                "RANTY_MODULES_PATH when enabled",
            ],
        },
        supported_file_extensions: ranty::RANTY_SUPPORTED_FILE_EXTENSIONS
            .iter()
            .map(|value| value.to_string())
            .collect(),
        stdlib_symbols: super::exported_symbols(),
        components,
    };

    let contract_json =
        serde_json::to_string_pretty(&contract).context("failed to serialize contract json")?;
    let copied_files = collect_synced_test_files(repo)?;

    Ok(GeneratedContract {
        contract_json: format!("{contract_json}\n"),
        copied_files,
    })
}

fn resolve_component_files(repo: &Path, component: &ComponentDefinition) -> Result<Vec<String>> {
    let mut matches = BTreeSet::new();

    for pattern in &component.rust_paths {
        let absolute_pattern = repo.join(pattern);
        let absolute_pattern = absolute_pattern
            .to_string_lossy()
            .replace('\\', "/");
        for entry in glob(&absolute_pattern)
            .with_context(|| format!("invalid parity glob pattern `{pattern}`"))?
        {
            let path = entry.with_context(|| format!("failed to expand parity glob `{pattern}`"))?;
            if path.is_dir() {
                continue;
            }
            let relative_path = path
                .strip_prefix(repo)
                .with_context(|| format!("path {} escaped repo root", path.display()))?;
            if path_has_hidden_component(relative_path) {
                continue;
            }
            let relative = relative_path
                .to_string_lossy()
                .replace('\\', "/");
            matches.insert(relative);
        }
    }

    Ok(matches.into_iter().collect())
}

fn signature_for_files(repo: &Path, files: &[String]) -> Result<String> {
    let mut hasher = Sha256::new();
    for relative in files {
        hasher.update(relative.as_bytes());
        hasher.update([0]);
        hasher.update(
            fs::read(repo.join(relative))
                .with_context(|| format!("failed to read {}", repo.join(relative).display()))?,
        );
        hasher.update([0xff]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

fn path_has_hidden_component(path: &Path) -> bool {
    path.components().any(|component| {
        component
            .as_os_str()
            .to_str()
            .is_some_and(|name| name.starts_with('.'))
    })
}

fn path_is_hidden_within(root: &Path, path: &Path) -> bool {
    path.strip_prefix(root)
        .ok()
        .is_some_and(path_has_hidden_component)
}

fn collect_parity_surface_files(repo: &Path) -> Result<BTreeSet<String>> {
    let mut files = BTreeSet::new();
    for root in ["src", SOURCE_CORPUS_ROOT, SOURCE_SOURCES_ROOT] {
        let absolute = repo.join(root);
        if !absolute.exists() {
            continue;
        }
        for entry in WalkDir::new(&absolute)
            .into_iter()
            .filter_entry(|entry| !path_is_hidden_within(&absolute, entry.path()))
            .filter_map(Result::ok)
        {
            if !entry.file_type().is_file() {
                continue;
            }
            if root == "src" && entry.path().extension() != Some(OsStr::new("rs")) {
                continue;
            }
            let relative = entry
                .path()
                .strip_prefix(repo)
                .with_context(|| format!("path {} escaped repo root", entry.path().display()))?
                .to_string_lossy()
                .replace('\\', "/");
            files.insert(relative);
        }
    }
    Ok(files)
}

fn collect_synced_test_files(repo: &Path) -> Result<BTreeMap<String, Vec<u8>>> {
    let mut files = BTreeMap::new();

    for (source_root, output_root) in [
        (SOURCE_CORPUS_ROOT, OUTPUT_CORPUS_ROOT),
        (SOURCE_SOURCES_ROOT, OUTPUT_SOURCES_ROOT),
    ] {
        let absolute_root = repo.join(source_root);
        if !absolute_root.exists() {
            continue;
        }

        for entry in WalkDir::new(&absolute_root)
            .into_iter()
            .filter_entry(|entry| !path_is_hidden_within(&absolute_root, entry.path()))
            .filter_map(Result::ok)
        {
            if !entry.file_type().is_file() {
                continue;
            }
            let relative_within_root = entry
                .path()
                .strip_prefix(&absolute_root)
                .with_context(|| format!("path {} escaped source root", entry.path().display()))?;
            let output_relative = PathBuf::from(output_root)
                .join(relative_within_root)
                .to_string_lossy()
                .replace('\\', "/");
            files.insert(
                output_relative,
                fs::read(entry.path())
                    .with_context(|| format!("failed to read {}", entry.path().display()))?,
            );
        }
    }

    Ok(files)
}

fn write_outputs(repo: &Path, generated: &GeneratedContract) -> Result<()> {
    let contract_path = repo.join(CONTRACT_PATH);
    if let Some(parent) = contract_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    fs::write(&contract_path, &generated.contract_json)
        .with_context(|| format!("failed to write {}", contract_path.display()))?;

    let tests_root = repo.join(OUTPUT_TESTS_ROOT);
    if tests_root.exists() {
        fs::remove_dir_all(&tests_root)
            .with_context(|| format!("failed to clear {}", tests_root.display()))?;
    }

    for (relative, contents) in &generated.copied_files {
        let target = repo.join(relative);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create {}", parent.display()))?;
        }
        fs::write(&target, contents)
            .with_context(|| format!("failed to write {}", target.display()))?;
    }

    Ok(())
}

fn verify_outputs(repo: &Path, generated: &GeneratedContract) -> Result<()> {
    let contract_path = repo.join(CONTRACT_PATH);
    let current_contract = fs::read_to_string(&contract_path)
        .with_context(|| format!("failed to read {}", contract_path.display()))?;
    if normalized_contract_for_verify(&current_contract)?
        != normalized_contract_for_verify(&generated.contract_json)?
    {
        bail!(
            "parity contract is out of date; run `cargo xtask parity build` and commit {}",
            CONTRACT_PATH
        );
    }

    let expected_files: BTreeSet<_> = generated.copied_files.keys().cloned().collect();
    let mut actual_files = BTreeSet::new();
    let actual_root = repo.join(OUTPUT_TESTS_ROOT);
    if actual_root.exists() {
        for entry in WalkDir::new(&actual_root)
            .into_iter()
            .filter_entry(|entry| !path_is_hidden_within(&actual_root, entry.path()))
            .filter_map(Result::ok)
        {
            if !entry.file_type().is_file() {
                continue;
            }
            actual_files.insert(
                entry.path()
                    .strip_prefix(repo)
                    .with_context(|| format!("path {} escaped repo root", entry.path().display()))?
                    .to_string_lossy()
                    .replace('\\', "/"),
            );
        }
    }

    let missing: Vec<_> = expected_files.difference(&actual_files).cloned().collect();
    let extra: Vec<_> = actual_files.difference(&expected_files).cloned().collect();
    if !missing.is_empty() || !extra.is_empty() {
        let mut messages = vec![];
        if !missing.is_empty() {
            messages.push(format!(
                "missing parity synced files:\n{}",
                missing
                    .into_iter()
                    .map(|path| format!("- {path}"))
                    .collect::<Vec<_>>()
                    .join("\n")
            ));
        }
        if !extra.is_empty() {
            messages.push(format!(
                "unexpected parity synced files:\n{}",
                extra
                    .into_iter()
                    .map(|path| format!("- {path}"))
                    .collect::<Vec<_>>()
                    .join("\n")
            ));
        }
        bail!(
            "{}\nrun `cargo xtask parity build` and commit {}",
            messages.join("\n"),
            PARITY_ROOT
        );
    }

    for (relative, expected_contents) in &generated.copied_files {
        let current = fs::read(repo.join(relative))
            .with_context(|| format!("failed to read {}", repo.join(relative).display()))?;
        if current != *expected_contents {
            bail!(
                "parity synced file `{relative}` is out of date; run `cargo xtask parity build`"
            );
        }
    }

    Ok(())
}

fn normalized_contract_for_verify(json: &str) -> Result<serde_json::Value> {
    let mut value: serde_json::Value =
        serde_json::from_str(json).context("failed to parse parity contract json")?;
    value
        .as_object_mut()
        .context("parity contract root must be a JSON object")?
        .remove("source_commit");
    Ok(value)
}

fn git_head_commit(repo: &Path) -> Result<String> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .current_dir(repo)
        .output()
        .context("failed to run git rev-parse HEAD")?;
    if !output.status.success() {
        bail!("git rev-parse HEAD failed");
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

#[cfg(test)]
mod tests {
    use super::{normalized_contract_for_verify, path_has_hidden_component};
    use std::path::Path;

    #[test]
    fn hidden_components_are_detected_anywhere_in_the_path() {
        assert!(path_has_hidden_component(Path::new(".DS_Store")));
        assert!(path_has_hidden_component(Path::new("tests/sources/.DS_Store")));
        assert!(path_has_hidden_component(Path::new("src/.cache/file.rs")));
        assert!(!path_has_hidden_component(Path::new("tests/sources/access/add_assign.ranty")));
    }

    #[test]
    fn parity_verify_ignores_source_commit_metadata() {
        let current = r#"{"schema_version":1,"source_commit":"aaa","components":[]}"#;
        let generated = r#"{"schema_version":1,"source_commit":"bbb","components":[]}"#;

        assert_eq!(
            normalized_contract_for_verify(current).unwrap(),
            normalized_contract_for_verify(generated).unwrap()
        );
    }
}
