# Getting Started

Rant can be used through the CLI or embedded as a Rust library.

## Install the CLI

```sh
cargo install rant --version 4.0.0 --features cli
```

From a checkout, you can also run the CLI directly:

```sh
cargo run --features cli -- --help
```

## Your first program

```rant example
[$greet:name] {
  Hello, `<name>!
}

[greet:world]
```

```text expected
Hello, world!
```

## CLI quickstart

```sh
# Start the REPL
rant

# Run inline code
rant --eval '[rep:3][sep:\s]{beep}'

# Run a file
rant examples/helloworld.rant
```

Execution priority is:

1. `--eval PROGRAM`
2. `FILE`
3. piped stdin
4. REPL

## Embed in Rust

```rust
use rant::Rant;

let mut rant = Rant::new();
let program = rant.compile_quiet("Hello, world!")?;
let output = rant.run(&program)?;
assert_eq!(output.to_string(), "Hello, world!");
# Ok::<(), Box<dyn std::error::Error>>(())
```
