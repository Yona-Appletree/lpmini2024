//! Type system for lp-data.
//!
//! This module provides the foundation for the introspectable data system,
//! separating type kinds, shapes (metadata), and values (runtime data).

#[allow(clippy::module_inception)]
pub mod kind;
pub mod shape;
pub mod value;

#[macro_use]
mod primitives;

pub mod array;
pub mod bool;
pub mod enum_struct;
pub mod enum_unit;
pub mod fixed;
pub mod int32;
pub mod option;
pub mod record;
pub mod vec2;
pub mod vec3;
pub mod vec4;

// Re-export commonly used items
// Re-export array traits and metadata from array module
pub use array::{ArrayMeta, ArrayMetaDyn, ArrayMetaStatic, ArrayShape, ArrayValue};
// Re-export enum traits and metadata from enum module
pub use enum_struct::{
    EnumStructMeta, EnumStructMetaDyn, EnumStructMetaStatic, EnumStructShape, EnumStructShapeDyn,
    EnumStructShapeStatic, EnumStructValue, EnumStructValueDyn, EnumStructVariantDyn,
    EnumStructVariantMeta, EnumStructVariantMetaDyn, EnumStructVariantMetaStatic,
    EnumStructVariantShape, EnumStructVariantStatic,
};
pub use enum_unit::{
    EnumUnitMeta, EnumUnitShape, EnumUnitValue, EnumUnitVariantMeta, EnumUnitVariantShape,
};
pub use kind::LpKind;
// Re-export option traits and metadata from option module
pub use option::{
    OptionMeta, OptionMetaDyn, OptionMetaStatic, OptionShape, OptionShapeDyn, OptionShapeStatic,
    OptionValue, OptionValueDyn,
};
// Re-export record traits and metadata from record module
pub use record::record_value::RecordValue;
pub use record::{RecordFieldMeta, RecordFieldShape, RecordShape};
// Re-export union traits and metadata from union module
pub use shape::LpShape;
pub use value::LpValue;
