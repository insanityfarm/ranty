# Variadic parameters

Functions support variadic parameters with the special symbols `*` and `+`.

A `*` parameter is optional and defaults to an empty list, while a `+` parameter is required and must contain at least one element.

Functions may only have up to one variadic parameter, and it must appear last in the signature.

```ranty
[$how-many: items*] {
    [len: <items>]
}

[how-many: foo; bar; baz] # Outputs "3"
```

`@lazy` cannot be combined with `*` or `+`.
If you need lazy evaluation, accept a single lazy parameter and decide inside the function how to realize it.
