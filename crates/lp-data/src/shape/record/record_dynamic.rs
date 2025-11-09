//! Dynamic record shape implementation.

use crate::shape::kind::LpKind;
use crate::shape::record::record_meta::RecordField;
use crate::shape::record::{RecordShape, RecordValue};
use crate::shape::shape::LpShape;
use crate::LpValue;
use lp_pool::collections::{LpString, LpVec};
use lp_pool::error::AllocError;
use lp_pool::LpBTreeMap;

/// Dynamic record shape (runtime-created structure).
pub struct DynamicRecordShape {
    pub name: LpString,
    pub fields: LpVec<RecordField>,
    pub ui: crate::shape::record::RecordUi,
}

impl DynamicRecordShape {
    /// Create a new dynamic record shape.
    pub fn try_new(name: &str, fields: LpVec<RecordField>) -> Result<Self, AllocError> {
        let name_str = LpString::try_from_str(name)?;
        Ok(Self {
            name: name_str,
            fields,
            ui: crate::shape::record::RecordUi::default(),
        })
    }
}

impl LpShape for DynamicRecordShape {
    fn kind(&self) -> LpKind {
        LpKind::Record
    }
}

impl RecordShape for DynamicRecordShape {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn fields(&self) -> &[RecordField] {
        self.fields.as_slice()
    }
}

impl core::fmt::Debug for DynamicRecordShape {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Can't format LpVec directly, so we show the length
        // Can't use as_str() directly in Debug, so we format it
        f.debug_struct("DynamicRecordShape")
            .field("name", &self.name)
            .field("fields_len", &self.fields.len())
            .field("ui", &self.ui)
            .finish()
    }
}

pub struct RecordValueDyn {
    pub shape: crate::shape::shape_ref::ShapeRef,
    pub fields: LpBTreeMap<LpString, LpValue>,
}

impl crate::shape::value::LpValueTrait for RecordValueDyn {
    fn shape(&self) -> &crate::shape::shape_ref::ShapeRef {
        &self.shape
    }

    fn kind(&self) -> crate::shape::kind::LpKind {
        crate::shape::kind::LpKind::Record
    }
}

impl RecordValue for RecordValueDyn {
    fn get_field(
        &self,
        name: &str,
    ) -> Result<&dyn crate::shape::value::LpValueTrait, crate::value::RuntimeError> {
        let name_str = LpString::try_from_str(name)
            .map_err(|_| crate::value::RuntimeError::AllocError(AllocError::OutOfMemory))?;
        let value = self.fields.get(&name_str).ok_or_else(|| {
            crate::value::RuntimeError::FieldNotFound {
                record_name: "dynamic",
                field_name: name,
            }
        })?;
        // TODO: Convert &LpValue to &dyn LpValueTrait
        // This requires LpValue to implement LpValueTrait
        // For now, return error
        Err(crate::value::RuntimeError::NotARecord)
    }

    fn get_field_mut(
        &mut self,
        name: &str,
    ) -> Result<&mut dyn crate::shape::value::LpValueTrait, crate::value::RuntimeError> {
        let name_str = LpString::try_from_str(name)
            .map_err(|_| crate::value::RuntimeError::AllocError(AllocError::OutOfMemory))?;
        let _value = self.fields.get_mut(&name_str).ok_or_else(|| {
            crate::value::RuntimeError::FieldNotFound {
                record_name: "dynamic",
                field_name: name,
            }
        })?;
        // TODO: Convert &mut LpValue to &mut dyn LpValueTrait
        Err(crate::value::RuntimeError::NotARecord)
    }

    fn set_field(&mut self, name: &str, value: LpValue) -> Result<(), crate::value::RuntimeError> {
        let name_str = LpString::try_from_str(name)
            .map_err(|_| crate::value::RuntimeError::AllocError(AllocError::OutOfMemory))?;
        self.fields
            .try_insert(name_str, value)
            .map_err(|_| crate::value::RuntimeError::AllocError(AllocError::OutOfMemory))?;
        Ok(())
    }
}
