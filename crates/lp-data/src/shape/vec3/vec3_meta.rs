//! Vec3 metadata and UI hints.

/// UI hints for 3D vectors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vec3Ui {
    Raw,
    Color,
    Position,
}

impl Default for Vec3Ui {
    fn default() -> Self {
        Self::Raw
    }
}
