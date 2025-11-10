//! LFO (Low Frequency Oscillator) node.

use lp_math::fixed::Fixed;

/// Configuration for an LFO node.
#[derive(Debug, Clone, PartialEq, lp_data_derive::LpValue, lp_data_derive::RecordValue)]
pub struct LfoConfig {
    /// Oscillation period in seconds.
    pub period: Fixed,
}

/// Runtime structure for an LFO node.
#[derive(Clone, lp_data_derive::LpValue, lp_data_derive::RecordValue)]
pub struct LfoNode {
    /// LFO configuration
    pub config: LfoConfig,
    /// LFO output value
    pub output: Fixed,
}

impl LfoNode {
    pub fn new(config: LfoConfig) -> Self {
        Self {
            config,
            output: Fixed::ZERO,
        }
    }
}
