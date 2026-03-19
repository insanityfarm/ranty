# Repetition State, Control Flow, and Attributes

You stay in the strip of lane between the gate and the stalls while the first queue thickens. Juniper taps the clipboard against your wrist. "Three beats here. Stop there. Let that one carry."

From this spot you can see Mira stacking trays in careful towers and Tavi counting kettle knocks under his breath, losing count whenever someone speaks to him. You remember queue-keepers at river ferries doing the same work with chalk, bells, and nerves. Now the script needs timing.

## `@step`, `[step]`, and `@total`

`@step` is zero-based. `[step]` is one-based. `@total` is the full planned count. This is the sort of count Juniper uses when checking whether a repeated cue is landing in the right place.

```ranty example
[rep:3][sep:" | "]{
  # Show zero-based step, one-based step, and total planned count.
  [cat: @step; "/"; [step]; "/"; @total]
}
```

```text expected
0/1/3 | 1/2/3 | 2/3/3
```

That tiny difference between zero-based and one-based counting matters often enough that it is worth learning early.

## `@continue` and `@break`

These two charms interrupt the nearest repeater. They are useful when you want to skip or stop a repeated cue without rewriting the whole block.

```ranty example
# Skip the rest of each iteration.
[rep:3]{bell @continue hush}\n
# Stop the repeater the first time it runs.
[rep:3]{bell @break hush}
```

```text expected
hushhushhush
hush
```

With an expression, each charm replaces the repeater's current output with the value on its right.

> See also: [@continue](../../language/keywords/continue.md), [@break](../../language/keywords/break.md)

## Protected blocks

At first, you forget the protection wrapper and let one block eat the repetition meant for another.

**Wrong attempt**

```ranty
[rep:3]{ hush{bell} }\n
{gong}
```

**What happened**

```text
hushbellhushbellhushbell
gong
```

The first block consumed the repetition setting, so the `gong` block only ran once.

```ranty example
[rep:3]@{ hush{bell} }\n
{gong}
```

```text expected
hushbell
gonggonggong
```

A protected block starts with `@{...}`. It keeps attribute changes from leaking and does not consume the caller's attributes.

> See also: [Protected blocks](../../language/blocks/protected-blocks.md)

## Reading and writing attribute state

The mutable attribute keywords have accessor forms too. That means you can store the current pacing rules, tweak them, and read them back when needed.

```ranty example
# Store the current separator, then reuse it.
<@sep = ", ">[rep:3][sep:<@sep>]{wait}
```

```text expected
wait, wait, wait
```

That is the same separator value being written to the current attribute frame and then read back out.

## Attribute keyword sugar

The keyword forms are shorter when you only want to affect the next block, which is common when Tavi just wants one quick sound cue.

```ranty example
@rep 3: {tap}\n
@sel "forward": {left|right}
```

```text expected
taptaptap
left
```

## Mutators

A mutator receives the current block element as a callback and can decide how to print it.

```ranty example
# Turn each selected element into uppercase before it prints.
@mut [?: elem] { [upper: [elem]] }: {wait}
```

```text expected
WAIT
```

If attributes feel mysterious, the simplest mental model is this: every block grabs the current attribute frame, uses it once, and then resets the consumed values. Protected blocks simply refuse to eat what they were handed.

> See also: [@mut](../../language/keywords/mut.md)

More information about repetition state, block attributes, and timing helpers can be found in the Ranty documentation for [Attributes](../../runtime/attributes.md), [@step](../../language/keywords/step.md), [@total](../../language/keywords/total.md), and [Blocks](../../language/blocks.md).

The queue outside the awnings starts to move like someone meant it to. Juniper stops tapping for a full breath. Then, from the tea stall, Tavi opens his notebook, and you see the next problem at once: the logic works, but the page has turned into brackets all the way down.

Previous: [Blocks, Selectors, and Sinking](06-blocks-selectors-and-sinking.md) | Next: [Pipes and Argument Spreading](08-pipes-and-argument-spreading.md)
