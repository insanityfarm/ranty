# Text, Comments, Escapes, and Hinting

Juniper leads you straight to the gate and raps the bare signboard with her knuckles.

"Start there," she says. "If the first line fails, the rest goes with it." At the nearest stall, the baker sets a warm tray down to cool. Farther along the lane, a kettle answers its owner with a thin, offended hiss. You have seen grander fairs than this one, but never a good one that forgot to greet its guests. Here at the gate, before variables, functions, or selectors, the market needs words.

## Ordinary text is already a program

If you type text into a Ranty program, that text becomes output.

**Input**

```ranty example
Welcome to the Wandering Moon-Market.
```

**Output**

```text expected
Welcome to the Wandering Moon-Market.
```

That simplicity is why Ranty fits this job so neatly. You start with plain narration and add machinery only where the narration asks for it.

## Spaces, line breaks, comments, and escapes

Juniper squints at the board, writes one version too fast, and grimaces at the result.

**Wrong attempt**

```ranty
Welcome   wanderers
```

**What happened**

```text
Welcome wanderers
```

Ranty tidies up same-line spaces between plain text fragments. If you really want to keep extra spaces, print them explicitly.

```ranty example
# Juniper wants the stretched-out spacing to stay on the sign.
# `\s` prints one literal space each time you use it.
Welcome\s\s\swanderers
```

```text expected
Welcome   wanderers
```

When Juniper tries stacking two plain fragments on separate lines, Ranty still treats them as one continuous run of text.

```ranty example
Welcome
wanderers
```

```text expected
Welcomewanderers
```

Comments are ignored, so you can leave notes for your future self while you work on Juniper's sign and the pile of revisions that will follow.

```ranty example
# Print literal braces around the market name.
\{Wandering Moon-Market\}\n
# `\s` prints a real space even when spacing would normally collapse.
Gate opens:\smoonrise
```

```text expected
{Wandering Moon-Market}
Gate opens: moonrise
```

The backslashes above are escape sequences. `\n` prints a line break, `\s` prints a literal space, and `\{` or `\}` lets you print characters that Ranty would normally treat as syntax.

## String literals keep exact text

Most plain text is broken into fragments and normalized. A string literal is more exact: it keeps its contents together as one string value.

```ranty example
# The outer quote marks begin and end the string literal.
# Inside a string literal, write "" to print one literal " character.
"The sign reads ""Tonight only!"""
```

```text expected
The sign reads "Tonight only!"
```

If you have never seen quote escaping before, that doubled `""` is the whole trick. Two quote marks inside the string literal turn into one quote mark in the output.

## Hinting tells Ranty to treat the next expression like prose

A backtick is a hint. It tells Ranty, "The next thing is an expression, but please let it behave like part of the sentence around it."

```ranty example
# The backtick hints the next expression unit.
Welcome to `"The Wandering Moon-Market"!
```

```text expected
Welcome to The Wandering Moon-Market!
```

You might reasonably ask whether the backtick matters here at all.

```ranty
Tonight, "The Wandering Moon-Market" opens at dusk.
```

```text
Tonight, The Wandering Moon-Market opens at dusk.
```

And with a hint:

```ranty example
Tonight, `"The Wandering Moon-Market" opens at dusk.
```

```text expected
Tonight, The Wandering Moon-Market opens at dusk.
```

The result is identical. That is why hinting can feel mysterious at first.

With a simple expression such as a string literal, Ranty often already prints the sentence in a friendly way. The backtick is still useful because it marks your intent: "Treat the next expression as prose." Right here at the gate it mostly reads like tidy handwriting. Later, when Juniper asks you to drop computed text into a spoken line, the difference becomes visible.

More information about text, comments, escape sequences, and string values can be found in the Ranty documentation for [Text](../../language/text.md), [Comments](../../language/comments.md), [Escape Sequences](../../language/escape-sequences.md), and [string](../../language/data-types/string.md).

The sign takes the lantern light beautifully. The baker at the nearest stall lifts a floury wrist in approval. Juniper turns from the gate toward that counter and the stack of blank cards waiting there. The market can greet strangers now. Next it needs a memory.

Previous: [Tutorial](../tutorial.md) | Next: [Accessors, Variables, Constants, and Nothing](02-accessors-variables-constants-and-nothing.md)
