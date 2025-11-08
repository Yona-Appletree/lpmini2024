use lp_data::LpSchema;
use lp_math::fixed::Fixed;
use serde::{Deserialize, Serialize};

/// LFO waveform shape enumeration
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, LpSchema)]
#[lp(schema(name = "LFO Waveform Shape",))]
pub enum LfoWaveformShape {
    Sine,
    Square,
    Triangle,
    Sawtooth,
}

/// Range type for LFO (min, max as Fixed)
#[derive(Serialize, Deserialize, Debug, Clone, LpSchema)]
#[lp(schema(name = "LFO Range",))]
pub struct LfoRange {
    /// Minimum value
    #[lp(field(ui(slider(min = 0.0, max = 1.0))))]
    pub min: Fixed,

    /// Maximum value
    #[lp(field(ui(slider(min = 0.0, max = 1.0))))]
    pub max: Fixed,
}

/// Configuration for an LFO (Low Frequency Oscillator)
#[derive(Serialize, Deserialize, Debug, Clone, LpSchema)]
#[lp(schema(name = "LFO Config",))]
pub struct LfoConfig {
    /// Period of oscillation in milliseconds
    #[lp(field(ui(slider(min = 10, max = 60000, step = 1))))]
    pub period_ms: i32,

    /// Range of oscillation
    pub range: LfoRange,

    /// Waveform shape
    pub shape: LfoWaveformShape,
}

// Type aliases for backward compatibility
pub type LfoShape = LfoWaveformShape;
