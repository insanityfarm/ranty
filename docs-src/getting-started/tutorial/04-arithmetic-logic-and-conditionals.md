# Arithmetic, Logic, and Conditionals

You stop in the middle of the lane, with the gate behind you and both stalls close enough to watch. Juniper studies the first knot of early wanderers and points with the tip of her pencil.

"If the crowd swells, ring the bell. If Mira nods off, close the awning. If that kettle says anything prophetic, print it." You have heard shakier stage directions. What the market lacks now is judgment: numbers, yes-or-no answers, and a way to choose the right line at the right moment.

## Arithmetic

Ranty supports the usual arithmetic operators, including exponentiation with `**` and unary negation with `@neg`. On opening night these are often quick workshop checks: counts, totals, and little sanity tests before a line reaches the final script.

You try a quick total for Juniper's setup notes without thinking about precedence.

**Wrong attempt**

```ranty
# Two gate posts and three lantern strings of four.
2 + 3 * 4
```

**What happened**

```text
14
```

Multiplication happens before addition. If you want the addition first, you need parentheses.

```ranty example
# Add the posts first, then give each place four hooks.
(2 + 3) * 4
```

```text expected
20
```

```ranty example
# A few quick opening-night calculations for Juniper's setup notes.
(2 + 3 * 4)\n
(10 - 4)\n
(10 / 2)\n
(10 % 4)\n
(2 ** 5)\n
(@neg 3)
```

```text expected
14
6
5
2
32
-3
```

## Comparison, booleans, and logic

Comparisons return booleans, and booleans in Ranty are `@true` and `@false`. Think of them as the Moon-Market's yes-or-no answers.

```ranty example
# Each line asks a yes-or-no question about the lane.
(1 @eq 1)\n
(1 @neq 2)\n
(7 @gt 4)\n
(1 @lt 2)\n
(2 @ge 2)\n
(1 @le 1)
```

```text expected
@true
@true
@true
@true
@true
@true
```

Logic operators let you combine those answers.

```ranty example
# Bell ready and lantern oil ready.
(@true & @true)\n
# Exactly one omen flag is raised.
(@true ^ @false)\n
# No tickets sold yet means "not busy" is true.
(@not 0)\n
# If no omen is set, print the safe fallback.
(<> | "keep the lane clear")
```

```text expected
@true
@true
@true
keep the lane clear
```

That last line is especially handy in prose work: `a | b` means "use `a` if it is truthy, otherwise use `b`." It is the kind of operator Juniper reaches for when a note may or may not be present.

## Compound assignment

If a setter would immediately reuse the old value, you can shorten the spelling.

```ranty example
# Juniper adds more shells, then doubles the total for a rough estimate.
<$shells = 10>
<shells += 5>
<shells *= 2>
<shells>
```

```text expected
30
```

The same pattern works for the other arithmetic and logic operators too.

## Conditional expressions read almost like narration

Ranty's conditional syntax looks wordy at first, but that wordiness is useful: it reads like a tiny set of stage directions for the market.

```ranty example
# Decide what Juniper should do with the awning.
<$mood = sleepy>
@if <mood> @eq sleepy: {
  Close the awning.
} @elseif <mood> @eq busy: {
  Ring the bell.
} @else: {
  Brew more tea.
}
```

```text expected
Close the awning.
```

## Expressions can collapse into one value

This is a small rule with big consequences: when an expression prints several compatible values, Ranty combines them from left to right.

```ranty example
# Juniper writes the shell cost in two compatible pieces.
3
0.25
```

```text expected
3.25
```

That is why earlier examples could build strings and collections out of smaller pieces. Ranty is always trying to fold compatible outputs together.

More information about operators, booleans, conditional expressions, and compound assignment can be found in the Ranty documentation for [Operators](../../language/operators.md), [bool](../../language/data-types/bool.md), [Conditional expressions](../../language/conditional-expressions.md), and [Compound Assignment](../../language/accessors/compound-assignment.md).

The script can answer back now. Juniper tucks the pencil behind one ear and turns you toward the bakery stall, where repeating one line too many times is becoming a problem you can actually hear. The next fix is reuse.

Previous: [Lists, Tuples, Maps, and Access Paths](03-lists-tuples-maps-and-access-paths.md) | Next: [Function Calls, Functions, and Ranges](05-function-calls-functions-and-ranges.md)
