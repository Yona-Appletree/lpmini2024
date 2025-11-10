//! Fixed-point number type support.

pub mod fixed_dyn;
pub mod fixed_meta;
pub mod fixed_shape;
pub mod fixed_static;
pub mod fixed_value;

#[cfg(test)]
mod fixed_tests;

pub use fixed_dyn::FixedShapeDyn;
pub use fixed_meta::{FixedMeta, FixedMetaDyn, FixedMetaStatic};
pub use fixed_shape::FixedShape;
pub use fixed_static::{FixedShapeStatic, FIXED_SHAPE};
