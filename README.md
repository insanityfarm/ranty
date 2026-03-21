# Ranty

**Ranty** is a dynamically-typed, multi-paradigm templating language designed primarily for procedural generation. It is designed with scalability in mind: it can handle tasks ranging from simple randomized string generation to more complex workloads such as procedural dialogue, character generation, and worldbuilding.

## Introducing Ranty

Ranty is the result of a long-standing desire for an all-in-one data templating tool made especially for creative applications like games and interactive art.

This is a fork of [Rant 4](https://github.com/rant-lang/rant), which appears to have been abandoned before its first release. Many thanks to [Robin Pederson](https://github.com/TheBerkin), Rant's original creator and maintainer, with additional appreciation for the contributions of [Leander Neiss](https://github.com/lanice) and [Tamme Schichler](https://github.com/Tamschi) to that codebase.

Ranty was created in 2026 with the assistance of agentic AI (OpenAI's [Codex](https://openai.com/codex/)). Our goal was to complete the apparent remaining work in the Rant 4 project, uplifting it from its last version (`v4.0.0-alpha.33`) to a stable, feature-complete release. While we were at it we took a stab at letting Codex implement most of the proposed or in-flight features described on the [Rant 5 Roadmap](https://github.com/orgs/rant-lang/projects/1/views/1). We made a good-faith effort to honor Robin Pederson's intent for these, but as they're fairly underspecified we had to do some speculation and decision-making of our own. So this disclaimer: As with all vibe-coded software, those features (and indeed all of Ranty) may not be as functional as intended. We encourage everyone to submit issues as concerns are found, and PRs if you want to help fix them!

## Features

**🧰 Painless API**  
Ranty has a no-nonsense API designed for ease of use. No getting lost in configuration hell. Integrating Ranty into your project only takes a few lines of code.

**💻 Cross-platform**  
Write once, run anywhere. The runtime works the same across Windows, macOS, Linux, and WebAssembly.

**✍ Templating that does more**  
Ranty is all about "printing": each lexical scope has an output to print to, which then prints itself to the parent output, and so on. This enables you to intuitively build strings, collections, and more in a familiar templating setting.

**🎨 Turing-complete!**
In addition to being a templating language, Ranty adopts declarative and imperative programming concepts with design influences from many other popular languages.

**✨ Generate anything — not just text**  
Ranty outputs arbitrary data structures using built-in value types such as strings, numbers, collections, closures, and more.

**🎲 Built with ♥ for RNG**  
Ranty is built with procedural generation in mind. Make use of a wide array of built-in utilities for generating random numbers, strings, booleans, lists, list subsets, and much more for all your randomization needs. The internal RNG can be manually seeded to produce repeatable outputs.

**🔱 Branching and beyond**  
Augment regular control flow behavior with iterative, randomized, and weighted branch selection.

**🧬 Delightful combinatorics**  
Perform nested mappings, filters, zips, combinations, and more with shorter, more readable code through Ranty's piping syntax.

**📝 Automatic text formatting**  
Passively format text output with capitalization, whitespace normalization, hinting, sinking, and number formatting.

**📦 Data sources**
Attach custom data sources to your Ranty execution context to give your scripts controlled access to external resources.

**🧩 Simple module system**  
Sharing code between Ranty programs is trivial. Write a `.ranty` module and `@require` it elsewhere. Legacy `.rant` files still load for compatibility. Need custom module resolution logic? No problem. You can write your own resolver and just plug it in.

**🔋 Batteries included**  
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

## Benchmarking

The benchmark suite measures **cold-start CLI performance** of the final distributable, not compile time. Each run spawns a fresh `ranty` process against a checked-in stress workload and times how long it takes to produce output.

This project uses [`hyperfine`](https://github.com/sharkdp/hyperfine) for benchmarking because it is built specifically for repeated external-command timing. In practice, that makes it a better fit than in-process microbenchmark tools for CLI startup measurements.

Install `hyperfine` using the package manager you prefer, for example:

```sh
brew install hyperfine
# or
cargo install hyperfine
```

Then build the release CLI and run the benchmark script:

```sh
cargo build --release --features cli --bin ranty
bash benchmarks/run-hyperfine.sh
```

The benchmark inputs live in `benchmarks/workloads`. The script writes machine-readable results to `benchmarks/latest-results.json`.

If you want to time a custom command manually with `hyperfine`, keep the same basic approach:

```sh
hyperfine -N --warmup 3 --runs 20 --output=pipe \
  './target/release/ranty --no-debug --no-warnings examples/helloworld.ranty'
```

Notes:

- `-N` tells `hyperfine` to execute the command directly instead of through a shell.
- `--output=pipe` keeps stdout generation in the measurement without terminal rendering noise.
- The suite is intentionally cold-start by process: every timed iteration launches a new CLI process.

## [Examples](./examples/)

This repository includes a collection of example Ranty scripts for you to learn from. Check them out.

## Documentation

The latest reference documentation is published at [insanityfarm.github.com/ranty](https://insanityfarm.github.com/ranty).

A bundled copy of the stable language and standard-library reference is also included in this repository under [docs/intro.html](./docs/intro.html).

## Ranty.js parity

The standalone TypeScript port lives at
[insanityfarm/ranty-js](https://github.com/insanityfarm/ranty-js). This Rust
repo is the authoritative upstream for shared language and runtime behavior.

Ranty's checked-in downstream contract for Ranty.js lives under
[`parity/ranty-js/`](./parity/ranty-js). Regenerate it with:

```sh
cargo xtask parity build
```

Verify it is current with:

```sh
cargo xtask parity verify
```

## [Changelog](./CHANGELOG.md)

The changelog summarizes the changes that landed in the stable fork and the work required to ship Ranty 1.0.0.

## License

Licensed under the MIT license in [LICENSE](LICENSE).
