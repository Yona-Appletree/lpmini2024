//! Array metadata and UI hints.

/// Metadata for array types.
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayType<T> {
    pub element: T,
    pub ui: ArrayUi,
}

impl<T> ArrayType<T> {
    pub const fn new(element: T) -> Self {
        Self {
            element,
            ui: ArrayUi::List,
        }
    }

    pub const fn with_ui(mut self, ui: ArrayUi) -> Self {
        self.ui = ui;
        self
    }
}

/// UI hints for arrays.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayUi {
    List,
}
