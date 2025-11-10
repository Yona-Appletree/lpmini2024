//! 32-bit signed integer type support.

pub mod int32_dyn;
pub mod int32_meta;
pub mod int32_shape;
pub mod int32_static;
pub mod int32_value;

#[cfg(test)]
mod int32_tests;

pub use int32_dyn::Int32ShapeDyn;
pub use int32_meta::{Int32Meta, Int32MetaDyn, Int32MetaStatic};
pub use int32_shape::Int32Shape;
pub use int32_static::{Int32ShapeStatic, INT32_SHAPE};
