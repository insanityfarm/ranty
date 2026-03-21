# Embedding in Rust

Ranty's Rust API is built around the [`Ranty`](https://docs.rs/ranty/latest/ranty/struct.Ranty.html) context.
This page covers the host-side surface for compiling programs, exposing host capabilities, controlling execution, and working with structured results.

## Creating a context

Most hosts start with one of these constructors:

- `Ranty::new()` for a default seed of `0` and the standard library loaded,
- `Ranty::with_seed(seed)` for deterministic host-controlled output,
- `Ranty::with_random_seed()` when each context should start from a fresh seed,
- `Ranty::with_options(options)` when the default setup is not enough.

```rust,ignore
use ranty::Ranty;

let mut ranty = Ranty::with_seed(0xdead_beef);
let program = ranty.compile_quiet("Hello, world!")?;
let output = ranty.run(&program)?;
assert_eq!(output.to_string(), "Hello, world!");
```

## Important context options

`RantyOptions` controls the runtime environment for a context:

- `use_stdlib`: loads the built-in globals when `true`; disable it for a custom prelude or a narrower sandbox.
- `debug_mode`: includes extra debug information in compiled programs and produces richer runtime diagnostics.
- `top_level_defs_are_globals`: keeps root-scope definitions after a run, which is useful for REPL-style hosts.
- `seed`: the initial RNG seed.
- `gc_allocation_threshold`: the number of managed allocations allowed between automatic cycle-collection passes.

```rust,ignore
use ranty::{Ranty, RantyOptions};

let mut ranty = Ranty::with_options(RantyOptions {
    use_stdlib: false,
    debug_mode: true,
    top_level_defs_are_globals: true,
    seed: 42,
    ..Default::default()
});
```

If you disable the stdlib, you are responsible for injecting any globals your programs expect.

## Compiling and collecting diagnostics

Ranty separates compilation failure from diagnostic reporting.
The `compile*` methods return `Result<_, CompilerError>`, while warnings and detailed syntax problems are reported through a `Reporter`.

Use:

- `compile` / `compile_named` for source strings,
- `compile_file` for file-backed programs,
- `compile_quiet`, `compile_quiet_named`, and `compile_file_quiet` when you do not need detailed messages.

`compile_named` is especially useful for inline strings because the provided name shows up in diagnostics.
`compile_file` also records the program path, which matters for relative module resolution.

```rust,ignore
use ranty::{
    compiler::{CompilerMessage, Problem, Reporter},
    Ranty,
};

let ranty = Ranty::new();
let mut messages = Vec::<CompilerMessage>::new();

let result = ranty.compile_named("{", &mut messages, "intro-script");
assert!(result.is_err());

for message in &messages {
    let code = message.code();
    let text = message.message();
    let pos = message.pos().map(|pos| format!("{}:{}", pos.line(), pos.col()));

    println!("{code} {pos:?} {text}");

    match message.info() {
        Problem::UnclosedBlock => println!("missing closing brace"),
        other => println!("other problem: {other:?}"),
    }
}
```

The built-in reporter implementations are:

- `()` to ignore all compiler messages,
- `Vec<CompilerMessage>` to collect them in memory.

## Passing inputs to a single run

`run()` executes a compiled program against the context as it currently exists.
`run_with()` lets you inject a map of root-scope locals for one execution.

These values are not added to the context's global table.
They exist only for that run.

```rust,ignore
use ranty::{IntoRanty, Ranty};
use std::collections::HashMap;

let mut ranty = Ranty::new();
let program = ranty.compile_quiet("Hello, <name>!")?;

let output = ranty.run_with(
    &program,
    Some(HashMap::from([("name".to_owned(), "Juniper".into_ranty())])),
)?;

assert_eq!(output.to_string(), "Hello, Juniper!");
```

## Globals and custom preludes

The context also exposes a persistent global table:

- `set_global()` writes a mutable global and auto-defines it if needed,
- `set_global_const()` writes an immutable global,
- `set_global_force()` overwrites an existing global even if it is constant,
- `get_global()`, `has_global()`, `delete_global()`, and `global_names()` inspect or manage the table.

This is how the standard library and CLI-only helpers are installed.
It is also the simplest way to build a custom prelude for your own host.

