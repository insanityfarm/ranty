# Introduction

Rant 4.0 is a procedural templating language for generating text and structured values. This book documents the shipped 4.0 language, runtime behavior, CLI, module system, and standard library.

The in-repo docs are maintained from three sources:

1. the current repo docs surface for already-audited 4.0 behavior,
2. the `rant-lang/reference` mdBook as the structural baseline,
3. the archived public docs as a cross-check for missing detail.

## Contents

- [**Getting Started**](getting-started.md) covers installation, the first program, and the CLI quickstart.
- [**Language**](language.md) covers text behavior, blocks, functions, accessors, keywords, operators, and conditionals.
- [**Runtime Features**](runtime.md) covers attributes, formatters, modules, determinism, and the CLI.
- [**Standard Library**](stdlib.md) groups every exported builtin by category and includes a generated inventory.
- [**Diagnostics**](compiler-messages.md) is regenerated from the current compiler and runtime sources.
- [**Appendix**](rant-3-vs-4.md) includes the Rant 3 versus 4 comparison and the glossary.
