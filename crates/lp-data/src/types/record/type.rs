//! Struct/record metadata and UI hints.

/// Metadata for a struct/record type.
#[derive(Debug, Clone, PartialEq)]
pub struct RecordType<T: 'static> {
    pub name: &'static str,
    pub fields: &'static [RecordField<T>],
    pub ui: RecordUi,
}

impl<T: 'static> RecordType<T> {
    pub const fn new(name: &'static str, fields: &'static [RecordField<T>]) -> Self {
        Self {
            name,
            fields,
            ui: RecordUi { collapsible: false },
        }
    }

    pub const fn with_ui(mut self, ui: RecordUi) -> Self {
        self.ui = ui;
        self
    }
}

/// Metadata for an individual record field.
#[derive(Debug, Clone, PartialEq)]
pub struct RecordField<T> {
    pub name: &'static str,
    pub ty: T,
    pub docs: Option<&'static str>,
}

impl<T> RecordField<T> {
    pub const fn new(name: &'static str, ty: T) -> Self {
        Self {
            name,
            ty,
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
}

impl Default for RecordUi {
    fn default() -> Self {
        Self { collapsible: false }
    }
}
