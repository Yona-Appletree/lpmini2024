//! Vec2 metadata and UI hints.

/// Metadata for a 2D vector value.
#[derive(Debug, Clone, PartialEq)]
pub struct Vec2Type {
    pub ui: Vec2Ui,
}

impl Vec2Type {
    pub const fn new(ui: Vec2Ui) -> Self {
        Self { ui }
    }

    pub const fn raw() -> Self {
        Self::new(Vec2Ui::Raw)
    }

    pub const fn position() -> Self {
        Self::new(Vec2Ui::Position)
    }
}

/// UI hints for vec2 fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vec2Ui {
    Raw,
    Position,
}
