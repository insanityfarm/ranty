# Function Calls, Functions, and Ranges

Back at the bakery stall, Mira catches you between customers and gives the same greeting to two different passersby in exactly the same cadence.

"Hear that?" Juniper says from the counter beside you. She tips her head down the lane, where Tavi has his notebook open at the tea stall. "Useful once. Tiresome twice." You have heard good hawkers braid one line into six without losing the tune. That is the problem here. The words are fine. They simply need to do more than once.

## Square brackets call functions

Square brackets are how you call a function. The general shape is `[name: arg; arg]`. Mira starts with the smallest possible request: one louder banner line for a tray that keeps selling out.

```ranty example
# Call one built-in helper with one argument.
[upper: saffron buns]
```

```text expected
SAFFRON BUNS
```

The name before the colon is the function name. The values after the colon are its arguments. That one pattern powers a huge amount of Ranty. Built-in helpers use it, and later your own named functions will use it too.

## Hinting matters more once the expression is computed

Back on the welcome-sign page, hinting looked almost optional. Now Mira asks for a shouted banner line, and you can finally see the case where it matters in a visible way.

**Wrong attempt**

```ranty
Call this out: [upper: saffron buns].
```

**What happened**

```text
Call this out:SAFFRON BUNS .
```

The computed function call did not settle into the sentence like an ordinary text fragment. This is the kind of case the backtick is for.

```ranty example
Call this out: `[upper: saffron buns].
```

```text expected
Call this out: SAFFRON BUNS.
```

Hinting does not exist to decorate every expression. It exists so you can tell Ranty, "This computed thing belongs in the prose flow."

## Ranges are compact number sequences

You also need simple ordered counts for market tasks. A range is a compact number sequence built by a function call.

```ranty example
# Count the three stall markers Juniper wants to hang.
<$steps = [range: 2; 5]>
First marker: `<steps/0>\n
Second marker: `<steps/1>\n
Last marker: `<steps/-1>
```

```text expected
First marker: 2
Second marker: 3
Last marker: 4
```

## Named functions and default parameters

Now that square brackets have a job, you can define functions of your own.

```ranty example
# A reusable greeting for vendors.
[$greet: name; title ? "vendor"] {
  Hello, `<name> the `<title>!
}

[greet: Mira]\n
[greet: Tavi; dream-chef]
```

```text expected
Hello, Mira the vendor!
Hello, Tavi the dream-chef!
```

The parameter `title ? "vendor"` is optional and has a default value. If the caller omits it, `"vendor"` is used.

> See also: [Optional parameters](../../language/functions/optional-parameters.md)

## Variadic parameters and `[call]`

Sometimes Juniper does not know in advance how many words a chant will need. Variadic parameters gather many arguments into one list.

```ranty example
[$chant: words+] {
  [join: <words>; " / "]
}

[chant: moon; bell; bread]\n
[call: <chant>; (: hush; now)]
```

```text expected
moon / bell / bread
hush / now
```

`+` means "one or more values." `*` would mean "zero or more values."

> See also: [Variadic parameters](../../language/functions/variadic-parameters.md)

## Lambdas and closures

Lambdas are anonymous functions. Closures are functions that remember the variables around them.

```ranty example
# Build a little bell counter for Tavi's test chimes.
[$make-counter: start ? 0] {
  <$count = <start>>
  [?] {
    <count += 1>
    <count>
  }
}

<$next-bell = [make-counter: 2]>
[next-bell],\s[next-bell],\s[next-bell]
```

```text expected
3, 4, 5
```

Each call to `[next-bell]` reaches back into the captured `count` variable and updates it.

> See also: [Lambdas](../../language/functions/lambdas.md)

## Lazy parameters

`@lazy` means the caller's argument is not evaluated until the function actually reads it.

```ranty example
# If Juniper omits the note, the default is only evaluated when needed.
[$preview: @lazy note ? [upper: hush]] {
  First look: <note>
}

[preview]
```

```text expected
First look: HUSH
```

The laziness matters most when the argument would be expensive, noisy, or random and you might never need it, which is often true for opening-night flavor lines.

## `@return` leaves a function early

`@return` stops the current function immediately.

```ranty example
[$sold-out] {
  Tea is ready. @return Come back tomorrow.
}

[sold-out]
```

```text expected
Come back tomorrow.
```

> See also: [@return](../../language/keywords/return.md)

## Shadowing, descoping, and globals

Scopes can reuse the same names. Descoping lets you climb outward on purpose, and a leading `/` lets you talk to the global scope directly.

```ranty example
<$voice = outer>
{
  <$voice = middle>
  {
    <$voice = inner>
    <^^voice>
  }
}\n
# Store the official market name globally.
<$/market-name = "Wandering Moon-Market">
# Shadow it locally.
<$market-name = "Pocket Bazaar">
</market-name>
```

```text expected
outer
Wandering Moon-Market
```

`<^^voice>` climbs two local scopes. `</market-name>` reads the global value even though a local variable shadows it.

> See also: [Globals & Descoping](../../language/accessors/globals-descoping.md)

## Function percolation

Even if a non-function variable shadows a function name, a call can still climb outward until it finds a function.

```ranty example
[$ring-bell] { ding }
{
  <$ring-bell = "not a function">
  [ring-bell]
}
```

```text expected
ding
```

More information about function calls, user-defined functions, and ranges can be found in the Ranty documentation for [Functions](../../language/functions.md) and [range](../../language/data-types/range.md).

Now Mira can reuse a greeting without wearing it thin, and Tavi can tuck little tricks into his notebook instead of rewriting them in every margin. Juniper steps back into the middle of the lane and lifts a hand toward the awnings and lanterns. "Better," she says. "Now make it feel alive."

Previous: [Arithmetic, Logic, and Conditionals](04-arithmetic-logic-and-conditionals.md) | Next: [Blocks, Selectors, and Sinking](06-blocks-selectors-and-sinking.md)
