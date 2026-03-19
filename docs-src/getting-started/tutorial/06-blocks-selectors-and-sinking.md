# Blocks, Selectors, and Sinking

You follow Juniper back to the middle of the lane, with the gate at your back and both stalls working within earshot. By now the market smells right: sweet bread, lamp smoke, wet canvas, hot brass. It still sounds flat.

Juniper sweeps a hand at the lane. "I need stray bells. A prop here. One omen there. Not the same three details forever." You have seen promising fairs die of repetition. Blocks are how you keep this one moving.

## A plain block picks one option

By default, a block chooses one element. Juniper uses that to pull one prop for an ambient note.

```ranty example
A {lantern|bell|map} swings above the lane.
```

```text expected
A map swings above the lane.
```

The result is random by default. In this tutorial, the docs verifier uses a fixed seed so the examples stay checkable.

## Sinking packs the next unit tightly

Back on the sign page, Juniper learned that Ranty pays close attention to spacing. Here that matters again, because sound effects and stage cues often need tighter control than ordinary prose.

**Wrong attempt**

```ranty
bell {gong} bell
```

**What happened**

```text
bell gong bell
```

Those spaces survived because the block was treated like ordinary text in the sentence.

```ranty example
# The sink removes the spaces that would normally survive around the block.
bell ~{gong} bell
```

```text expected
bellgongbell
```

The `~` sink tells Ranty not to preserve text-style spacing around the next unit.

## Repetition, separators, and selector modes

Selectors decide how a block walks through its elements. Here you use one to draft a repeating ambient loop for the lane.

```ranty example
# Repeat four picks, separated by commas, moving forward through the block.
Ambient loop: `[rep:4][sep:", "][sel:[mksel:forward]]{mist|music|tea}
```

```text expected
Ambient loop: mist, music, tea, mist
```

`forward` walks from left to right and wraps back to the start.

## Entanglement keeps distant choices in sync

If two blocks share the same selector handle, they can stay in sync.

```ranty example
# Reuse one selector so both blocks make the same pick.
<$sync = [mksel:one]>
Tavi's omen card reads: `[sel:<sync>]{cat|owl} carries `[sel:<sync>]{lantern|ink}.
```

```text expected
Tavi's omen card reads: owl carries ink.
```

The selector picked once and reused the same choice in both places, giving Tavi one coherent omen instead of two unrelated scraps.

## Weights and match selection

`@weight` changes how likely an element is to be chosen. `@on` tags an element for `match` selectors.

```ranty example
# Only the `rare` entries can match, and weight 0 removes one of them.
Rare jar: `[match: rare]{ordinary|meteor-sugar @on rare @weight 1|sleep-dust @on rare @weight 0}
```

```text expected
Rare jar: meteor-sugar
```

Because only the tagged `rare` elements are eligible, and one of them has weight `0`, the result becomes deterministic.

> See also: [@weight](../../language/keywords/weight.md), [@on](../../language/keywords/on.md)

## Selector handles are real values

You can store a selector, inspect its type, skip it forward, and freeze it in place. That turns random-looking tables into something you can actually reason about.

```ranty example
# Build a reusable route selector for ambient directions.
<$route = [mksel:forward]>
First route: `[sel:<route>]{north|east|south}\n
Selector kind: `[type: <route>]\n
[sel-skip: <route>]
[sel-freeze: <route>]
Frozen now: `[sel-frozen: <route>]\n
Next route: `[sel:<route>]{north|east|south}
```

```text expected
First route: north
Selector kind: selector
Frozen now: @true
Next route: south
```

More information about blocks, selectors, and sinked text can be found in the Ranty documentation for [Blocks](../../language/blocks.md), [selector](../../language/data-types/selector.md), and [Text](../../language/text.md).

The lane starts to breathe. From here you can hear Mira's stall bustle and Tavi's omens stop tripping over themselves. Juniper, hearing the rhythm improve, starts counting under her breath. That makes the next problem obvious: timing.

Previous: [Function Calls, Functions, and Ranges](05-function-calls-functions-and-ranges.md) | Next: [Repetition State, Control Flow, and Attributes](07-repetition-state-control-flow-and-attributes.md)
