# Globals & Descoping

Accessors support two ways of explicitly accessing variables in parent scopes: **global accessors** and **descoping**.

## Explicitly accessing globals

The top level of a Ranty program implicitly defines a local variable scope; 
because of this, top-level variables will only persist for the duration of the program.

To explicitly access a variable in the global scope, prefix the path with the `/` character.

Similarly to the descope operator, this method also allows you to override shadowing on globals.

```ranty
<$foo = Hello from program scope!>
<$/foo = Hello from global scope!>

Local: <foo>\n
Global: </foo>\n

##
    Output:

    Local: Hello from program scope!
    Global: Hello from global scope!
##
```

## The 'descope' operator

If a child scope shadows a parent variable and you want to access the parent variable explicitly, you can use the **descope operator**.

To do this, prefix the variable name with a `^` character.

```ranty
<$a = foo>
{
    <$a = bar>
    Shadowed: <a>\n
    Descoped: <^a>\n
}

##
    Output:

    Shadowed: bar
    Descoped: foo
##
```

Adding more than one `^` character in a row will skip up to that many scopes.
These operations are called "_n_-descopes", where _n_ is the number of scopes skipped:
For example, `<^^foo>` is a 2-descope, `<^^^foo>` is a 3-descope, and so on.

```ranty
# Define `test`
<$test = foo>
{
    # Shadow `test`
    <$test = bar>
    {
        # Shadow `test` again
        <$test = baz>
        <^^test> <^test> <test>
    }
}
## Output: "foo bar baz"
```

While the compiler imposes no explicit limit on descope depth, use cases for large descope depths are rare and it is recommended to avoid them when possible.

```ranty
# A 360-descope.
< ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
foo>
```