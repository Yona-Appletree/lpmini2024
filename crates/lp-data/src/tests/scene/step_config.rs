use lp_math::dec32::Dec32;

/// Union type for different step configurations
#[derive(
    Debug, Clone, PartialEq, lp_data_derive::EnumStructValue, serde::Serialize, serde::Deserialize,
)]
pub enum StepConfig {
    /// Expression step variant
    Expr {
        /// Expression output value
        output: Dec32,
        /// Expression parameter count
        param_count: i32,
    },
    /// Palette step variant
    Palette {
        /// Palette size
        size: i32,
        /// Brightness multiplier
        brightness: Dec32,
    },
    /// Blur step variant
    Blur {
        /// Blur radius
        radius: Dec32,
    },
}
