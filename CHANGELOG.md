# Changelog

## 1.0.0

Stable release highlights:

* Stabilized control-flow behavior for `@return`, `@break`, and `@continue`
* Aligned conditional truthiness rules with the shipped runtime
* Formalized module resolution, caching, and cycle detection
* Stabilized `@edit`, hinting, sinking, and whitespace normalization semantics
* Hardened the CLI and REPL contract, including deterministic seed parsing and exit codes
* Added integration coverage for modules, control flow, output behavior, conditionals, the CLI, and the exported standard-library surface
* Rebuilt the in-repo documentation set for the stable Ranty language and toolchain
