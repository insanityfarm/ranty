# The `nothing` type

To represent the lack of a value, Ranty has the `nothing` type, 
which has only one possible value, represented in Ranty by the literal `<>` (referred to as the "nothing literal").

The `nothing` type acts as a placeholder value or indicator that no value is provided or needed.

Printing a value of type `nothing` will have no effect on the output.

```ranty
# The nothing literal prints nothing.
[type: <>]  # nothing

# An empty expression also prints nothing.
[type: ]  # nothing
```

## Uses for the nothing literal

Even though an empty expression prints the same `nothing` value as `<>`, using `<>` explicitly can change program behavior in some cases; specifically, collection initializers.

For example, this empty list initializer will have a length of 0:

```ranty
[len: (:)] # 0
```

If we want to initialize the list with a single entry containing nothing, we can use the `nothing` literal to make this distinction:

```ranty
[len: (: <>)] # 1
```

This also works for specifying a final empty entry in a list or tuple, where Ranty would otherwise ignore the trailing `;`:

```ranty
[len: (1;)]     # 1
[len: (1; <>)]  # 2
```