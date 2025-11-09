//! Option shape types.

pub mod option_dynamic;
pub mod option_meta;
pub mod option_static;
pub mod option_value;

#[cfg(test)]
mod option_tests;

pub use option_dynamic::DynamicOptionShape;
pub use option_static::StaticOptionShape;
// OptionValue struct is available via option_value module, not re-exported to avoid conflict with trait
