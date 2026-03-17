# @break

`@break` exits the nearest active repeater immediately.

With an expression, `@break value` becomes the repeater result. Without an expression, the current element output becomes the repeater result.

It can exit through nested blocks owned by the repeater, but it does not cross function boundaries. Using it where no repeater is reachable raises a control-flow runtime error.

```rant
[rep:3]{
  before
  { @break stop }
  after
}
# -> stop
```
