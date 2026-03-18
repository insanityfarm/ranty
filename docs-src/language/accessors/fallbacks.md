# Fallbacks

A **fallback**, (or **fallback expression**) is an optional component of a getter that can return a default value if the getter fails (such as due to a missing variable, index, or key).

A fallback expression only runs if the data requested by the associated getter is not found; otherwise, it is ignored completely.

To use a fallback, it must be added to the end of the accessor, within the brackets, with a leading `?` character as shown below.

Fallbacks are still relevant when using lazy bindings:

* getter fallbacks use `?`, while lazy definitions use `?=`; the forms are related in spelling only
* omitted optional `@lazy` parameters without defaults are still absent and should be read with a fallback

### Example

```ranty
# Store a value to use as a fallback
<$fallback = "I don't exist!">

{
    # Define a variable `foo`
    <$foo = "I exist!">

    # Get `foo` with fallback
    <foo ? <fallback>> # -> "I exist!"
}\n

# Getting `foo` again out of scope will trigger the fallback
<foo ? <fallback>> # -> "I don't exist!"

# Getting `foo` without a fallback here would crash the program
<foo> # error
```

This also applies to optional lazy parameters:

```ranty
[$card: @lazy subtitle?] {
    <subtitle ? "(no subtitle)">
}
```

See also [Lazy definitions](../accessors.md#lazy-definitions) and [Lazy parameters](../functions.md#lazy-parameters).
