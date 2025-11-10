use crate::memory::LpBoxDyn;

use crate::kind::value::{LpValueBox, LpValueRef, LpValueRefMut};
use crate::kind::{LpValue, RecordShape};
use crate::RuntimeError;

/// Trait for record values that have fields.
pub trait RecordValue: LpValue {
    fn shape(&self) -> &dyn RecordShape;

    /// Get a field value by index.
    ///
    /// Returns the value at the given index. To get the field name, use `shape().get_field(index)`.
    fn get_field_by_index(&self, index: usize) -> Result<LpValueRef<'_>, RuntimeError>;

    /// Get a field value by index (mutable).
    ///
    /// Returns the value at the given index. To get the field name, use `shape().get_field(index)`.
    fn get_field_by_index_mut(&mut self, index: usize) -> Result<LpValueRefMut<'_>, RuntimeError>;

    /// Find the index of a field by name.
    ///
    /// Returns the index if found, or an error if the field doesn't exist.
    fn get_field_index(&self, name: &str) -> Result<usize, RuntimeError> {
        let shape = RecordValue::shape(self);
        for i in 0..shape.field_count() {
            if let Some(field_shape) = shape.get_field(i) {
                if field_shape.name() == name {
                    return Ok(i);
                }
            }
        }
        Err(RuntimeError::field_not_found(shape.meta().name(), name))
    }

    /// Get a field by name (immutable).
    ///
    /// Convenience method that uses `get_field_index` and `get_field_by_index`.
    fn get_field(&self, name: &str) -> Result<LpValueRef<'_>, RuntimeError> {
        let index = self.get_field_index(name)?;
        self.get_field_by_index(index)
    }

    /// Get a field by name (mutable).
    ///
    /// Convenience method that uses `get_field_index` and `get_field_by_index_mut`.
    fn get_field_mut(&mut self, name: &str) -> Result<LpValueRefMut<'_>, RuntimeError> {
        let index = self.get_field_index(name)?;
        self.get_field_by_index_mut(index)
    }
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
            LpValueRef::Int32(_) => None,
            LpValueRef::Bool(_) => None,
            LpValueRef::Vec2(_) => None,
            LpValueRef::Vec3(_) => None,
            LpValueRef::Vec4(_) => None,
            LpValueRef::Record(v) => Some(*v),
            LpValueRef::Enum(_) => None,
        }
    }
}

impl<'a> LpValueRefMut<'a> {
    /// Try to get a mutable reference to the value as RecordValue.
    pub fn as_record_mut(&mut self) -> Option<&mut dyn RecordValue> {
        match self {
            LpValueRefMut::Fixed(_) => None,
            LpValueRefMut::Int32(_) => None,
            LpValueRefMut::Bool(_) => None,
            LpValueRefMut::Vec2(_) => None,
            LpValueRefMut::Vec3(_) => None,
            LpValueRefMut::Vec4(_) => None,
            LpValueRefMut::Record(v) => Some(*v),
            LpValueRefMut::Enum(_) => None,
        }
    }
}
