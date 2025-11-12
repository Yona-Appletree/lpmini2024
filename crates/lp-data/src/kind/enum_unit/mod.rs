//! Enum type support.

pub mod enum_dyn;
pub mod enum_meta;
pub mod enum_shape;
pub mod enum_static;
pub mod enum_value;

pub use enum_dyn::EnumUnitShapeDyn;
pub use enum_meta::{
    EnumUnitMeta, EnumUnitMetaDyn, EnumUnitMetaStatic, EnumUnitVariantMeta, EnumUnitVariantMetaDyn,
    EnumUnitVariantMetaStatic,
};
pub use enum_shape::{EnumUnitShape, EnumUnitVariantShape};
pub use enum_static::{EnumUnitShapeStatic, EnumUnitVariantStatic};
pub use enum_value::EnumUnitValue;
