use lp_math::fixed::Fixed;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// LFO waveform shape enumeration
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Copy, PartialEq, Eq)]
pub enum LfoWaveformShape {
    Sine,
    Square,
    Triangle,
    Sawtooth,
}

/// Range type for LFO (min, max as Fixed)
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub struct LfoRange {
    /// Minimum value
    pub min: Fixed,

    /// Maximum value
    pub max: Fixed,
}

/// Configuration for an LFO (Low Frequency Oscillator)
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub struct LfoConfig {
    /// Period of oscillation in milliseconds
    pub period_ms: i32,

    /// Range of oscillation
    pub range: LfoRange,

    /// Waveform shape
    pub shape: LfoWaveformShape,
}

// Type aliases for backward compatibility
pub type LfoShape = LfoWaveformShape;
