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
[%mksel: selector-mode]
```

Creates and returns a selector with the specified mode.

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

Advances `selector` without printing any selected value.

## sel-freeze

```rant
[%sel-freeze: selector; frozen?]
```

Sets the frozen state of `selector`. Omitting `frozen` freezes it.

## sel-frozen

```rant
[%sel-frozen: selector]
```

Prints whether `selector` is currently frozen.

## reset-attrs

```rant
[%reset-attrs]
```

Resets the current attribute state back to the runtime defaults.

## step

```rant
[%step]
```

Prints the current repeater step value.

## step-index

```rant
[%step-index]
```

Prints the zero-based iteration index of the active repeater.

## step-count

```rant
[%step-count]
```

Prints the total number of iterations scheduled for the active repeater.
