use lp_math::fixed::Fixed;

use crate::nodes::lfo::lfo_waveform::LfoWaveform;

fn default_max() -> Fixed {
    Fixed::ONE
}

/// Input parameters for LFO node.
#[derive(
    Debug, Clone, PartialEq, lp_data_derive::RecordValue, serde::Serialize, serde::Deserialize,
)]
pub struct LfoInput {
    /// Period of oscillation in milliseconds.
    pub period_ms: i32,

    /// Waveform shape.
    #[serde(default)]
    #[lp(enum_unit)]
    pub shape: crate::nodes::lfo::lfo_waveform::LfoWaveform,

    /// Minimum output value.
    pub min: Fixed,

    /// Maximum output value.
    #[serde(default = "default_max")]
    pub max: Fixed,
}

impl Default for LfoInput {
    fn default() -> Self {
        Self {
            period_ms: 1000,
            shape: LfoWaveform::Sine,
            min: Fixed::ZERO,
            max: default_max(),
        }
    }
}

impl LfoInput {
    pub fn new(period_ms: i32, min: Fixed, max: Fixed) -> Self {
        Self {
            period_ms,
            shape: LfoWaveform::Sine,
            min,
            max,
        }
    }
}
