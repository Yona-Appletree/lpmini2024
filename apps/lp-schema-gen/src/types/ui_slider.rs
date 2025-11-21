use lp_math::dec32::Dec32;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
/// Configuration for a UI slider control
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UiSliderConfig {
    /// Minimum value
    pub min: Dec32,

    /// Maximum value
    pub max: Dec32,

    /// Step size
    pub step: Dec32,

    /// Default value
    pub default: Dec32,
}

impl Default for UiSliderConfig {
    fn default() -> Self {
        UiSliderConfig {
            min: Dec32::from_f32(0.0),
            max: Dec32::from_f32(1.0),
            step: Dec32::from_f32(0.01),
            default: Dec32::from_f32(0.5),
        }
    }
}

// Manual implementation until LpSchema derive is dec32
// This type needs to implement LpValue first - for now this is a placeholder
// TODO: Implement LpValue for this type or fix LpSchema derive
