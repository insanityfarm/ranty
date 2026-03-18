# Optional parameters

A function parameter can be made optional using the `?` modifier after the parameter name; this means that the caller is not required to pass an argument to it.

When an optional parameter is omitted, the variable won't exist in the function body; as a result, accessing it can fail, causing an error.
To prevent this from happening, you need to use a [fallback expression](../accessors/fallbacks.md) to provide a default value.

> Please note that all optional parameters must appear after all required parameters, and before any variadic parameter;
> breaking this order will cause a compiler error.

```ranty
# Generates a map for a pet with a name and species (defaults to "dog")
[$gen-pet: name; species?] {
    (::
        name = <name>;
        species = <species ? "dog">; # Fallback to "dog" if species is undefined
    )
}
```

## Default values

Optional parameters can also define a default expression directly in the signature:

```ranty
[$gen-pet: name; species ? "dog"] {
    (::
        name = <name>;
        species = <species>;
    )
}
```

For ordinary parameters, this default is eager: if the caller omits `species`, Ranty evaluates `"dog"` immediately when the call is bound.

## Lazy optional parameters

Prefix an optional parameter with `@lazy` to defer the caller's argument until the parameter is first accessed:

```ranty
[$show-subtitle: @lazy subtitle?] {
    <subtitle ? "(none)">
}
```

If `subtitle` is omitted here, it is still absent and the fallback remains necessary.
`@lazy` changes when a provided value is evaluated; it does not make missing optional parameters automatically exist.

Lazy optional parameters can also have lazy defaults:

```ranty
[$title-line: @lazy title ? [pick-title]] {
    Title: <title>
}
```

If `title` is omitted, `[pick-title]` is captured and deferred until `<title>` is first accessed.
If `title` is never read, the default expression never runs.

See also [Lazy parameters](../functions.md#lazy-parameters) and [Fallbacks](../accessors/fallbacks.md).
