# Output modifiers

Output modifiers are block elements that transform the caller's current output before the modifier's own result is written back.

## The @edit operator

`@edit` consumes the caller's current output, optionally binds it to a local name, and replaces it with the modifier body.

```ranty example
"example" { @edit x: `<x> `<x> }
```

```text expected
example example
```

If no binding name is supplied, the previous output is discarded and only the modifier result is kept.

```ranty example
"example" { @edit: "overwritten" }
```

```text expected
overwritten
```

## Placement rules

`@edit` must appear at the start of a block element. Misplaced `@edit` is a compile-time error.

## Example: accumulating across repeats

```ranty example
[%factorial: n] {
  1 [rep: <n>] {@edit x: <x> * [step]}
}

[factorial: 6]
```

```text expected
720
```
