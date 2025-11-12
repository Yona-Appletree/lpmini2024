//! Union type support.

pub mod enum_struct_dyn;
pub mod enum_struct_meta;
pub mod enum_struct_shape;
pub mod enum_struct_static;
pub mod enum_struct_value;
pub mod enum_struct_value_dyn;

pub use enum_struct_dyn::{EnumStructShapeDyn, EnumStructVariantDyn};
pub use enum_struct_meta::{
    EnumStructMeta, EnumStructMetaDyn, EnumStructMetaStatic, EnumStructVariantMeta,
    EnumStructVariantMetaDyn, EnumStructVariantMetaStatic,
};
pub use enum_struct_shape::{EnumStructShape, EnumStructVariantShape};
pub use enum_struct_static::{EnumStructShapeStatic, EnumStructVariantStatic};
pub use enum_struct_value::EnumStructValue;
pub use enum_struct_value_dyn::EnumStructValueDyn;
