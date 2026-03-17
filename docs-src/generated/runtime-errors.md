## Runtime Error Categories

| Category | Summary |
| --- | --- |
| `ARG_ERROR` | Invalid argument passed to function. |
| `ARG_MISMATCH_ERROR` | Too few/many arguments were passed to a function. |
| `ASSERT_ERROR` | Assertion failed. |
| `CONTROL_FLOW_ERROR` | Error during control flow operation (e.g. return or break). |
| `DATA_SOURCE_ERROR` | Error occurred during data source operation. |
| `INDEX_ERROR` | Error occurred while indexing value. |
| `INTERNAL_ERROR` | Internal VM error, usually indicating a bug or corrupted data. |
| `INVALID_ACCESS_ERROR` | Variable access error, such as attempting to access a nonexistent variable or write to a constant. |
| `INVALID_OP_ERROR` | Operation is not valid for the current program state. |
| `INVOKE_ERROR` | Tried to invoke a non-function. |
| `KEY_ERROR` | Error occurred while keying value. |
| `MODULE_ERROR` | Error occurred while trying to load a module. |
| `SELECTOR_ERROR` | Error occurred while iterating selector. |
| `SLICE_ERROR` | Error occurred while slicing value. |
| `STACK_OVERFLOW_ERROR` | Stack has overflowed. |
| `STACK_UNDERFLOW_ERROR` | Stack has underflowed. |
| `TYPE_ERROR` | Error occurred due to unexpected value type. |
| `USER_ERROR` | Error manually triggered by program. |
| `VALUE_ERROR` | Error occurred when creating value. |
