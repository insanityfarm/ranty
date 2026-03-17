# Standard Library: General functions

## alt

```ranty
[%alt: a; ...rest]
```

Prints the first argument that is not `nothing`.

## call

```ranty
[%call: func; args?]
```

Calls `func` with an optional list of argument values.

## cat

```ranty
[%cat: ...values]
```

Prints each argument into the current scope.

## either

```ranty
[%either: condition; true-value; false-value]
```

Prints `true-value` when `condition` is true, otherwise `false-value`.

## len

```ranty
[%len: value]
```

Prints the length of a string, list, map, range, or other length-aware value.

## type

```ranty
[%type: value]
```

Prints the runtime type name of `value`.

## seed

```ranty
[%seed]
```

Prints the currently active RNG seed as an `int`.

## tap

```ranty
[%tap: ...]
```

Consumes arguments and produces no output. This is useful as a no-op sink in pipe chains.

## print

```ranty
[%print: ...values]
```

Prints values directly into the caller's output scope.

## range

```ranty
[%range: a; b?; step?]
```

Builds a half-open integer range.

## require

```ranty
[%require: module-path]
```

Imports a module through the active module resolver.

## irange

```ranty
[%irange: a; b?; step?]
```

Builds an inclusive integer range.

## fork

```ranty
[%fork: seed?]
```

Pushes a derived RNG onto the RNG stack. Integer and string seeds are both supported.

## unfork

```ranty
[%unfork]
```

Pops the most recent derived RNG and resumes the previous RNG state.

## try

```ranty
[%try: context; handler?]
```

Runs `context` and optionally dispatches runtime failures to `handler`.

## ds-request

```ranty
[%ds-request: id; ...args]
```

Calls a registered data source by ID and prints its result.

## ds-query-sources

```ranty
[%ds-query-sources]
```

Prints the list of currently registered data-source IDs.

## proto

```ranty
[%proto: map]
```

Prints the prototype map of `map`, or `nothing` if no prototype is set.

Prototype maps are used only as lookup fallbacks for missing keys.
They do not merge physically into the map, and they are not treated as bound objects.

### Example

```ranty
<$obj = (::)>
<$proto = (:: flavor = vanilla)>
[set-proto: <obj>; <proto>]

[proto: <obj>]\n
<obj/flavor>

##
  Output:

  (:: flavor = vanilla)
  vanilla
##
```

## set-proto

```ranty
[%set-proto: map; proto?]
```

Sets or clears the prototype map for `map`.

Pass `<>` to clear the current prototype.

Prototype assignment is validated eagerly.
If a call to `[set-proto]` would create a cycle, Ranty raises a runtime error instead of allowing the assignment.

### Examples

```ranty
# Attach a prototype
<$obj = (::)>
<$proto = (:: flavor = vanilla)>
[set-proto: <obj>; <proto>]
<obj/flavor>
# -> vanilla
```

```ranty
# Clear a prototype
<$obj = (::)>
<$proto = (:: flavor = vanilla)>
[set-proto: <obj>; <proto>]
[set-proto: <obj>; <>]
<obj/flavor ? missing>
# -> missing
```

```ranty
# Cycles are rejected
<$a = (::)>
<$b = (::)>
[set-proto: <a>; <b>]
[set-proto: <b>; <a>] # runtime error
```

## error

```ranty
[%error: message?]
```

Raises a `USER_ERROR` runtime failure with an optional message.
