//! Array shape types.

pub mod array_dynamic;
pub mod array_meta;
pub mod array_static;
pub mod array_value;

#[cfg(test)]
mod array_tests;

pub use array_dynamic::DynamicArrayShape;
pub use array_meta::ArrayUi;
pub use array_static::StaticArrayShape;
// ArrayValue struct is available via array_value module, not re-exported to avoid conflict with trait
