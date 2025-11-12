//! Test node with all primitive types.

use lp_math::fixed::{Fixed, Vec2, Vec3, Vec4};

use crate::tests::scene::step_config::StepConfig;

/// LFO waveform shape enumeration
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    lp_data_derive::EnumValue,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum LfoWaveform {
    Sine,
    Square,
    Triangle,
    Sawtooth,
}

/// Configuration for a test node with all primitive types.
#[derive(
    Debug, Clone, PartialEq, lp_data_derive::RecordValue, serde::Serialize, serde::Deserialize,
)]
pub struct TestNodeConfig {
    /// Oscillation period in seconds.
    pub period: Fixed,

    /// Waveform shape
    #[lp(enum_unit)]
    pub waveform: LfoWaveform,

    /// Integer count value
    pub count: i32,

    /// Boolean enabled flag
    pub enabled: bool,

    /// 2D position
    pub position: Vec2,

    /// 3D rotation
    pub rotation: Vec3,

    /// 4D color (RGBA)
    pub color: Vec4,

    /// Step configuration (enum_struct type example)
    #[lp(enum_struct)]
    pub step_config: StepConfig,
}

/// Runtime structure for a test node.
#[derive(Clone, lp_data_derive::RecordValue, serde::Serialize, serde::Deserialize)]
pub struct TestNode {
    /// Node configuration
    pub config: TestNodeConfig,
    /// Output value
    pub output: Fixed,
}

impl TestNode {
    pub fn new(config: TestNodeConfig) -> Self {
        Self {
            config,
            output: Fixed::ZERO,
        }
    }
}
