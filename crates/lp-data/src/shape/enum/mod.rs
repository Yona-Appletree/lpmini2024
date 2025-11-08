//! Enum shape types.

pub mod enum_dynamic;
pub mod enum_meta;
pub mod enum_static;

#[cfg(test)]
mod enum_tests;

pub use enum_dynamic::DynamicEnumShape;
pub use enum_meta::{EnumUi, EnumVariant};
pub use enum_static::StaticEnumShape;
