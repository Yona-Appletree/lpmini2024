//! Vec4 metadata and UI hints.

/// Metadata for a 4D vector value.
#[derive(Debug, Clone, PartialEq)]
pub struct Vec4Type {
    pub ui: Vec4Ui,
}

impl Vec4Type {
    pub const fn raw() -> Self {
        Self { ui: Vec4Ui::Raw }
    }

    pub const fn color() -> Self {
        Self { ui: Vec4Ui::Color }
    }
}

/// UI hints for Vec4.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vec4Ui {
    Raw,
    Color,
}
