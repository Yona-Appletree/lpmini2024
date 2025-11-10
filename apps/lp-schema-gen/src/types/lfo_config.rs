use lp_math::fixed::Fixed;
use serde::{Deserialize, Serialize};

/// LFO waveform shape enumeration
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum LfoWaveformShape {
    Sine,
    Square,
    Triangle,
    Sawtooth,
}

impl Default for LfoWaveformShape {
    fn default() -> Self {
        LfoWaveformShape::Sine
    }
}

/// Range type for LFO (min, max as Fixed)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LfoRange {
    /// Minimum value
    pub min: Fixed,

    /// Maximum value
    pub max: Fixed,
}

impl Default for LfoRange {
    fn default() -> Self {
        LfoRange {
            min: Fixed::from_f32(0.0),
            max: Fixed::from_f32(1.0),
        }
    }
}

/// Configuration for an LFO (Low Frequency Oscillator)
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
pub type LfoShape = LfoWaveformShape;

// Manual implementations until LpSchema derive is fixed
// These types need to implement LpValue first - for now this is a placeholder
// TODO: Implement LpValue for these types or fix LpSchema derive
