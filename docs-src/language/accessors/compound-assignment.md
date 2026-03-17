# Compound Assignment

Compound assignment is syntactic sugar to shorten expressions in which a binary operation (such as addition) uses a variable for the left-hand side, then assigns the result back to that variable.

More generally, it expands `a OP= b` into `a = a OP b`, where `OP` is any supported binary operator.

## Syntax

All binary arithmetic and logic operators support **compound assignment** using a second set of operators.
Simply suffix the operation with `=` like the following:

```ranty
<$a = 10>

<a *= 10> # desugars into: <a = <a> * 10>
<a += 1> # desugars into: <a = <a> + 1>
# etc...
```

## Supported operators

| Operation      | Base operator       | Compound             |
|----------------|---------------------|----------------------|
| Addition       | `+`                 | `+=`                 |
| Subtraction    | `-`                 | `-=`                 |
| Multiplication | `*`                 | `*=`                 |
| Division       | `/`                 | `/=`                 |
| Modulo         | `%`                 | `%=`                 |
| Exponentiation | `**`                | `**=`                |
| Logical AND    | `&`                 | `&=`                 |
| Logical OR     | <code>&#124;</code> | <code>&#124;=</code> |
| Logical XOR    | `^`                 | `^=`                 |

Attribute keyword accessors such as `<@rep = 3>` are not compound-assignable. Forms like
`<@rep += 1>` are compile-time errors.
