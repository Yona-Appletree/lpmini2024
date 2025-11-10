//! LFO (Low Frequency Oscillator) node.

use lp_math::fixed::Fixed;

/// LFO waveform shape enumeration
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    lp_data_derive::EnumValue,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum LfoWaveform {
    Sine,
    Square,
    Triangle,
    Sawtooth,
}

/// Configuration for an LFO node.
#[derive(
    Debug, Clone, PartialEq, lp_data_derive::RecordValue, serde::Serialize, serde::Deserialize,
)]
pub struct LfoConfig {
    /// Oscillation period in seconds.
    pub period: Fixed,

    /// Waveform shape
    #[lp(enum)]
    pub waveform: LfoWaveform,
}

/// Runtime structure for an LFO node.
#[derive(Clone, lp_data_derive::RecordValue, serde::Serialize, serde::Deserialize)]
pub struct LfoNode {
    /// LFO configuration
    pub config: LfoConfig,
    /// LFO output value
    pub output: Fixed,
}

impl LfoNode {
    pub fn new(config: LfoConfig) -> Self {
        Self {
            config,
            output: Fixed::ZERO,
        }
    }
}
