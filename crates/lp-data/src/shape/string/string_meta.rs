//! String metadata and UI hints.

/// UI hints for strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringUi {
    SingleLine,
    MultiLine,
}

impl Default for StringUi {
    fn default() -> Self {
        Self::SingleLine
    }
}
