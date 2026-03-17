# Prototype Patterns

This page collects practical ways to use map prototypes well.

If you have not read the core semantics yet, start with [Map Prototypes](./map-prototypes.md).

## Shared defaults

One of the simplest uses for prototypes is shared default data.

```ranty
<$default-npc = (::
  role = villager;
  mood = calm;
  hp = 10;
)>

<$merchant = (:: name = Tavi)>
<$guard = (:: name = Mira; hp = 14)>

[set-proto: <merchant>; <default-npc>]
[set-proto: <guard>; <default-npc>]

<merchant/name>: <merchant/role>, <merchant/mood>, <merchant/hp>\n
<guard/name>: <guard/role>, <guard/mood>, <guard/hp>

##
  Output:

  Tavi: villager, calm, 10
  Mira: villager, calm, 14
##
```

This keeps common fields in one place while still allowing each object to override specific values.

## Behavior mixins

A prototype can also serve as a behavior bundle:

```ranty
<$describable = (::
  describe = [?: obj] {
    <obj/name> the <obj/species>
  };
)>

<$pet = (:: name = Poppy; species = cat)>
[set-proto: <pet>; <describable>]

[pet/describe: <pet>]
# -> Poppy the cat
```

Because Ranty does not inject a receiver automatically, the pattern is to pass the object explicitly.

## Object factories

Factories pair naturally with prototypes.
The factory creates a fresh map, then attaches the shared prototype:

```ranty
<$counter-proto = (::
  inc = [?: counter] {
    <counter/value = [add: <counter/value>; 1]>
  };
  read = [?: counter] { <counter/value> };
)>

[$make-counter: start ? 0] {
  <
    $counter = (:: value = <start>);
    [set-proto: <counter>; <counter-proto>];
  >
  <counter>
}

<$counter-a = [make-counter]>
<$counter-b = [make-counter: 10]>

[counter-a/inc: <counter-a>]
[counter-a/inc: <counter-a>]
[counter-b/inc: <counter-b>]

[counter-a/read: <counter-a>], [counter-b/read: <counter-b>]
# -> 2, 11
```

This pattern is a good fit when you want many small objects that share the same behavior.

## Layered specialization

Instead of mutating a shared prototype, create a new prototype that inherits from the shared one.
This lets you extend behavior safely for one family of objects without affecting the base layer.

```ranty
<$weapon = (::
  describe = [?: obj] { <obj/name> deals <obj/damage> damage. };
)>

<$magic-weapon = (:: element = arcane)>
[set-proto: <magic-weapon>; <weapon>]

<$staff = (:: name = Emberstaff; damage = 8)>
[set-proto: <staff>; <magic-weapon>]

[staff/describe: <staff>]\n
<staff/element>

##
  Output:

  Emberstaff deals 8 damage.
  arcane
##
```

This layering approach is usually safer than editing a widely shared prototype in place.

## Shared prototype updates

Remember that maps are by-reference values.
If multiple objects point at the same prototype map, changing that prototype changes what all of them inherit:

```ranty
<$proto = (:: flavor = vanilla)>
<$a = (::)>
<$b = (::)>

[set-proto: <a>; <proto>]
[set-proto: <b>; <proto>]
<proto/flavor = chocolate>

<a/flavor>, <b/flavor>
# -> chocolate, chocolate
```

That can be useful, but it can also be surprising.
If you want a safer extension point, prefer the layered specialization pattern above.

## Patterns to avoid

### Expecting enumeration to inherit

Prototype lookup does not change `[keys]`, `[values]`, or `[has]`:

```ranty
<$obj = (:: own = 1)>
<$proto = (:: inherited = 2)>
[set-proto: <obj>; <proto>]

[keys: <obj>]
# -> (: own)
```

If you need a merged view of the whole chain, build it explicitly in your own code.

### Expecting writes to update shared defaults

Assigning to an inherited key writes locally:

```ranty
<$defaults = (:: mood = calm)>
<$npc = (::)>
[set-proto: <npc>; <defaults>]

<npc/mood = angry>
<npc/mood>, <defaults/mood>
# -> angry, calm
```

If you actually want to update the shared default, write to the prototype map itself.

### Expecting methods to know the caller automatically

Prototype methods do not receive a hidden receiver:

```ranty
<$proto = (:: greet = [?: name] { Hello,\s<name>! })>
<$obj = (:: name = Ranty)>
[set-proto: <obj>; <proto>]

[obj/greet: <obj/name>]
# -> Hello, Ranty!
```

When a method needs object state, pass the object explicitly.
