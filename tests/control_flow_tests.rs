mod common;

use assert_matches::assert_matches;
use common::{run, run_str};
use rant::runtime::RuntimeErrorType;

#[test]
fn continue_requires_a_repeater() {
    let err = run("@continue").expect_err("continue outside repeater should fail");
    assert_matches!(err.error_type, RuntimeErrorType::ControlFlowError);
    assert_eq!(
        err.to_string(),
        "[CONTROL_FLOW_ERROR] no reachable repeater to interrupt"
    );
}

#[test]
fn break_requires_a_repeater() {
    let err = run("@break").expect_err("break outside repeater should fail");
    assert_matches!(err.error_type, RuntimeErrorType::ControlFlowError);
    assert_eq!(
        err.to_string(),
        "[CONTROL_FLOW_ERROR] no reachable repeater to interrupt"
    );
}

#[test]
fn continue_can_cross_nested_blocks() {
    assert_eq!(
        run_str(r#"[rep:3]{start { @continue next } end}"#),
        "nextnextnext"
    );
}

#[test]
fn break_can_cross_nested_blocks() {
    assert_eq!(run_str(r#"[rep:3]{start { @break stop } end}"#), "stop");
}

#[test]
fn continue_does_not_cross_function_boundaries() {
    let err = run(r#"
[$skip] { @continue skip }
[rep:3]{[skip]}
"#)
    .expect_err("continue inside function should not escape to repeater");

    assert_matches!(err.error_type, RuntimeErrorType::ControlFlowError);
    assert_eq!(
        err.to_string(),
        "[CONTROL_FLOW_ERROR] no reachable repeater to interrupt"
    );
}

#[test]
fn break_does_not_cross_function_boundaries() {
    let err = run(r#"
[$stop] { @break stop }
[rep:3]{[stop]}
"#)
    .expect_err("break inside function should not escape to repeater");

    assert_matches!(err.error_type, RuntimeErrorType::ControlFlowError);
    assert_eq!(
        err.to_string(),
        "[CONTROL_FLOW_ERROR] no reachable repeater to interrupt"
    );
}

#[test]
fn return_inside_function_called_from_repeater_returns_function_value_only() {
    assert_eq!(
        run_str(
            r#"
[$emit] {
  @return done
}
[rep:3]{[emit]}
"#,
        ),
        "donedonedone"
    );
}
