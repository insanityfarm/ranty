mod common;

use assert_matches::assert_matches;
use common::{run, run_str};
use ranty::runtime::{RuntimeError, RuntimeErrorType};

#[test]
fn inherited_map_value_lookup() {
    assert_eq!(
        run_str(
            r#"
            <$obj = (::)>
            <$proto = (:: flavor = vanilla)>
            [set-proto: <obj>; <proto>]
            <obj/flavor>
            "#
        ),
        "vanilla"
    );
}

#[test]
fn own_map_values_shadow_prototype_values() {
    assert_eq!(
        run_str(
            r#"
            <$obj = (:: flavor = chocolate)>
            <$proto = (:: flavor = vanilla)>
            [set-proto: <obj>; <proto>]
            <obj/flavor>
            "#
        ),
        "chocolate"
    );
}

#[test]
fn multi_hop_prototype_lookup_works() {
    assert_eq!(
        run_str(
            r#"
            <$obj = (::)>
            <$proto = (::)>
            <$base = (:: flavor = mint)>
            [set-proto: <proto>; <base>]
            [set-proto: <obj>; <proto>]
            <obj/flavor>
            "#
        ),
        "mint"
    );
}

#[test]
fn inherited_functions_are_callable() {
    assert_eq!(
        run_str(
            r#"
            <$obj = (::)>
            <$proto = (:: greet = [?: name] { Hello,\s<name>! })>
            [set-proto: <obj>; <proto>]
            [obj/greet: Ranty]
            "#
        ),
        "Hello, Ranty!"
    );
}

#[test]
fn getter_fallback_uses_full_prototype_chain() {
    assert_eq!(
        run_str(
            r#"
            <$obj = (::)>
            <$proto = (:: flavor = vanilla)>
            [set-proto: <obj>; <proto>]
            <obj/flavor ? oops>, <obj/missing ? oops>
            "#
        ),
        "vanilla, oops"
    );
}

#[test]
fn writing_to_inherited_key_stays_local() {
    assert_eq!(
        run_str(
            r#"
            <$obj = (::)>
            <$proto = (:: flavor = vanilla)>
            [set-proto: <obj>; <proto>]
            <obj/flavor = chocolate>
            <obj/flavor>, <proto/flavor>
            "#
        ),
        "chocolate, vanilla"
    );
}

#[test]
fn removing_and_taking_only_affect_local_keys() {
    assert_eq!(
        run_str(
            r#"
            <$obj = (:: flavor = chocolate; local = here)>
            <$proto = (:: flavor = vanilla; inherited = there)>
            [set-proto: <obj>; <proto>]
            [remove: <obj>; flavor]
            [take: <obj>; local], <obj/flavor>, <proto/flavor>, <obj/inherited ? missing>, <obj/local ? missing>
            "#
        ),
        "here, vanilla, vanilla, there, missing"
    );
}

#[test]
fn lookup_only_utilities_remain_own_only() {
    assert_eq!(
        run_str(
            r#"
            <$obj = (:: own = 1)>
            <$proto = (:: inherited = 2)>
            [set-proto: <obj>; <proto>]
            [has: <obj>; own]\n
            [has: <obj>; inherited]\n
            [len: [keys: <obj>]]\n
            [len: [values: <obj>]]\n
            [translate: (: own; inherited); <obj>]\n
            <obj>
            "#
        ),
        "@true\n@false\n1\n1\n(: 1; inherited)\n(:: own = 1)"
    );
}

#[test]
fn direct_prototype_cycles_are_rejected() {
    assert_matches!(
        run(r#"
            <$obj = (::)>
            [set-proto: <obj>; <obj>]
            "#),
        Err(RuntimeError {
            error_type: RuntimeErrorType::ArgumentError,
            ..
        })
    );
}

#[test]
fn indirect_prototype_cycles_are_rejected() {
    assert_matches!(
        run(r#"
            <$obj = (::)>
            <$proto = (::)>
            <$base = (::)>
            [set-proto: <obj>; <proto>]
            [set-proto: <proto>; <base>]
            [set-proto: <base>; <obj>]
            "#),
        Err(RuntimeError {
            error_type: RuntimeErrorType::ArgumentError,
            ..
        })
    );
}
