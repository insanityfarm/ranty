# @step

The `@step` keyword reads the current block's zero-based iteration index.

> Warning:
> `@step` is not the same as `[step]`.
> `[step]` is 1-based, while `@step` is 0-based.
> The stdlib helper equivalent of `@step` is `[step-index]`.

`@step` is read-only. You may use it as a plain expression or with `<@step>`, but assigning to it
is a compile-time error.

If no repeater is active, `@step` reads as `0`.

```rant example
[rep:4][sep:", "]{@step}
```

```text expected
0, 1, 2, 3
```
