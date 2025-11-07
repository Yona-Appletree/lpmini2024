#![cfg_attr(not(feature = "std"), no_std)]
//! Shared data model for `lpmini` runtime metadata.
//!
//! ## Engine-core integration
//! The circular mapping helpers in `engine-core/src/test_engine/mapping/circular.rs`
//! expect a configuration struct describing ring counts, directions, and geometry.
//! This crate's `LpType` and `Annotations` APIs are intended to express that
//! configuration metadata so both the compiler and UI layers can consume a single
//! source of truth.
//!
//! ```ignore
//! use lp_data::ty::{LpField, LpStructType, LpType};
//!
//! let mut struct_ty = LpStructType::new("CircleMappingConfig");
//! struct_ty.add_field(LpField::new("ring_counts", LpType::array(LpType::int32())));
//! struct_ty.add_field(LpField::new("radius", LpType::fixed32()));
//! let config_type = LpType::structure(struct_ty);
//! ```
//!
//! Once engine-core is ready to adopt `LpData`, the concrete config struct can use
//! the builder pattern above to register its schema.

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod annotation;
pub mod registry;
pub mod schema;
pub mod ty;
pub mod value;

pub use registry::{LpDataType, TypeRegistry};

#[cfg(test)]
mod tests;

