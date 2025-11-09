//! Boolean metadata and UI hints.

/// UI hints for booleans.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoolUi {
    Checkbox,
    Toggle,
}

impl Default for BoolUi {
    fn default() -> Self {
        Self::Checkbox
    }
}
