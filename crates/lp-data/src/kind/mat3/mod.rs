//! 3x3 matrix type support.

pub mod mat3_dyn;
pub mod mat3_meta;
pub mod mat3_shape;
pub mod mat3_static;
pub mod mat3_value;

#[cfg(test)]
mod mat3_tests;

pub use mat3_dyn::Mat3ShapeDyn;
pub use mat3_meta::{Mat3Meta, Mat3MetaDyn, Mat3MetaStatic};
pub use mat3_shape::Mat3Shape;
pub use mat3_static::{Mat3ShapeStatic, MAT3_SHAPE};
