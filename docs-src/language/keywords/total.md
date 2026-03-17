# @total

The `@total` keyword reads the active block's total iteration count.

> Warning:
> `@total` is not the same as `[step-count]`.
> `[step-count]` reports the resolver's scheduled count and returns `0` for `forever`.
> `@total` returns `<>` for infinite repeaters.

`@total` is read-only. You may use it as a plain expression or with `<@total>`, but assigning to it
is a compile-time error.

If no repeater is active, `@total` reads as `0`.

```ranty example
[rep:3]{[eq: @total; 3] @break}
```

```text expected
@true
```

```ranty example
[rep: forever]{[eq: @total; <>] @break}
```

```text expected
@true
```
