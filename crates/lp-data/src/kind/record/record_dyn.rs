//! Dynamic shape implementation for Record.

extern crate alloc;

use alloc::{string::String, vec::Vec};

use super::record_meta::{RecordFieldMeta, RecordFieldMetaDyn};
use super::record_shape::{RecordFieldShape, RecordShape};
use crate::kind::{kind::LpKind, shape::LpShape};

/// Dynamic field in a record shape.
pub struct RecordFieldDyn {
    /// Field name.
    pub name: String,

    /// Shape of the field's value.
    pub shape: &'static dyn LpShape,

    /// Field metadata.
    pub meta: RecordFieldMetaDyn,
}

impl RecordFieldShape for RecordFieldDyn {
    fn name(&self) -> &str {
        &self.name
    }

    fn shape(&self) -> &'static dyn LpShape {
        self.shape
    }

    fn meta(&self) -> &dyn RecordFieldMeta {
        &self.meta
    }
}

/// Dynamic record shape.
pub struct RecordShapeDyn {
    /// Name of this record type.
    pub name: String,

    /// Fields in this record.
    pub fields: Vec<RecordFieldDyn>,
}

impl RecordShapeDyn {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            fields: Vec::new(),
        }
    }
}

impl LpShape for RecordShapeDyn {
    fn kind(&self) -> LpKind {
        LpKind::Record
    }
}

impl RecordShape for RecordShapeDyn {
    fn name(&self) -> &str {
        &self.name
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

impl Clone for RecordFieldDyn {
    fn clone(&self) -> Self {
        RecordFieldDyn {
            name: self.name.clone(),
            shape: self.shape,
            meta: RecordFieldMetaDyn {
                docs: self.meta.docs.clone(),
            },
        }
    }
}

impl Clone for RecordShapeDyn {
    fn clone(&self) -> Self {
        RecordShapeDyn {
            name: self.name.clone(),
            fields: self.fields.clone(),
        }
    }
}
