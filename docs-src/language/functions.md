# Functions

A function is a reusable block of code that runs when something calls it. 
Functions can optionally accept inputs and print output back to the caller.

In addition to Ranty's built-in functions, you can also define your own and use them however you see fit.

## Calling functions

To call a function, place the function's name inside a pair of square brackets:

```ranty
[dig]
```
This calls the `[dig]` function from the standard library, which prints a random decimal digit.
The output of this call then flows into the caller's output, producing our digit:
```
8
```

### Arguments

Additionally, functions can accept arguments.
When a function requires arguments, we first add a colon `:` after the function's name, followed by a list of arguments separated by semicolons `;`.

Here are a few examples of how this works: 

```ranty
# Function with no arguments
[func-name]

# Function with one argument
[func-name: arg1]

# Function with multiple arguments, delimited by ';'
[func-name: arg1; arg2]
```


## Defining functions

All functions are objects of the `function` type, and can be treated like any other variable.
When we define a function, we are simply defining a variable and storing it there.

To define a function, we need the following:
* The **function signature**, enclosed in square brackets `[ ]`, containing:
    * The name of the function,
    * A list of parameters to accept
* The **function body**, enclosed by curly braces `{ }`.

```ranty
# Defines a parameterless function named `say-hello` that prints "Hello"
[$say-hello] {
    Hello
}

# Defines a function `greet` with parameter `name` that prints a custom greeting
# Arguments can be accessed like regular variables.
[$greet: name] {
    Hello, <name>!
}

# Defines a function `flip-coin` that returns the value of either `heads` and `tails`
[$flip-coin: heads; tails] {
    {<heads>|<tails>}
}
```

Parameters can also be marked optional with `?`, lazy with `@lazy`, or variadic with `*` / `+`.
See the sections below for the detailed rules and interactions.

Like other variables, functions can also be made constant by using `%` in place of `$` when defining them:

```ranty
# Regular functions can be overwritten
[$foo] { hello! }
[$foo] { goodbye! }

# Constant function; can't be overwritten
[%foo] { 
    important stuff... 
}

# If we try, the compiler will catch it and produce an error
[%foo] { # ERROR: constant redefinition
    this won't work
}
```

> **Note:**
>
> All of Ranty's standard library functions are constants, and thus can't be modified&mdash;but they can still be shadowed in child scopes.


#### Default arguments

You can also specify a custom default value for an optional parameter by adding an expression after the `?` modifier.

For ordinary parameters, the default expression is eager: each time the function is called without that parameter, Ranty runs the expression immediately and uses its output as the argument.

```ranty
# Modification of the previous [gen-pet] to use a default value instead of calling [alt]
[$gen-pet: name; species ? "dog"] { 
    (::
        name = <name>;
        species = <species>; # No fallback needed!
    )
}
```

> **Note:**
>
> Default argument expressions can capture external variables just like the body of the function they're attached to.

#### Lazy parameters

Place `@lazy` before a parameter name to defer evaluation of the caller's argument until the parameter is first accessed.

```ranty
[$ignore: @lazy value] {
    done
}

[ignore: [expensive-call]]
```

In the example above, `[expensive-call]` is never evaluated because the function body never reads `<value>`.

Lazy parameters are call-by-need:

* the caller's argument expression is captured instead of evaluated immediately
* the first read forces the argument
* the result is memoized, so later reads reuse the same value

```ranty
[$dup: @lazy x] {
    <x>,<x>
}

[dup: [build-value]]
```

`@lazy` also works with optional parameters and defaults:

```ranty
[$greet: @lazy name ? [pick-name]] {
    Hello, <name>!
}
```

If `[pick-name]` is needed, it is still deferred until `<name>` is first accessed.
If the caller supplies `name`, the default expression is ignored completely.

An optional lazy parameter without a default is still absent when omitted, just like any other optional parameter:

```ranty
[$show-subtitle: @lazy subtitle?] {
    <subtitle ? "(none)">
}
```

Lazy parameters can also be captured by returned closures, and they capture outer variables by reference:

```ranty
[$defer: @lazy x] {
    [?] { <x> }
}

<$value = 1>
<$reader = [defer: <value>]>
<value = 2>
[reader] # -> 2
```

`@lazy` is supported on user-defined function and lambda parameters, but not on variadic parameters.

> **See also:**
>
> [Lazy definitions](./accessors.md#lazy-definitions), [Optional parameters](./functions/optional-parameters.md), [Lambdas](./functions/lambdas.md), and [Variadic parameters](./functions/variadic-parameters.md).



## Closures

A function can access variables defined outside of its body.
When this happens, it "captures" a reference to that variable for use inside the function.
As a result, changes made to a captured variable persist between calls. 
Functions that capture variables are called **closures**.

Variable capturing can also be used to maintain a persistent state within a closure instance:
even if the original variable falls out of scope, the closure still keeps it alive.

```ranty
# Create a function with a persistent state
{
    <$foo-num = 1>
    # Define a function [next-foo] in the parent scope
    [$^next-foo] {
        # Modify `foo-num` from inside the function
        foo <$n = <foo-num>; foo-num = [add: <foo-num>; 1]; n>
    }
} # `foo-num` falls out of scope here, but [next-foo] can still access it

# Call [next-foo] multiple times
[rep:4][sep:\n]
{
    [next-foo]
}
##
  Output:

  foo 1
  foo 2
  foo 3
  foo 4
##
```

### Limitations on variable capture

Capturing is only supported on variables accessed locally from the function body or default argument expressions.
Descoped and explicit global accessors do not capture variables.

The same by-reference capture behavior is used by lazy definition initializers and lazy parameter/default thunks.
If a captured variable changes before a lazy binding is first forced, the forced value observes the latest captured state.

## Calling shadowed functions

The situation may occasionally arise where you accidentally (or intentionally)
define a non-function variable with the same name as a function from a parent scope (e.g. a stdlib function) 
and then try to call it: 

```ranty
<$rep = "not a function">
[rep:10] # Wait a sec...
[sep:\n]
{
    # ...
}
```

Some might (understandably) assume that this would crash the program, but this code actually still works!

When this happens, Ranty will perform what is known as **function percolation**:
the runtime will search each parent scope up to the global scope until it finds a function with the same name, and then call it as normal.

Function percolation only applies to function calls, so getters will still correctly retrieve the new variable instead of the function.
