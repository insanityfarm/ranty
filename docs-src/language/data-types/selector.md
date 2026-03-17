# The `selector` type

The `selector` type is a handle to block-selection state. Selectors can be created with
[`[mksel]`](../../stdlib/control-flow.md#mksel) or, for match selection, by setting the active
selector with [`[match]`](../../stdlib/control-flow.md#match).

## Cursor-driven selectors

Most selector modes advance through a block using an internal cursor. Those selectors support the
standard cursor operations:

- [`[sel-skip]`](../../stdlib/control-flow.md#sel-skip)
- [`[sel-freeze]`](../../stdlib/control-flow.md#sel-freeze)
- [`[sel-frozen]`](../../stdlib/control-flow.md#sel-frozen)

## Match selectors

The `match` mode is value-driven instead of cursor-driven. When a match selector is applied to a
block, it compares its stored match value against each element's [`@on`](../keywords/on.md) trigger:

- matching tagged elements form the candidate pool,
- if none match, untagged elements become the fallback pool,
- if that pool is also empty, the block raises a selector error.

Because match selectors do not advance through a cursor, `sel-skip`, `sel-freeze`, and `sel-frozen`
are runtime errors for them.
