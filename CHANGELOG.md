# Changelog

## 1.0.0

Ranty's first stable release completes the unreleased Rant 4 line and ships it as Ranty v1.0.0.

* Closed the remaining stable-release work: [`@return`/`@break`/`@continue`](https://insanityfarm.github.io/ranty/stdlib/control-flow.html) now unwind predictably, [conditionals](https://insanityfarm.github.io/ranty/language/conditional-expressions.html) use the shipped truthiness rules, [modules](https://insanityfarm.github.io/ranty/modules.html) have stable resolution/caching/cycle detection, `@edit`/hinting/sinking/whitespace semantics are nailed down, the [CLI](https://insanityfarm.github.io/ranty/cli.html) contract is deterministic, and the release is backed by broad integration coverage.
* Added [`@on`-driven match selection](https://insanityfarm.github.io/ranty/language/keywords/on.html) plus [attribute keyword accessors and sugar](https://insanityfarm.github.io/ranty/runtime/attributes.html), giving blocks first-class pattern-style dispatch alongside readable `@rep`/`@sep`/`@sel`/`@mut` state access and delivering the long-requested [pattern matching work from Rant issue #6](https://github.com/rant-lang/rant/issues/6).
* Completed [map prototype support](https://insanityfarm.github.io/ranty/language/data-types/map-prototypes.html) with documented inheritance rules, prototype-aware lookup, and cycle rejection on `[set-proto]`, fulfilling the prototype inheritance work tracked in [Rant issue #1](https://github.com/rant-lang/rant/issues/1).
* Added garbage collection, including reclamation of cyclic collections and closure/native capture cycles, as part of the [roadmap GC milestone](https://github.com/orgs/rant-lang/projects/1/views/1?pane=issue&itemId=1444127).
* Added [`?=` lazy definitions and `@lazy` parameters](https://insanityfarm.github.io/ranty/language/accessors.html), with call-by-need memoization, descendant forcing, and lazy-cycle detection, covering the roadmap's lazy-values work.
* Added [recursive nested-block expansion](https://insanityfarm.github.io/ranty/language/blocks.html), including lifted `@weight`/`@on` metadata and protected-block expansion barriers, covering the roadmap's block-expansion work.
* Rebranded the project from [Rant to Ranty](https://github.com/insanityfarm/ranty/commit/89265f9b712f843af8d6130a606d8a1a057d3c27), set the stable line to v1.0.0, and renamed the default branch to [`main`](https://github.com/insanityfarm/ranty/tree/main).
* Rebuilt the [documentation set](https://insanityfarm.github.io/ranty/) around Ranty, expanded reference coverage for the new and stabilized features, and added a full [getting-started tutorial](https://insanityfarm.github.io/ranty/getting-started/tutorial.html) for new users.
* Added [benchmark workloads](https://github.com/insanityfarm/ranty/tree/main/benchmarks) for core runtime scenarios.
* Added [parity-tracking automation and fixtures](https://github.com/insanityfarm/ranty/tree/main/parity/ranty-js) for the downstream TypeScript port.
