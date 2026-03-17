mod common;

use std::path::{Path, PathBuf};

use common::{compile_with_reporter, EnvVarGuard, TempWorkspace};
use ranty::{DefaultModuleResolver, Ranty, RantyOptions};

fn compile_and_run_file(path: &Path, ranty: &mut Ranty) -> Result<String, String> {
    let program = ranty
        .compile_file_quiet(path)
        .map_err(|err| format!("compile failed: {err}"))?;
    ranty.run(&program)
        .map(|value| value.to_string())
        .map_err(|err| err.to_string())
}

#[test]
fn require_resolves_relative_paths_without_extensions() {
    let workspace = TempWorkspace::new();
    let root = workspace.write(
        "main.ranty",
        r#"
@require "modules/seq"
[seq/value]
"#,
    );
    workspace.write(
        "modules/seq.ranty",
        r#"
<%module = (::)>
[$module/value] {
  42
}
<module>
"#,
    );

    let mut ranty = Ranty::with_seed(1);
    let output = compile_and_run_file(&root, &mut ranty).expect("module load should succeed");
    assert_eq!(output, "42");
}

#[test]
fn require_reports_missing_modules() {
    let workspace = TempWorkspace::new();
    let root = workspace.write("main.ranty", r#"@require "missing/module""#);
    let mut ranty = Ranty::new();
    let err = compile_and_run_file(&root, &mut ranty).expect_err("missing module should fail");
    assert!(err.contains("[MODULE_ERROR]"));
    assert!(err.contains("module 'missing/module' not found"));
}

#[test]
fn require_prefers_ranty_when_both_extensions_exist() {
    let workspace = TempWorkspace::new();
    let root = workspace.write(
        "main.ranty",
        r#"
@require "mods/shared"
[shared/value]
"#,
    );
    workspace.write(
        "mods/shared.ranty",
        r#"
<%module = (::)>
[$module/value] { modern }
<module>
"#,
    );
    workspace.write(
        "mods/shared.rant",
        r#"
<%module = (::)>
[$module/value] { legacy }
<module>
"#,
    );

    let mut ranty = Ranty::new();
    let output = compile_and_run_file(&root, &mut ranty).expect("module load should succeed");
    assert_eq!(output, "modern");
}

#[test]
fn require_can_load_legacy_rant_when_no_ranty_exists() {
    let workspace = TempWorkspace::new();
    let root = workspace.write(
        "main.ranty",
        r#"
@require "legacy"
[legacy/value]
"#,
    );
    workspace.write(
        "legacy.rant",
        r#"
<%module = (::)>
[$module/value] { legacy-only }
<module>
"#,
    );

    let mut ranty = Ranty::new();
    let output = compile_and_run_file(&root, &mut ranty).expect("legacy module should load");
    assert_eq!(output, "legacy-only");
}

#[test]
fn require_can_mix_explicit_ranty_and_legacy_rant_paths() {
    let workspace = TempWorkspace::new();
    let root = workspace.write(
        "main.ranty",
        r#"
@require modern: "mods/shared.ranty"
@require legacy: "mods/shared.rant"
[modern/value][legacy/value]
"#,
    );
    workspace.write(
        "mods/shared.ranty",
        r#"
<%module = (::)>
[$module/value] { modern }
<module>
"#,
    );
    workspace.write(
        "mods/shared.rant",
        r#"
<%module = (::)>
[$module/value] { legacy }
<module>
"#,
    );

    let mut ranty = Ranty::new();
    let output = compile_and_run_file(&root, &mut ranty).expect("mixed module load should work");
    assert_eq!(output, "modernlegacy");
}

#[test]
fn require_can_load_tracked_legacy_rant_fixtures() {
    let fixture_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/sources/compat");
    let entry = fixture_root.join("module_entry.rant");

    let mut ranty = Ranty::new();
    let output =
        compile_and_run_file(&entry, &mut ranty).expect("tracked legacy fixtures should load");
    assert_eq!(output, "legacy fixture:tracked-legacy-module");
}

#[test]
fn require_reports_module_compile_failures() {
    let workspace = TempWorkspace::new();
    let root = workspace.write("main.ranty", r#"@require "broken""#);
    workspace.write("broken.ranty", "{");

    let mut ranty = Ranty::new();
    let err = compile_and_run_file(&root, &mut ranty).expect_err("broken module should fail");
    assert!(err.contains("[MODULE_ERROR]"));
    assert!(err.contains("failed to compile"));
}

#[test]
fn require_propagates_module_runtime_failures() {
    let workspace = TempWorkspace::new();
    let root = workspace.write("main.ranty", r#"@require "broken""#);
    workspace.write("broken.ranty", r#"[error: "boom"]"#);

    let mut ranty = Ranty::new();
    let err =
        compile_and_run_file(&root, &mut ranty).expect_err("module runtime error should surface");
    assert!(err.contains("[USER_ERROR]"));
    assert!(err.contains("boom"));
}

#[test]
fn require_returns_cached_modules_for_repeated_imports() {
    let workspace = TempWorkspace::new();
    let root = workspace.write(
        "main.ranty",
        r#"
@require "./mods/randomized"
@require again: "mods/randomized"
[assert-eq: [randomized/value]; [again/value]]
[randomized/value]
"#,
    );
    workspace.write(
        "mods/randomized.ranty",
        r#"
<%module = (::)>
<$value = [rand: 1; 100]>
[$module/value] {
  <value>
}
<module>
"#,
    );

    let mut ranty = Ranty::with_seed(7);
    let output = compile_and_run_file(&root, &mut ranty).expect("module load should succeed");
    assert!(
        !output.is_empty(),
        "cached module should still be usable after re-import"
    );
}

#[test]
fn require_detects_cyclic_imports() {
    let workspace = TempWorkspace::new();
    let root = workspace.write("main.ranty", r#"@require "a""#);
    workspace.write("a.ranty", "<%module = (::)> @require \"b\" <module>");
    workspace.write("b.ranty", "<%module = (::)> @require \"a\" <module>");

    let mut ranty = Ranty::new();
    let err = compile_and_run_file(&root, &mut ranty).expect_err("cyclic imports should fail");
    assert!(err.contains("[MODULE_ERROR]"));
    assert!(err.contains("cyclic module import detected"));
}

#[test]
fn require_can_use_global_module_path() {
    let workspace = TempWorkspace::new();
    let global_modules = workspace.path().join("global-modules");
    let _guard = EnvVarGuard::set(DefaultModuleResolver::ENV_MODULES_PATH_KEY, &global_modules);
    workspace.write(
        "global-modules/shared.ranty",
        r#"
<%module = (::)>
[$module/value] {
  from-global
}
<module>
"#,
    );

    let mut ranty = Ranty::with_options(RantyOptions {
        debug_mode: true,
        ..Default::default()
    })
    .using_module_resolver(DefaultModuleResolver {
        enable_global_modules: true,
        local_modules_path: Some(
            workspace
                .path()
                .join("local")
                .to_string_lossy()
                .into_owned(),
        ),
    });

    let program = ranty
        .compile_quiet(r#"@require "shared" [shared/value]"#)
        .expect("failed to compile program");
    let output = ranty.run(&program).expect("global module should resolve");
    assert_eq!(output.to_string(), "from-global");
}

#[test]
fn invalid_require_argument_emits_stable_diagnostic() {
    let (result, messages) = compile_with_reporter("@require alias: 42");
    assert!(result.is_err());
    assert_eq!(messages[0].code(), "R0203");
    assert_eq!(
        messages[0].message(),
        "@require path should be a string literal"
    );
}
