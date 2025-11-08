//! Dynamic record shape implementation.

use lp_pool::collections::{LpString, LpVec};
use lp_pool::error::AllocError;

use crate::shape::kind::LpKind;
use crate::shape::record::record_meta::RecordField;
use crate::shape::shape::{LpShape, RecordShape};

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
        &self.fields
    }
}

impl core::fmt::Debug for DynamicRecordShape {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DynamicRecordShape")
            .field("name", self.name.as_str())
            .field("fields", &self.fields)
            .field("ui", &self.ui)
            .finish()
    }
}
