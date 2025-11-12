//! Array (homogeneous collection) type support.

pub mod array_dyn;
pub mod array_meta;
pub mod array_shape;
pub mod array_static;
pub mod array_value;
pub mod array_value_dyn;

#[cfg(test)]
mod array_tests;

pub use array_dyn::ArrayShapeDyn;
pub use array_meta::{ArrayMeta, ArrayMetaDyn, ArrayMetaStatic};
pub use array_shape::ArrayShape;
pub use array_static::{ArrayShapeStatic, ARRAY_SHAPE};
pub use array_value::ArrayValue;
pub use array_value_dyn::ArrayValueDyn;
