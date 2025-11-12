use lp_math::fixed::Fixed;

/// Union type for different step configurations
#[derive(
    Debug, Clone, PartialEq, lp_data_derive::EnumStructValue, serde::Serialize, serde::Deserialize,
)]
pub enum StepConfig {
    /// Expression step variant
    Expr {
        /// Expression output value
        output: Fixed,
        /// Expression parameter count
        param_count: i32,
    },
    /// Palette step variant
    Palette {
        /// Palette size
        size: i32,
        /// Brightness multiplier
        brightness: Fixed,
    },
    /// Blur step variant
    Blur {
        /// Blur radius
        radius: Fixed,
    },
}
