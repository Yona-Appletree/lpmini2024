use lp_math::fixed::Fixed;
use serde::{Deserialize, Serialize};

/// Configuration for a UI slider control
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UiSliderConfig {
    /// Minimum value
    pub min: Fixed,

    /// Maximum value
    pub max: Fixed,

    /// Step size
    pub step: Fixed,

    /// Default value
    pub default: Fixed,
}

impl Default for UiSliderConfig {
    fn default() -> Self {
        UiSliderConfig {
            min: Fixed::from_f32(0.0),
            max: Fixed::from_f32(1.0),
            step: Fixed::from_f32(0.01),
            default: Fixed::from_f32(0.5),
        }
    }
}

// Manual implementation until LpSchema derive is fixed
// This type needs to implement LpValue first - for now this is a placeholder
// TODO: Implement LpValue for this type or fix LpSchema derive
