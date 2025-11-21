use lp_math::dec32::Dec32;

use crate::nodes::lfo::lfo_waveform::LfoWaveform;

fn default_max() -> Dec32 {
    Dec32::ONE
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
    pub min: Dec32,

    /// Maximum output value.
    #[serde(default = "default_max")]
    pub max: Dec32,
}

impl Default for LfoInput {
    fn default() -> Self {
        Self {
            period_ms: 1000,
            shape: LfoWaveform::Sine,
            min: Dec32::ZERO,
            max: default_max(),
        }
    }
}

impl LfoInput {
    pub fn new(period_ms: i32, min: Dec32, max: Dec32) -> Self {
        Self {
            period_ms,
            shape: LfoWaveform::Sine,
            min,
            max,
        }
    }
}
