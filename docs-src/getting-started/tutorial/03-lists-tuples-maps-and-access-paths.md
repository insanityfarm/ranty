# Lists, Tuples, Maps, and Access Paths

Juniper leads you farther down the lane to the tea stall, where a narrow prep board has vanished beneath labels, kettle tags, and one chipped cup full of pins.

The man guarding the mess taps the nearest kettle with one fingernail and says, "Tavi. If you can sort this lot, I will owe you tea." Bakery items run down one margin of Juniper's clipboard. Kettle labels crawl up another. Stall facts fill every gap between. You remember caravan clerks who tried to tame this sort of mess with string and colored chalk. Ranty has kinder containers than string.

## Lists, tuples, and maps in plain English

If you are not a programmer, treat these as three different kinds of storage furniture:

- A `list` is a shelf of items in order. You can replace items, remove them, or add more later.
- A `tuple` is a tied bundle of items in order. You use it when the shape itself is meant to stay fixed.
- A `map` is a cabinet of labeled drawers. You look values up by name instead of by position.

Those ideas matter more than the terminology. The syntax is just how you tell Ranty what shape this heap of market notes ought to take.

## Lists and tuples keep ordered data

Tavi slides two different scraps across the prep board: one loose row of kettle tags that may be reordered, and one fixed three-part kettle call that always stays in the same order. That is the difference between a list and a tuple.

- Lists use `(: item; item; item)`.
- Tuples use `(item; item; item)`.
- In both, semicolons separate items.
- This tutorial writes one space after each semicolon to keep the items easy to scan.

```ranty example
# A list is an ordered shelf Tavi may reshuffle later.
<$goods = (: mint-steam; ember-tea; moon-sugar)>
# A tuple is an ordered bundle whose three parts belong together.
<$kettle-call = (pour; steep; serve)>
First tag: <goods/0>\n
Last tag: <goods/-1>\n
Middle step: <kettle-call/1>
```

```text expected
First tag: mint-steam
Last tag: moon-sugar
Middle step: steep
```

The `/0`, `/-1`, and `/1` parts are access paths. They let you reach into an ordered container by position. Indices are zero-based, so `/0` means "the first item" and `/1` means "the second item." Negative indices count backward from the end, which is handy when Juniper only cares about the last tag on a row.

Lists and tuples may both be ordered, but they are not interchangeable. A list is mutable and resizable. A tuple is immutable and fixed-shape. Use a list when the number of items may change. Use a tuple when the structure itself is part of the meaning.

If you ever need a one-item tuple later, write it with a trailing semicolon, like `(hush;)`. That extra semicolon is what marks it as a tuple with one item.

## A list can work, but this job wants labels

Juniper taps three lines on the clipboard: stall name, stall kind, and the blend for tonight. "I can keep the order straight for one stall," she says, "but not for twelve." A list can store those facts, but only if everyone remembers which numbered slot means what.

**A first pass**

```ranty
# Slot 0 is the stall name, slot 1 is the kind, slot 2 is the night blend.
<$stall = (: "Tavi's Tea Stall"; tea; blue-steam)>
<stall/0>\n
<stall/1>\n
<stall/2>
```

**What happened**

```text
Tavi's Tea Stall
tea
blue-steam
```

That code is valid, and it does work. It is just not good enough for this task. Juniper has too many moving parts for numbered memory games. She wants labeled drawers instead of shelf positions.

## Maps hold named facts

Maps use `(:: ... )`.

```ranty example
# Build a stall record for Tavi's prep board.
<$secret-key = "aftertaste">
<$stall-name = "Tavi's Tea Stall">
<$kind = tea>
<$stall = (::
  # Shorthand: use the current values of these variables.
  <stall-name>;
  <kind>;
  # A quoted key can contain spaces.
  "night blend" = blue-steam;
  # A dynamic key lets Juniper choose the drawer name at runtime.
  (<secret-key>) = moon-peel;
)>
<stall/stall-name>\n
<stall/kind>\n
<stall/("night blend")>\n
<stall/(<secret-key>)>
```

```text expected
Tavi's Tea Stall
tea
blue-steam
moon-peel
```

This one record gives Juniper one tidy record she can actually trust, and it also shows four useful map tricks:

- `<stall-name>;` is shorthand for `stall-name = <stall-name>;`
- plain identifiers such as `kind` work as keys
- quoted keys can contain spaces
- `(<secret-key>)` creates a key from another value

## An empty shelf is not the same as a shelf holding `nothing`

This is where `nothing` becomes useful in a practical way. You need to distinguish between "this slot exists but is blank" and "this slot does not exist yet."

```ranty example
# One shelf contains a real first slot whose value is `nothing`.
<$one-gap = (: <>)>
# This shelf has no first slot at all.
<$empty = (:)>
First slot: <one-gap/0 ? "missing">\n
Second slot: <one-gap/1 ? "missing">\n
Empty shelf: <empty/0 ? "missing">
```

```text expected
First slot: 
Second slot: missing
Empty shelf: missing
```

The first shelf really does have an item at index `0`; it just happens to be `nothing`, so nothing prints after `First slot:`. The fallback does not run there because the read succeeded. The second and third reads are genuinely missing, so their fallbacks do run.

## Slices read parts of a list, and splices replace them

A slice reads a chunk of an ordered collection. A splice writes one back in. This is exactly what you want when Tavi renames the middle of a row of kettle tags without rewriting the whole board.

```ranty example
# Start with four label drafts.
<$goods = (: mint-steam; dusk-ginger; bell-anise; rain-clove)>
# Read items 1 through 2.
<goods/1..3>\n
# Replace a shorter slice with a longer list.
<goods/1..2 = (: blue-steam; star-peel)>
<goods>
```

```text expected
(: dusk-ginger; bell-anise)
(: mint-steam; blue-steam; star-peel; bell-anise; rain-clove)
```

`1..3` means "start at index 1, stop before index 3."

## Anonymous accessors let you reach into a value directly

An anonymous accessor lets you index into a value without storing it first. It is a good shortcut when you are moving too fast to wait for extra setup.

```ranty example
# Index directly into a tag literal when Tavi only needs the first letter.
<("blue-steam")/0>
```

```text expected
b
```

This example says, "Take this string literal, treat it like a collection, and give me its first item."

More information about lists, tuples, maps, and access paths can be found in the Ranty documentation for [list](../../language/data-types/list.md), [tuple](../../language/data-types/tuple.md), [map](../../language/data-types/map.md), and [Access Paths](../../language/accessors/access-paths.md).

> See also: [Anonymous Accessors](../../language/accessors/anonymous.md)

The prep board starts to look deliberate. Tavi's labels have names, and the bakery notes on Juniper's clipboard finally sit still beside them. Juniper takes the clipboard back toward the middle of the lane, where the first early wanderers are beginning to bunch beneath the awnings. Next the script must learn judgment.

Previous: [Accessors, Variables, Constants, and Nothing](02-accessors-variables-constants-and-nothing.md) | Next: [Arithmetic, Logic, and Conditionals](04-arithmetic-logic-and-conditionals.md)
