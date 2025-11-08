#![cfg_attr(not(feature = "std"), no_std)]
//! Shared data model for `lpmini` runtime types.
//!
//! Combine `#[derive(LpSchema)]` with per-field attributes to describe runtime
//! data structures in a UI-friendly way. The derive generates rich types that
//! downstream tooling can consume to build forms, validators, and schema exports.

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod metadata;
pub mod registry;
pub mod registry_old;
pub mod shape;
pub mod types;
pub mod value;

// Re-export metadata (backwards compatibility - will be removed)
pub use metadata::{
    ArrayType, ArrayUi, BoolScalar, BoolUi, EnumType, EnumUi, EnumVariant, FixedScalar,
    Int32Scalar, LpType, LpTypeMeta, NumberUi, OptionType, RecordField, RecordType, RecordUi,
    SliderUi, StringScalar, StringUi, TypeRef, Vec2Type, Vec2Ui, Vec3Type, Vec3Ui, Vec4Type,
    Vec4Ui,
};

// Re-export MapType
pub use types::MapType;

// Re-export old registry (backwards compatibility - will be removed)
pub use registry_old::{LpDescribe, SchemaRegistration, TypeRegistry};

// Re-export new registries
pub use registry::{RuntimeRegistry, StaticRegistry};

// Re-export values
pub use value::{LpValue, RuntimeError};

// Re-export scalar value functions
pub use bool_value::{as_bool, bool};
pub use fixed_value::{as_fixed, fixed};
pub use int32_value::{as_int32, int32};
pub use string_value::{as_string, string};
pub use types::{
    bool as bool_value, fixed as fixed_value, int32 as int32_value, string as string_value,
};

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
