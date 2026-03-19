# Ranty Standard Library

The Ranty Standard Library contains the builtin functions and constants loaded into a fresh `Ranty` context.

Builtins are loaded into a fresh context as predeclared global values. Functions such as `len`, `keys`, and `upper` are constant globals, so you can call them immediately with ordinary bracket syntax like `[len: value]`, but you cannot reassign them in place.

## Categories
The detailed reference pages below remain the canonical prose documentation for each category. The generated inventory that follows is rebuilt from the current export surface on every docs build and is used by the audit checks to catch drift.

- [General functions](./stdlib/general.md)
- [Attribute and control-flow functions](./stdlib/control-flow.md)
- [Collection functions](./stdlib/collections.md)
- [Generator functions](./stdlib/generators.md)
- [Formatting functions](./stdlib/formatting.md)
- [String functions](./stdlib/strings.md)
- [Boolean functions](./stdlib/boolean.md)
- [Comparison functions](./stdlib/comparison.md)
- [Math functions](./stdlib/math.md)
- [Conversion functions](./stdlib/conversion.md)
- [Verification functions](./stdlib/verification.md)
- [Assertion functions](./stdlib/assertion.md)
- [Constants](./stdlib/constants.md)

{{#include generated/stdlib-inventory.md}}
