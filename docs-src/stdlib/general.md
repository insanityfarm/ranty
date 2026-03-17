# Standard Library: General functions

## alt

```rant
[%alt: a; ...rest]
```

Prints the first argument that is not `nothing`.

## call

```rant
[%call: func; args?]
```

Calls `func` with an optional list of argument values.

## cat

```rant
[%cat: ...values]
```

Prints each argument into the current scope.

## either

```rant
[%either: condition; true-value; false-value]
```

Prints `true-value` when `condition` is true, otherwise `false-value`.

## len

```rant
[%len: value]
```

Prints the length of a string, list, map, range, or other length-aware value.

## type

```rant
[%type: value]
```

Prints the runtime type name of `value`.

## seed

```rant
[%seed]
```

Prints the currently active RNG seed as an `int`.

## tap

```rant
[%tap: ...]
```

Consumes arguments and produces no output. This is useful as a no-op sink in pipe chains.

## print

```rant
[%print: ...values]
```

Prints values directly into the caller's output scope.

## range

```rant
[%range: a; b?; step?]
```

Builds a half-open integer range.

## require

```rant
[%require: module-path]
```

Imports a module through the active module resolver.

## irange

```rant
[%irange: a; b?; step?]
```

Builds an inclusive integer range.

## fork

```rant
[%fork: seed?]
```

Pushes a derived RNG onto the RNG stack. Integer and string seeds are both supported.

## unfork

```rant
[%unfork]
```

Pops the most recent derived RNG and resumes the previous RNG state.

## try

```rant
[%try: context; handler?]
```

Runs `context` and optionally dispatches runtime failures to `handler`.

## ds-request

```rant
[%ds-request: id; ...args]
```

Calls a registered data source by ID and prints its result.

## ds-query-sources

```rant
[%ds-query-sources]
```

Prints the list of currently registered data-source IDs.

## proto

```rant
[%proto: map]
```

Prints the prototype map of `map`, or `nothing` if no prototype is set.

Prototype maps are used only as lookup fallbacks for missing keys.
They do not merge physically into the map, and they are not treated as bound objects.

### Example

```rant
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

```rant
[%set-proto: map; proto?]
```

Sets or clears the prototype map for `map`.

Pass `<>` to clear the current prototype.

Prototype assignment is validated eagerly.
If a call to `[set-proto]` would create a cycle, Rant raises a runtime error instead of allowing the assignment.

### Examples

```rant
# Attach a prototype
<$obj = (::)>
<$proto = (:: flavor = vanilla)>
[set-proto: <obj>; <proto>]
<obj/flavor>
# -> vanilla
```

```rant
# Clear a prototype
<$obj = (::)>
<$proto = (:: flavor = vanilla)>
[set-proto: <obj>; <proto>]
[set-proto: <obj>; <>]
<obj/flavor ? missing>
# -> missing
```

```rant
# Cycles are rejected
<$a = (::)>
<$b = (::)>
[set-proto: <a>; <b>]
[set-proto: <b>; <a>] # runtime error
```

## error

```rant
[%error: message?]
```

Raises a `USER_ERROR` runtime failure with an optional message.
