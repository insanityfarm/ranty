# Text

In Ranty, text is made from fragments, whitespace, and hinted expression units. Any text is also valid Ranty source, but whitespace is normalized unless you opt into other behavior.

## Fragments and same-line whitespace

Plain source text becomes output fragments. By default, same-line whitespace between adjacent fragments or hinted elements is normalized to a single ASCII space.

```ranty example
One  two   three
```

```text expected
One two three
```

Line breaks do not add spaces by themselves.

```ranty example
Water
melon
```

```text expected
Watermelon
```

Escaped whitespace such as `\s` and `\t` always prints literally.

## Hinting

A backtick before an expression unit marks it as **hinted**. Hinted units participate in surrounding whitespace as if they were ordinary fragments.

```ranty example
<$name = "world">Hello, `<name>!
```

```text expected
Hello, world!
```

Some units become implicitly hinted in text-heavy positions, and `@text` can mark definitions and parameters as auto-hinted. See [`@text`](keywords/text.md) for the full rules.

## Sinking

`~` does the opposite of a hint: it tells the compiler not to treat the next expression unit like text. This is useful when formatting code with spaces that should not survive into output.

```ranty example
{\:} ~{\(}
```

```text expected
:(
```

The sink and hint operators are compile-time annotations. Using them in unsupported positions is a compiler error.

## String literals

String literals count as single text units and preserve their contents without normal fragment splitting:

```ranty
"This string literal includes ""quoted"" text"
```

Multi-line string literals are also valid.

## Text vs. `string`

`string` is a runtime value type. Text is part of the source program. Normalization, hinting, sinking, and formatters can make the printed result differ from the original source spelling.
