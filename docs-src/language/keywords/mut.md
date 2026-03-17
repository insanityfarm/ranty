# @mut

The `@mut` keyword reads or writes the mutator attribute for the next block.

## Forms

- `@mut` reads the current mutator function or `<>` when no mutator is active.
- `<@mut>` also reads the current mutator.
- `<@mut = expr>` writes the current attribute frame.
- `@mut expr: { ... }` applies a mutator to the block that follows immediately.

The assigned value must be a function or `<>` to clear the mutator.

```ranty example
@mut [?: elem] { [elem]! }: {foo}
```

```text expected
foo!
```

Mutators run once per selected block element and receive the element as a callback parameter.
