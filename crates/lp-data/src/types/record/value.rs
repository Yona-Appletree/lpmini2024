//! Static struct value handling.

#[cfg(feature = "alloc")]
use alloc::string::String;

use lp_pool::collections::vec::LpVec;
use lp_pool::error::AllocError;

use crate::metadata::{LpType, TypeRef};
use crate::value::{LpValue, RuntimeError};

/// Static struct value storage (from Rust structs).
pub struct StructValue {
    pub struct_type: TypeRef,
    pub fields: LpVec<LpValue>,
}

impl StructValue {
    /// Create a new struct value from RecordType metadata.
    ///
    /// Initializes all fields to default values based on their types.
    pub fn try_new(struct_type: TypeRef) -> Result<Self, AllocError> {
        let record_ty = match &struct_type.ty {
            LpType::Record(rt) => rt,
            _ => return Err(AllocError::InvalidLayout), // Wrong type
        };

        let mut fields = LpVec::new();
        for field in record_ty.fields.iter() {
            // For nested records, use try_struct; for other types, use try_new
            let field_value = match &field.ty.ty {
                crate::metadata::LpType::Record(_) => LpValue::try_struct(field.ty)?,
                _ => LpValue::try_new(field.ty)?,
            };
            fields.try_push(field_value)?;
        }

        Ok(Self {
            struct_type,
            fields,
        })
    }

    /// Get a field by name.
    pub fn get_field(&self, name: &str) -> Result<&LpValue, RuntimeError> {
        let record_ty = match &self.struct_type.ty {
            LpType::Record(rt) => rt,
            _ => return Err(RuntimeError::NotARecord),
        };

        let field_index = record_ty
            .fields
            .iter()
            .position(|field| field.name == name)
            .ok_or_else(|| {
                // Find the static field name from metadata
                let field_name = record_ty
                    .fields
                    .iter()
                    .find(|f| f.name == name)
                    .map(|f| f.name)
                    .unwrap_or("");
                RuntimeError::FieldNotFound {
                    record_name: record_ty.name,
                    field_name,
                }
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
        let record_ty = match &self.struct_type.ty {
            LpType::Record(rt) => rt,
            _ => return Err(RuntimeError::NotARecord),
        };

        let field_index = record_ty
            .fields
            .iter()
            .position(|field| field.name == name)
            .ok_or_else(|| {
                // Find the static field name from metadata
                let field_name = record_ty
                    .fields
                    .iter()
                    .find(|f| f.name == name)
                    .map(|f| f.name)
                    .unwrap_or("");
                RuntimeError::FieldNotFound {
                    record_name: record_ty.name,
                    field_name,
                }
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
