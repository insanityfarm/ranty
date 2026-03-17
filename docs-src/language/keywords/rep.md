# @rep

The `@rep` keyword reads or writes the repetitions attribute.

## Forms

- `@rep` reads the current repetitions value.
- `<@rep>` also reads the current repetitions value.
- `<@rep = expr>` writes the current attribute frame.
- `@rep expr: { ... }` applies a repetition value to the block that follows immediately.

`@rep` accepts the same values as `[rep]`: nonnegative integers or the repetition mode strings
`"once"`, `"all"`, and `"forever"`.

```rant example
@rep 3: {x}
```

```text expected
xxx
```

The accessor form is specialized syntax. Globals, descoping, fallbacks, and compound assignment are
not supported.
