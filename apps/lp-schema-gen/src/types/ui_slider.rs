use lp_math::fixed::Fixed;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Configuration for a UI slider control
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
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
