use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec::Vec;
use lp_math::fixed::{Fixed, Vec2, Vec3};
use serde::{Deserialize, Serialize};

use crate as lp_data;
use crate::{LpDescribe, LpSchema};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, LpSchema)]
#[lp(schema(name = "LFO Config", docs = "Configuration for an LFO node."))]
pub struct LfoConfig {
    #[lp(field(
        ui(slider(min = 10, max = 60000, step = 1)),
        docs = "Oscillation period in milliseconds."
    ))]
    pub period_ms: i32,
}

/// Runtime structure for an LFO node.
///
/// This represents the runtime state of an LFO node with its config and output.
#[derive(Debug, Clone, PartialEq)]
pub struct LfoNode {
    pub config: LfoConfig,
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
