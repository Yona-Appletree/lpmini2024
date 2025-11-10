//! Static shape implementation for Record.

use super::record_meta::{RecordFieldMeta, RecordFieldMetaStatic, RecordMeta, RecordMetaStatic};
use super::record_shape::{RecordFieldShape, RecordShape};
use crate::kind::{kind::LpKind, shape::LpShape};

/// Static field in a record shape.
pub struct RecordFieldStatic {
    /// Field name.
    pub name: &'static str,

    /// Shape of the field's value.
    pub shape: &'static dyn LpShape,

    /// Field metadata.
    pub meta: RecordFieldMetaStatic,
}

impl RecordFieldShape for RecordFieldStatic {
    fn name(&self) -> &str {
        self.name
    }

    fn shape(&self) -> &'static dyn LpShape {
        self.shape
    }

    fn meta(&self) -> &dyn RecordFieldMeta {
        &self.meta
    }
}

/// Static record shape.
///
/// Uses `&'static` references for zero-cost storage.
pub struct RecordShapeStatic {
    /// Metadata for this record shape.
    pub meta: RecordMetaStatic,

    /// Fields in this record.
    pub fields: &'static [RecordFieldStatic],
}

impl LpShape for RecordShapeStatic {
    fn kind(&self) -> LpKind {
        LpKind::Record
    }
}

impl RecordShape for RecordShapeStatic {
    fn meta(&self) -> &dyn RecordMeta {
        &self.meta as &dyn RecordMeta
    }

    fn field_count(&self) -> usize {
        self.fields.len()
    }

    fn get_field(&self, index: usize) -> Option<&dyn RecordFieldShape> {
        self.fields.get(index).map(|f| f as &dyn RecordFieldShape)
    }

    fn find_field(&self, name: &str) -> Option<&dyn RecordFieldShape> {
        self.fields
            .iter()
            .find(|f| f.name == name)
            .map(|f| f as &dyn RecordFieldShape)
    }
}
