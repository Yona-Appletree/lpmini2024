//! 32-bit integer metadata and UI hints.

use crate::types::fixed::NumberUi;

/// Metadata for 32-bit integers.
#[derive(Debug, Clone, PartialEq)]
pub struct Int32Scalar {
    pub ui: NumberUi,
}

impl Int32Scalar {
    pub const fn with_ui(mut self, ui: NumberUi) -> Self {
        self.ui = ui;
        self
    }
}
