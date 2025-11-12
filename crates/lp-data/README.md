# lp-data

Type system and schema registry for runtime metadata.

## Overview

`lp-data` provides a shared data model for expressing types, values, and metadata in the lpmini system. It enables runtime type introspection and schema generation for configuration data, bridging the gap between compile-time structures and runtime metadata.

## Features

- **Type system** for primitives, structures, enums, and arrays
- **Runtime values** with type-safe access
- **Annotations** for additional metadata
- **Schema registry** for type registration and lookup
- **Serialization support** (via `serde`, optional)
- **JSON Schema generation** (via `schemars`, optional)
- **no_std compatible** with optional `alloc` support

## Usage

### Defining Types

```rust
use lp_data::ty::{LpField, LpStructType, LpType};

// Create a struct type
let mut struct_ty = LpStructType::new("CircleMappingConfig");
struct_ty.add_field(LpField::new("ring_counts", LpType::array(LpType::int32())));
struct_ty.add_field(LpField::new("radius", LpType::fixed32()));
let config_type = LpType::structure(struct_ty);
```

### Type Registry

```rust
use lp_data::{TypeRegistry, LpDataType};

// Register types for lookup
let mut registry = TypeRegistry::new();
registry.register("MyConfig", config_type);

// Retrieve types by name
let ty = registry.get("MyConfig");
```

### Runtime Values

```rust
use lp_data::value::Value;

// Create values with runtime type information
let int_value = Value::int32(42);
let float_value = Value::fixed32(1.5);
let vec_value = Value::vec2(0.5, 0.5);
```

### Arrays

Arrays are homogeneous collections that store elements of the same type:

```rust
use lp_data::kind::array::{ArrayShapeDyn, ArrayValueDyn, ArrayValue};
use lp_data::kind::array::array_meta::ArrayMetaDyn;
use lp_data::kind::int32::int32_static::INT32_SHAPE;
use lp_data::kind::value::LpValueBox;

// Create an array shape for Int32 elements
let shape = ArrayShapeDyn {
    meta: ArrayMetaDyn {
        name: "Int32Array".to_string(),
        docs: None,
    },
    element_shape: &INT32_SHAPE,
    len: 0,
};

// Create a dynamic array value
let mut array = ArrayValueDyn::new(shape);

// Add elements
array.push(LpValueBox::from(42i32))?;
array.push(LpValueBox::from(100i32))?;

// Access elements
let first = array.get_element(0)?;
assert_eq!(first.as_lp_value().shape().kind(), LpKind::Int32);
```

## Feature Flags

- `alloc` (default): Enable heap allocations
- `std`: Enable standard library features and `schemars`
- `serde_json`: Enable JSON serialization
