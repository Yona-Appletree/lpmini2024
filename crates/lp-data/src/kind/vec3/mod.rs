//! 3D vector type support.

pub mod vec3_dyn;
pub mod vec3_meta;
pub mod vec3_shape;
pub mod vec3_static;
pub mod vec3_value;

#[cfg(test)]
mod vec3_tests;

pub use vec3_dyn::Vec3ShapeDyn;
pub use vec3_meta::{Vec3Meta, Vec3MetaDyn, Vec3MetaStatic};
pub use vec3_shape::Vec3Shape;
pub use vec3_static::{Vec3ShapeStatic, VEC3_SHAPE};
