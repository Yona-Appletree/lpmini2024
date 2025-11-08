//! String metadata and UI hints.

/// Metadata for string values.
#[derive(Debug, Clone, PartialEq)]
pub struct StringScalar {
    pub ui: StringUi,
}

impl StringScalar {
    pub const fn with_ui(mut self, ui: StringUi) -> Self {
        self.ui = ui;
        self
    }
}

/// UI hints for string scalars.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringUi {
    SingleLine,
    MultiLine,
}
