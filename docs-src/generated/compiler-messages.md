## Compiler Messages

| Code | Severity | Message Template |
| --- | --- | --- |
| `R0000` | error | `unexpected token: '{}'` |
| `R0001` | error | `expected token: '{}'` |
| `R0002` | error | `integer literal is out of range for the \`int\` type; consider changing it (or if applicable, using a string instead)` |
| `R0003` | error | `float literal is out of range for the \`float\` type; consider changing it (or if applicable, using a string instead)` |
| `R0004` | error | `invalid escape character: '{}'` |
| `R0005` | error | `invalid code point in unicode escape: '{}'` |
| `R0006` | error | `unclosed block; expected '}'` |
| `R0007` | error | `unclosed function call; expected ']'` |
| `R0008` | error | `unclosed function signature; expected ']' followed by body block` |
| `R0009` | error | `unclosed accessor; expected '>'` |
| `R0010` | error | `unclosed string literal` |
| `R0011` | error | `unclosed list initializer; expected ')'` |
| `R0012` | error | `unclosed map initializer; expected ')'` |
| `R0013` | error | `unclosed tuple; expected ')'` |
| `R0014` | error | `unclosed parenthetical; expected ')'` |
| `R0015` | error | `unclosed condition; expected ':'` |
| `R0021` | error | `{} is not allowed after {}` |
| `R0022` | error | `missing body in function definition` |
| `R0023` | error | `unclosed function body; expected '}}'` |
| `R0024` | error | `invalid parameter '{}'; must be a valid identifier or '*'` |
| `R0025` | error | `duplicate parameter '{}' in function signature` |
| `R0026` | error | `multiple variadic parameters are not allowed` |
| `R0027` | error | `temporal assignment pipe could redefine variable '{}'` |
| `R0028` | error | `temporal assignment pipe could redefine constant '{}'` |
| `R0029` | error | `lazy parameters cannot be variadic` |
| `R0040` | error | `dynamic key blocks can't have more than one element; if branching is desired, create an inner block` |
| `R0041` | error | `duplicate @{} modifier on block element` |
| `R0060` | error | `can't assign a value to an expression; try assigning to a child of it instead` |
| `R0061` | error | `identifier required but is missing` |
| `R0062` | error | `'{}' is not a valid identifier; identifiers may only use alphanumerics, underscores, and hyphens (but cannot be only digits)` |
| `R0063` | error | `access paths cannot start with an index; consider using a variable or anonymous value here instead` |
| `R0064` | error | `access paths cannot start with a slice; consider using a variable or anonymous value here instead` |
| `R0065` | error | `invalid slice bound: '{}'` |
| `R0066` | error | `no pipe value is available in this scope` |
| `R0067` | error | `access to optional argument '{}' can fail; add a fallback to the accessor or specify a default argument` |
| `R0068` | error | `invalid shorthand; only variable getters are supported` |
| `R0100` | error | `reassignment of known constant '{}'` |
| `R0101` | error | `redefinition of known constant '{}'` |
| `R0130` | error | `sink is not valid on {}` |
| `R0131` | error | `hint is not valid on {}` |
| `R0200` | error | `invalid keyword: '@{}'` |
| `R0201` | error | `@weight is not allowed in this context` |
| `R0202` | error | `missing argument for @require` |
| `R0203` | error | `@require path should be a string literal` |
| `R0204` | error | `condition cannot be empty` |
| `R0205` | error | `attribute keyword '@{}' does not support this accessor form` |
| `R0206` | error | `attribute keyword '@{}' is read-only` |
| `R0207` | error | `@on is not allowed in this context` |
| `R0250` | error | `expected operand` |
| `R0251` | error | `expected left-hand operand` |
| `R0252` | error | `expected right-hand operand` |
| `R1000` | warning | `variable '{}' is defined but never used` |
| `R1002` | warning | `parameter '{}' is never used` |
| `R1003` | warning | `function '{}' is defined but never used` |
| `R1004` | warning | `function '{}' is empty` |
| `R1005` | warning | `nested function definition can't be made constant; function will be mutable` |
| `R2100` | error | `file not found: '{}'` |
| `R2101` | error | `filesystem error: {}` |
