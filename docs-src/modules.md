# Modules

Rant modules are ordinary `.rant` files whose top-level result becomes the imported module value. In typical use a module returns a map of functions and values.

## Writing modules

A module is just a program that returns a value, usually a map:

```rant
# seq.rant

<%module = (::)>

[$module/fib: n] {
  (:) [rep: <n>] { @edit f: <f> (<f/-2 ? 0> + <f/-1 ? 1>) }
}

<module>
```

## Importing modules

Use `@require` to load a module and bind it by filename:

```rant
@require "seq"

[seq/fib: 16]
```

The default resolver appends `.rant` automatically and normalizes relative paths before loading.

## Search order

When you use `@require`, the default resolver checks the following locations in order:

1. the importing program's directory, if the caller came from a file-backed program,
2. the resolver's local modules path, or the host working directory when no local path is configured,
3. the global modules path from `RANT_MODULES_PATH` when global modules are enabled.

## Caching

Modules are cached per `Rant` context. Requiring the same module again returns the cached value instead of recompiling or re-running the module body.

## Relative paths

Relative paths are resolved relative to the active search roots:

```rant
@require "rant_modules/my-module"
```

## Failures

- Missing modules raise `MODULE_ERROR`.
- Compile failures in module source raise `MODULE_ERROR` with compiler diagnostics attached to the runtime error description.
- Runtime failures during module initialization propagate as ordinary runtime errors from the module body.
- Cyclic imports are rejected deterministically as `MODULE_ERROR`.
