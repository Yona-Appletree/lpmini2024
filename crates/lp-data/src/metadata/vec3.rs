//! Vec3 metadata and UI hints.

/// Metadata for a 3D vector value.
#[derive(Debug, Clone, PartialEq)]
pub struct Vec3Type {
    pub ui: Vec3Ui,
}

impl Vec3Type {
    pub const fn new(ui: Vec3Ui) -> Self {
        Self { ui }
    }

    pub const fn raw() -> Self {
        Self::new(Vec3Ui::Raw)
    }

    pub const fn color() -> Self {
        Self::new(Vec3Ui::Color)
    }
}

/// UI hints for vec3 fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vec3Ui {
    Raw,
    Color,
}
