# Conditional expressions

Conditional expressions are built from `@if`, `@elseif`, and `@else`. Conditions are evaluated from left to right until the first truthy branch succeeds.

```rant example
@if 0: {
  zero
} @else: {
  nonzero
}
```

```text expected
nonzero
```

## Truthiness

All condition values are coerced to `bool` using the shipped 4.0 truthiness rules.

| Data type                        | Evaluation                                      |
|----------------------------------|-------------------------------------------------|
| `bool`                           | Unchanged.                                      |
| `string`, `list`, `map`, `range` | `@true` when non-empty; otherwise, `@false`.    |
| `float`, `int`                   | `@true` when nonzero. `NaN` is falsey.          |
| `nothing`                        | Always `@false`.                                |
| `tuple`, `selector`, `function`  | Always `@true`.                                 |

## Short-circuiting

Once a branch succeeds, later conditions are not evaluated and their bodies are not run.

```rant example
@if @true: {
  pass
} @elseif [error: "should not run"]: {
  fail
}
```

```text expected
pass
```
