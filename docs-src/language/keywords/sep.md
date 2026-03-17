# @sep

The `@sep` keyword reads or writes the separator attribute used between repeated block iterations.

## Forms

- `@sep` reads the current separator value.
- `<@sep>` also reads the current separator value.
- `<@sep = expr>` writes the current attribute frame.
- `@sep expr: { ... }` applies a separator to the block that follows immediately.

```ranty example
<@sep = ",">[rep:3][sep:<@sep>]{x}
```

```text expected
x,x,x
```

As with the other mutable attribute keywords, the accessor form does not support globals, descoping,
fallbacks, or compound assignment.
