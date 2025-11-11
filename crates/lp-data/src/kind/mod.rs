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

pub mod bool;
pub mod enum_;
pub mod fixed;
pub mod int32;
pub mod record;
pub mod vec2;
pub mod vec3;
pub mod vec4;

// Re-export commonly used items
// Re-export enum traits and metadata from enum module
pub use enum_::{EnumMeta, EnumShape, EnumValue, EnumVariantMeta, EnumVariantShape};
pub use kind::LpKind;
// Re-export record traits and metadata from record module
pub use record::record_value::RecordValue;
pub use record::{RecordFieldMeta, RecordFieldShape, RecordShape};
pub use shape::LpShape;
pub use value::LpValue;
