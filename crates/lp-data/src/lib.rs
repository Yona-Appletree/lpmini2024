#![cfg_attr(not(feature = "std"), no_std)]
//! Shared data model for `lpmini` runtime metadata.
//!
//! Combine `#[derive(LpSchema)]` with per-field attributes to describe runtime
//! data structures in a UI-friendly way. The derive generates rich metadata that
//! downstream tooling can consume to build forms, validators, and schema exports.

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod metadata;
pub mod registry;
pub mod value;
pub use metadata::{
    ArrayType, ArrayUi, BoolScalar, BoolUi, EnumType, EnumUi, EnumVariant, FixedScalar,
    Int32Scalar, LpScalarType, LpType, LpTypeMeta, NumberUi, OptionType, RecordField, RecordType,
    RecordUi, SliderUi, StringScalar, StringUi, TypeRef, Vec2Type, Vec2Ui, Vec3Type, Vec3Ui,
    Vec4Type, Vec4Ui,
};
pub use registry::{LpDescribe, SchemaRegistration, TypeRegistry};
pub use value::{LpValue, RuntimeError};

#[cfg(feature = "derive")]
pub use lp_data_derive::LpSchema;

#[cfg(feature = "serde")]
pub trait LpSerialize: serde::Serialize {}

#[cfg(feature = "serde")]
impl<T> LpSerialize for T where T: serde::Serialize {}

#[cfg(feature = "serde")]
pub trait LpDeserialize<'de>: serde::Deserialize<'de> {}

#[cfg(feature = "serde")]
impl<'de, T> LpDeserialize<'de> for T where T: serde::Deserialize<'de> {}

#[cfg(test)]
mod tests;
