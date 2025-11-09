//! Vec2 metadata and UI hints.

/// UI hints for 2D vectors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vec2Ui {
    Raw,
    Color,
    Position,
}

impl Default for Vec2Ui {
    fn default() -> Self {
        Self::Raw
    }
}
