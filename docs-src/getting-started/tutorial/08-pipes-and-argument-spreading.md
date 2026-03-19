# Pipes and Argument Spreading

You cross to the tea stall, where Tavi clears a space on the counter with the expression of someone handling a mildly cursed object. He lays the notebook between the kettles and looks at you hopefully.

Every useful line is nested inside three more useful lines. Even Juniper, reading over your shoulder, winces. You have untangled worse, but not by staring harder. You need cleaner flow.

## A simple pipe

```ranty example
# Keep only the longer banner words from Tavi's draft list.
[split: "mist bells awnings tea"; \s
  |> filter: []; [?: word] { [len: <word>] @gt 4 }
  |> join: ", "]
```

```text expected
bells, awnings
```

Read that like a little assembly line:

1. split the string into a list,
2. keep only words longer than three characters,
3. join the survivors back together.

## Pipeval and the assignment pipe

`[]` means "the value currently moving through the pipe." The next example uses it for one arithmetic check and one saved shout cue.

```ranty example
[$make-adder: a] {
  [?: b] { <a> + <b> }
}

# Feed the pipe value into a returned function.
[cat: "Bell count check: "; [make-adder: 7 |> []: 5]; \n]
# Store a pipe result without printing it.
[upper: "soft bells" > $shout]
[cat: "Saved shout: "; <shout>]
```

```text expected
Bell count check: 12
Saved shout: SOFT BELLS
```

The assignment pipe `>` stores the current pipe value in a variable and is automatically sinked, so it does not print by itself.

## Parametric spread

`*` unpacks a list into ordinary function arguments. That is perfect for Tavi's notebook lines, where one prepared list needs to become several arguments.

At first, you forget the `*` and hand the whole list over as one argument.

**Wrong attempt**

```ranty
[$omen-line: parts*] { [join: <parts>; ", "] }
<$parts = (: blue-steam; kettle-song; moon-peel)>
[omen-line: <parts>]
```

**What happened**

```text
(: blue-steam; kettle-song; moon-peel)
```

Without the spread operator, the function receives one argument whose value is the whole list.

```ranty example
[$omen-line: parts*] {
  [join: <parts>; ", "]
}

<$parts = (: blue-steam; kettle-song; moon-peel)>
[omen-line: *<parts>]
```

```text expected
blue-steam, kettle-song, moon-peel
```

## Temporal spread

`**` is stranger and more fun: it calls the function once for each item in the list. Tavi uses that sort of trick for repeated omen beats.

```ranty example
[$announce: item] {
  Omen: `<item>\n
}

[announce: **(: blue-steam; three-bells)]
```

```text expected
Omen: blue-steam
Omen: three-bells
```

## Synchronized and complex spread

Labeled temporal spreads move in lockstep. Complex spread does temporal and parametric spread at once. Both are useful when paired bits of scene data must stay matched.

```ranty example
# The matching labels keep the two lists paired row by row.
[cat: *pair*(: moon; star); " + "; *pair*(: tea; jam); \n]
```

```text expected
moon + tea
star + jam
```

```ranty example
[$pair-line: left; right] {
  [cat: <left>; " + "; <right>; \n]
}

[pair-line: ***(: (: moon; tea); (: star; jam))]
```

```text expected
moon + tea
star + jam
```

## A couple more useful helpers

`[map]` transforms each element. `[zip]` walks two lists together. They are small helpers, but they clean up a lot of market prep work.

```ranty example
[map: (: hush; steam); [?: item] { [upper: <item>] }]\n
[zip: (: moon; star); (: steam; jam); [?: left; right] { [cat: <left>; "-"; <right>] }]
```

```text expected
(: HUSH; STEAM)
(: moon-steam; star-jam)
```

More information about pipes and spreading can be found in the Ranty documentation for [Piping](../../language/functions/piping.md), [Argument spreading](../../language/functions/argument-spreading.md), and [Variadic parameters](../../language/functions/variadic-parameters.md).

The notebook reads from left to right again. Tavi smooths the page as if it might bite him less now. Juniper takes the script back and leads you toward the gate lantern, where the final pages will be read aloud. "Good," she says. "Now make it sound as if we meant it all along."

Previous: [Repetition State, Control Flow, and Attributes](07-repetition-state-control-flow-and-attributes.md) | Next: [Output Editing and Formatting](09-output-editing-and-formatting.md)
