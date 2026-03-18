mod common;

use assert_matches::assert_matches;
use common::compile_with_reporter;
use ranty::runtime::{RuntimeError, RuntimeResult, VM};
use ranty::{IntoRanty, Ranty, RantyList, RantyValue};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Mutex,
};

static LAZY_COUNTER: AtomicUsize = AtomicUsize::new(0);
static LAZY_TEST_LOCK: Mutex<()> = Mutex::new(());

fn reset_counter() {
    LAZY_COUNTER.store(0, Ordering::SeqCst);
}

fn counter() -> usize {
    LAZY_COUNTER.load(Ordering::SeqCst)
}

fn next_value(vm: &mut VM, _: ()) -> RuntimeResult<()> {
    let next = LAZY_COUNTER.fetch_add(1, Ordering::SeqCst) + 1;
    vm.cur_frame_mut().write(next as i64);
    Ok(())
}

fn next_list(vm: &mut VM, _: ()) -> RuntimeResult<()> {
    LAZY_COUNTER.fetch_add(1, Ordering::SeqCst);
    vm.cur_frame_mut().write(
        RantyList::from(vec![RantyValue::Int(1), RantyValue::Int(2), RantyValue::Int(3)])
            .into_ranty(),
    );
    Ok(())
}

fn ok_func(vm: &mut VM, _: ()) -> RuntimeResult<()> {
    vm.cur_frame_mut().write("ok");
    Ok(())
}

fn make_func(vm: &mut VM, _: ()) -> RuntimeResult<()> {
    LAZY_COUNTER.fetch_add(1, Ordering::SeqCst);
    vm.cur_frame_mut().write(RantyValue::from_func(ok_func));
    Ok(())
}

fn run_with_lazy_helpers(source: &str) -> Result<String, RuntimeError> {
    reset_counter();
    let mut ranty = Ranty::new();
    ranty.set_global_const("next", RantyValue::from_func(next_value));
    ranty.set_global_const("next-list", RantyValue::from_func(next_list));
    ranty.set_global_const("make-func", RantyValue::from_func(make_func));
    let program = ranty
        .compile_quiet(source)
        .expect("failed to compile lazy test program");
    ranty.run(&program).map(|value| value.to_string())
}

#[test]
fn lazy_definition_forces_once() {
    let _guard = LAZY_TEST_LOCK.lock().unwrap();
    let output = run_with_lazy_helpers(r#"<$lazy ?= [next]><lazy>,<lazy>"#).unwrap();
    assert_eq!(output, "1,1");
    assert_eq!(counter(), 1);
}

#[test]
fn lazy_definition_can_be_overwritten_before_force() {
    let _guard = LAZY_TEST_LOCK.lock().unwrap();
    let output = run_with_lazy_helpers(r#"<$lazy ?= [next]><lazy = 7><lazy>"#).unwrap();
    assert_eq!(output, "7");
    assert_eq!(counter(), 0);
}

#[test]
fn lazy_constant_memoizes() {
    let _guard = LAZY_TEST_LOCK.lock().unwrap();
    let output = run_with_lazy_helpers(r#"<%lazy ?= [next]><lazy>,<lazy>"#).unwrap();
    assert_eq!(output, "1,1");
    assert_eq!(counter(), 1);
}

#[test]
fn lazy_parameter_can_skip_unused_argument() {
    let _guard = LAZY_TEST_LOCK.lock().unwrap();
    let output = run_with_lazy_helpers(r#"[$ignore: @lazy x] { ok }[ignore: [next]]"#).unwrap();
    assert_eq!(output, "ok");
    assert_eq!(counter(), 0);
}

#[test]
fn lazy_parameter_forces_once_when_read_multiple_times() {
    let _guard = LAZY_TEST_LOCK.lock().unwrap();
    let output = run_with_lazy_helpers(r#"[$dup: @lazy x] { <x>,<x> }[dup: [next]]"#).unwrap();
    assert_eq!(output, "1,1");
    assert_eq!(counter(), 1);
}

#[test]
fn lazy_optional_default_only_runs_when_accessed() {
    let _guard = LAZY_TEST_LOCK.lock().unwrap();
    let output = run_with_lazy_helpers(
        r#"
[$unused: @lazy x ? [next]] { ok }
[$used: @lazy x ? [next]] { <x>,<x> }
[unused]\n[used]
"#,
    )
    .unwrap();
    assert_eq!(output, "ok\n1,1");
    assert_eq!(counter(), 1);
}

#[test]
fn lazy_definition_captures_by_reference() {
    let _guard = LAZY_TEST_LOCK.lock().unwrap();
    let output = run_with_lazy_helpers(r#"<$value = 1><$lazy ?= <value>><value = 2><lazy>"#).unwrap();
    assert_eq!(output, "2");
}

#[test]
fn lazy_argument_capture_survives_for_closure_use() {
    let _guard = LAZY_TEST_LOCK.lock().unwrap();
    let output = run_with_lazy_helpers(
        r#"
[$defer: @lazy x] {
  [?] { <x> }
}
<$value = 1>
<$reader = [defer: <value>]>
<value = 2>
[reader]
"#,
    )
    .unwrap();
    assert_eq!(output, "2");
}

#[test]
fn descendant_setter_forces_lazy_root() {
    let _guard = LAZY_TEST_LOCK.lock().unwrap();
    let output = run_with_lazy_helpers(r#"<$items ?= [next-list]><items/0 = 9><items/0>,<items/1>,<items/2>"#).unwrap();
    assert_eq!(output, "9,2,3");
    assert_eq!(counter(), 1);
}

#[test]
fn function_lookup_forces_lazy_binding() {
    let _guard = LAZY_TEST_LOCK.lock().unwrap();
    let output = run_with_lazy_helpers(r#"<$f ?= [make-func]>[f]"#).unwrap();
    assert_eq!(output, "ok");
    assert_eq!(counter(), 1);
}

#[test]
fn self_referential_lazy_binding_raises_runtime_error() {
    let _guard = LAZY_TEST_LOCK.lock().unwrap();
    reset_counter();
    let mut ranty = Ranty::new();
    let program = ranty
        .compile_quiet(r#"<$x ?= <x>><x>"#)
        .expect("failed to compile self-referential lazy test");
    assert_matches!(
        ranty.run(&program),
        Err(RuntimeError {
            error_type: ranty::runtime::RuntimeErrorType::LazyBindingCycle,
            ..
        })
    );
}

#[test]
fn lazy_variadic_parameters_are_rejected() {
    let (result, messages) = compile_with_reporter(r#"[$bad: @lazy xs*] { <xs> }"#);
    assert!(result.is_err(), "lazy variadic parameter should fail to compile");
    assert!(messages.iter().any(|msg| msg.code() == "R0029"));
}

#[test]
fn lazy_optional_parameter_without_default_is_still_fallible() {
    let (result, messages) = compile_with_reporter(r#"[$bad: @lazy x?] { <x> }"#);
    assert!(result.is_err(), "fallible lazy optional access should fail to compile");
    assert!(messages.iter().any(|msg| msg.code() == "R0067"));
}

#[test]
fn lazy_constant_reassignment_is_rejected() {
    let (result, messages) = compile_with_reporter(r#"<%x ?= 1><x = 2>"#);
    assert!(result.is_err(), "lazy constant reassignment should fail to compile");
    assert!(messages.iter().any(|msg| msg.code() == "R0100"));
}
