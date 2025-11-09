//! Enum shape types.

pub mod enum_dynamic;
pub mod enum_meta;
pub mod enum_static;
pub mod enum_value;

#[cfg(test)]
mod enum_tests;

pub use enum_dynamic::DynamicEnumShape;
pub use enum_meta::{EnumUi, EnumVariant};
pub use enum_static::StaticEnumShape;
// EnumValue struct is available via enum_value module, not re-exported to avoid conflict with trait
