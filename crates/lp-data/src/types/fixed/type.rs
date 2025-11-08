//! Fixed-point number metadata and UI hints.

/// Metadata for fixed-point numbers.
#[derive(Debug, Clone, PartialEq)]
pub struct FixedScalar {
    pub ui: NumberUi,
}

impl FixedScalar {
    pub const fn with_ui(mut self, ui: NumberUi) -> Self {
        self.ui = ui;
        self
    }
}

/// UI hints for numeric scalars.
#[derive(Debug, Clone, PartialEq)]
pub enum NumberUi {
    Textbox,
    Slider(SliderUi),
}

/// Slider configuration for numeric inputs.
#[derive(Debug, Clone, PartialEq)]
pub struct SliderUi {
    pub min: f64,
    pub max: f64,
    pub step: Option<f64>,
}

impl SliderUi {
    pub const fn new(min: f64, max: f64) -> Self {
        Self {
            min,
            max,
            step: None,
        }
    }

    pub const fn with_step(mut self, step: f64) -> Self {
        self.step = Some(step);
        self
    }
}
