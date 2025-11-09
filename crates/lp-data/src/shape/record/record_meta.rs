//! Record metadata and UI hints.

use crate::shape::shape_ref::ShapeRef;

/// Metadata for an individual record field.
#[derive(Debug)]
pub struct RecordField {
    pub name: &'static str,
    pub shape: &'static ShapeRef,
    pub docs: Option<&'static str>,
}

impl RecordField {
    pub const fn new(name: &'static str, shape: &'static ShapeRef) -> Self {
        Self {
            name,
            shape,
            docs: None,
        }
    }

    pub const fn with_docs(mut self, docs: &'static str) -> Self {
        self.docs = Some(docs);
        self
    }
}

/// UI hints for struct layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordUi {
    pub collapsible: bool,
}

impl RecordUi {
    pub const fn collapsible() -> Self {
        Self { collapsible: true }
    }

    pub const fn default() -> Self {
        Self { collapsible: false }
    }
}

impl Default for RecordUi {
    fn default() -> Self {
        Self { collapsible: false }
    }
}
