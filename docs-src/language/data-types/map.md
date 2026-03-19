# The `map` type

The `map` type represents an unordered, mutable, resizable collection of zero or more key-value pairs, where each key is a unique string.

Map keys are always coerced to strings; if you try to access a map using a non-string key, the key will be automatically coerced to a string before use.

Map initializers are similar to list initializers, but the contents must begin with a double-colon (`::`).

Map keys come in two flavors:
* **Static keys:** Evaluated at compile-time. They must be identifiers or string literals.
* **Dynamic keys:** Evaluated at run-time. They must be blocks.

```ranty
# Create an empty map
<$empty-map = (::)>

# Create a map with various value types
<$secret-key = occupation>
<$location = US>

(::
    # Regular keys and values
    name = Alex;
    age = 25;

    # Shorthand for `location = <location>`
    <location>;

    # String literals can specify keys that aren't valid identifiers
    "favorite color" = {red|green|blue};

    # An expression can also provide the key
    (<secret-key>) = painter
)
```

## Prototypes

Maps can also inherit keys and functions from an optional **prototype** map.

Prototype lookup is fallback-based:

* getters check the map's own keys first
* if the key is missing, Ranty walks the prototype chain
* setters still write only to the target map

Use `[set-proto]` to attach or clear a prototype and `[proto]` to inspect the current one.

```ranty
<$obj = (::)>
<$proto = (:: flavor = vanilla)>
[set-proto: <obj>; <proto>]

<obj/flavor>   # -> vanilla
[proto: <obj>] # -> (:: flavor = vanilla)
```

Prototype inheritance in Ranty is intentionally small:

* own keys shadow inherited keys
* inherited functions are ordinary functions, not bound methods
* utility functions such as `[keys]` and `[has]` still inspect only the map's own stored keys

See [Map Prototypes](./map-prototypes.md) for the full mental model and [Prototype Patterns](./prototype-patterns.md) for worked examples.
