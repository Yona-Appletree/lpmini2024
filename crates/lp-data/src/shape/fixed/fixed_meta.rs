//! Fixed-point metadata and UI hints.

/// UI hints for fixed-point numbers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixedUi {
    Textbox,
    Slider { min: i32, max: i32 },
}

impl Default for FixedUi {
    fn default() -> Self {
        Self::Textbox
    }
}
