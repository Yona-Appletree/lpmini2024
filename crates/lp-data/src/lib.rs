#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
//! Shared data model for `lpmini` runtime types.
//!
//! Combine `#[derive(LpSchema)]` with per-field attributes to describe runtime
//! data structures in a UI-friendly way. The derive generates rich types that
//! downstream tooling can consume to build forms, validators, and schema exports.

#[cfg(feature = "alloc")]
extern crate alloc;

// pub mod registry; // TODO: Implement registry module
pub mod kind;
pub mod memory;
pub mod value;

// Re-export new registries
// pub use registry::{RuntimeRegistry, StaticRegistry};
#[cfg(feature = "derive")]
pub use lp_data_derive::LpSchema;
pub use value::RuntimeError;

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
