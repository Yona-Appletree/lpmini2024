//! Boolean metadata and UI hints.

/// Metadata for booleans.
#[derive(Debug, Clone, PartialEq)]
pub struct BoolScalar {
    pub ui: BoolUi,
}

impl BoolScalar {
    pub const fn with_ui(mut self, ui: BoolUi) -> Self {
        self.ui = ui;
        self
    }
}

/// UI hints for boolean inputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoolUi {
    Checkbox,
    Toggle,
}
