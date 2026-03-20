#![cfg(feature = "cli")]

mod common;

use common::{assert_cli_case, cli_case_named, load_cli_corpus};

fn assert_named_case(name: &str) {
    let corpus = load_cli_corpus();
    let case = cli_case_named(&corpus, name);
    assert_cli_case(case);
}

#[test]
fn cli_help_mentions_ranty_and_supported_flags() {
    assert_named_case("cli_help_mentions_ranty_and_supported_flags");
}

#[test]
fn cli_version_reports_the_current_build_version() {
    assert_named_case("cli_version_reports_the_current_build_version");
}

#[test]
fn cli_runs_a_file() {
    assert_named_case("cli_runs_a_file");
}

#[test]
fn eval_takes_precedence_over_file_input() {
    assert_named_case("eval_takes_precedence_over_file_input");
}

#[test]
fn cli_runs_piped_stdin_when_no_other_source_is_selected() {
    assert_named_case("cli_runs_piped_stdin_when_no_other_source_is_selected");
}

#[test]
fn cli_uses_deterministic_hex_seeds() {
    assert_named_case("cli_uses_deterministic_hex_seeds");
}

#[test]
fn cli_rejects_invalid_seed_values() {
    assert_named_case("cli_rejects_invalid_seed_values");
}

#[test]
fn cli_can_suppress_warnings() {
    assert_named_case("cli_can_suppress_warnings");
}

#[test]
fn cli_accepts_no_debug_flag() {
    assert_named_case("cli_accepts_no_debug_flag");
}

#[test]
fn cli_bench_mode_reports_compile_and_execution_timing() {
    assert_named_case("cli_bench_mode_reports_compile_and_execution_timing");
}

#[test]
fn cli_returns_dataerr_for_compile_failures() {
    assert_named_case("cli_returns_dataerr_for_compile_failures");
}

#[test]
fn cli_returns_software_for_runtime_failures() {
    assert_named_case("cli_returns_software_for_runtime_failures");
}
