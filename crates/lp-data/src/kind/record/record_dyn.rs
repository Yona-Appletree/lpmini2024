//! Dynamic shape implementation for Record.

use super::record_meta::{RecordFieldMeta, RecordFieldMetaDyn};
use super::record_shape::{RecordFieldShape, RecordShape};
use crate::kind::{kind::LpKind, shape::LpShape};
use lp_pool::LpVec;

/// Dynamic field in a record shape.
///
/// Allocated in lp-pool.
pub struct RecordFieldDyn {
    /// Field name.
    pub name: lp_pool::LpString,

    /// Shape of the field's value.
    pub shape: &'static dyn LpShape,

    /// Field metadata.
    pub meta: RecordFieldMetaDyn,
}

impl RecordFieldShape for RecordFieldDyn {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn shape(&self) -> &'static dyn LpShape {
        self.shape
    }

    fn meta(&self) -> &dyn RecordFieldMeta {
        &self.meta
    }
}

/// Dynamic record shape.
///
/// Allocated in lp-pool.
pub struct RecordShapeDyn {
    /// Name of this record type.
    pub name: lp_pool::LpString,

    /// Fields in this record.
    pub fields: LpVec<RecordFieldDyn>,
}

impl LpShape for RecordShapeDyn {
    fn kind(&self) -> LpKind {
        LpKind::Record
    }
}

impl RecordShape for RecordShapeDyn {
    fn field_count(&self) -> usize {
        self.fields.len()
    }

    fn get_field(&self, index: usize) -> Option<&dyn RecordFieldShape> {
        self.fields.get(index).map(|f| f as &dyn RecordFieldShape)
    }

    fn find_field(&self, name: &str) -> Option<&dyn RecordFieldShape> {
        self.fields
            .iter()
            .find(|f| f.name.as_str() == name)
            .map(|f| f as &dyn RecordFieldShape)
    }
}
