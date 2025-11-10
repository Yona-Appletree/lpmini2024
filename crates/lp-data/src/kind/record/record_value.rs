use crate::kind::value::{LpValueBox, LpValueRef, LpValueRefMut};
use crate::kind::{LpValue, RecordShape};
use crate::RuntimeError;
use lp_pool::LpBoxDyn;

/// Trait for record values that have fields.
pub trait RecordValue: LpValue {
    fn shape(&self) -> &dyn RecordShape;

    /// Get a field by name (immutable).
    fn get_field(&self, name: &str) -> Result<LpValueRef<'_>, RuntimeError>;

    /// Get a field by name (mutable).
    fn get_field_mut(&mut self, name: &str) -> Result<LpValueRefMut<'_>, RuntimeError>;

    /// Set a field value.
    ///
    /// The exact parameter type is still being determined (see DESIGN.md).
    fn set_field(&mut self, name: &str, value: &dyn LpValue) -> Result<(), RuntimeError>;

    /// Get the number of fields in this record.
    fn field_count(&self) -> usize;

    /// Get a field by index, returning both the field name and value.
    ///
    /// Returns `(field_name, field_value)` for the field at the given index.
    /// This allows iteration over fields without cloning.
    fn get_field_by_index(&self, index: usize) -> Result<(&str, LpValueRef<'_>), RuntimeError>;
}

impl From<LpBoxDyn<dyn RecordValue>> for LpValueBox {
    fn from(value: LpBoxDyn<dyn RecordValue>) -> Self {
        LpValueBox::Record(value)
    }
}

impl<'a> LpValueRef<'a> {
    /// Try to get a reference to the value as RecordValue.
    pub fn as_record(&self) -> Option<&'a dyn RecordValue> {
        match self {
            LpValueRef::Fixed(_) => None,
            LpValueRef::Record(v) => Some(*v),
        }
    }
}

impl<'a> LpValueRefMut<'a> {
    /// Try to get a mutable reference to the value as RecordValue.
    pub fn as_record_mut(&mut self) -> Option<&mut dyn RecordValue> {
        match self {
            LpValueRefMut::Fixed(_) => None,
            LpValueRefMut::Record(v) => Some(*v),
        }
    }
}
