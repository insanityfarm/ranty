# @on

The `@on` keyword tags a block element with a match trigger. It is only valid as suffix metadata
inside a block element.

`@on` works with `match` selectors created by `[mksel: match; value]` or by the `[match: value]`
helper.

## Resolution rules

When a block is resolved with a `match` selector:

- all elements whose `@on` value equals the selector's match value form the candidate pool,
- if none match, all untagged elements become the fallback pool,
- if that pool is also empty, the block raises a runtime selector error.

If the block is weighted, the final choice is made using the weights inside the selected pool.
`@on` expressions are evaluated once when the block is prepared, not once per iteration.

Each block element may use at most one `@on` and one `@weight`. When both are present, they can
appear in either order after any optional `@edit` prefix.

```rant example
[match: foo]{yes @on foo|no @on bar|fallback}
```

```text expected
yes
```

`@on` outside a block element is a compile-time error.
