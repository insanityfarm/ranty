mod common;

use common::run_str;

fn branch(condition: &str) -> String {
    run_str(&format!("@if {condition}: {{truthy}} @else: {{falsy}}"))
}

#[test]
fn truthiness_matches_stable_scalar_rules() {
    assert_eq!(branch(r#""""#), "falsy");
    assert_eq!(branch(r#""text""#), "truthy");
    assert_eq!(branch("0"), "falsy");
    assert_eq!(branch("-1"), "truthy");
    assert_eq!(branch("0.0"), "falsy");
    assert_eq!(branch("0.5"), "truthy");
    assert_eq!(branch("<INFINITY>"), "truthy");
    assert_eq!(branch("<NAN>"), "falsy");
    assert_eq!(branch("<>"), "falsy");
}

#[test]
fn truthiness_matches_collection_rules() {
    assert_eq!(branch("[list]"), "falsy");
    assert_eq!(branch("[list: 1]"), "truthy");
    assert_eq!(branch("[assoc: [list]; [list]]"), "falsy");
    assert_eq!(branch("[assoc: [list: a]; [list: 1]]"), "truthy");
    assert_eq!(branch("[tuple]"), "truthy");
}

#[test]
fn conditional_branches_short_circuit() {
    assert_eq!(
        run_str(r#"@if @true: {pass} @elseif [error: "should not run"]: {fail}"#),
        "pass"
    );
    assert_eq!(
        run_str(r#"@if @false: {[error: "should not run"]} @else: {fallback}"#),
        "fallback"
    );
}

#[test]
fn nested_conditionals_select_only_matching_branch() {
    assert_eq!(
        run_str(
            r#"
@if [list]:
{
  outer-truthy
}
@elseif @true:
{
  @if "":
  {
    bad
  }
  @else:
  {
    nested-fallback
  }
}
@else:
{
  bad
}
"#,
        ),
        "nested-fallback"
    );
}
