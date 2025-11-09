//! Schema types for Record shapes.
//!
//! Note: Metadata types are in `record_meta.rs`.

use super::record_meta::RecordFieldMeta;
use crate::kind::shape::LpShape;

/// Trait for record shapes that have fields.
pub trait RecordShape: LpShape {
    /// Get the name of this record type.
    fn name(&self) -> &str;

    /// Get the number of fields in this record.
    fn field_count(&self) -> usize;

    /// Get a field by index.
    fn get_field(&self, index: usize) -> Option<&dyn RecordFieldShape>;

    /// Find a field by name.
    fn find_field(&self, name: &str) -> Option<&dyn RecordFieldShape>;
}

/// Trait for record field shapes.
pub trait RecordFieldShape {
    /// Get the name of this field.
    fn name(&self) -> &str;

    /// Get the shape of this field's value.
    fn shape(&self) -> &'static dyn LpShape;

    /// Get the metadata for this field.
    fn meta(&self) -> &dyn RecordFieldMeta;
}
