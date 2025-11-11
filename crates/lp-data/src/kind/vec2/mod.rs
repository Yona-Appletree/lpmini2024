//! 2D vector type support.

pub mod vec2_dyn;
pub mod vec2_meta;
pub mod vec2_shape;
pub mod vec2_static;
pub mod vec2_value;

#[cfg(test)]
mod vec2_tests;

pub use vec2_dyn::Vec2ShapeDyn;
pub use vec2_meta::{Vec2Meta, Vec2MetaDyn, Vec2MetaStatic};
pub use vec2_shape::Vec2Shape;
pub use vec2_static::{Vec2ShapeStatic, VEC2_SHAPE};
