//! Static struct value handling.

#[cfg(feature = "alloc")]
use alloc::string::String;

use lp_pool::collections::vec::LpVec;
use lp_pool::error::AllocError;

use crate::shape::shape::RecordShape;
use crate::shape::shape_ref::{RecordShapeRef, ShapeRef};
use crate::value::{LpValue, RuntimeError};

/// Static struct value storage (from Rust structs).
pub struct StructValue {
    pub shape: ShapeRef,
    pub fields: LpVec<LpValue>,
}

impl StructValue {
    /// Create a new struct value from a ShapeRef.
    ///
    /// Initializes all fields to default values based on their shapes.
    pub fn try_new(shape: ShapeRef) -> Result<Self, AllocError> {
        let record_shape = match &shape {
            ShapeRef::Record(RecordShapeRef::Static(rs)) => rs,
            _ => return Err(AllocError::InvalidLayout),
        };

        let mut fields = LpVec::new();
        for field in record_shape.fields() {
            // Create default value for each field based on its shape
            let field_value = LpValue::try_new_from_shape(&field.shape)?;
            fields.try_push(field_value)?;
        }

        Ok(Self { shape, fields })
    }

    /// Get a field by name.
    pub fn get_field(&self, name: &str) -> Result<&LpValue, RuntimeError> {
        let record_shape = match &self.shape {
            ShapeRef::Record(RecordShapeRef::Static(rs)) => rs,
            _ => return Err(RuntimeError::NotARecord),
        };

        let field_index = record_shape
            .fields()
            .iter()
            .position(|field| field.name == name)
            .ok_or_else(|| RuntimeError::FieldNotFound {
                record_name: record_shape.name(),
                field_name: name,
            })?;

        self.fields
            .get(field_index)
            .ok_or_else(|| RuntimeError::IndexOutOfBounds {
                array_len: self.fields.len(),
                index: field_index,
            })
    }

    /// Get a mutable field by name.
    pub fn get_field_mut(&mut self, name: &str) -> Result<&mut LpValue, RuntimeError> {
        let record_shape = match &self.shape {
            ShapeRef::Record(RecordShapeRef::Static(rs)) => rs,
            _ => return Err(RuntimeError::NotARecord),
        };

        let field_index = record_shape
            .fields()
            .iter()
            .position(|field| field.name == name)
            .ok_or_else(|| RuntimeError::FieldNotFound {
                record_name: record_shape.name(),
                field_name: name,
            })?;

        let len = self.fields.len();
        self.fields
            .get_mut(field_index)
            .ok_or_else(|| RuntimeError::IndexOutOfBounds {
                array_len: len,
                index: field_index,
            })
    }

    /// Set a field value.
    pub fn try_set_field(&mut self, name: &str, value: LpValue) -> Result<(), RuntimeError> {
        let field = self.get_field_mut(name)?;
        *field = value;
        Ok(())
    }
}
