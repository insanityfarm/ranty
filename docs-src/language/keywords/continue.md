# @continue

`@continue` skips the remainder of the nearest active repeater iteration and starts the next iteration.

With an expression, `@continue value` passes that value to the repeater for the current iteration. Without an expression, the current element output is used.

It can cross nested blocks that belong to the same repeater, but it does not cross function boundaries. Using it where no repeater is reachable raises a control-flow runtime error.

```ranty
[rep:3]{
  before
  { @continue next }
  after
}
# -> nextnextnext
```
