# Getting Started

Ranty can be used through the CLI or embedded as a Rust library.

If you want a slower, story-driven walk through the language, head to the [Tutorial](getting-started/tutorial.md).

## Install the CLI

```sh
cargo install ranty --version 1.0.0 --features cli
```

Ranty source files and modules should normally use the `.ranty` file extension. For compatibility, legacy `.rant` files and modules are also supported.

From a checkout, you can also run the CLI directly:

```sh
cargo run --features cli -- --help
```

## Your first program

```ranty example
[$greet:name] {
  Hello, <name>!
}

[greet:world]
```

```text expected
Hello, world!
```

## CLI quickstart

```sh
# Start the REPL
ranty

# Run inline code
ranty --eval '[rep:3][sep:\s]{beep}'

# Run a file
ranty examples/helloworld.ranty
```

Execution priority is:

1. `--eval PROGRAM`
2. `FILE`
3. piped stdin
4. REPL

## Embed in Rust

```rust,ignore
use ranty::Ranty;

let mut ranty = Ranty::new();
let program = ranty.compile_quiet("Hello, world!")?;
let output = ranty.run(&program)?;
assert_eq!(output.to_string(), "Hello, world!");
```

If you are embedding Ranty, see [Embedding in Rust](runtime/embedding-in-rust.md) for the full host API. For integration points beyond plain compile-and-run, see [Data Sources](runtime/data-sources.md) and [Module Resolvers](runtime/module-resolvers.md).
