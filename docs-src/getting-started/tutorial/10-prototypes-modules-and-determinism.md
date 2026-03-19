# Prototypes, Modules, and Determinism

Back at the gate-side trunk, with the lane open before you and both stalls finally in sight, you set the last pages beside Juniper's bell.

Moonrise is close enough to silver the wagon rims. Mira works the bakery counter nearest the gate. Farther down the lane, Tavi polishes a kettle lid with his sleeve as if brightness might fix fate. Juniper pretends not to watch the queue and fails. You have not seen a market cut it this close since the Glass Coast rains. One last assembly, then the gates open.

## Prototypes make maps share behavior

```ranty example
# A prototype with a shared mood and description function.
<$stall-proto = (::
  mood = cozy;
  describe = [?: stall] {
    `<stall/name> hums with `<stall/mood> light.
  };
)>
<$stall = (:: name = "Moon Bakery")>
[set-proto: <stall>; <stall-proto>]
[stall/describe: <stall>]
```

```text expected
Moon Bakery hums with cozy light.
```

The function stored on the prototype is just a normal function found through map lookup. Ranty does not pass a hidden `self`, so you hand the map in explicitly as `stall`. That keeps the magic small and readable when you are this close to opening.

## Writes stay local, even when reads inherit

At first glance, it looks as if changing one stall might overwrite the shared defaults for every other stall.

**Wrong expectation**

```ranty
<$defaults = (:: mood = calm)>
<$stall = (::)>
[set-proto: <stall>; <defaults>]
<stall/mood = excited>
<defaults/mood>
```

**What happened**

```text
calm
```

The prototype kept its original value. Writes stay on the receiving map.

```ranty example
<$defaults = (:: mood = calm)>
<$stall = (::)>
[set-proto: <stall>; <defaults>]
<stall/mood = excited>
[proto: <stall>]\n
<stall/mood>\n
<defaults/mood>
```

```text expected
(:: mood = calm)
excited
calm
```

## Modules

The next example loads a tracked tutorial fixture from `tests/sources/tutorial/`. The fixture itself loads a sibling module through a relative path, so this doubles as a tiny module-system tour. This is the opening line you will hand back to Juniper when the gates swing wide.

```ranty example
@require kit: "tests/sources/tutorial/moon-market-kit"
[kit/opening-line: Juniper]
```

```text expected
violet awnings rustle over Juniper's stall.
Specialty: ink-jam
Omen: the tea hums blue
```

## Determinism with forks

`[fork]` lets you branch the random stream, and `[unfork]` returns to the previous one.

```ranty example
@require kit: "tests/sources/tutorial/moon-market-kit"
[fork: 99][kit/omen]\n
[unfork]
[kit/omen]
```

```text expected
a moth lands on the till
three bells ring at once
```

That is the useful kind of determinism: the main stream stays predictable, but a nested sub-generator can have its own private randomness. Tavi gets his eerie side channel, and Juniper still gets reproducible tests.

## Opening night, all together

The Moon-Market generator you have been assembling is still small, but it already has the same moving parts as many bigger interactive-fiction tools: reusable text, stored world state, branching tables, helper functions, formatting control, and modular pieces you can shuffle into new shapes.

If you want to run the final example from the CLI, the docs verifier uses the same basic idea:

```sh
ranty --seed 1 --eval '@require kit: "tests/sources/tutorial/moon-market-kit" [kit/opening-line: Juniper]'
```

```text
violet awnings rustle over Juniper's stall.
Specialty: ink-jam
Omen: the tea hums blue
```

More information about prototypes, modules, and deterministic helpers can be found in the Ranty documentation for [Map Prototypes](../../language/data-types/map-prototypes.md), [Prototype Patterns](../../language/data-types/prototype-patterns.md), [@require](../../language/keywords/require.md), [Modules](../../modules.md), and [General](../../stdlib/general.md).

Juniper rings the bell at the gate and finally lets her shoulders drop. Mira slides the first tray of buns onto the counter nearest you and flashes a flour-bright grin. Farther down the lane, Tavi lifts the tea kettle, hears it sing properly at last, and laughs out loud. The awnings stir, the lanterns catch, and the Wandering Moon-Market opens on time. Juniper gives you one grave nod of thanks. When the first rush thins, Mira presses a warm star bun into your hand, and Tavi promises your omen will always get the best kettle.

Mission accomplished.

## Coverage ledger

| Feature family | First tutorial page |
| --- | --- |
| Text, comments, escape sequences, hinting, string literals | [Text, Comments, Escapes, and Hinting](01-text-comments-escapes-and-hinting.md) |
| Accessors, setters, constants, `nothing`, fallback syntax, lazy definitions | [Accessors, Variables, Constants, and Nothing](02-accessors-variables-constants-and-nothing.md) |
| Lists, tuples, maps, access paths, slices, dynamic keys, anonymous accessors | [Lists, Tuples, Maps, and Access Paths](03-lists-tuples-maps-and-access-paths.md) |
| Arithmetic, logic, comparison, truthiness, booleans, conditional expressions, compound assignment | [Arithmetic, Logic, and Conditionals](04-arithmetic-logic-and-conditionals.md) |
| Square-bracket calls, built-in helpers, ranges, user functions, lambdas, optional/default/lazy/variadic parameters, closures, `@return`, globals, descoping | [Function Calls, Functions, and Ranges](05-function-calls-functions-and-ranges.md) |
| Blocks, sinking, selectors, repetition, separators, `@weight`, `@on`, `match`, selector handles | [Blocks, Selectors, and Sinking](06-blocks-selectors-and-sinking.md) |
| `@step`, `@total`, `[step]`, control-flow charms, protected blocks, attribute keywords and mutators | [Repetition State, Control Flow, and Attributes](07-repetition-state-control-flow-and-attributes.md) |
| Piping, pipeval, assignment pipe, parametric spread, temporal spread, synchronized spread, complex spread | [Pipes and Argument Spreading](08-pipes-and-argument-spreading.md) |
| `@edit`, `@text`, whitespace formatting, number formatting, string polish helpers | [Output Editing and Formatting](09-output-editing-and-formatting.md) |
| Prototypes, modules, `@require`, determinism with `fork` and `unfork` | [Prototypes, Modules, and Determinism](10-prototypes-modules-and-determinism.md) |

From here, the best next step is not to reread the tutorial. It is to steal one page of it and turn that page into your own scene generator.

Previous: [Output Editing and Formatting](09-output-editing-and-formatting.md)
