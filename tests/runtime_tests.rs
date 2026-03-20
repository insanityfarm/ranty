/*
  Runtime Tests

  These are tests that verify the runtime (+ stdlib) works as expected.
  It is assumed that all test programs in this file compile successfully.

*/

#[cfg(feature = "cli")]
mod common;

use ranty::*;

use assert_matches::*;

#[cfg(feature = "cli")]
use common::{
    discover_executable_fixture_files, load_fixture_corpus, load_fuzz_corpus, relevant_stderr,
    repo_root, run_cli,
};

#[macro_export]
macro_rules! test_ranty_file {
    ($src_path:literal raises $runtime_err_variant:pat) => {{
        use ranty::runtime::{RuntimeError, RuntimeErrorType::*};
        let mut r = Ranty::with_options(RantyOptions {
            debug_mode: true,
            ..Default::default()
        });
        let pgm = r
            .compile_quiet(include_str!($src_path))
            .expect("failed to compile program");
        assert_matches!(
            r.run(&pgm)
                .map(|output| output.to_string())
                .as_ref()
                .map(|o| o.as_str()),
            Err(RuntimeError {
                error_type: $runtime_err_variant,
                ..
            })
        );
    }};
    ($src_path:literal) => {{
        let mut r = Ranty::with_options(RantyOptions {
            debug_mode: true,
            ..Default::default()
        });
        let pgm = r
            .compile_quiet(include_str!($src_path))
            .expect("failed to compile program");
        assert_matches!(
            r.run(&pgm)
                .map(|output| output.to_string())
                .as_ref()
                .map(|o| o.as_str()),
            Ok(_)
        );
    }};
    ($src_path:literal, $expected:literal) => {{
        let mut r = Ranty::with_options(RantyOptions {
            debug_mode: true,
            ..Default::default()
        });
        let pgm = r
            .compile_quiet(include_str!($src_path))
            .expect("failed to compile program");
        assert_matches!(
            r.run(&pgm)
                .map(|output| output.to_string())
                .as_ref()
                .map(|o| o.as_str()),
            Ok($expected)
        );
    }};
}

macro_rules! test_ranty {
    ($src:literal, $expected:literal) => {{
        let mut r = Ranty::new();
        let pgm = r.compile_quiet($src).expect("failed to compile program");
        assert_matches!(
            r.run(&pgm)
                .map(|output| output.to_string())
                .as_ref()
                .map(|o| o.as_str()),
            Ok($expected)
        );
    }};
}

#[test]
fn empty_program() {
    test_ranty!("", "");
}

#[test]
fn single_fragment() {
    test_ranty!("foo", "foo");
}

#[test]
fn spaced_fragments() {
    test_ranty!("foo bar", "foo bar");
}

#[test]
fn sinked_block() {
    test_ranty!("test ~{test} test", "testtesttest");
}

#[test]
fn single_element_block() {
    test_ranty!("{test}", "test");
}

#[test]
fn repeater() {
    test_ranty!("[rep:10]{a}", "aaaaaaaaaa");
}

