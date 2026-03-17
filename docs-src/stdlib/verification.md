# Standard Library: Verification functions

## is-some

```ranty

[%is-some: value] 

```
&rarr; `bool`

Returns `@true` if `value` is any non-`nothing` type.

### Paraneters

**`value`** &larr; `any` <br/>
The value to check.


## is-between

```ranty

[%is-between: value; a; b]

```
&rarr; `bool`

Returns `@true` if `value` is bewteen `a` and `b` (both inclusive).

### Parameters

**`value`** &larr; `any` <br/>
The value to check.

**`a`** &larr; `any` <br/>
The first bound.

**`b`** &larr; `any` <br/>
The second bound.


## is-bool

```ranty

[%is-bool: value]

```
&rarr; `bool`

Returns `@true` if `value` is of type `bool`.

### Parameters

**`value`** &larr; `any` <br/>
The value to check.


## is-nothing

```ranty

[%is-nothing: value]

```
&rarr; `bool`

Returns `@true` if `value` is of type `nothing`.

### Parameters

**`value`** &larr; `any` <br/>
The value to check.


## is-even

```ranty

[%is-even: number]

```
&rarr; `bool`

Returns `@true` if `number` is an even number.

### Parameters

**`number`** &larr; `int | float` <br/>
The number to check.

## is-factor

```ranty

[%is-factor: value; factor]

```
&rarr; `bool`

Returns `@true` if `value` is evenly divisible by `factor`.


## is-float

```ranty

[%is-float: value]

```
&rarr; `bool`

Returns `@true` if `value` is of type `float`.

### Parameters

**`value`** &larr; `any` <br/>
The value to check.


## is-int

```ranty

[%is-int: value]

```
&rarr; `bool`

Returns `@true` if `value` is of type `int`.

### Parameters

**`value`** &larr; `any` <br/>
The value to check.


## is-nan

```ranty

[%is-nan: value]

```
&rarr; `bool`

Returns `@true` if `value` is of type `float` and equal to NaN (Not a Number).

### Parameters

**`value`** &larr; `any` <br/>
The value to check.


## is-number

```ranty

[%is-number: value]

```
&rarr; `bool`

Returns `@true` if `value` is of type `int` or `float`.

### Parameters

**`value`** &larr; `any` <br/>
The value to check.

## is

```ranty

[%is: value; type-name]

```
&rarr; `bool`

Returns `@true` when `value`'s runtime type name exactly matches `type-name`.


## is-odd

```ranty

[%is-odd: number]

```
&rarr; `bool`

Returns `@true` if `number` is an odd number.

### Parameters

**`value`** &larr; `any` <br/>
The value to check.


## is-string

```ranty

[%is-string: value]

```
&rarr; `bool`

Returns `@true` if `value` is of type `string`.


### Parameters

**`value`** &larr; `any` <br/>
The value to check.
