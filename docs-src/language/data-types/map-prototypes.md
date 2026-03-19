# Map Prototypes

Map prototypes let one `map` fall back to another `map` during member lookup.

In simple terms, a map can have:

* its own keys
* an optional **prototype**
* a prototype whose prototype points at another map, and so on

When Ranty reads a map key, it checks the map's own keys first.
If the key is missing, it walks up the prototype chain until it finds a match or runs out of prototypes.

## Mental model

Prototype inheritance in Ranty is intentionally small and predictable:

* **Own keys win.** A key stored directly on the map shadows the same key on any prototype.
* **Reads inherit.** Getters and method-like calls can resolve inherited members.
* **Writes stay local.** Setting a key never writes through to a prototype.
* **Methods are just functions.** An inherited function does not receive a hidden `self` or `this`.
* **Utilities stay raw.** Functions such as `[has]`, `[keys]`, `[values]`, and `[translate]` still use only the map's own stored keys.

This means prototypes are best thought of as a lookup fallback mechanism, not as a full object system.

## Creating and inspecting prototypes

Use `[set-proto]` to attach or clear a prototype, and `[proto]` to read the current prototype:

```ranty
<$obj = (::)>
<$proto = (:: flavor = vanilla)>

[set-proto: <obj>; <proto>]
[proto: <obj>]   # -> (:: flavor = vanilla)

[set-proto: <obj>; <>]
[proto: <obj>]   # -> <> (nothing value, no output printed)
```

## Inherited reads

If a key is not stored directly on the map, the getter continues through the prototype chain:

```ranty
<$obj = (::)>
<$proto = (:: flavor = vanilla; color = blue)>
[set-proto: <obj>; <proto>]

<obj/flavor>\n
<obj/color>\n

##
  Output:

  vanilla
  blue
##
```

## Shadowing inherited members

Own keys always beat inherited keys:

```ranty
<$obj = (:: flavor = chocolate)>
<$proto = (:: flavor = vanilla; color = blue)>
[set-proto: <obj>; <proto>]

<obj/flavor>\n
<obj/color>\n

##
  Output:

  chocolate
  blue
##
```

## Local writes do not mutate the prototype

If you assign to an inherited key, Ranty creates or updates a local key on the receiver map:

```ranty
<$obj = (::)>
<$proto = (:: flavor = vanilla)>
[set-proto: <obj>; <proto>]

<obj/flavor = chocolate>

Object: <obj/flavor>\n
Proto: <proto/flavor>

##
  Output:

  Object: chocolate
  Proto: vanilla
##
```

This is one of the most important rules to remember: inheritance affects lookup, not storage.

## Prototype methods

Maps can store functions, so a prototype can also provide shared behavior:

```ranty
<$animal = (:: speak = [?: name] { `<name> says hello. })>
<$cat = (:: name = Miso)>
[set-proto: <cat>; <animal>]

[cat/speak: <cat/name>]
# -> Miso says hello.
```

The function above is just an ordinary function found through prototype lookup.
Ranty does **not** inject the receiver map automatically.
If a method needs the map, pass it explicitly:

```ranty
<$animal = (:: rename = [?: obj; new-name] { <obj/name = <new-name>> })>
<$cat = (:: name = Miso)>
[set-proto: <cat>; <animal>]

[cat/rename: <cat>; Pixel]
<cat/name>
# -> Pixel
```

## Multi-hop chains

Prototype lookup continues across multiple links:

```ranty
<$base = (:: category = dessert)>
<$proto = (:: flavor = vanilla)>
<$obj = (:: name = custard)>

[set-proto: <proto>; <base>]
[set-proto: <obj>; <proto>]

<obj/name>\n
<obj/flavor>\n
<obj/category>

##
  Output:

  custard
  vanilla
  dessert
##
```

## Lookup-only behavior

Some operations intentionally ignore prototypes and inspect only keys physically stored on the map.

This includes:

* `[has]`
* `[keys]`
* `[values]`
* `[translate]`
* map display formatting

Example:

```ranty
<$obj = (:: own = 1)>
<$proto = (:: inherited = 2)>
[set-proto: <obj>; <proto>]

[has: <obj>; own]\n
[has: <obj>; inherited]\n
[keys: <obj>]\n
[values: <obj>]\n
[translate: (: own; inherited); <obj>]\n
<obj>

##
  Output:

  @true
  @false
  (: own)
  (: 1)
  (: 1; inherited)
  (:: own = 1)
##
```

This behavior is intentional.
Prototype inheritance changes how map lookups work, but it does not turn a map into a merged view of the whole chain.

## Cycle rejection

Prototype chains cannot contain cycles.
Ranty rejects any `[set-proto]` call that would make a map eventually inherit from itself:

```ranty
<$a = (::)>
<$b = (::)>

[set-proto: <a>; <b>]
[set-proto: <b>; <a>] # runtime error
```

Rejecting cycles keeps member lookup finite and predictable.

## When to use prototypes

Prototypes work best when you want:

* shared defaults
* shared helper functions
* layered configuration maps
* small object-like values without adding new syntax

For larger worked examples, see [Prototype Patterns](./prototype-patterns.md).
