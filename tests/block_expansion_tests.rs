mod common;

use common::{compile_with_reporter, run_str};

#[test]
fn nested_blocks_expand_into_parent_for_repetition() {
    assert_eq!(run_str(r#"[rep:all][sel:[mksel:forward]]{A{B|C}}"#), "ABAC");
}

#[test]
fn nested_blocks_expand_recursively_left_to_right() {
    assert_eq!(
        run_str(r#"[rep:all][sel:[mksel:forward]]{A{B|C}{1|2}}"#),
        "AB1AB2AC1AC2"
    );
}

#[test]
fn lifted_match_triggers_participate_in_outer_match_selection() {
    assert_eq!(
        run_str(r#"[match: foo]{{yes @on foo|no @on bar}|fallback}"#),
        "yes"
    );
}

#[test]
fn expanded_children_preserve_edit_boundaries() {
    assert_eq!(
        run_str(r#"[rep:all][sel:[mksel:forward]]{"seed"{@edit x: `<x>B|@edit x: `<x>C}}"#),
        "seedBseedC"
    );
}

#[test]
fn protected_blocks_remain_expansion_barriers() {
    assert_eq!(run_str(r#"[rep:all]{A[sel:[mksel:forward]]@{B|C}}"#), "AB");
}

#[test]
fn lifted_weight_conflicts_are_compile_errors() {
    let (result, messages) = compile_with_reporter(r#"{A{B @weight 1|C @weight 2} @weight 3}"#);
    assert!(result.is_err(), "duplicate lifted @weight should fail");
    assert!(messages.iter().any(|msg| msg.code() == "R0041"));
}

#[test]
fn lifted_match_trigger_conflicts_are_compile_errors() {
    let (result, messages) = compile_with_reporter(r#"{A{B @on foo|C @on bar} @on baz}"#);
    assert!(result.is_err(), "duplicate lifted @on should fail");
    assert!(messages.iter().any(|msg| msg.code() == "R0041"));
}
