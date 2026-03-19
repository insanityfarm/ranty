# Accessors, Variables, Constants, and Nothing

Juniper brings you to the bakery stall nearest the gate and drops a stack of blank cards onto the counter.

The baker catches the top card before it slides, dusts flour from her thumb, and says, "Mira. If these things are going to remember me, start with the buns." "The market name never changes," Juniper adds. "Everything else does." Mira calls out a new tray count without looking up from the oven. Somewhere farther down the lane a kettle lid clatters, but here at the counter the problem is simple: fixed facts, changing facts, and cards that should know the difference.

## Defining and reading values

`<$name = ...>` defines a mutable variable. `<name>` reads it back out. Your first task is to make one clean stall line for Mira.

```ranty example
# Store three details about Mira's stall at once.
<$vendor = "Mira"; $snack = "star buns"; $price = 3>
Tonight, `<vendor> sells `<snack> for `<price> shells.
```

```text expected
Tonight, Mira sells star buns for 3 shells.
```

That one line also shows a multi-part accessor: you can do several accessor operations inside one pair of angle brackets by separating them with `;`. It is a nice fit for stall cards and setup notes, where several related facts often arrive together.

## Mutable variables, constants, and setters

Ranty gives you two common kinds of named storage:

- `$name` is a mutable variable. You expect to change it later.
- `%name` is a constant. You want it to stay fixed once defined.

That distinction matters here. Lantern counts change. The carved name above the gate does not.

```ranty example
# `%` makes a constant for facts that should stay fixed.
<%market-name = "Wandering Moon-Market">
# `$` makes a variable for facts Juniper may revise.
<$lanterns = 2>
# This setter updates the variable when another lantern bundle arrives.
<lanterns = 3>
Market header: `<market-name>\n
Lanterns above Mira's stall: `<lanterns>
```

```text expected
Market header: Wandering Moon-Market
Lanterns above Mira's stall: 3
```

Use `%` for carved facts and `$` for moving pieces. If you mark a changing value as fixed, Ranty objects at once:

**Wrong attempt**

```ranty
# Juniper mistakenly marks the lantern count as fixed.
<%lantern-count = 5>
<lantern-count = 6>
```

**What happened**

```text
error[R0100]: reassignment of known constant 'lantern-count'
...
Compile failed (1 error found)
```

If Juniper truly expects the number to change, it should have been a variable instead:

```ranty example
# This time the changing count is stored in `$`.
<$lantern-count = 5>
<lantern-count = 6>
Lanterns above Mira's stall: `<lantern-count>
```

```text expected
Lanterns above Mira's stall: 6
```

So when you choose between `$` and `%`, you are really choosing whether that piece of market data is meant to change.

## Missing values, fallbacks, and the `nothing` literal

Sometimes a stall card simply does not have a detail yet. Ranty has a special value for that idea: `<>`, pronounced "nothing." Think of it as a deliberate blank, not a dramatic error.

On its own, `nothing` prints as nothing at all. A missing read without any protection is an error, but a fallback gives you a gentler result.

```ranty example
# Juniper knows the title, but Mira has not picked a subtitle yet.
<$title = "Moon pears">
Card title: `<title?>\n
Card subtitle:\n
<subtitle?>\n
Safe subtitle: `<subtitle ? "(no subtitle yet)">
```

```text expected
Card title: Moon pears
Card subtitle:

Safe subtitle: (no subtitle yet)
```

Two useful details are hiding in there:

- `<subtitle?>` means "try to read it; if it is missing, quietly produce nothing."
- `<subtitle ? "...">` means "if it is missing, use this fallback instead."

## Lazy definitions wait until you actually read them

Sometimes you need to set wording aside until the last possible moment. That is what `?=` means.

```ranty example
# Juniper starts with one opening word.
<$opening-word = dawn>
# Define `chant` lazily: do not read `opening-word` yet.
<$chant ?= <opening-word>>
# She changes the word before `chant` is ever read.
<opening-word = dusk>
Opening chant: `<chant>
```

```text expected
Opening chant: dusk
```

Because the definition was lazy, `<opening-word>` was not read until `<chant>` was read. By then, Juniper had already changed the opening word from `dawn` to `dusk`, which is exactly what she wanted.

More information about accessors, fallbacks, and `nothing` can be found in the Ranty documentation for [Accessors](../../language/accessors.md), [Fallbacks](../../language/accessors/fallbacks.md), and [nothing](../../language/data-types/nothing.md).

Mira's stall card stops changing shape every time a bun leaves the oven. Juniper stacks the finished cards, then nods farther down the lane toward a tea stall half-buried under labels and kettle tags. One counter is in order. The next one is worse.

Previous: [Text, Comments, Escapes, and Hinting](01-text-comments-escapes-and-hinting.md) | Next: [Lists, Tuples, Maps, and Access Paths](03-lists-tuples-maps-and-access-paths.md)
