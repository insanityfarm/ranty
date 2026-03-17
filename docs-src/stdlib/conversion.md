# Standard Library: Conversion functions

## to-float

```ranty

[%to-float: value]

```
&rarr; `float | nothing`

Attempts to convert `value` to a `float` value and prints the result.
If the conversion fails, prints nothing.

### Parameters

**`value`** &larr; `any` <br/>
The input value to convert.


## to-int

```ranty

[%to-int: value]

```
&rarr; `int | nothing`

Attempts to convert `value` to an `int` value and prints the result.
If the conversion fails, prints nothing.

### Parameters

**`value`** &larr; `any` <br/>
The input value to convert.


## to-string

```ranty

[%to-string: value]

```
&rarr; `string | nothing`

Attempts to convert `value` to a `string` value and prints the result.
If the conversion fails, prints nothing.

### Parameters

**`value`** &larr; `any` <br/>
The input value to convert.

## to-bool

```ranty

[%to-bool: value]

```
&rarr; `bool`

Converts `value` to a boolean using Ranty's runtime truthiness rules.


## to-list

```ranty

[%to-list: value]

```
&rarr; `list | nothing`

Attempts to convert `value` to a `list` and prints the result.
If the conversion fails, prints nothing.

### Parameters

**`value`** &larr; `any` <br/>
The input value to convert.

### Conversion behavior

**From `string`**

Passing a `string` value into this function returns a `list` of the string's graphemes.

```ranty
<%letters = [to-list: "hello"]>
[assert-eq: <letters>; (: h; e; l; l; o)]
```

**From `list`**

Passing a `list` value into this function prints a shallow copy of it.
This is equivalent to calling `[copy]` on the list.

**From `range`**

Passing a `range` value into this function prints a list of the range's elements in order.

```ranty
<%seq = [range: 0; 5 |> to-list]>
[assert-eq: <seq>; (: 0; 1; 2; 3; 4)]
```

## to-tuple

```ranty

[%to-tuple: value]

```
&rarr; `tuple | nothing`

Attempts to convert a value to a `tuple` and prints the result.
If the conversion fails, prints nothing.
