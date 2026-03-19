# Output Editing and Formatting

Under the lantern by the gate, Juniper reads the latest pages aloud, then hands them over without a word.

From here you can hear the market gathering behind you, but only the script is in your hands. That is how you know you are close. The bakery lines are accurate but stiff. The omen lines are eerie in the wrong places. You know this stage of the work: the script functions, and still does not sound like itself. This page is about revising output after it exists, helping inserted values sit naturally in prose, and adding the last bit of shine.

## `@edit` can rewrite what came before

The `@edit` operator grabs the caller's current output and lets you replace it.

```ranty example
"Opening bell" { @edit x: `<x>, then `<x> again. }
```

```text expected
Opening bell, then Opening bell again.
```

If you do not need the old output, you can discard it completely.

```ranty example
"rough note" { @edit: "Opening bell." }
```

```text expected
Opening bell.
```

## `@text` for values, parameters, and whole functions

Auto-hinting is a wonderful gift to prose-heavy code. It tells Ranty, "Whenever this value shows up in text, give it proper breathing room."

At first, you write the sentence without `@text`.

**Wrong attempt**

```ranty
[$pitch: treat] { Fresh <treat> for moonlit travelers. }
Say hello to Mira. [pitch: saffron buns]
```

**What happened**

```text
Say hello to Mira.Fresh saffron buns for moonlit travelers.
```

The function result is correct, but it lands in the prose too tightly.

```ranty example
# Auto-hint this stored value.
<@text $vendor = Mira>

# Auto-hint the parameter and the whole function result.
[$pitch: @text treat] @text {
  Fresh <treat> for moonlit travelers.
}

Say hello to <vendor>. [pitch: saffron buns]
```

```text expected
Say hello to Mira. Fresh saffron buns for moonlit travelers.
```

## Whitespace formatting

`[ws-fmt]` changes how printed whitespace is normalized in the current scope. Juniper uses that when she wants to test how a sign or chant will breathe on the page.

```ranty example
[ws-fmt: verbatim]Welcome   travelers\n
[ws-fmt: ignore-all]Welcome   travelers
```

```text expected
Welcome   travelers
Welcometravelers
```

## Number formatting

`[num-fmt]` changes how numbers print when they appear in text output. Tavi likes this for labels and coded tokens.

```ranty example
# Print 255 as an alternate, uppercase, padded hexadecimal value.
Lantern tag: `[num-fmt: (:: system = hex; upper = @true; alt = @true; padding = 4)]255\n
```

```text expected
Lantern tag: 0x00FF
```

## A few finishing tools

These helpers are not new syntax, but they are excellent for final cleanup.

```ranty example
[trim: "  opening bell  "]\n
[upper: hush]\n
[lower: GLOW]
```

```text expected
opening bell
HUSH
glow
```

More information about output polish can be found in the Ranty documentation for [Output modifiers](../../language/output-modifiers.md), [@text](../../language/keywords/text.md), [Whitespace](../../runtime/whitespace-formatter.md), [Numbers](../../runtime/number-formatter.md), and [Strings](../../stdlib/strings.md).

At last the words sound at home beneath these awnings. The bakery patter is warm. The omen lines prickle in the right places. Juniper folds the pages once, crisply, and carries them to the trunk beside the gate where the last parts are waiting. One task remains: make the whole contraption sturdy enough to survive the night.

Previous: [Pipes and Argument Spreading](08-pipes-and-argument-spreading.md) | Next: [Prototypes, Modules, and Determinism](10-prototypes-modules-and-determinism.md)