#[test]
fn repeater_with_value_sep() {
    test_ranty!(r#"[rep:10][sep:\s]{a}"#, "a a a a a a a a a a");
}

#[test]
fn repeater_with_closure_sep() {
    test_ranty!(r#"[rep:10][sep:[?]{b}]{a}"#, "abababababababababa");
}

#[test]
fn selector_forward() {
    test_ranty!(
        r#"[rep:16][sel:[mksel:forward]]{a|b|c|d|e|f|g|h}"#,
        "abcdefghabcdefgh"
    );
}

#[test]
fn selector_reverse() {
    test_ranty!(
        r#"[rep:16][sel:[mksel:reverse]]{a|b|c|d|e|f|g|h}"#,
        "hgfedcbahgfedcba"
    );
}

#[test]
fn selector_ping() {
    test_ranty!(
        r#"[rep:16][sel:[mksel:ping]]{a|b|c|d|e|f|g|h}"#,
        "abcdefghgfedcbab"
    );
}

#[test]
fn selector_pong() {
    test_ranty!(
        r#"[rep:16][sel:[mksel:pong]]{a|b|c|d|e|f|g|h}"#,
        "hgfedcbabcdefghg"
    );
}

#[test]
fn selector_freeze_state_is_observable() {
    test_ranty!(r#"<$sel=[mksel:forward]>[sel-frozen:<sel>]"#, "@false");
    test_ranty!(
        r#"<$sel=[mksel:forward]>[sel-freeze:<sel>][sel-frozen:<sel>]"#,
        "@true"
    );
}

#[test]
fn func_no_params() {
    test_ranty!(r#"[$example-func]{test}[example-func]"#, "test");
}

#[test]
fn func_with_required_param() {
    test_ranty!(r#"[$square:x]{[mul:<x>;<x>]} [square:3]"#, "9");
}

#[test]
fn func_with_variadic_star() {
    test_ranty_file!(
        "sources/func/func_with_variadic_star.ranty",
        "\na\na b\na b c\na b c d"
    );
}

#[test]
fn func_with_variadic_plus() {
    test_ranty_file!(
        "sources/func/func_with_variadic_plus.ranty",
        "a\na b\na b c\na b c d"
    );
}

#[test]
fn func_with_optional_param() {
    test_ranty_file!("sources/func/func_with_optional_param.ranty", "foo\nbar");
}

#[test]
fn shadowed_global() {
    test_ranty!(r#"<$/test=foo><$test=bar><test>"#, "bar")
}

#[test]
fn shadowed_local() {
    test_ranty!(r#"<$test=foo>{<$test=bar><test>}"#, "bar")
}

#[test]
fn override_shadowed_global_with_explicit_global() {
    test_ranty!(r#"<$/example=foo><$example=bar></example>"#, "foo");
}

#[test]
fn override_shadowed_global_with_descope() {
    test_ranty!(r#"<$/example=foo><$example=bar><^example>"#, "foo");
}

#[test]
fn override_shadowed_local_with_descope() {
    test_ranty!(r#"<$test=foo>{<$test=bar><^test>}"#, "foo")
}

#[test]
fn override_shadowed_locals_with_multi_descope() {
    test_ranty_file!(
        "sources/access/override_shadowed_locals_with_multi_descope.ranty",
        "foo bar baz"
    );
}

#[test]
fn empty_def() {
    test_ranty_file!("sources/access/empty-def.ranty");
}

#[test]
fn multi_accessor_defs() {
    test_ranty!(r#"<$foo=8; $bar=2; $baz=[sub:<foo>;<bar>]; baz>"#, "6");
}

#[test]
fn multi_accessor_reassign() {
    test_ranty!(r#"<$foo=bar; foo=baz; foo>"#, "baz");
}

#[test]
fn multi_accessor_delim_term() {
    test_ranty!(r#"<$foo=8; $bar=2; $baz=[add:<foo>;<bar>]; baz;>"#, "10");
}

#[test]
fn dynamic_index_setter() {
    test_ranty_file!("sources/access/dynamic_index_setter.ranty", "1, 2, 4");
}

#[test]
fn dynamic_multi_index_setter() {
    test_ranty_file!(
        "sources/access/dynamic_multi_index_setter.ranty",
        "1, 2, 4, 4, 5, 6"
    )
}

#[test]
fn closure_capture_var() {
    test_ranty_file!("sources/closure/closure_capture_var.ranty");
}

#[test]
fn closure_capture_arg() {
    test_ranty_file!("sources/closure/closure_capture_arg.ranty");
}

#[test]
fn closure_mutate_captured_value() {
    test_ranty_file!(
        "sources/closure/closure_mutate_captured_value.ranty",
        "0 1 2 3"
    );
}

#[test]
fn filter_with_native_predicate() {
    test_ranty_file!(
        "sources/collections/filter_with_native_predicate.ranty",
        "1, 3, 5, 7, 9"
    );
}

#[test]
fn filter_with_user_predicate() {
    test_ranty_file!(
        "sources/collections/filter_with_user_predicate.ranty",
        "1, 3, 5, 7, 9"
    );
}

#[test]
fn map_with_native_callback() {
    test_ranty_file!(
        "sources/collections/map_with_native_callback.ranty",
        "-1, -2, -3, -4, -5, -6, -7, -8, -9, -10"
    )
}

#[test]
fn map_with_user_callback() {
    test_ranty_file!(
        "sources/collections/map_with_user_callback.ranty",
        "-1, -2, -3, -4, -5, -6, -7, -8, -9, -10"
    )
}

#[test]
fn zip_with_native_callback() {
    test_ranty_file!(
        "sources/collections/zip_with_native_callback.ranty",
        "5, 7, 9"
    );
}

#[test]
fn zip_with_user_callback() {
    test_ranty_file!(
        "sources/collections/zip_with_user_callback.ranty",
        "5, 7, 9"
    );
}

#[test]
fn func_percolation() {
    test_ranty_file!(
        "sources/func/func_percolation.ranty",
        "global\nlocal\nvery local"
    );
}

#[test]
fn anon_getter() {
    test_ranty_file!("sources/anonymous/anon_getter.ranty", "foobar");
}

#[test]
fn dynamic_anon_getter() {
    test_ranty_file!("sources/anonymous/dynamic_anon_getter.ranty", "6");
}

#[test]
fn anon_setter() {
    test_ranty_file!("sources/anonymous/anon_setter.ranty", "bazqux");
}

#[test]
fn dynamic_anon_setter() {
    test_ranty_file!("sources/anonymous/dynamic_anon_setter.ranty", "7");
}

#[test]
fn inv_index_get() {
    test_ranty_file!("sources/access/inv_index_get.ranty", "3, 2, 1");
}

#[test]
fn inv_index_set() {
    test_ranty_file!("sources/access/inv_index_set.ranty", "4, 5, 6");
}

#[test]
fn function_piping() {
    test_ranty_file!("sources/func/function_piping.ranty", "the fox the dog");
}

#[test]
fn function_piping_callback() {
    test_ranty_file!("sources/func/function_piping_callback.ranty", "foobar")
}

#[test]
fn func_assignment_pipe_set() {
    test_ranty_file!("sources/func/assignment_pipe_set.ranty");
}

#[test]
fn func_assignment_pipe_def_var() {
    test_ranty_file!("sources/func/assignment_pipe_def_var.ranty");
}

#[test]
fn func_assignment_pipe_def_const() {
    test_ranty_file!("sources/func/assignment_pipe_def_const.ranty");
}

#[test]
fn func_pipecall_pipeval() {
    test_ranty_file!("sources/func/pipecall_pipeval.ranty");
}

#[test]
fn getter_fallback_from_var() {
    test_ranty_file!(
        "sources/access/getter_fallback_from_var.ranty",
        "123, fallback"
    )
}

#[test]
fn getter_fallback_from_index() {
    test_ranty_file!(
        "sources/access/getter_fallback_from_index.ranty",
        "foo, bar, baz, oops"
    )
}

#[test]
fn getter_fallback_from_key() {
    test_ranty_file!(
        "sources/access/getter_fallback_from_key.ranty",
        "foo, bar, baz, oops"
    )
}

#[test]
fn charms_top_level_return() {
    test_ranty_file!("sources/charms/top_level_return.ranty");
}

#[test]
fn charms_func_return_output() {
    test_ranty_file!("sources/charms/func_return_output.ranty");
}

#[test]
fn charms_func_return_value() {
    test_ranty_file!("sources/charms/func_return_value.ranty");
}

#[test]
fn charms_rep_continue_output() {
    test_ranty_file!("sources/charms/rep_continue_output.ranty");
}

#[test]
fn charms_rep_continue_value() {
    test_ranty_file!("sources/charms/rep_continue_value.ranty");
}

#[test]
fn charms_rep_break_output() {
    test_ranty_file!("sources/charms/rep_break_output.ranty");
}

#[test]
fn charms_rep_break_value() {
    test_ranty_file!("sources/charms/rep_break_value.ranty");
}

#[test]
fn charms_weight_all_zero() {
    test_ranty_file!("sources/charms/weight_all_zero.ranty");
}

#[test]
fn assert_pass() {
    test_ranty_file!("sources/assert/assert_pass.ranty");
}

#[test]
fn assert_fail() {
    test_ranty_file!("sources/assert/assert_fail.ranty" raises AssertError);
}

#[test]
fn math_min() {
    test_ranty_file!("sources/math/min.ranty");
}

#[test]
fn math_max() {
    test_ranty_file!("sources/math/max.ranty");
}

#[test]
fn slice_list_full() {
    test_ranty_file!("sources/slice/list/full.ranty");
}

#[test]
fn slice_list_between_static() {
    test_ranty_file!("sources/slice/list/between_static.ranty");
}

#[test]
fn slice_list_from_static() {
    test_ranty_file!("sources/slice/list/from_static.ranty");
}

#[test]
fn slice_list_to_static() {
    test_ranty_file!("sources/slice/list/to_static.ranty");
}

#[test]
fn slice_list_between_dynamic() {
    test_ranty_file!("sources/slice/list/between_dynamic.ranty");
}

#[test]
fn slice_list_from_dynamic() {
    test_ranty_file!("sources/slice/list/from_dynamic.ranty");
}

#[test]
fn slice_list_to_dynamic() {
    test_ranty_file!("sources/slice/list/to_dynamic.ranty");
}

#[test]
fn slice_tuple_full() {
    test_ranty_file!("sources/slice/tuple/full.ranty");
}

#[test]
fn slice_tuple_between_static() {
    test_ranty_file!("sources/slice/tuple/between_static.ranty");
}

#[test]
fn slice_tuple_from_static() {
    test_ranty_file!("sources/slice/tuple/from_static.ranty");
}

#[test]
fn slice_tuple_to_static() {
    test_ranty_file!("sources/slice/tuple/to_static.ranty");
}

#[test]
fn slice_tuple_between_dynamic() {
    test_ranty_file!("sources/slice/tuple/between_dynamic.ranty");
}

#[test]
fn slice_tuple_from_dynamic() {
    test_ranty_file!("sources/slice/tuple/from_dynamic.ranty");
}

#[test]
fn slice_tuple_to_dynamic() {
    test_ranty_file!("sources/slice/tuple/to_dynamic.ranty");
}

#[test]
fn slice_string_full() {
    test_ranty_file!("sources/slice/string/full.ranty");
}

#[test]
fn slice_string_between_static() {
    test_ranty_file!("sources/slice/string/between_static.ranty");
}

#[test]
fn slice_string_from_static() {
    test_ranty_file!("sources/slice/string/from_static.ranty");
}

#[test]
fn slice_string_to_static() {
    test_ranty_file!("sources/slice/string/to_static.ranty");
}

#[test]
fn slice_string_between_dynamic() {
    test_ranty_file!("sources/slice/string/between_dynamic.ranty");
}

#[test]
fn slice_string_from_dynamic() {
    test_ranty_file!("sources/slice/string/from_dynamic.ranty");
}

#[test]
fn slice_string_to_dynamic() {
    test_ranty_file!("sources/slice/string/to_dynamic.ranty");
}

#[test]
fn slice_range_full() {
    test_ranty_file!("sources/slice/range/full.ranty");
}

#[test]
fn slice_range_between_static() {
    test_ranty_file!("sources/slice/range/between_static.ranty");
}

#[test]
fn slice_range_from_static() {
    test_ranty_file!("sources/slice/range/from_static.ranty");
}

#[test]
fn slice_range_to_static() {
    test_ranty_file!("sources/slice/range/to_static.ranty");
}

#[test]
fn slice_range_between_dynamic() {
    test_ranty_file!("sources/slice/range/between_dynamic.ranty");
}

#[test]
fn slice_range_from_dynamic() {
    test_ranty_file!("sources/slice/range/from_dynamic.ranty");
}

#[test]
fn slice_range_to_dynamic() {
    test_ranty_file!("sources/slice/range/to_dynamic.ranty");
}

#[test]
fn splice_static_from_tuple() {
    test_ranty_file!("sources/splice/static_from_tuple.ranty");
}

#[test]
fn splice_dynamic_from_tuple() {
    test_ranty_file!("sources/splice/dynamic_from_tuple.ranty");
}

#[test]
fn splice_static_from_list() {
    test_ranty_file!("sources/splice/static_from_list.ranty");
}

#[test]
fn splice_dynamic_from_list() {
    test_ranty_file!("sources/splice/dynamic_from_list.ranty");
}

#[test]
fn modules_require() {
    test_ranty_file!("sources/modules/require.ranty");
}

#[test]
fn spread_all() {
    test_ranty_file!("sources/spread/spread_all.ranty");
}

#[test]
fn spread_inner() {
    test_ranty_file!("sources/spread/spread_inner.ranty");
}

#[test]
fn spread_left() {
    test_ranty_file!("sources/spread/spread_left.ranty");
}

#[test]
fn spread_right() {
    test_ranty_file!("sources/spread/spread_right.ranty");
}

#[test]
fn spread_multi() {
    test_ranty_file!("sources/spread/spread_multi.ranty");
}

#[test]
fn spread_variadic_star() {
    test_ranty_file!("sources/spread/spread_variadic_star.ranty");
}

#[test]
fn spread_variadic_plus() {
    test_ranty_file!("sources/spread/spread_variadic_plus.ranty");
}

#[test]
fn const_define() {
    test_ranty_file!("sources/const/const_define.ranty");
}

#[test]
fn const_function() {
    test_ranty_file!("sources/const/const_function.ranty");
}

#[test]
fn redef_var_with_const() {
    test_ranty_file!("sources/const/redef_var_with_const.ranty");
}

#[test]
fn const_shadow() {
    test_ranty_file!("sources/const/const_shadow.ranty");
}

#[test]
fn list_autoconcat() {
    test_ranty_file!("sources/collections/list_autoconcat.ranty");
}

#[test]
fn list_autoconcat_repeater() {
    test_ranty_file!("sources/collections/list_autoconcat_repeater.ranty");
}

#[test]
fn tuple_autoconcat() {
    test_ranty_file!("sources/collections/tuple_autoconcat.ranty");
}

#[test]
fn tuple_autoconcat_repeater() {
    test_ranty_file!("sources/collections/tuple_autoconcat_repeater.ranty");
}

#[test]
fn list_tuple_autoconcat() {
    test_ranty_file!("sources/collections/list_tuple_autoconcat.ranty");
}

#[test]
fn map_autoconcat() {
    test_ranty_file!("sources/collections/map_autoconcat.ranty");
}

#[test]
fn temporal_one() {
    test_ranty_file!("sources/temporal/temporal_one.ranty");
}

#[test]
fn temporal_one_mixed() {
    test_ranty_file!("sources/temporal/temporal_one_mixed.ranty");
}

#[test]
fn temporal_two_samesize() {
    test_ranty_file!("sources/temporal/temporal_two_samesize.ranty");
}

#[test]
fn temporal_two_samesize_mixed() {
    test_ranty_file!("sources/temporal/temporal_two_samesize_mixed.ranty");
}

#[test]
fn temporal_two_samesize_sync() {
    test_ranty_file!("sources/temporal/temporal_two_samesize_sync.ranty");
}

#[test]
fn temporal_two_diffsize() {
    test_ranty_file!("sources/temporal/temporal_two_diffsize.ranty");
}

#[test]
fn temporal_two_diffsize_mixed() {
    test_ranty_file!("sources/temporal/temporal_two_diffsize_mixed.ranty");
}

#[test]
fn temporal_pipe_temporal() {
    test_ranty_file!("sources/temporal/temporal_pipe_temporal.ranty");
}

#[test]
fn range_forward() {
    test_ranty_file!("sources/range/range_forward.ranty");
}

#[test]
fn range_reverse() {
    test_ranty_file!("sources/range/range_reverse.ranty");
}

#[test]
fn range_forward_step_divisible() {
    test_ranty_file!("sources/range/range_forward_step_divisible.ranty");
}

#[test]
fn range_reverse_step_divisible() {
    test_ranty_file!("sources/range/range_reverse_step_divisible.ranty");
}

#[test]
fn range_forward_step_indivisible() {
    test_ranty_file!("sources/range/range_forward_step_indivisible.ranty");
}

#[test]
fn range_reverse_step_indivisible() {
    test_ranty_file!("sources/range/range_reverse_step_indivisible.ranty");
}

#[test]
fn branch_if() {
    test_ranty_file!("sources/branch/if.ranty")
}

#[test]
fn branch_if_else() {
    test_ranty_file!("sources/branch/if-else.ranty")
}

#[test]
fn branch_if_elseif() {
    test_ranty_file!("sources/branch/if-elseif.ranty")
}

#[test]
fn branch_if_elseif_else() {
    test_ranty_file!("sources/branch/if-elseif-else.ranty")
}

#[test]
fn ops_and() {
    test_ranty_file!("sources/ops/and.ranty")
}

#[test]
fn ops_and_short_circuit() {
    test_ranty_file!("sources/ops/and_short_circuit.ranty")
}

#[test]
fn ops_cmp() {
    test_ranty_file!("sources/ops/cmp.ranty")
}

#[test]
fn ops_math() {
    test_ranty_file!("sources/ops/math.ranty")
}

#[test]
fn ops_not() {
    test_ranty_file!("sources/ops/not.ranty")
}

#[test]
fn ops_or() {
    test_ranty_file!("sources/ops/or.ranty")
}

#[test]
fn ops_or_short_circuit() {
    test_ranty_file!("sources/ops/or_short_circuit.ranty")
}

#[test]
fn ops_xor() {
    test_ranty_file!("sources/ops/xor.ranty")
}

#[test]
fn access_add_assign() {
    test_ranty_file!("sources/access/add_assign.ranty");
}

#[test]
fn access_sub_assign() {
    test_ranty_file!("sources/access/sub_assign.ranty");
}

#[test]
fn access_mul_assign() {
    test_ranty_file!("sources/access/mul_assign.ranty");
}

#[test]
fn access_div_assign() {
    test_ranty_file!("sources/access/div_assign.ranty");
}

#[test]
fn access_pow_assign() {
    test_ranty_file!("sources/access/pow_assign.ranty");
}

#[test]
fn access_mod_assign() {
    test_ranty_file!("sources/access/mod_assign.ranty");
}

#[test]
fn access_and_assign() {
    test_ranty_file!("sources/access/and_assign.ranty");
}

#[test]
fn access_or_assign() {
    test_ranty_file!("sources/access/or_assign.ranty");
}

#[cfg(feature = "cli")]
#[test]
fn corpus_fixture_set_matches_the_executable_non_tutorial_fixtures() {
    let corpus = load_fixture_corpus();
    let corpus_files = corpus
        .cases
        .iter()
        .map(|case| case.file.clone())
        .collect::<Vec<_>>();

    assert_eq!(corpus_files, discover_executable_fixture_files());
}

#[cfg(feature = "cli")]
#[test]
fn executable_fixture_corpus_matches_rust_cli_output() {
    let corpus = load_fixture_corpus();

    for case in corpus.cases {
        let output = run_cli(std::slice::from_ref(&case.file), None, &repo_root());
        assert_eq!(
            output.status, case.status,
            "fixture '{}' exit mismatch",
            case.file
        );
        assert_eq!(
            output.stdout, case.stdout,
            "fixture '{}' stdout mismatch",
            case.file
        );
        assert_eq!(
            relevant_stderr(&output.stderr, output.status),
            case.stderr,
            "fixture '{}' stderr mismatch",
            case.file
        );
    }
}

#[cfg(feature = "cli")]
#[test]
fn fuzz_corpus_matches_rust_cli_output() {
    let corpus = load_fuzz_corpus();

    for case in corpus.cases {
        let mut args = vec!["--eval".to_owned(), case.source.clone()];
        if let Some(seed) = &case.seed {
            args.splice(0..0, ["--seed".to_owned(), seed.clone()]);
        }

        let output = run_cli(&args, None, &repo_root());
        assert_eq!(
            output.status, case.status,
            "fuzz case '{}' exit mismatch",
            case.label
        );
        assert_eq!(
            output.stdout, case.stdout,
            "fuzz case '{}' stdout mismatch",
            case.label
        );
        assert_eq!(
            relevant_stderr(&output.stderr, output.status),
            case.stderr,
            "fuzz case '{}' stderr mismatch",
            case.label
        );
    }
}
