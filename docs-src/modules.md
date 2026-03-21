# Modules

Ranty modules are `.ranty` files whose top-level result becomes the imported module value. In typical use a module returns a map of functions and values.

For compatibility with Rant 4, legacy `.rant` modules are also supported.

## Writing modules

A module is just a program that returns a value, usually a map:

```ranty
# seq.ranty

<%module = (::)>

[$module/fib: n] {
  (:) [rep: <n>] { @edit f: <f> (<f/-2 ? 0> + <f/-1 ? 1>) }
}

<module>
```

## Importing modules

Use `@require` to load a module and bind it by filename:

```ranty
@require "test-module"
```

The default resolver prefers `.ranty` when no extension is supplied, falls back to `.rant`, preserves explicit `.ranty` and `.rant` paths, and normalizes relative paths before loading.

Embedding hosts are not limited to this behavior.
They can replace the default filesystem resolver entirely; see [Module Resolvers](runtime/module-resolvers.md).

## Search order

When you use `@require`, the default resolver checks the following locations in order:

1. the importing program's directory, if the caller came from a file-backed program,
2. the resolver's local modules path, or the host working directory when no local path is configured,
3. the global modules path from `RANTY_MODULES_PATH` when global modules are enabled.

## Caching

Modules are cached per `Ranty` context. Requiring the same module again returns the cached value instead of recompiling or re-running the module body.

## Relative paths

Relative paths are resolved relative to the active search roots:

```ranty
@require tm: "test-module"
```

## Failures

- Missing modules raise `MODULE_ERROR`.
- Compile failures in module source raise `MODULE_ERROR` with compiler diagnostics attached to the runtime error description.
- Runtime failures during module initialization propagate as ordinary runtime errors from the module body.
- Cyclic imports are rejected deterministically as `MODULE_ERROR`.

## Host customization

The behavior described above is specific to the built-in `DefaultModuleResolver`.

Embedding hosts can instead:

- serve modules from memory or another backing store,
- restrict which module paths are legal,
- disable modules entirely with `NoModuleResolver`.

See [Module Resolvers](runtime/module-resolvers.md) for the resolver contract, `DefaultModuleResolver` configuration, and host-side preloading with `try_load_global_module()`.
