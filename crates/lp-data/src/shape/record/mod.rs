//! Record/struct shape types.

pub mod record_dynamic;
pub mod record_meta;
pub mod record_static;
pub mod record_value_static;

#[cfg(test)]
mod record_tests;
mod record_value;

use crate::shape::{LpShape, LpValueTrait};
use crate::LpValue;
pub use record_dynamic::DynamicRecordShape;
pub use record_meta::{RecordField, RecordUi};
pub use record_static::StaticRecordShape;
pub use record_value_static::StructValue;

/// Value operations for record/struct types.
pub trait RecordValue: LpValueTrait {
    /// Get a field by name.
    fn get_field(&self, name: &str) -> Result<&dyn LpValueTrait, crate::value::RuntimeError>;

    /// Get a mutable field by name.
    fn get_field_mut(
        &mut self,
        name: &str,
    ) -> Result<&mut dyn LpValueTrait, crate::value::RuntimeError>;

    /// Set a field value.
    fn set_field(&mut self, name: &str, value: LpValue) -> Result<(), crate::value::RuntimeError>;
}

/// Shape for record/struct types.
pub trait RecordShape: LpShape {
    /// Returns the name of the record type.
    fn name(&self) -> &str;

    /// Returns the fields of the record.
    fn fields(&self) -> &[crate::shape::record::RecordField];
}
