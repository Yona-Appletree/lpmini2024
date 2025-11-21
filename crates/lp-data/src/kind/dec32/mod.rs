//! Dec32-point number type support.

pub mod dec32_dyn;
pub mod dec32_meta;
pub mod dec32_shape;
pub mod dec32_static;
pub mod dec32_value;

#[cfg(test)]
mod dec32_tests;

pub use dec32_dyn::Dec32ShapeDyn;
pub use dec32_meta::{Dec32Meta, Dec32MetaDyn, Dec32MetaStatic};
pub use dec32_shape::Dec32Shape;
pub use dec32_static::{Dec32ShapeStatic, DEC32_SHAPE};
