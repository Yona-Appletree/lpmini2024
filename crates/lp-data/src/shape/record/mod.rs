//! Record/struct shape types.

pub mod record_dynamic;
pub mod record_meta;
pub mod record_static;

#[cfg(test)]
mod record_tests;

pub use record_dynamic::DynamicRecordShape;
pub use record_meta::{RecordField, RecordUi};
pub use record_static::StaticRecordShape;
