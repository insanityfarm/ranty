# Lambdas

Ranty implements anonymous (nameless) functions as **lambda expressions**, commonly referred to as simply **lambdas**.
The body of a lambda can also capture variables from its environment, just like regular named functions.

Basic lambda expression syntax with no parameters consists of a `?` symbol inside brackets preceding the function body. 

```ranty
# Creates a function and places it on the output
[?] { Hello from lambda expression! }
```

> **Note:**
>
> If your lambda body contains a block at the root level, you will need to enclose it in another block to differentiate it from a function body block.
>
> Example:
> ```ranty
> # Compiler error
> [?] { foo | bar }
>
> # Correct syntax
> [?] {{ foo | bar }}
> ```

## Parameterization

If you want to add parameters to a lambda, just specify the parameters after the colon as you would with a normal function definition:

```ranty
[?: param1; param2] { ... }
```

Lambda parameters support the same modifiers as named functions, including `@lazy` and optional defaults:

```ranty
[?: @lazy name ? "friend"] { Hello, <name>! }
```

See [Lazy parameters](../functions.md#lazy-parameters) for the full evaluation rules.


## Calling an anonymous function

Since lambdas produce `function` objects just like named functions, the call syntax is very similar. Simple use an anonymous access path to supply the function in the call.

```ranty
# Define a function that returns a parameterless function
[$get-anon-func] {
    [?]{Hello from anonymous function}
}

[([get-anon-func])]         # -> "Hello from anonymous function"

# Define a function that returns a function with a single parameter
[$get-greet-func] {
    [?:name]{Hello, <name>!}
}

[([get-greet-func]): Ranty]  # "Hello, Ranty!"
```

### Using piping to call an anonymous function produced by another function

The above example could be simplified with [piping](./piping.md): just call `[get-anon-func]`, then call `[]` directly.

```ranty
[get-anon-func |> []: ## args... ##]
```
