//! Enum type support.

pub mod enum_dyn;
pub mod enum_meta;
pub mod enum_shape;
pub mod enum_static;
pub mod enum_value;

pub use enum_dyn::EnumShapeDyn;
pub use enum_meta::{
    EnumMeta, EnumMetaDyn, EnumMetaStatic, EnumVariantMeta, EnumVariantMetaDyn,
    EnumVariantMetaStatic,
};
pub use enum_shape::{EnumShape, EnumVariantShape};
pub use enum_static::{EnumShapeStatic, EnumVariantStatic};
pub use enum_value::EnumValue;
