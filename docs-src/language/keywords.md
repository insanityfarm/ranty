# Keywords

**Keywords** are reserved tokens starting with `@` that perform specialized operations,
such as returning from a function or assigning metadata.

Stable 4.0 uses keywords in four main roles:

- control flow: `@return`, `@continue`, `@break`
- block metadata: `@weight`, `@on`
- runtime attributes: `@rep`, `@sep`, `@sel`, `@mut`, `@step`, `@total`
- auto-hinting: `@text`

The attribute keywords are additive syntax over the runtime attribute system described in
[Attributes](../runtime/attributes.md). The mutable attribute keywords can be read as plain
expressions, accessed through angle-bracket syntax, or applied directly to an immediate block
with `@kw expr: { ... }`.
