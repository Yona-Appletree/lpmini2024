//! Array shape types.

pub mod array_dynamic;
pub mod array_meta;
pub mod array_static;

#[cfg(test)]
mod array_tests;

pub use array_dynamic::DynamicArrayShape;
pub use array_meta::ArrayUi;
pub use array_static::StaticArrayShape;