```rust,ignore
use ranty::{Ranty, RantyValue};

let mut ranty = Ranty::new();
ranty.set_global_const("build-mode", RantyValue::String("debug".into()));

assert_eq!(
    ranty.get_global("build-mode"),
    Some(RantyValue::String("debug".into()))
);
assert!(ranty.global_names().any(|name| name == "build-mode"));
```

## Exposing native Rust functions

Native Rust functions can be wrapped as `RantyValue::Function` values and placed into globals or returned from other native helpers.

`RantyValue::from_func(...)` registers a plain native function.
`RantyValue::from_captured_func(...)` registers a native function with an explicit capture list.

Argument and return conversion is handled by the conversion traits:

- `TryFromRanty` and `FromRantyArgs` for incoming arguments,
- `IntoRanty` and `TryIntoRanty` for outgoing values,
- `VarArgs<T>` for optional variadic tails,
- `RequiredVarArgs<T>` for required variadic tails.

```rust,ignore
use ranty::runtime::{RuntimeError, VM};
use ranty::{Ranty, RantyValue, VarArgs};

fn csv(vm: &mut VM, parts: VarArgs<String>) -> Result<(), RuntimeError> {
    vm.cur_frame_mut().write(parts.join(", "));
    Ok(())
}

let mut ranty = Ranty::new();
ranty.set_global_const("csv", RantyValue::from_func(csv));

let program = ranty.compile_quiet(r#"[csv: moon; tea; lantern]"#)?;
assert_eq!(ranty.run(&program)?.to_string(), "moon, tea, lantern");
```

Captured native functions use the same calling convention, but receive an extra `&[RantyValue]` slice containing the captured values.

## RNG control

Ranty's script-level random behavior is already documented in [CLI / REPL](../cli.md) for `--seed`, in [Standard Library: General functions](../stdlib/general.md#seed) for `[seed]`, `[fork]`, and `[unfork]`, and in [Standard Library: Generators](../stdlib/generators.md) for the random generator functions.
From the host side, the important methods are:

- `with_seed()` and `with_random_seed()` when constructing the context,
- `seed()` to inspect the current root seed,
- `set_seed()` to replace the active RNG,
- `reset_seed()` to restart the RNG from its current root seed.

```rust,ignore
use ranty::Ranty;

let mut ranty = Ranty::with_seed(7);
let original_seed = ranty.seed();

ranty.set_seed(99);
assert_eq!(ranty.seed(), 99);

ranty.reset_seed();
assert_eq!(ranty.seed(), 99);
assert_ne!(original_seed, ranty.seed());
```

This is useful when a host wants stable generation across sessions, or when benchmark and test harnesses need exact replay behavior.

## Working with structured results

`run()` and `run_with()` return `RantyValue`, not just strings.
If a program produces a map, list, tuple, range, function, or selector, the host receives that value directly.

That means you can treat Ranty as a structured generator and only stringify the result when you actually want rendered text.

```rust,ignore
use ranty::{Ranty, RantyValue};

let mut ranty = Ranty::new();
let program = ranty.compile_quiet(
    r#"
    (:: name = "Moon Bakery"; tags = (: cozy; open-late))
    "#,
)?;

let value = ranty.run(&program)?;

match value {
    RantyValue::Map(map) => {
        let map = map.borrow();
        assert_eq!(
            map.raw_get("name"),
            Some(&RantyValue::String("Moon Bakery".into()))
        );
        assert!(matches!(map.raw_get("tags"), Some(RantyValue::List(_))));
    }
    other => panic!("expected map, got {}", other.type_name()),
}
```

If you do want rendered text, call `to_string()` on the returned `RantyValue`.
For by-reference value types such as lists, tuples, maps, functions, and selectors, the returned value owns the handle you need to inspect it.

## Garbage collection controls

Ranty performs automatic cycle collection, but hosts can also manage it directly:

- `RantyOptions::gc_allocation_threshold` tunes how often automatic collection runs,
- `Ranty::collect_garbage()` forces collection for the current context,
- `ranty::collect_garbage()` forces collection for the current thread's Ranty heap.

Lower thresholds can be useful for long-lived contexts that create many cyclic values.
Higher thresholds can reduce collection overhead when short-term memory churn is more important than immediate reclamation.

## See also

- [Getting Started](../getting-started.md)
- [Data Sources](data-sources.md)
- [Module Resolvers](module-resolvers.md)
- [Diagnostics](../compiler-messages.md)
