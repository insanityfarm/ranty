# Data Sources

Ranty can call **data sources** supplied by the embedding host at runtime.
A data source is registered on a `Ranty` context under a short ID and is then available to scripts through [[ds-request]](../stdlib/general.md#ds-request).

Unlike [modules](../modules.md), data sources are not Ranty code.
They are Rust-side integration points for controlled access to external state.

## When to use them

Use a module when the dependency itself is written in Ranty and should be loaded through `@require`.

Use a data source when the host application needs to expose something that lives outside Ranty, such as:

- application or game state,
- asset metadata,
- filesystem or network lookups,
- database queries,
- process or build information.

Because data sources are host code, the host decides what capabilities are exposed and how they are sandboxed.

## Registering a data source

Implement `DataSource` and register it with `Ranty::add_data_source`.

```rust,ignore
use ranty::{Ranty, RantyValue};
use ranty::data::{DataSource, DataSourceError};

#[derive(Debug)]
struct BuildInfoSource;

impl DataSource for BuildInfoSource {
    fn type_id(&self) -> &str {
        "build-info"
    }

    fn request_data(&self, _args: Vec<RantyValue>) -> Result<RantyValue, DataSourceError> {
        Ok(RantyValue::String("stable".into()))
    }
}

let mut ranty = Ranty::new();
ranty
    .add_data_source(BuildInfoSource)
    .expect("data source IDs should be unique");
```

The `type_id()` string is the script-facing ID.
It must be unique within a single `Ranty` context.
Registering a second data source with the same ID returns an error.

## Calling data sources from scripts

Scripts access data sources through the [Standard Library: General functions](../stdlib/general.md) helpers:

```ranty
[ds-query-sources]
[ds-request: build-info]
```

- [[ds-query-sources]](../stdlib/general.md#ds-query-sources) prints the IDs currently registered on the context.
- [[ds-request: id; ...args]](../stdlib/general.md#ds-request) calls the named data source with zero or more arguments.

The arguments arrive at `request_data()` as `Vec<RantyValue>`.
The host is responsible for validating argument count and types.

## Return values and errors

A data source returns any ordinary `RantyValue`.
That means it can provide strings, numbers, lists, maps, functions, or `nothing`, depending on what makes sense for the integration.

Use:

- `DataSourceError::User` for caller mistakes such as bad arguments or unknown keys,
- `DataSourceError::Internal` for host-side failures such as I/O or service errors.

Both surface to scripts as `DATA_SOURCE_ERROR`.
If a script calls an ID that is not registered, [[ds-request]](../stdlib/general.md#ds-request) also raises `DATA_SOURCE_ERROR`.

## Lifetime and scope

Data sources are attached to a single `Ranty` context.
They are not global to the process and they are not serialized into compiled programs.

The host can manage them with:

- `add_data_source()` to register,
- `remove_data_source()` to unregister one,
- `clear_data_sources()` to remove all of them,
- `iter_data_sources()` to inspect the current registry.

This makes it practical to give different contexts different capability sets.

## Security model

Ranty does not grant ambient filesystem, network, or database access on its own.
All such access must be provided explicitly through host code.

That means data sources are the right place to:

- enforce allowlists,
- validate arguments,
- redact or shape returned data,
- translate host failures into stable script-facing errors.

In short, a data source should expose the smallest capability surface the script actually needs.

## See also

- [Standard Library: General functions](../stdlib/general.md)
- [Modules](../modules.md)
- [Module Resolvers](module-resolvers.md)
