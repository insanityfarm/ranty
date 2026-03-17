# Rant

Rant is a procedural templating language for generating text and structured values. It combines text-first syntax, weighted and repeating blocks, functions, modules, deterministic randomness, and a batteries-included standard library aimed at content generation workflows.

## Highlights

* Text-focused output with whitespace normalization, hinting, sinking, and output modifiers
* Functions, closures, accessors, conditional expressions, and weighted block selection
* Deterministic randomness with explicit seeds and RNG forking
* Filesystem modules through `@require`
* A CLI, REPL, and embeddable Rust library

## Install The CLI

```sh
cargo install rant --version 4.0.0 --features cli
```

Then run a script, an inline program, or the REPL:

```sh
# Start the REPL
rant

# Run inline code
rant --eval '[rep:3][sep:\s]{beep}'

# Run a file
rant examples/helloworld.rant
```

## Embed In Rust

```rust
use rant::Rant;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  let mut rant = Rant::new();
  let program = rant.compile_quiet(r#"
  [$greet:name] {
    {Hello|Hi|Hey}, <name>!
  }

  [greet:world]
  "#)?;

  let output = rant.run(&program)?;
  println!("{}", output);
  Ok(())
}
```

## Documentation

The stable language and standard-library reference is included in this repository under [docs/intro.html](./docs/intro.html).

## Examples

The [examples](./examples/) directory contains complete Rant programs covering modules, functions, weighting, and other common patterns.

## License

Licensed under either of:

* Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](./LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
