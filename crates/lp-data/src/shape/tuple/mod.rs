//! Tuple shape types.

pub mod tuple_dynamic;
pub mod tuple_meta;
pub mod tuple_static;
pub mod tuple_value;

#[cfg(test)]
mod tuple_tests;

pub use tuple_dynamic::DynamicTupleShape;
pub use tuple_static::StaticTupleShape;
// TupleValue struct is available via tuple_value module, not re-exported to avoid conflict with trait
