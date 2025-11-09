//! Map/dynamic record shape types.

pub mod map_dynamic;
pub mod map_meta;
pub mod map_static;
pub mod map_value;

#[cfg(test)]
mod map_tests;

pub use map_dynamic::DynamicMapShape;
pub use map_static::StaticMapShape;
pub use map_value::MapValue;
