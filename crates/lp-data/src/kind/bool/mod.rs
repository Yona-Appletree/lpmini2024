//! Boolean type support.

pub mod bool_dyn;
pub mod bool_meta;
pub mod bool_shape;
pub mod bool_static;
pub mod bool_value;

pub use bool_dyn::BoolShapeDyn;
pub use bool_meta::{BoolMeta, BoolMetaDyn, BoolMetaStatic};
pub use bool_shape::BoolShape;
pub use bool_static::{BoolShapeStatic, BOOL_SHAPE};
