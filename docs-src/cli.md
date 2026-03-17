# CLI / REPL

The `rant` CLI runs inline code, files, or piped stdin. When no source is provided and stdin is a TTY, it starts the interactive REPL.

## Execution order

1. `--eval PROGRAM`
2. `FILE`
3. piped stdin
4. REPL

## Flags

| Flag | Description |
| --- | --- |
| `-e`, `--eval` | Runs an inline program string. |
| `-s`, `--seed` | Sets the initial RNG seed as 1 to 16 hexadecimal digits, with an optional `0x` prefix. |
| `-b`, `--bench-mode` | Prints compile and execution timing. |
| `-W`, `--no-warnings` | Suppresses compiler warnings. |
| `-D`, `--no-debug` | Disables debug symbol emission during compilation. |

## Exit codes

| Code | Meaning |
| --- | --- |
| `0` | Success. |
| `64` | Invalid CLI usage, such as an invalid seed. |
| `65` | Compilation failed. |
| `66` | Input file not found. |
| `70` | Runtime execution failed. |

## REPL behavior

The REPL keeps top-level definitions between lines and suppresses noisy unused-variable and unused-function warnings that would otherwise fire on every entry.

## Examples

```sh
rant --seed deadbeef --eval '[rand:1;6]'
rant examples/hello.rant
printf '"from stdin"' | rant
```
