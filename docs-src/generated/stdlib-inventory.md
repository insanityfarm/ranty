## Stdlib Inventory

| Symbol | Category | Call Form | Summary | Canonical Location |
| --- | --- | --- | --- | --- |
| `BUILD_VERSION` | Constants | `BUILD_VERSION` | The build version of the Rant library being used. | [stdlib/constants.md#build_version](stdlib/constants.md#build_version) |
| `EPSILON` | Constants | `EPSILON` | The smallest possible `float` value greater than zero. | [stdlib/constants.md#epsilon](stdlib/constants.md#epsilon) |
| `INFINITY` | Constants | `INFINITY` | The floating-point special value for positive infinity. | [stdlib/constants.md#infinity](stdlib/constants.md#infinity) |
| `MAX_FLOAT` | Constants | `MAX_FLOAT` | The largest representable finite value of the `float` type. | [stdlib/constants.md#max_float](stdlib/constants.md#max_float) |
| `MAX_INT` | Constants | `MAX_INT` | The largest representable value of the `int` type. | [stdlib/constants.md#max_int](stdlib/constants.md#max_int) |
| `MIN_FLOAT` | Constants | `MIN_FLOAT` | The smallest representable finite value of the `float` type. | [stdlib/constants.md#min_float](stdlib/constants.md#min_float) |
| `MIN_INT` | Constants | `MIN_INT` | The smallest representable value of the `int` type. | [stdlib/constants.md#min_int](stdlib/constants.md#min_int) |
| `NAN` | Constants | `NAN` | The floating-point special value for NaN (Not a Number). | [stdlib/constants.md#nan](stdlib/constants.md#nan) |
| `NEG_INFINITY` | Constants | `NEG_INFINITY` | The floating-point special value for negative infinity. | [stdlib/constants.md#neg_infinity](stdlib/constants.md#neg_infinity) |
| `RANT_VERSION` | Constants | `RANT_VERSION` | The version of the Rant language currently being used. | [stdlib/constants.md#rant_version](stdlib/constants.md#rant_version) |
| `abs` | Math | `[%abs: num]` | Prints the absolute value of `num`. | [stdlib/math.md#abs](stdlib/math.md#abs) |
| `acos` | Math | `[%acos: x]` | Calculates the arccosine (in radians) of `x`. | [stdlib/math.md#acos](stdlib/math.md#acos) |
| `add` | Math | `[%add: lhs; rhs]` | Adds two values and prints the sum. | [stdlib/math.md#add](stdlib/math.md#add) |
| `alpha` | Generators | `[%alpha: count ? 1]` | Prints a uniformly random lowercase alphabetic string. | [stdlib/generators.md#alpha](stdlib/generators.md#alpha) |
| `alt` | General | `[%alt: a; ...rest]` | Prints the first argument that is not `nothing`. | [stdlib/general.md#alt](stdlib/general.md#alt) |
| `and` | Boolean | `[%and: a; b; c*]` | Performs a boolean AND operation on the operands and returns the result. | [stdlib/boolean.md#and](stdlib/boolean.md#and) |
| `asin` | Math | `[%asin: x]` | Calculates the arcsine (in radians) of `x`. | [stdlib/math.md#asin](stdlib/math.md#asin) |
| `assert` | Assertion | `[%assert: condition; message?]` | Verifies that `condition` is true before continuing program execution. | [stdlib/assertion.md#assert](stdlib/assertion.md#assert) |
| `assert-eq` | Assertion | `[%assert-eq: actual; expected; message?]` | Verifies that `expected` and `actual` are equal before continuing program execution. | [stdlib/assertion.md#assert-eq](stdlib/assertion.md#assert-eq) |
| `assert-neq` | Assertion | `[%assert-neq: actual; unexpected; message?]` | Verifies that `unexpected` and `actual` are not equal before continuing program execution. | [stdlib/assertion.md#assert-neq](stdlib/assertion.md#assert-neq) |
| `assert-not` | Assertion | `[%assert-not: condition; message?]` | Verifies that `condition` is false before continuing program execution. | [stdlib/assertion.md#assert-not](stdlib/assertion.md#assert-not) |
| `assoc` | Collections | `[%assoc: keys; values]` | Creates a map from a list of keys and a list of values. Each key in `keys` will be matched with the value at the same index in `values`. | [stdlib/collections.md#assoc](stdlib/collections.md#assoc) |
| `atan` | Math | `[%atan: x]` | Calculates the arctangent (in radians) of `x`. | [stdlib/math.md#atan](stdlib/math.md#atan) |
| `atan2` | Math | `[%atan2: y; x]` | Calculates the four-quadrant arctangent (in radians) of \\(\frac{y}{x}\\). | [stdlib/math.md#atan2](stdlib/math.md#atan2) |
| `augment` | Collections | `[%augment: dst-map; src-map]` | Clones `dst-map`, adds the values from `src-map` to the values with matching keys on `dst-map`, then returns the resulting map. | [stdlib/collections.md#augment](stdlib/collections.md#augment) |
| `augment-self` | Collections | `[%augment-self: dst-map; src-map]` | Adds the values from `src-map` to the values with matching keys on `dst-map`. | [stdlib/collections.md#augment-self](stdlib/collections.md#augment-self) |
| `augment-thru` | Collections | `[%augment-self: dst-map; src-map]` | Adds the values from `src-map` to the values with matching keys on `dst-map`, then prints `dst-map`. | [stdlib/collections.md#augment-thru](stdlib/collections.md#augment-thru) |
| `call` | General | `[%call: func; args?]` | Calls `func` with an optional list of argument values. | [stdlib/general.md#call](stdlib/general.md#call) |
| `cat` | General | `[%cat: ...values]` | Prints each argument into the current scope. | [stdlib/general.md#cat](stdlib/general.md#cat) |
| `ceil` | Math | `[%ceil: val]` | Prints the smallest integer that is greater than or equal to `val`. | [stdlib/math.md#ceil](stdlib/math.md#ceil) |
| `char` | Strings | `[%char: code]` | Prints the Unicode character represented by the Unicode code point `code`. If `code` doesn't correspond to a valid character, prints nothing. | [stdlib/strings.md#char](stdlib/strings.md#char) |
| `chunks` | Collections | `[%chunks: collection; count]` | Splits `collection` into `count` sub-slices and prints a new list containing the results, making a best-effort to make each chunk the same size. | [stdlib/collections.md#chunks](stdlib/collections.md#chunks) |
| `clamp` | Math | `[%clamp: value; a; b]` | Prints `value` clamped to the inclusive range between `a` and `b`. | [stdlib/math.md#clamp](stdlib/math.md#clamp) |
| `clear` | Collections | `[%clear: collection]` | Removes all elements from a list or map. | [stdlib/collections.md#clear](stdlib/collections.md#clear) |
| `cos` | Math | `[%cos: x]` | Calculates the cosine of `x`. | [stdlib/math.md#cos](stdlib/math.md#cos) |
| `dig` | Generators | `[%dig: count ? 1]` | Prints a uniformly random decimal digit. If `count` is specified, repeats `count` times. | [stdlib/generators.md#dig](stdlib/generators.md#dig) |
| `digh` | Generators | `[%digh: count ? 1]` | Prints a uniformly random lowercase hexadecimal digit. If `count` is specified, repeats `count` times. | [stdlib/generators.md#digh](stdlib/generators.md#digh) |
| `dignz` | Generators | `[%dignz: count ? 1]` | Prints a uniformly random non-zero decimal digit. If `count` is specified, repeats `count` times. | [stdlib/generators.md#dignz](stdlib/generators.md#dignz) |
| `div` | Math | `[%div: lhs; rhs]` | Divides two values and prints the quotient. | [stdlib/math.md#div](stdlib/math.md#div) |
| `ds-query-sources` | General | `[%ds-query-sources]` | Prints the list of currently registered data-source IDs. | [stdlib/general.md#ds-query-sources](stdlib/general.md#ds-query-sources) |
| `ds-request` | General | `[%ds-request: id; ...args]` | Calls a registered data source by ID and prints its result. | [stdlib/general.md#ds-request](stdlib/general.md#ds-request) |
| `either` | General | `[%either: condition; true-value; false-value]` | Prints `true-value` when `condition` is true, otherwise `false-value`. | [stdlib/general.md#either](stdlib/general.md#either) |
| `else` | Attributes & Control Flow | `[%else]` | Marks the next block as the fallback branch after a previous conditional block. | [stdlib/control-flow.md#else](stdlib/control-flow.md#else) |
| `elseif` | Attributes & Control Flow | `[%elseif: condition]` | Marks the next block as an `else if` branch following a previous conditional block. | [stdlib/control-flow.md#elseif](stdlib/control-flow.md#elseif) |
| `eq` | Comparison | `[%eq: lhs; rhs]` | Returns `@true` if `lhs` and `rhs` are equal. | [stdlib/comparison.md#eq](stdlib/comparison.md#eq) |
| `error` | General | `[%error: message?]` | Raises a `USER_ERROR` runtime failure with an optional message. | [stdlib/general.md#error](stdlib/general.md#error) |
| `fill-self` | Collections | `[%fill-self: list; value]` | Mutates `list` in place by replacing every element with `value`. | [stdlib/collections.md#fill-self](stdlib/collections.md#fill-self) |
| `fill-thru` | Collections | `[%fill-thru: list; value]` | Mutates `list` in place and then prints the same list handle. | [stdlib/collections.md#fill-thru](stdlib/collections.md#fill-thru) |
| `filter` | Collections | `[%filter: list; predicate]` | Runs a predicate function against all items in a list and returns another list containing only the values that the predicate returned `@true` on. | [stdlib/collections.md#filter](stdlib/collections.md#filter) |
| `floor` | Math | `[%floor: val]` | Prints the largest integer that is less than or equal to `val`. | [stdlib/math.md#floor](stdlib/math.md#floor) |
| `fork` | General | `[%fork: seed?]` | Pushes a derived RNG onto the RNG stack. Integer and string seeds are both supported. | [stdlib/general.md#fork](stdlib/general.md#fork) |
| `frac` | Math | `[%frac: val]` | Prints the fractional part of `val`. | [stdlib/math.md#frac](stdlib/math.md#frac) |
| `ge` | Comparison | `[%ge: lhs; rhs]` | Returns `@true` if `lhs` is greater than or equal to `rhs`. | [stdlib/comparison.md#ge](stdlib/comparison.md#ge) |
| `gt` | Comparison | `[%gt: lhs; rhs]` | Returns `@true` if `lhs` is greater than `rhs`. | [stdlib/comparison.md#gt](stdlib/comparison.md#gt) |
| `has` | Collections | `[%has: collection; value]` | Returns a boolean indicating whether `value` occurs in `collection`. | [stdlib/collections.md#has](stdlib/collections.md#has) |
| `if` | Attributes & Control Flow | `[%if: condition]` | Marks the next block as conditional and resolves it only when `condition` is truthy. | [stdlib/control-flow.md#if](stdlib/control-flow.md#if) |
| `indent` | Strings | `[%indent: text; indent]` | Prefixes every line in `text` with the string `indent`. | [stdlib/strings.md#indent](stdlib/strings.md#indent) |
| `index-of` | Collections | `[%index-of: list; value]` | Returns the index of the first occurrence of `value` in `list`. If no match is found, returns `<>`. | [stdlib/collections.md#index-of](stdlib/collections.md#index-of) |
| `insert` | Collections | `[%insert: collection; value; pos]` | Inserts `value` into a list or map at the position `pos`. | [stdlib/collections.md#insert](stdlib/collections.md#insert) |
| `irange` | General | `[%irange: a; b?; step?]` | Builds an inclusive integer range. | [stdlib/general.md#irange](stdlib/general.md#irange) |
| `is` | Verification | `[%is: value; type-name]` | Returns `@true` when `value`'s runtime type name exactly matches `type-name`. | [stdlib/verification.md#is](stdlib/verification.md#is) |
| `is-between` | Verification | `[%is-between: value; a; b]` | Returns `@true` if `value` is bewteen `a` and `b` (both inclusive). | [stdlib/verification.md#is-between](stdlib/verification.md#is-between) |
| `is-bool` | Verification | `[%is-bool: value]` | Returns `@true` if `value` is of type `bool`. | [stdlib/verification.md#is-bool](stdlib/verification.md#is-bool) |
| `is-even` | Verification | `[%is-even: number]` | Returns `@true` if `number` is an even number. | [stdlib/verification.md#is-even](stdlib/verification.md#is-even) |
| `is-factor` | Verification | `[%is-factor: value; factor]` | Returns `@true` if `value` is evenly divisible by `factor`. | [stdlib/verification.md#is-factor](stdlib/verification.md#is-factor) |
| `is-float` | Verification | `[%is-float: value]` | Returns `@true` if `value` is of type `float`. | [stdlib/verification.md#is-float](stdlib/verification.md#is-float) |
| `is-int` | Verification | `[%is-int: value]` | Returns `@true` if `value` is of type `int`. | [stdlib/verification.md#is-int](stdlib/verification.md#is-int) |
| `is-nan` | Verification | `[%is-nan: value]` | Returns `@true` if `value` is of type `float` and equal to NaN (Not a Number). | [stdlib/verification.md#is-nan](stdlib/verification.md#is-nan) |
| `is-nothing` | Verification | `[%is-nothing: value]` | Returns `@true` if `value` is of type `nothing`. | [stdlib/verification.md#is-nothing](stdlib/verification.md#is-nothing) |
| `is-number` | Verification | `[%is-number: value]` | Returns `@true` if `value` is of type `int` or `float`. | [stdlib/verification.md#is-number](stdlib/verification.md#is-number) |
| `is-odd` | Verification | `[%is-odd: number]` | Returns `@true` if `number` is an odd number. | [stdlib/verification.md#is-odd](stdlib/verification.md#is-odd) |
| `is-some` | Verification | `[%is-some: value]` | Returns `@true` if `value` is any non-`nothing` type. | [stdlib/verification.md#is-some](stdlib/verification.md#is-some) |
| `is-string` | Verification | `[%is-string: value]` | Returns `@true` if `value` is of type `string`. | [stdlib/verification.md#is-string](stdlib/verification.md#is-string) |
| `join` | Collections | `[%join: list; separator?]` | Prints the elements of a list in order separated by the `separator` value. | [stdlib/collections.md#join](stdlib/collections.md#join) |
| `keys` | Collections | `[%keys: map]` | Prints a list of the keys stored in `map`. | [stdlib/collections.md#keys](stdlib/collections.md#keys) |
| `last-index-of` | Collections | `[%last-index-of: list; value]` | Returns the index of the last occurrence of `value` in `list`. If no match is found, returns `<>`. | [stdlib/collections.md#last-index-of](stdlib/collections.md#last-index-of) |
| `le` | Comparison | `[%le: lhs; rhs]` | Returns `@true` if `lhs` is less than or equal to `rhs`. | [stdlib/comparison.md#le](stdlib/comparison.md#le) |
| `len` | General | `[%len: value]` | Prints the length of a string, list, map, range, or other length-aware value. | [stdlib/general.md#len](stdlib/general.md#len) |
| `lines` | Strings | `[%lines: str]` | Splits string `str` by line feed characters (0x0A, `\n`) and returns a list of the results. | [stdlib/strings.md#lines](stdlib/strings.md#lines) |
| `list` | Collections | `[%list: values*]` | Prints a `list` containing the arguments. | [stdlib/collections.md#list](stdlib/collections.md#list) |
| `lower` | Strings | `[%lower: str]` | Converts string `str` to lowercase and returns the result. | [stdlib/strings.md#lower](stdlib/strings.md#lower) |
| `lt` | Comparison | `[%lt: lhs; rhs]` | Returns `@true` if `lhs` is less than `rhs`. | [stdlib/comparison.md#lt](stdlib/comparison.md#lt) |
| `map` | Collections | `[%map: list; map-func]` | Calls the supplied function on each item in a list and returns another list with the results in the same order. | [stdlib/collections.md#map](stdlib/collections.md#map) |
| `max` | Math | `[%max: values+]` | Returns the largest value in `values`. | [stdlib/math.md#max](stdlib/math.md#max) |
| `maybe` | Generators | `[%maybe: p ? 0.5]` | Returns a `bool` value with `p` probability of being true. | [stdlib/generators.md#maybe](stdlib/generators.md#maybe) |
| `min` | Math | `[%min: values+]` | Returns the smallest value in `values`. | [stdlib/math.md#min](stdlib/math.md#min) |
| `mksel` | Attributes & Control Flow | `[%mksel: selector-mode]` | Creates and returns a selector with the specified mode. | [stdlib/control-flow.md#mksel](stdlib/control-flow.md#mksel) |
| `mod` | Math | `[%mod: lhs; rhs]` | Gets the modulus of two values. | [stdlib/math.md#mod](stdlib/math.md#mod) |
| `mul` | Math | `[%mul: lhs; rhs]` | Multiplies two values and prints the product. | [stdlib/math.md#mul](stdlib/math.md#mul) |
| `mul-add` | Math | `[%mul-add: lhs; rhs; add]` | Multiplies two values, then adds another value to the result. | [stdlib/math.md#mul-add](stdlib/math.md#mul-add) |
| `mut` | Attributes & Control Flow | `[%mut: mutator?]` | Sets the mutator function for the next block, or clears it when passed `nothing`. | [stdlib/control-flow.md#mut](stdlib/control-flow.md#mut) |
| `neg` | Math | `[%neg: val]` | Negates a value and prints it. | [stdlib/math.md#neg](stdlib/math.md#neg) |
| `neq` | Comparison | `[%neq: lhs; rhs]` | Returns `@true` if `lhs` and `rhs` are not equal. | [stdlib/comparison.md#neq](stdlib/comparison.md#neq) |
| `nlist` | Collections | `[%nlist: values*]` | Prints a single-element list whose only value is the list constructed from `values`. | [stdlib/collections.md#nlist](stdlib/collections.md#nlist) |
| `not` | Boolean | `[%not: a]` | Returns the inverse of the input boolean value. | [stdlib/boolean.md#not](stdlib/boolean.md#not) |
| `num-fmt` | Formatting | `[%num-fmt: options?; depth ? 0]` | Gets or sets the number formatting options for the calling scope. | [stdlib/formatting.md#num-fmt](stdlib/formatting.md#num-fmt) |
| `num-fmt-alt` | Formatting | `[%num-fmt-alt: flag?; depth ? 0]` | Gets or sets the number formatter's alternate format flag. | [stdlib/formatting.md#num-fmt-alt](stdlib/formatting.md#num-fmt-alt) |
| `num-fmt-decimal-sep` | Formatting | `[%num-fmt-decimal-sep: decimal-sep?; depth ? 0]` | Gets or sets the number formatter's decimal separator. | [stdlib/formatting.md#num-fmt-decimal-sep](stdlib/formatting.md#num-fmt-decimal-sep) |
| `num-fmt-endian` | Formatting | `[%num-fmt-endian: endianness?; depth ? 0]` | Gets or sets the number format's endianness. This affects the byte order of power-of-two base formats such as `binary` and `hex`. | [stdlib/formatting.md#num-fmt-endian](stdlib/formatting.md#num-fmt-endian) |
| `num-fmt-group-sep` | Formatting | `[%num-fmt-group-sep: group-sep?; depth ? 0]` | Gets or sets the number formatter's digit group separator. | [stdlib/formatting.md#num-fmt-group-sep](stdlib/formatting.md#num-fmt-group-sep) |
| `num-fmt-infinity` | Formatting | `[%num-fmt-infinity: infinity?; depth ? 0]` | Gets or sets the number formatter's infinity style. | [stdlib/formatting.md#num-fmt-infinity](stdlib/formatting.md#num-fmt-infinity) |
| `num-fmt-padding` | Formatting | `[%num-fmt-padding: padding?; depth ? 0]` | Gets or sets the number formatter's digit padding size. | [stdlib/formatting.md#num-fmt-padding](stdlib/formatting.md#num-fmt-padding) |
| `num-fmt-precision` | Formatting | `[%num-fmt-precision: precision?; depth ? 0]` | Gets or sets the number formatter's decimal precision. | [stdlib/formatting.md#num-fmt-precision](stdlib/formatting.md#num-fmt-precision) |
| `num-fmt-sign` | Formatting | `[%num-fmt-sign: sign?; depth ? 0]` | Gets or sets the number formatter's sign style. | [stdlib/formatting.md#num-fmt-sign](stdlib/formatting.md#num-fmt-sign) |
| `num-fmt-system` | Formatting | `[%num-fmt-system: system?; depth ? 0]` | Gets or sets the number formatter's numeral system. | [stdlib/formatting.md#num-fmt-system](stdlib/formatting.md#num-fmt-system) |
| `num-fmt-upper` | Formatting | `[%num-fmt-upper: flag?; depth ? 0]` | Gets or sets the number formatter's uppercase formatting flag. | [stdlib/formatting.md#num-fmt-upper](stdlib/formatting.md#num-fmt-upper) |
| `or` | Boolean | `[%or: a; b; c*]` | Performs a boolean inclusive OR operation on the operands and returns the result. | [stdlib/boolean.md#or](stdlib/boolean.md#or) |
| `ord` | Strings | `[%ord: ch]` | Prints the Unicode code point of the specified character as an `int` value. If an empty string is passed, prints nothing. | [stdlib/strings.md#ord](stdlib/strings.md#ord) |
| `ord-all` | Strings | `[%ord-all: str]` | Prints a list of `int` values containing the Unicode code points contained in `str`. | [stdlib/strings.md#ord-all](stdlib/strings.md#ord-all) |
| `oxford-join` | Collections | `[%oxford-join: comma; conj; comma-conj; list]` | A variant of the `join` function for conveniently formatting comma-separated lists. | [stdlib/collections.md#oxford-join](stdlib/collections.md#oxford-join) |
| `pick` | Generators | `[%pick: collection]` | Prints a random element from an ordered collection. | [stdlib/generators.md#pick](stdlib/generators.md#pick) |
| `pick-sparse` | Generators | `[%pick-sparse: first; ...rest]` | Randomly selects one value from a sparse weighted set of arguments by using each argument's length as its weight. | [stdlib/generators.md#pick-sparse](stdlib/generators.md#pick-sparse) |
| `pickn` | Generators | `[%pickn: collection; count]` | Prints a list containing `count` random elements sampled from `collection`. | [stdlib/generators.md#pickn](stdlib/generators.md#pickn) |
| `pop` | Collections | `[%pop: list]` | Removes the last value from a list and prints it. | [stdlib/collections.md#pop](stdlib/collections.md#pop) |
| `pow` | Math | `[%pow: x; y]` | Calculates \\(x^y\\) and prints the result. Both `x` and `y` can be `int` or `float`. | [stdlib/math.md#pow](stdlib/math.md#pow) |
| `print` | General | `[%print: ...values]` | Prints values directly into the caller's output scope. | [stdlib/general.md#print](stdlib/general.md#print) |
| `proto` | General | `[%proto: map]` | Prints the prototype map of `map`, or `nothing` if no prototype is set. | [stdlib/general.md#proto](stdlib/general.md#proto) |
| `push` | Collections | `[%push: list; value]` | Appends a value to the end of a list. | [stdlib/collections.md#push](stdlib/collections.md#push) |
| `rand` | Generators | `[%rand: a; b]` | Prints a random integer with uniform distribution between `a` and `b` (both inclusive). | [stdlib/generators.md#rand](stdlib/generators.md#rand) |
| `rand-list` | Generators | `[%rand-list: a; b; n]` | Prints a list of `n` random integers with uniform distribution between `a` and `b` (both inclusive). | [stdlib/generators.md#rand-list](stdlib/generators.md#rand-list) |
| `rand-list-sum` | Generators | `[%rand-list-sum: input; count; variance]` | Generates a list of `count` random numbers, whose sum equals `input`, and with a maximum absolute difference of `variance`. | [stdlib/generators.md#rand-list-sum](stdlib/generators.md#rand-list-sum) |
| `randf` | Generators | `[%randf: a; b]` | Prints a random float with uniform distribution between `a` (inclusive) and `b` (exclusive). | [stdlib/generators.md#randf](stdlib/generators.md#randf) |
| `randf-list` | Generators | `[%randf-list: a; b; n]` | Prints a list of `n` random floats with uniform distribution between `a` (inclusive) and `b` (exclusive). | [stdlib/generators.md#randf-list](stdlib/generators.md#randf-list) |
| `range` | General | `[%range: a; b?; step?]` | Builds a half-open integer range. | [stdlib/general.md#range](stdlib/general.md#range) |
| `recip` | Math | `[%recip: n]` | Gets the reciprocal of a value. | [stdlib/math.md#recip](stdlib/math.md#recip) |
| `remove` | Collections | `[%remove: collection; pos]` | Removes the value at the `pos` from a list or map. | [stdlib/collections.md#remove](stdlib/collections.md#remove) |
| `rep` | Attributes & Control Flow | `[%rep: reps]` | Sets the repetition count or repetition mode for the next block. | [stdlib/control-flow.md#rep](stdlib/control-flow.md#rep) |
| `require` | General | `[%require: module-path]` | Imports a module through the active module resolver. | [stdlib/general.md#require](stdlib/general.md#require) |
| `reset-attrs` | Attributes & Control Flow | `[%reset-attrs]` | Resets the current attribute state back to the runtime defaults. | [stdlib/control-flow.md#reset-attrs](stdlib/control-flow.md#reset-attrs) |
| `rev` | Collections | `[%rev: collection]` | Prints a reversed copy of the input collection. | [stdlib/collections.md#rev](stdlib/collections.md#rev) |
| `seed` | General | `[%seed]` | Prints the currently active RNG seed as an `int`. | [stdlib/general.md#seed](stdlib/general.md#seed) |
| `seg` | Strings | `[%seg: str; size]` | Segments the input text into a list of strings of `size` length. | [stdlib/strings.md#seg](stdlib/strings.md#seg) |
| `sel` | Attributes & Control Flow | `[%sel: selector?]` | Sets the active selector for the next block. With no argument, prints the current selector or `nothing`. | [stdlib/control-flow.md#sel](stdlib/control-flow.md#sel) |
| `sel-freeze` | Attributes & Control Flow | `[%sel-freeze: selector; frozen?]` | Sets the frozen state of `selector`. Omitting `frozen` freezes it. | [stdlib/control-flow.md#sel-freeze](stdlib/control-flow.md#sel-freeze) |
| `sel-frozen` | Attributes & Control Flow | `[%sel-frozen: selector]` | Prints whether `selector` is currently frozen. | [stdlib/control-flow.md#sel-frozen](stdlib/control-flow.md#sel-frozen) |
| `sel-skip` | Attributes & Control Flow | `[%sel-skip: selector; n?]` | Advances `selector` without printing any selected value. | [stdlib/control-flow.md#sel-skip](stdlib/control-flow.md#sel-skip) |
| `sep` | Attributes & Control Flow | `[%sep: separator]` | Sets the separator value for repeated block iterations. | [stdlib/control-flow.md#sep](stdlib/control-flow.md#sep) |
| `set-proto` | General | `[%set-proto: map; proto?]` | Sets or clears the prototype map for `map`. | [stdlib/general.md#set-proto](stdlib/general.md#set-proto) |
| `shuffle` | Collections | `[%shuffle: list]` | Creates a shuffled copy of a list. | [stdlib/collections.md#shuffle](stdlib/collections.md#shuffle) |
| `shuffle-self` | Collections | `[%shuffle-self: list]` | Shuffles the elements of a list in-place. | [stdlib/collections.md#shuffle-self](stdlib/collections.md#shuffle-self) |
| `shuffle-thru` | Collections | `[%shuffle-thru: list]` | Shuffles the elements of a list in-place, then prints the list. | [stdlib/collections.md#shuffle-thru](stdlib/collections.md#shuffle-thru) |
| `sift` | Collections | `[%sift: list; target-size]` | Returns a copy of a list with random elements removed until the number of elements in the list copy reaches `target-size`. If the number of elements in the list is less than or equal to `target-size`, this function simply returns an exact copy of the original list. | [stdlib/collections.md#sift](stdlib/collections.md#sift) |
| `sift-self` | Collections | `[%sift-self: list; target-size]` | Removes random elements from a list in-place until the number of elements in the list reaches `target-size`. If the number of elements in the list is less than or equal to `target-size`, this function does nothing. | [stdlib/collections.md#sift-self](stdlib/collections.md#sift-self) |
| `sift-thru` | Collections | `[%sift-thru: list; target-size]` | Removes random elements from a list in-place until the number of elements in the list reaches `target-size`. If the number of elements in the list is less than or equal to `target-size`, this function does nothing. | [stdlib/collections.md#sift-thru](stdlib/collections.md#sift-thru) |
| `sin` | Math | `[%sin: x]` | Calculates the sine of `x`. | [stdlib/math.md#sin](stdlib/math.md#sin) |
| `sort` | Collections | `[%sort: list]` | Creates a copy of a list with its elements sorted in ascending order. | [stdlib/collections.md#sort](stdlib/collections.md#sort) |
| `sort-self` | Collections | `[%sort-self: list]` | Sorts the elements of a list in-place in ascending order. | [stdlib/collections.md#sort-self](stdlib/collections.md#sort-self) |
| `sort-thru` | Collections | `[%sort-thru: list]` | Sorts the elements of a list in-place in ascending order, then prints the list. | [stdlib/collections.md#sort-thru](stdlib/collections.md#sort-thru) |
| `split` | Strings | `[%split: str; sep?]` | Splits the input text by `sep` into a list of strings. If `sep` is omitted, splits into characters. | [stdlib/strings.md#split](stdlib/strings.md#split) |
| `sqrt` | Math | `[%sqrt: x]` | Calculates the square root of `x`. | [stdlib/math.md#sqrt](stdlib/math.md#sqrt) |
| `squish` | Collections | `[%squish: list; target-size]` | Returns a copy of a list with random adjacent elements merged using addition until the number of elements in the list copy reaches `target-size`. If the number of elements in the list is less than or equal to `target-size`, this function simply returns an exact copy of the original list. | [stdlib/collections.md#squish](stdlib/collections.md#squish) |
| `squish-self` | Collections | `[%squish-self: list; target-size]` | Merges random adjacent elements in a list using addition until the number of elements in the list reaches `target-size`. If the number of elements in the list is less than or equal to `target-size`, this function does nothing. | [stdlib/collections.md#squish-self](stdlib/collections.md#squish-self) |
| `squish-thru` | Collections | `[%squish-thru: list; target-size]` | Merges random adjacent elements in a list using addition until the number of elements in the list reaches `target-size`, then prints the list. If the number of elements in the list is less than or equal to `target-size`, this function does nothing. | [stdlib/collections.md#squish-thru](stdlib/collections.md#squish-thru) |
| `step` | Attributes & Control Flow | `[%step]` | Prints the current repeater step value. | [stdlib/control-flow.md#step](stdlib/control-flow.md#step) |
| `step-count` | Attributes & Control Flow | `[%step-count]` | Prints the total number of iterations scheduled for the active repeater. | [stdlib/control-flow.md#step-count](stdlib/control-flow.md#step-count) |
| `step-index` | Attributes & Control Flow | `[%step-index]` | Prints the zero-based iteration index of the active repeater. | [stdlib/control-flow.md#step-index](stdlib/control-flow.md#step-index) |
| `string-replace` | Strings | `[%string-replace: input; query; replacement]` | Prints `input` with every occurrence of `query` replaced by `replacement`. | [stdlib/strings.md#string-replace](stdlib/strings.md#string-replace) |
| `sub` | Math | `[%sub: lhs; rhs]` | Prints the difference between two values. | [stdlib/math.md#sub](stdlib/math.md#sub) |
| `sum` | Collections | `[%sum: list]` | Adds the elements of a list together from left to right and prints the result. | [stdlib/collections.md#sum](stdlib/collections.md#sum) |
| `take` | Collections | `[%take: collection; pos]` | Removes the value at `pos` from a list or map and prints it. | [stdlib/collections.md#take](stdlib/collections.md#take) |
| `tan` | Math | `[%tan: x]` | Calculates the tangent of `x`. | [stdlib/math.md#tan](stdlib/math.md#tan) |
| `tap` | General | `[%tap: ...]` | Consumes arguments and produces no output. This is useful as a no-op sink in pipe chains. | [stdlib/general.md#tap](stdlib/general.md#tap) |
| `to-bool` | Conversion | `[%to-bool: value]` | Converts `value` to a boolean using Rant's runtime truthiness rules. | [stdlib/conversion.md#to-bool](stdlib/conversion.md#to-bool) |
| `to-float` | Conversion | `[%to-float: value]` | Attempts to convert `value` to a `float` value and prints the result. If the conversion fails, prints nothing. | [stdlib/conversion.md#to-float](stdlib/conversion.md#to-float) |
| `to-int` | Conversion | `[%to-int: value]` | Attempts to convert `value` to an `int` value and prints the result. If the conversion fails, prints nothing. | [stdlib/conversion.md#to-int](stdlib/conversion.md#to-int) |
| `to-list` | Conversion | `[%to-list: value]` | Attempts to convert `value` to a `list` and prints the result. If the conversion fails, prints nothing. | [stdlib/conversion.md#to-list](stdlib/conversion.md#to-list) |
| `to-string` | Conversion | `[%to-string: value]` | Attempts to convert `value` to a `string` value and prints the result. If the conversion fails, prints nothing. | [stdlib/conversion.md#to-string](stdlib/conversion.md#to-string) |
| `to-tuple` | Conversion | `[%to-tuple: value]` | Attempts to convert a value to a `tuple` and prints the result. If the conversion fails, prints nothing. | [stdlib/conversion.md#to-tuple](stdlib/conversion.md#to-tuple) |
| `translate` | Collections | `[%translate: list; map]` | Matches each item in a list to a map and returns a list with the corresponding map values. Values that have no corresponding key in the map are passed through as-is. | [stdlib/collections.md#translate](stdlib/collections.md#translate) |
| `trim` | Strings | `[%trim: str]` | Prints `str` with leading and trailing whitespace removed. | [stdlib/strings.md#trim](stdlib/strings.md#trim) |
| `try` | General | `[%try: context; handler?]` | Runs `context` and optionally dispatches runtime failures to `handler`. | [stdlib/general.md#try](stdlib/general.md#try) |
| `tuple` | Collections | `[%tuple: values*]` | Prints a `tuple` containing the arguments. | [stdlib/collections.md#tuple](stdlib/collections.md#tuple) |
| `type` | General | `[%type: value]` | Prints the runtime type name of `value`. | [stdlib/general.md#type](stdlib/general.md#type) |
| `unfork` | General | `[%unfork]` | Pops the most recent derived RNG and resumes the previous RNG state. | [stdlib/general.md#unfork](stdlib/general.md#unfork) |
| `upper` | Strings | `[%upper: str]` | Converts string `str` to uppercase and returns the result. | [stdlib/strings.md#upper](stdlib/strings.md#upper) |
| `values` | Collections | `[%values: map]` | Prints a list of the values stored in `map`. | [stdlib/collections.md#values](stdlib/collections.md#values) |
| `ws-fmt` | Formatting | `[%ws-fmt: mode?; custom?]` | Gets or sets the whitespace normalization mode for the current scope. `custom` is only used when `mode` is `custom`. | [stdlib/formatting.md#ws-fmt](stdlib/formatting.md#ws-fmt) |
| `xor` | Boolean | `[%xor: a; b]` | Performs a boolean XOR operation on the operands and returns the result. | [stdlib/boolean.md#xor](stdlib/boolean.md#xor) |
| `zip` | Collections | `[%zip: list-a; list-b; zip-func]` | Returns a new list that combines each pair of items from the two input lists using the specified function. The lists do not need to be the same length; if there is a difference, it will be made up with empties. | [stdlib/collections.md#zip](stdlib/collections.md#zip) |
