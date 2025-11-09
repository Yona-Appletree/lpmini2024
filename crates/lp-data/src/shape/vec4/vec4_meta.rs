//! Vec4 metadata and UI hints.

/// UI hints for 4D vectors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vec4Ui {
    Raw,
    Color,
    Position,
}

impl Default for Vec4Ui {
    fn default() -> Self {
        Self::Raw
    }
}
