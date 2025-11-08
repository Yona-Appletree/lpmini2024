//! Vec4 metadata and UI hints.

/// Metadata for a 4D vector value.
#[derive(Debug, Clone, PartialEq)]
pub struct Vec4Type {
    pub ui: Vec4Ui,
}

impl Vec4Type {
    pub const fn new(ui: Vec4Ui) -> Self {
        Self { ui }
    }

    pub const fn raw() -> Self {
        Self::new(Vec4Ui::Raw)
    }
}

/// UI hints for vec4 fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vec4Ui {
    Raw,
}
