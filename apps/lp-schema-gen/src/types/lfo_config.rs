use lp_math::dec32::Dec32;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
/// LFO waveform shape enumeration
#[derive(Default, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum LfoWaveformShape {
    #[default]
    Sine,
    Square,
    Triangle,
    Sawtooth,
}

/// Range type for LFO (min, max as Dec32)
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LfoRange {
    /// Minimum value
    pub min: Dec32,

    /// Maximum value
    pub max: Dec32,
}

impl Default for LfoRange {
    fn default() -> Self {
        LfoRange {
            min: Dec32::from_f32(0.0),
            max: Dec32::from_f32(1.0),
        }
    }
}

/// Configuration for an LFO (Low Frequency Oscillator)
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LfoConfig {
    /// Period of oscillation in milliseconds
    pub period_ms: i32,

    /// Range of oscillation
    pub range: LfoRange,

    /// Waveform shape
    pub shape: LfoWaveformShape,
}

impl Default for LfoConfig {
    fn default() -> Self {
        LfoConfig {
            period_ms: 1000,
            range: LfoRange::default(),
            shape: LfoWaveformShape::Sine,
        }
    }
}

// Type aliases for backward compatibility
#[allow(dead_code)]
pub type LfoShape = LfoWaveformShape;

// Manual implementations until LpSchema derive is dec32
// These types need to implement LpValue first - for now this is a placeholder
// TODO: Implement LpValue for these types or fix LpSchema derive
