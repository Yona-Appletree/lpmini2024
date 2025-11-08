use lp_data::LpSchema;
use lp_math::fixed::Fixed;
use serde::{Deserialize, Serialize};

/// Configuration for a UI slider control
#[derive(Serialize, Deserialize, Debug, Clone, LpSchema)]
#[lp(schema(name = "UI Slider Config",))]
pub struct UiSliderConfig {
    /// Minimum value
    #[lp(field(ui(slider(min = 0.0, max = 1.0))))]
    pub min: Fixed,

    /// Maximum value
    #[lp(field(ui(slider(min = 0.0, max = 1.0))))]
    pub max: Fixed,

    /// Step size
    #[lp(field(ui(slider(min = 0.0, max = 1.0, step = 0.01))))]
    pub step: Fixed,

    /// Default value
    #[lp(field(ui(slider(min = 0.0, max = 1.0))))]
    pub default: Fixed,
}
