# Standard Library: Attributes & Control Flow

## if

```rant
[%if: condition]
```

Marks the next block as conditional and resolves it only when `condition` is truthy.

## elseif

```rant
[%elseif: condition]
```

Marks the next block as an `else if` branch following a previous conditional block.

## else

```rant
[%else]
```

Marks the next block as the fallback branch after a previous conditional block.

## mksel

```rant
[%mksel: selector-mode; match-value?]
```

Creates and returns a selector with the specified mode. `match` mode requires a match value;
all other modes reject one.

### Options for `selector-mode`

{{ #include ../_tables/selector-modes.md }}

## rep

```rant
[%rep: reps]
```

Sets the repetition count or repetition mode for the next block.

## sel

```rant
[%sel: selector?]
```

Sets the active selector for the next block. With no argument, prints the current selector or `nothing`.

## match

```rant
[%match: value]
```

Sets the active selector for the next block to a `match` selector bound to `value`.

## sep

```rant
[%sep: separator]
```

Sets the separator value for repeated block iterations.

## mut

```rant
[%mut: mutator?]
```

Sets the mutator function for the next block, or clears it when passed `nothing`.

## sel-skip

```rant
[%sel-skip: selector; n?]
```

Advances `selector` without printing any selected value. This is unsupported for `match` selectors.

## sel-freeze

```rant
[%sel-freeze: selector; frozen?]
```

Sets the frozen state of `selector`. Omitting `frozen` freezes it. This is unsupported for `match` selectors.

## sel-frozen

```rant
[%sel-frozen: selector]
```

Prints whether `selector` is currently frozen. This is unsupported for `match` selectors.

## reset-attrs

```rant
[%reset-attrs]
```

Resets the current attribute state back to the runtime defaults.

## step

```rant
[%step]
```

Prints the current repeater step value using a 1-based index.

## step-index

```rant
[%step-index]
```

Prints the zero-based iteration index of the active repeater.

## step-count

```rant
[%step-count]
```

Prints the total number of iterations scheduled for the active repeater. Infinite repeaters report `0`.
