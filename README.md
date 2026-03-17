# Ranty

This is a fork of the seemingly abandoned [Rant 4](https://github.com/rant-lang/rant) project. Thanks to [Robin Pederson](https://github.com/TheBerkin), the original project creator, and additional thanks to [Leander Neiss](https://github.com/lanice) and [Tamme Schichler](https://github.com/Tamschi) for their contributions.

Ranty was created in 2026 by prompting agentic AI to complete the apparent remaining work in `Rant v4.0.0-alpha.33` so the fork could reach the next stable, release-ready milestone. It also includes implementations of the proposed or in-flight features described on the [Rant 5 Roadmap](https://github.com/orgs/rant-lang/projects/1/views/1). A good-faith effort was made to honor the original maintainer's intent, but some speculation and decision-making was required to execute on those descriptions. As with all vibe-coded software, those features and indeed all of Ranty may not be as functional as intended. Issues and pull requests are welcome.

**Ranty** is a dynamically-typed, multi-paradigm templating language designed primarily for procedural generation. It is designed with scalability in mind: it can handle tasks ranging from simple randomized string generation to more complex workloads such as procedural dialogue, character generation, and worldbuilding.

## Introducing Ranty

Ranty is the result of a long-standing desire for an all-in-one data templating tool made especially for creative applications like games and interactive art.

Ranty keeps the reimagined syntax, standard library, interpreter, and runtime behavior that made the upstream project compelling, while shipping them as a stable, release-ready fork with first-class `.ranty` source files and continued compatibility for legacy `.rant` files.

## Features

**Painless API**  
Ranty has a no-nonsense API designed for ease of use. No getting lost in configuration hell. Integrating Ranty into your project only takes a few lines of code.

**Cross-platform**  
Write once, run anywhere. The runtime works the same across Windows, macOS, Linux, and WebAssembly.

**Templating that does more**  
Ranty is all about "printing": each lexical scope has an output to print to, which then prints itself to the parent output, and so on. This enables you to intuitively build strings, collections, and more in a familiar templating setting.

**Generate structured values, not just text**  
Ranty outputs arbitrary data structures using built-in value types such as strings, numbers, collections, closures, and more.

**Deterministic randomness**  
Ranty is built with procedural generation in mind. The internal RNG can be manually seeded, forked, and reused to produce repeatable outputs.

**Branching and beyond**  
Augment regular control flow behavior with iterative, randomized, and weighted branch selection.

**Delightful combinatorics**  
Perform nested mappings, filters, zips, combinations, and more with shorter, more readable code through the language's piping syntax.

**Automatic text formatting**  
Passively format text output with capitalization, whitespace normalization, hinting, sinking, and number formatting.

**Simple module system**  
Sharing code between Ranty programs is trivial. Write a `.ranty` module and `@require` it elsewhere. Legacy `.rant` files still load for compatibility.

**Batteries included**  
A comprehensive standard library, CLI, REPL, and embeddable Rust library cover common content-generation workflows out of the box.

## Getting started

Ranty is written in [Rust](https://rust-lang.org), so you'll need the [toolchain](https://www.rust-lang.org/tools/install) to build it.

### CLI

Install the CLI from Cargo with:

```sh
$ cargo install ranty --version 1.0.0 --features cli
```

Then run it:

```sh
# Launch the REPL
$ ranty

# Get a full list of options
$ ranty -h

# Run an inline script and display output
$ ranty -e '[rep:3] [sep:\s] {b{ee|i|o|u}bbity}'

# Run helloworld.ranty and send output to result.txt
$ ranty examples/helloworld.ranty > result.txt
```

### Library

To use Ranty in a Rust project, add the `ranty` crate to `Cargo.toml`:

```toml
[dependencies]
ranty = "*"
```

You can run a Ranty program with just a few lines of code:

```rust
use ranty::Ranty;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  let mut ranty = Ranty::new();
  let program = ranty.compile_quiet(r#"
  [$greet:name] {
    {Hello|Hi|Hey} <name>!
  }
  [greet:world]
  "#)?;

  let output = ranty.run(&program)?;
  println!("{}", output);

  Ok(())
}
```

## [Examples](./examples/)

This repository includes a collection of example Ranty scripts for you to learn from. Check them out.

## Documentation

The latest reference documentation is published at [insanityfarm.github.com/ranty](https://insanityfarm.github.com/ranty).

A bundled copy of the stable language and standard-library reference is also included in this repository under [docs/intro.html](./docs/intro.html).

## [Changelog](./CHANGELOG.md)

The changelog summarizes the changes that landed in the stable fork and the work required to ship Ranty 1.0.0.

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
