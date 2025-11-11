//! 4D vector type support.

pub mod vec4_dyn;
pub mod vec4_meta;
pub mod vec4_shape;
pub mod vec4_static;
pub mod vec4_value;

#[cfg(test)]
mod vec4_tests;

pub use vec4_dyn::Vec4ShapeDyn;
pub use vec4_meta::{Vec4Meta, Vec4MetaDyn, Vec4MetaStatic};
pub use vec4_shape::Vec4Shape;
pub use vec4_static::{Vec4ShapeStatic, VEC4_SHAPE};
