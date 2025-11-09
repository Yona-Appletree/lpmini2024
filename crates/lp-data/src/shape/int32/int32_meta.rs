//! Int32 metadata and UI hints.

/// UI hints for 32-bit integers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Int32Ui {
    Textbox,
    Slider { min: i32, max: i32 },
}

impl Default for Int32Ui {
    fn default() -> Self {
        Self::Textbox
    }
}
