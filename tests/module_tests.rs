mod common;

use std::path::Path;

use common::{compile_with_reporter, EnvVarGuard, TempWorkspace};
use rant::{DefaultModuleResolver, Rant, RantOptions};

fn compile_and_run_file(path: &Path, rant: &mut Rant) -> Result<String, String> {
  let program = rant
    .compile_file_quiet(path)
    .map_err(|err| format!("compile failed: {err}"))?;
  rant.run(&program)
    .map(|value| value.to_string())
    .map_err(|err| err.to_string())
}

#[test]
fn require_resolves_relative_paths_without_extensions() {
  let workspace = TempWorkspace::new();
  let root = workspace.write(
    "main.rant",
    r#"
@require "modules/seq"
[seq/value]
"#,
  );
  workspace.write(
    "modules/seq.rant",
    r#"
<%module = (::)>
[$module/value] {
  42
}
<module>
"#,
  );

  let mut rant = Rant::with_seed(1);
  let output = compile_and_run_file(&root, &mut rant).expect("module load should succeed");
  assert_eq!(output, "42");
}

#[test]
fn require_reports_missing_modules() {
  let workspace = TempWorkspace::new();
  let root = workspace.write("main.rant", r#"@require "missing/module""#);
  let mut rant = Rant::new();
  let err = compile_and_run_file(&root, &mut rant).expect_err("missing module should fail");
  assert!(err.contains("[MODULE_ERROR]"));
  assert!(err.contains("module 'missing/module' not found"));
}

#[test]
fn require_reports_module_compile_failures() {
  let workspace = TempWorkspace::new();
  let root = workspace.write("main.rant", r#"@require "broken""#);
  workspace.write("broken.rant", "{");

  let mut rant = Rant::new();
  let err = compile_and_run_file(&root, &mut rant).expect_err("broken module should fail");
  assert!(err.contains("[MODULE_ERROR]"));
  assert!(err.contains("failed to compile"));
}

#[test]
fn require_propagates_module_runtime_failures() {
  let workspace = TempWorkspace::new();
  let root = workspace.write("main.rant", r#"@require "broken""#);
  workspace.write("broken.rant", r#"[error: "boom"]"#);

  let mut rant = Rant::new();
  let err = compile_and_run_file(&root, &mut rant).expect_err("module runtime error should surface");
  assert!(err.contains("[USER_ERROR]"));
  assert!(err.contains("boom"));
}

#[test]
fn require_returns_cached_modules_for_repeated_imports() {
  let workspace = TempWorkspace::new();
  let root = workspace.write(
    "main.rant",
    r#"
@require "./mods/randomized"
@require again: "mods/randomized"
[assert-eq: [randomized/value]; [again/value]]
[randomized/value]
"#,
  );
  workspace.write(
    "mods/randomized.rant",
    r#"
<%module = (::)>
<$value = [rand: 1; 100]>
[$module/value] {
  <value>
}
<module>
"#,
  );

  let mut rant = Rant::with_seed(7);
  let output = compile_and_run_file(&root, &mut rant).expect("module load should succeed");
  assert!(!output.is_empty(), "cached module should still be usable after re-import");
}

#[test]
fn require_detects_cyclic_imports() {
  let workspace = TempWorkspace::new();
  let root = workspace.write("main.rant", r#"@require "a""#);
  workspace.write("a.rant", "<%module = (::)> @require \"b\" <module>");
  workspace.write("b.rant", "<%module = (::)> @require \"a\" <module>");

  let mut rant = Rant::new();
  let err = compile_and_run_file(&root, &mut rant).expect_err("cyclic imports should fail");
  assert!(err.contains("[MODULE_ERROR]"));
  assert!(err.contains("cyclic module import detected"));
}

#[test]
fn require_can_use_global_module_path() {
  let workspace = TempWorkspace::new();
  let global_modules = workspace.path().join("global-modules");
  let _guard = EnvVarGuard::set(DefaultModuleResolver::ENV_MODULES_PATH_KEY, &global_modules);
  workspace.write(
    "global-modules/shared.rant",
    r#"
<%module = (::)>
[$module/value] {
  from-global
}
<module>
"#,
  );

  let mut rant = Rant::with_options(RantOptions {
    debug_mode: true,
    ..Default::default()
  })
  .using_module_resolver(DefaultModuleResolver {
    enable_global_modules: true,
    local_modules_path: Some(workspace.path().join("local").to_string_lossy().into_owned()),
  });

  let program = rant
    .compile_quiet(r#"@require "shared" [shared/value]"#)
    .expect("failed to compile program");
  let output = rant.run(&program).expect("global module should resolve");
  assert_eq!(output.to_string(), "from-global");
}

#[test]
fn invalid_require_argument_emits_stable_diagnostic() {
  let (result, messages) = compile_with_reporter("@require alias: 42");
  assert!(result.is_err());
  assert_eq!(messages[0].code(), "R0203");
  assert_eq!(messages[0].message(), "@require path should be a string literal");
}
