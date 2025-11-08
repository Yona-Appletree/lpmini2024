//! Vec2 metadata and UI hints.

/// Metadata for a 2D vector value.
#[derive(Debug, Clone, PartialEq)]
pub struct Vec2Type {
    pub ui: Vec2Ui,
}

impl Vec2Type {
    pub const fn raw() -> Self {
        Self { ui: Vec2Ui::Raw }
    }

    pub const fn position() -> Self {
        Self {
            ui: Vec2Ui::Position,
        }
    }

    pub const fn color() -> Self {
        Self { ui: Vec2Ui::Color }
    }
}

/// UI hints for Vec2.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vec2Ui {
    Raw,
    Position,
    Color,
}
