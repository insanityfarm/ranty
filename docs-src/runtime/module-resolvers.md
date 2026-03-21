# Module Resolvers

Ranty's module system is built on a pluggable **module resolver** contract.
The default filesystem behavior is documented on [Modules](../modules.md); this page covers host-side customization.

## What a resolver does

A resolver takes the module path from `@require` or `[require]` and turns it into a compiled `RantyProgram`.

Resolver implementations can:

- load modules from the filesystem,
- serve virtual or in-memory modules,
- restrict modules to a sandbox or allowlist,
- redirect module names to generated source,
- disable modules entirely.

The resolver does **not** execute the module body or build the final imported value.
It only resolves the source program.
Module execution and caching still happen in the `Ranty` runtime.

## Installing a custom resolver

Use `Ranty::using_module_resolver(...)` to replace the default resolver for a context.

```rust,ignore
use ranty::{
    compiler::CompilerError,
    ModuleResolveError, ModuleResolveErrorReason, ModuleResolveResult, ModuleResolver, Ranty,
    RantyProgramInfo,
};
use std::collections::HashMap;

#[derive(Debug, Default)]
struct MemoryResolver {
    modules: HashMap<String, String>,
}

impl ModuleResolver for MemoryResolver {
    fn try_resolve(
        &self,
        context: &mut Ranty,
        module_path: &str,
        _dependant: Option<&RantyProgramInfo>,
    ) -> ModuleResolveResult {
        let source = self.modules.get(module_path).ok_or_else(|| ModuleResolveError {
            name: module_path.to_owned(),
            reason: ModuleResolveErrorReason::NotFound,
        })?;

        let mut problems = vec![];
        context
            .compile_named(source, &mut problems, module_path)
            .map_err(|err| ModuleResolveError {
                name: module_path.to_owned(),
                reason: match err {
                    CompilerError::SyntaxError => ModuleResolveErrorReason::CompileFailed(problems),
                    CompilerError::IOError(kind) => ModuleResolveErrorReason::FileIOError(kind),
                },
            })
    }
}
```

```rust,ignore
let resolver = MemoryResolver {
    modules: [(
        "kit".to_owned(),
        r#"
<%module = (::)>
[$module/value] { from memory }
<module>
"#
        .to_owned(),
    )]
    .into_iter()
    .collect(),
};

let mut ranty = Ranty::new().using_module_resolver(resolver);
let program = ranty.compile_quiet(r#"@require kit: "kit" [kit/value]"#)?;
assert_eq!(ranty.run(&program)?.to_string(), "from memory");
```

## The resolver contract

The `ModuleResolver::try_resolve()` method receives:

- `context`, so the resolver can compile source using the active `Ranty` settings,
- `module_path`, the raw path string from the script,
- `dependant`, metadata about the requesting program when available.

`dependant` is useful for relative-resolution strategies.
File-backed programs usually provide a path.
Programs compiled from raw strings may not.

## Default behavior

The built-in `DefaultModuleResolver` loads `.ranty` and legacy `.rant` files from:

1. the requesting program's directory,
2. the configured local modules path or current working directory,
3. the global modules path from `RANTY_MODULES_PATH`, when enabled.

See [Modules](../modules.md) for the end-user view of that behavior.

## Configuring the built-in resolver

You do not need a fully custom resolver just to change the built-in filesystem search behavior.
`DefaultModuleResolver` exposes a small configuration surface:

- `enable_global_modules`: when `true`, also search the path named by `RANTY_MODULES_PATH`.
- `local_modules_path`: preferred local search root. If it is `None`, the resolver falls back to the host process's current working directory.
- `DefaultModuleResolver::ENV_MODULES_PATH_KEY`: the constant string name of the global-modules environment variable.

```rust,ignore
use ranty::{DefaultModuleResolver, Ranty};

let mut ranty = Ranty::new().using_module_resolver(DefaultModuleResolver {
    enable_global_modules: false,
    local_modules_path: Some("./game-data/modules".to_owned()),
});

let program = ranty.compile_quiet(r#"@require "npc-kit" [npc-kit/name]"#)?;
let output = ranty.run(&program)?;
let _ = output;
```

This keeps the default resolution strategy, file-extension fallback, and compile behavior, while letting the host choose which search roots are active.

## Disabling modules

If an embedding host does not want scripts to load modules at all, it can install `NoModuleResolver`.
All module requests will then fail with `NotFound`.

## Errors

Resolvers should report failures through `ModuleResolveErrorReason`:

- `NotFound` when the requested module does not exist,
- `CompileFailed` when the source exists but fails to compile,
- `FileIOError` for host-side I/O failures when that distinction is useful.

Runtime errors from the module body happen later, after resolution succeeds.
Those are not resolver errors.

## Caching

Resolved modules are cached per `Ranty` context.
Requiring the same module again within one context reuses the cached module value instead of recompiling or re-running it.

That means a resolver can focus on lookup and compilation.
The higher-level module lifecycle is still managed by the runtime.

## Preloading modules from Rust

Hosts can also load and cache a module before any script requests it by calling `Ranty::try_load_global_module(...)`.

```rust,ignore
use ranty::{DefaultModuleResolver, Ranty};

let mut ranty = Ranty::new().using_module_resolver(DefaultModuleResolver {
    enable_global_modules: false,
    local_modules_path: Some("./game-data/modules".to_owned()),
});

ranty.try_load_global_module("npc-kit")?;

let program = ranty.compile_quiet(r#"@require "npc-kit" [npc-kit/name]"#)?;
let output = ranty.run(&program)?;
let _ = output;
```

This method:

- resolves the module immediately through the active resolver,
- runs the module body once,
- stores the resulting module value in the context cache.

Subsequent `@require` calls in that context can then reuse the cached value.

Unlike `@require`, host-side preloading does not have a requesting program.
That means relative resolution does **not** start from a dependant file path.
It resolves only against the resolver's local and global search roots.

`try_load_global_module()` returns `ModuleLoadError`, which distinguishes:

- invalid module paths,
- resolver failures (`ModuleResolveError`),
- runtime failures that occur while the module initializes.

That is a slightly higher-level API than `ModuleResolver::try_resolve()`, which only reports lookup and compilation outcomes.

## Design guidance

Custom resolvers work best when they stay predictable:

- keep path semantics simple and well documented,
- prefer stable IDs over ambient lookup when serving virtual modules,
- treat `dependant` as optional,
- return deterministic errors for missing or invalid modules.

If scripts also need controlled access to non-code resources, pair a custom resolver with [Data Sources](data-sources.md) rather than overloading modules to do both jobs.

## See also

- [Embedding in Rust](embedding-in-rust.md)
- [Modules](../modules.md)
- [@require](../language/keywords/require.md)
- [Standard Library: General functions](../stdlib/general.md)
