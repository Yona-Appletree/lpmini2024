# lp-data-derive

Procedural macros for deriving LP data types.

## Overview

`lp-data-derive` provides derive macros for automatic implementation of LP data type traits. It simplifies schema generation and serialization for types used in the lpmini system.

## Usage

```rust
use lp_data_derive::LpDataType;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

#[derive(LpDataType, JsonSchema, Serialize, Deserialize)]
struct MyConfig {
    #[lpschema(description = "Number of LED rings")]
    ring_count: i32,

    #[lpschema(description = "Base radius in meters")]
    radius: f32,
}
```

## Features

- **Automatic schema derivation** via `JsonSchema`
- **Attribute support** for `#[lpschema(...)]` metadata
- **Integration** with `serde` and `schemars`

## Note

This crate is primarily for future extensibility. Currently, users should include `JsonSchema` in their derive list alongside `LpDataType`.
