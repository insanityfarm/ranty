# Introduction

> Found an error? Want something improved? [Submit an issue](https://github.com/insanityfarm/ranty/issues) or a pull request!

[Ranty](https://github.com/insanityfarm/ranty) is a procedural templating language for generating text and structured values. This book documents the current language, runtime behavior, CLI, module system, and standard library.

This documentation is a reference for the language features, standard library, and major runtime features of Ranty.

## Contents

- [**Getting Started**](getting-started.md) covers installation, the first program, and the CLI quickstart.
- [**Language**](language.md) covers text behavior, blocks, functions, accessors, keywords, operators, and conditionals.
- [**Runtime Features**](runtime.md) covers [Attributes](runtime/attributes.md), [Formatters](runtime/formatters.md), [Embedding in Rust](runtime/embedding-in-rust.md), [Data Sources](runtime/data-sources.md), [Modules](modules.md), [Module Resolvers](runtime/module-resolvers.md), and [CLI / REPL](cli.md).
- [**Standard Library**](stdlib.md) groups every exported builtin by category and includes a generated inventory.
- [**Diagnostics**](compiler-messages.md) is regenerated from the current compiler and runtime sources.
- [**Appendix**](rant-3-vs-ranty.md) includes the comparison of Rant 3 and Ranty, plus the glossary.

## Translations

If you want to see this documentation in your language, please [file an issue](https://github.com/insanityfarm/ranty/issues) and we'll work towards an official translation.

## Other helpful resources

For Rust API documentation, see [docs.rs](https://docs.rs/rant) (at this time, the link is for Rant 4, not Ranty).

If you want a guided, page-by-page walkthrough, start with the [Tutorial](getting-started/tutorial.md). For a quicker setup path, see [Getting Started](getting-started.md).
