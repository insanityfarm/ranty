use std::cell::RefCell;

use crate::gc::Weak;
use crate::runtime::{RuntimeError, VM};
use crate::{
    collect_garbage, Ranty, RantyList, RantyListHandle, RantyMap, RantyOptions, RantyTuple,
    RantyValue,
};

thread_local! {
    static TRACKED_LIST: RefCell<Option<Weak<RefCell<RantyList>>>> = const { RefCell::new(None) };
    static SPAWNED_CYCLES: RefCell<Vec<Weak<RefCell<RantyList>>>> = const { RefCell::new(Vec::new()) };
}

fn track_list(_: &mut VM, list: RantyListHandle) -> Result<(), RuntimeError> {
    TRACKED_LIST.with(|slot| {
        *slot.borrow_mut() = Some(list.downgrade());
    });
    Ok(())
}

fn spawn_cycle(_: &mut VM, _: ()) -> Result<(), RuntimeError> {
    let list = RantyList::new().into_handle();
    list.borrow_mut().push(RantyValue::List(list.clone()));
    SPAWNED_CYCLES.with(|cycles| cycles.borrow_mut().push(list.downgrade()));
    drop(list);

    // Force another allocation so threshold-based collection can reclaim the dead cycle mid-run.
    let trigger = RantyList::new().into_handle();
    drop(trigger);

    Ok(())
}

fn alive_cycles(vm: &mut VM, _: ()) -> Result<(), RuntimeError> {
    let alive = SPAWNED_CYCLES.with(|cycles| {
        cycles
            .borrow()
            .iter()
            .filter(|weak| weak.upgrade().is_some())
            .count()
    });
    vm.cur_frame_mut().write(alive as i64);
    Ok(())
}

fn noop_captured(_: &mut VM, _: (), _: &[RantyValue]) -> Result<(), RuntimeError> {
    Ok(())
}

fn clear_test_state() {
    TRACKED_LIST.with(|slot| {
        *slot.borrow_mut() = None;
    });
    SPAWNED_CYCLES.with(|cycles| cycles.borrow_mut().clear());
}

#[test]
fn self_referential_lists_are_collected() {
    let list = RantyList::new().into_handle();
    let weak = list.downgrade();
    list.borrow_mut().push(RantyValue::List(list.clone()));

    drop(list);
    collect_garbage();

    assert!(weak.upgrade().is_none());
}

#[test]
fn mutually_recursive_maps_are_collected() {
    let map_a = RantyMap::new().into_handle();
    let map_b = RantyMap::new().into_handle();
    let weak_a = map_a.downgrade();
    let weak_b = map_b.downgrade();

    map_a
        .borrow_mut()
        .raw_set("other", RantyValue::Map(map_b.clone()));
    map_b
        .borrow_mut()
        .raw_set("other", RantyValue::Map(map_a.clone()));

    drop(map_a);
    drop(map_b);
    collect_garbage();

    assert!(weak_a.upgrade().is_none());
    assert!(weak_b.upgrade().is_none());
}

#[test]
fn explicit_native_captures_participate_in_cycle_collection() {
    let list = RantyList::new().into_handle();
    let weak = list.downgrade();
    let func = RantyValue::from_captured_func(vec![RantyValue::List(list.clone())], noop_captured);

    list.borrow_mut().push(func);
    drop(list);
    collect_garbage();

    assert!(weak.upgrade().is_none());
}

#[test]
fn run_end_collection_reclaims_closure_capture_cycles() {
    clear_test_state();

    let mut ranty = Ranty::new();
    ranty.set_global_const("track", RantyValue::from_func(track_list));
    let program = ranty
        .compile_quiet(
            r#"
            <$list = (: )>
            [track: <list>]
            <$f = [?]{<list>; <>}>
            [push: <list>; <f>]
            <>
            "#,
        )
        .expect("failed to compile test script");

    ranty.run(&program).expect("failed to run test script");

    let tracked = TRACKED_LIST.with(|slot| slot.borrow().clone());
    assert!(tracked
        .expect("missing tracked weak list")
        .upgrade()
        .is_none());
}

#[test]
fn threshold_collection_can_reclaim_cycles_before_run_end() {
    clear_test_state();

    let mut ranty = Ranty::with_options(RantyOptions {
        gc_allocation_threshold: 1,
        ..Default::default()
    });
    ranty.set_global_const("spawn-cycle", RantyValue::from_func(spawn_cycle));
    ranty.set_global_const("alive-cycles", RantyValue::from_func(alive_cycles));

    let program = ranty
        .compile_quiet(
            r#"
            [rep:8]{[spawn-cycle]}
            [alive-cycles]
            "#,
        )
        .expect("failed to compile test script");

    let result = ranty.run(&program).expect("failed to run test script");
    assert_eq!(result, RantyValue::Int(0));
}

#[test]
fn cyclic_list_and_tuple_equality_terminates() {
    let list_a = RantyList::new().into_handle();
    list_a.borrow_mut().push(RantyValue::List(list_a.clone()));

    let list_b = RantyList::new().into_handle();
    list_b.borrow_mut().push(RantyValue::List(list_b.clone()));

    assert_eq!(
        RantyValue::List(list_a.clone()),
        RantyValue::List(list_b.clone())
    );
    assert!(RantyValue::List(list_a).to_string().contains("..."));

    let tuple_list_a = RantyList::new().into_handle();
    let tuple_a = RantyTuple::from(vec![RantyValue::List(tuple_list_a.clone())]).into_handle();
    tuple_list_a
        .borrow_mut()
        .push(RantyValue::Tuple(tuple_a.clone()));

    let tuple_list_b = RantyList::new().into_handle();
    let tuple_b = RantyTuple::from(vec![RantyValue::List(tuple_list_b.clone())]).into_handle();
    tuple_list_b
        .borrow_mut()
        .push(RantyValue::Tuple(tuple_b.clone()));

    assert_eq!(RantyValue::Tuple(tuple_a), RantyValue::Tuple(tuple_b));
}
