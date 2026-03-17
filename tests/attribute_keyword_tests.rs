mod common;

use common::{compile_with_reporter, run_str};

#[test]
fn attribute_accessors_can_round_trip_mutable_attribute_state() {
    assert_eq!(run_str(r#"<@rep = 3>[rep:<@rep>]{x}"#), "xxx");
    assert_eq!(run_str(r#"<@sep = ",">[rep:3][sep:<@sep>]{x}"#), "x,x,x");
    assert_eq!(
        run_str(r#"<@sel = "forward">[rep:4][sel:<@sel>]{a|b}"#),
        "abab"
    );
    assert_eq!(
        run_str(
            r#"
<$mutator = [?: elem] { [elem]! }>
<@sel = "forward">
<@mut = <mutator>>
[rep: all][sel:<@sel>][mut:<@mut>]{a|b}
"#,
        ),
        "a!b!"
    );
}

#[test]
fn attribute_keywords_are_readable_as_plain_expressions() {
    assert_eq!(run_str(r#"[rep:3]{[eq: @step; 0] @break}"#), "@true");
    assert_eq!(run_str(r#"[rep:3]{[neq: @step; [step]] @break}"#), "@true");
    assert_eq!(run_str(r#"[rep:3]{[eq: @total; 3] @break}"#), "@true");
    assert_eq!(
        run_str(r#"[rep: forever]{[eq: @total; <>] @break}"#),
        "@true"
    );
}

#[test]
fn attribute_keyword_block_sugar_applies_to_immediate_blocks() {
    assert_eq!(run_str(r#"@rep 3: {x}"#), "xxx");
    assert_eq!(run_str(r#"@sel "forward": {a|b}"#), "a");
    assert_eq!(run_str(r#"@mut [?: elem] { [elem]! }: {foo}"#), "foo!");
}

#[test]
fn read_only_attribute_keywords_cannot_be_assigned() {
    let (result, messages) = compile_with_reporter("<@step = 1>");
    assert!(result.is_err(), "assigning to @step should fail");
    assert!(messages.iter().any(|msg| msg.code() == "R0206"));

    let (result, messages) = compile_with_reporter("@total 1: {x}");
    assert!(result.is_err(), "block sugar on @total should fail");
    assert!(messages.iter().any(|msg| msg.code() == "R0206"));
}

#[test]
fn attribute_keywords_reject_unsupported_accessor_forms() {
    let (result, messages) = compile_with_reporter("<@rep += 1>");
    assert!(result.is_err(), "compound assignment on @rep should fail");
    assert!(messages.iter().any(|msg| msg.code() == "R0205"));
}
