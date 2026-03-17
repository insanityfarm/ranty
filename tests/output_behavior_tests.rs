mod common;

use common::{compile_with_reporter, run_str};

#[test]
fn edit_can_duplicate_parent_output() {
    assert_eq!(
        run_str(r#""example" { @edit x: `<x> `<x> }"#),
        "example example"
    );
}

#[test]
fn edit_preserves_explicit_trailing_whitespace() {
    assert_eq!(run_str(r#""example " { @edit x: `<x> }"#), "example ");
}

#[test]
fn edit_can_discard_parent_output() {
    assert_eq!(
        run_str(r#""example" { @edit: "overwritten" }"#),
        "overwritten"
    );
}

#[test]
fn edit_supports_accumulating_values_across_repeats() {
    assert_eq!(
        run_str(
            r#"
[%factorial: n] {
  1 [rep: <n>] {@edit x: <x> * [step]}
}

[factorial: 6]
"#,
        ),
        "720"
    );
}

#[test]
fn edit_must_start_a_block_element() {
    let (result, messages) = compile_with_reporter(r#"{foo @edit x: <x>}"#);
    assert!(result.is_err(), "misplaced @edit should fail to compile");
    assert!(
        !messages.is_empty(),
        "misplaced @edit should emit a diagnostic"
    );
}

#[test]
fn hint_must_target_a_supported_expression_unit() {
    let (result, messages) = compile_with_reporter("`@break");
    assert!(result.is_err(), "misplaced hint should fail to compile");
    assert!(messages.iter().any(|msg| msg.code() == "R0131"));
}

#[test]
fn sink_must_target_a_supported_expression_unit() {
    let (result, messages) = compile_with_reporter("~@break");
    assert!(result.is_err(), "misplaced sink should fail to compile");
    assert!(messages.iter().any(|msg| msg.code() == "R0130"));
}

#[test]
fn whitespace_is_normalized_on_the_same_line() {
    assert_eq!(run_str("One  two   three"), "One two three");
}

#[test]
fn line_breaks_between_fragments_are_ignored() {
    assert_eq!(run_str("Water\nmelon"), "Watermelon");
}

#[test]
fn hinting_preserves_whitespace_around_expression_units() {
    assert_eq!(
        run_str(r#"<$name = "world">Hello, `<name>!"#),
        "Hello, world!"
    );
}

#[test]
fn sinking_removes_whitespace_between_adjacent_text_units() {
    assert_eq!(
        run_str(r#"{\:} ~{\(}"#),
        ":(",
        "sinking should keep adjacent text compact"
    );
}
