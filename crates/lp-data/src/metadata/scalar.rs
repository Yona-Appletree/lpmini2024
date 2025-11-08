//! Scalar type metadata definitions and UI hints.
//!
//! This module models the primitive scalar types that `lp-data` supports today
//! along with the UI affordances that should be rendered for each flavor.

/// Scalar types supported by the schema system.
#[derive(Debug, Clone, PartialEq)]
pub enum LpScalarType {
    String(StringScalar),
    Fixed(FixedScalar),
    Int32(Int32Scalar),
    Bool(BoolScalar),
}

impl LpScalarType {
    /// Default metadata for a string scalar.
    pub const fn string() -> Self {
        Self::String(StringScalar {
            ui: StringUi::SingleLine,
        })
    }

    /// Default metadata for a fixed-point number.
    pub const fn fixed() -> Self {
        Self::Fixed(FixedScalar {
            ui: NumberUi::Textbox,
        })
    }

    /// Default metadata for a 32-bit integer.
    pub const fn int32() -> Self {
        Self::Int32(Int32Scalar {
            ui: NumberUi::Textbox,
        })
    }

    /// Default metadata for a boolean.
    pub const fn boolean() -> Self {
        Self::Bool(BoolScalar {
            ui: BoolUi::Checkbox,
        })
    }
}

/// Metadata for string values.
#[derive(Debug, Clone, PartialEq)]
pub struct StringScalar {
    pub ui: StringUi,
}

impl StringScalar {
    pub const fn with_ui(mut self, ui: StringUi) -> Self {
        self.ui = ui;
        self
    }
}

/// UI hints for string scalars.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringUi {
    SingleLine,
    MultiLine,
}

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

/// Metadata for 32-bit integers.
#[derive(Debug, Clone, PartialEq)]
pub struct Int32Scalar {
    pub ui: NumberUi,
}

impl Int32Scalar {
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

/// Metadata for booleans.
#[derive(Debug, Clone, PartialEq)]
pub struct BoolScalar {
    pub ui: BoolUi,
}

impl BoolScalar {
    pub const fn with_ui(mut self, ui: BoolUi) -> Self {
        self.ui = ui;
        self
    }
}

/// UI hints for boolean inputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoolUi {
    Checkbox,
    Toggle,
}
