//! Dynamic record shape implementation.

use lp_pool::collections::{LpString, LpVec};
use lp_pool::error::AllocError;
use lp_pool::LpBTreeMap;
use crate::LpValue;
use crate::shape::kind::LpKind;
use crate::shape::record::record_meta::RecordField;
use crate::shape::record::{RecordShape, RecordValue};
use crate::shape::shape::LpShape;

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
    pub shape: DynamicRecordShape,
    pub fields: LpBTreeMap<LpString, LpValue>
}

impl RecordValue for RecordValueDyn {
    // todo: implement
}