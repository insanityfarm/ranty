# Comparison of Rant 3 and Ranty

[Rant 3](https://github.com/TheBerkin/rant3) (deprecated as of June 2020) was released in April 2017. Since then, several critical design flaws have been identified in the language.
Ranty aims to address all of those flaws while retaining the ergonomics of the original design.

Ranty is a fork of [Rant 4](https://github.com/rant-lang/rant), which was intended as a ground-up rewrite (in Rust) from the original Rant developer but sadly appears to have been abandoned before a stable release could be completed.

It is important to note that Rant 3 code is _not_ forwards-compatible with either Rant 4 or Ranty. Similarly, Ranty extends Rant 4 but should not be assumed interoperable with it. All three should be regarded as separate languages.

## Feature breakdown

| Feature name                                |          Rant 3           |            Ranty             |
|---------------------------------------------|:-------------------------:|:-----------------------------:|
| Runtime platform                            |         .NET-only         |            Native             |
| Blocks                                      |         &#x2705;          |           &#x2705;            |
| Block weights                               |         &#x2705;          |           &#x2705;            |
| Selectors                                   |      *(named only)*       |           &#x2705;            |
| Queries                                     |         &#x2705;          |           &#x274c;            |
| Output channels                             |         &#x2705;          |           &#x274c;            |
| Hints/sinks                                 |         &#x274c;          |           &#x2705;            |
| Variables                                   |    *(via stdlib only)*    |           &#x2705;            |
| Variable access fallbacks                   |         &#x274c;          |           &#x2705;            |
| Operators                                   |    *(via stdlib only)*    |           &#x2705;            |
| Named functions                             |         &#x2705;          |           &#x2705;            |
| Anonymous functions                         |         &#x274c;          |           &#x2705;            |
| Piping                                      |         &#x274c;          |           &#x2705;            |
| Variable capture in functions               |         &#x274c;          |           &#x2705;            |
| Variadic function parameters                | *(stdlib functions only)* |           &#x2705;            |
| Optional function parameters                | *(stdlib functions only)* |           &#x2705;            |
| Parameter spread notation                   |         &#x274c;          |           &#x2705;            |
| Collection initializers                     |         &#x274c;          |           &#x2705;            |
| Conditional expressions                     |         &#x274c;          |           &#x2705;            |
| Slice notation                              |         &#x274c;          |           &#x2705;            |
| Dependency management                       | &#x2705;<br/>*(packages)* |   &#x2705;<br/>*(modules)*    |
| Resource management                         | &#x2705;<br/>*(packages)* | &#x2705;<br/>*(data sources)* |
| Print support for non-string values         |         &#x274c;          |           &#x2705;            |
| Automatic number formatting                 |         &#x2705;          |           &#x2705;            |
| Explicit global accessors                   |         &#x274c;          |           &#x2705;            |
| Explicit parent scope accessors (descoping) |         &#x274c;          |           &#x2705;            |
| Unit type                                   |         &#x274c;          |           &#x2705;            |
| Babylonian cuneiform support                |         &#x274c;          |           &#x2705;            |

<p align="center">
(&#x2705; = implemented; &#x274c; = not implemented)
</p>

## Multiple outputs

Rant 3 allowed programs to produce multiple outputs via channels. That design mostly compensated for the lack of first-class collection literals, but it also made nested scopes harder to reason about because inner scopes could not work with more than one active output at a time.

Ranty removes channels in favor of ordinary value flow. If a program needs to produce several results, it can return a `list`, `tuple`, or `map` instead of writing to parallel outputs.

## Resource management

Rant 3 used `.rantpkg` package files to bundle programs and string tables. That required a separate packaging step and combined code loading with data loading.

Ranty splits those concerns:

- modules handle code dependency management;
- data sources handle external data access.

## Querying

Rant 3 included a built-in query system for filtering and printing entries from packaged string tables.

Ranty removes query expressions from the language itself. The equivalent workflow is to pull data through a module or data source and then manipulate it with ordinary language features and stdlib functions.

## Variables

Variables in Rant 3 were mostly manipulated through standard-library functions such as `[v]`, `[vn]`, and `[vs]`.

Ranty replaces that system with [accessors](language/accessors.md), giving variables, keys, indices, descoping, and fallbacks a consistent language-level syntax.

### Example

Before:
```text
# Rant 3 syntax
[vn: foo; 1]
[vn: bar; 2]
[add: [v: foo]; [v: bar]]
# -> 3
```

After:
```ranty
# Ranty syntax
<$foo = 1; $bar = 2>
<foo> + <bar>
# -> 3
```

## Printing values

Rant 3 effectively treated printed output as strings only. Working with numbers and collections often required specialized helper functions because values would not naturally flow through the runtime as structured types.

Ranty prints and carries structured values directly. Lists, tuples, maps, numbers, selectors, functions, and `nothing` all participate in ordinary evaluation without being flattened to strings first.

## Functions

Rant 3 distinguished between native functions and user-defined subroutines, and those lived in different conceptual spaces.

Ranty has a single [`function`](language/data-types/function.md) type for named functions, lambdas, and stdlib callables. Functions can be stored in variables, passed around, returned, and captured like other values. The language also supports [lambda expressions](language/functions/lambdas.md), which Rant 3 did not.

## Collections

Rant 3 had `list` and `map` types, but creating them relied on stdlib helpers, which made collection-heavy code harder to read and less composable.

Ranty adds collection initializer syntax, so lists, tuples, and maps can be created directly inside expressions without temporary variable plumbing.

## Variadic and optional parameters

Rant 3 only supported variadic and optional parameters on native functions.

Ranty allows user-defined functions and lambdas to declare optional and variadic parameters directly, including default values for optional parameters and both `*` and `+` variadic forms.

## Selectors

Rant 3 kept selectors in a separate named object space that lived for the duration of the program. Reusing or discarding them required explicit naming and lifecycle management.

In Ranty, selectors are ordinary values of type [`selector`](language/data-types/selector.md). They can be created once, stored in variables, shared across blocks, or created inline for one-off use.

### Examples

**Reusing a selector**

Before:
```text
# Rant 3 syntax
[x:foo;locked]
{a|b|c|d|e|f|g|h}
[x:foo;locked]
{1|2|3|4|5|6|7|8}
```

After:
```ranty
# Ranty syntax
<%sync = [mksel: one]>
[sel: <sync>] {a|b|c|d|e|f|g|h}
[sel: <sync>] {1|2|3|4|5|6|7|8}
```

**Single-use selector**

Before:
```text
# Rant 3 syntax
[x:foo;deck][rep:each]{A|B|C|D|E|F|G|H}
[xdel:foo]
```

After:
```ranty
Ranty syntax:
[sel: deck] [rep: all] {A|B|C|D|E|F|G|H}
```
