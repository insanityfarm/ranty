mod common;

use assert_matches::assert_matches;
use common::{compile_with_reporter, run, run_str};
use ranty::runtime::RuntimeErrorType;

#[test]
fn match_selector_runs_matching_tagged_branches() {
    assert_eq!(
        run_str(r#"[match: foo]{yes @on foo|no @on bar|fallback}"#),
        "yes"
    );
}

#[test]
fn match_selector_uses_untagged_branches_as_default_pool() {
    assert_eq!(run_str(r#"[match: baz]{yes @on foo|fallback}"#), "fallback");
}

#[test]
fn match_selector_uses_existing_value_equality_rules() {
    assert_eq!(run_str(r#"[match: 1]{float @on 1.0|fallback}"#), "float");
}

#[test]
fn match_selector_applies_weights_within_the_matching_pool() {
    assert_eq!(
        run_str(r#"[match: foo]{skip @on foo @weight 0|pick @on foo @weight 1|fallback}"#),
        "pick"
    );
}

#[test]
fn match_selector_errors_when_no_candidate_branch_exists() {
    let err = run(r#"[match: foo]{bar @on bar}"#)
        .expect_err("missing match candidates should fail at runtime");
    assert_matches!(err.error_type, RuntimeErrorType::SelectorError(_));
    assert_eq!(
        err.to_string(),
        "[SELECTOR_ERROR] match selector could not find a selectable branch"
    );
}

#[test]
fn match_selectors_reject_cursor_operations() {
    for source in [
        r#"<$sel=[mksel: match; foo]>[sel-skip:<sel>]"#,
        r#"<$sel=[mksel: match; foo]>[sel-freeze:<sel>]"#,
        r#"<$sel=[mksel: match; foo]>[sel-frozen:<sel>]"#,
    ] {
        let err = run(source).expect_err("cursor operations on match selectors should fail");
        assert_matches!(err.error_type, RuntimeErrorType::SelectorError(_));
    }
}

#[test]
fn misplaced_or_duplicate_on_metadata_is_rejected() {
    let (result, messages) = compile_with_reporter("@on foo");
    assert!(result.is_err(), "@on outside a block element should fail");
    assert!(messages.iter().any(|msg| msg.code() == "R0207"));

    let (result, messages) = compile_with_reporter(r#"{foo @on a @on b}"#);
    assert!(result.is_err(), "duplicate @on metadata should fail");
    assert!(messages.iter().any(|msg| msg.code() == "R0041"));
}
