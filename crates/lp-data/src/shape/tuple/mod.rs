//! Tuple shape types.

pub mod tuple_dynamic;
pub mod tuple_meta;
pub mod tuple_static;

#[cfg(test)]
mod tuple_tests;

pub use tuple_dynamic::DynamicTupleShape;
pub use tuple_static::StaticTupleShape;
