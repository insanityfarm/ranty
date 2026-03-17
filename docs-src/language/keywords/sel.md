# @sel

The `@sel` keyword reads or writes the active selector attribute for the next block.

## Forms

- `@sel` reads the current selector or `<>` when no selector is active.
- `<@sel>` also reads the current selector.
- `<@sel = expr>` writes the current attribute frame.
- `@sel expr: { ... }` applies a selector to the block that follows immediately.

The assigned value may be a selector handle, a selector mode string such as `"forward"`, or `<>`
to clear the selector.

```rant example
@sel "forward": {a|b}
```

```text expected
a
```

To create a value-driven selector instead of a cursor-driven one, use `[match: value]` or
`[mksel: match; value]`.
