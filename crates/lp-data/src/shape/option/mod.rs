//! Option shape types.

pub mod option_dynamic;
pub mod option_meta;
pub mod option_static;

#[cfg(test)]
mod option_tests;

pub use option_dynamic::DynamicOptionShape;
pub use option_static::StaticOptionShape;
