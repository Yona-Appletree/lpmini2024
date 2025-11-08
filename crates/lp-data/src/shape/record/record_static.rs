//! Static record shape implementation.

use crate::shape::kind::LpKind;
use crate::shape::record::record_meta::RecordField;
use crate::shape::record::RecordShape;
use crate::shape::shape::LpShape;

/// Static record shape (compile-time known structure).
pub struct StaticRecordShape {
    pub name: &'static str,
    pub fields: &'static [RecordField],
    pub ui: crate::shape::record::RecordUi,
}

impl LpShape for StaticRecordShape {
    fn kind(&self) -> LpKind {
        LpKind::Record
    }
}

impl RecordShape for StaticRecordShape {
    fn name(&self) -> &str {
        self.name
    }

    fn fields(&self) -> &[RecordField] {
        self.fields
    }
}

impl core::fmt::Debug for StaticRecordShape {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("StaticRecordShape")
            .field("name", &self.name)
            .field("fields", &self.fields)
            .field("ui", &self.ui)
            .finish()
    }
}
