use lp_math::fixed::Vec3;
use serde::{Deserialize, Serialize};

use crate as lp_data;
use crate::LpSchema;

/// Runtime structure for a Perlin3 node.
///
/// This represents the runtime state of a Perlin3 noise node with its input.
#[derive(Debug, Clone, PartialEq)]
pub struct Perlin3Node {
    pub input: Perlin3Input,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, LpSchema)]
#[lp(schema(
    name = "Perlin3 Input",
    docs = "Input configuration for Perlin3 noise node."
))]
pub struct Perlin3Input {
    #[lp(field(docs = "Position vector for noise evaluation."))]
    pub pos: Vec3,
}

impl Perlin3Node {
    pub fn new() -> Self {
        Self {
            input: Perlin3Input {
                pos: Vec3::new(
                    lp_math::fixed::Fixed::ZERO,
                    lp_math::fixed::Fixed::ZERO,
                    lp_math::fixed::Fixed::ZERO,
                ),
            },
        }
    }
}

impl Default for Perlin3Node {
    fn default() -> Self {
        Self::new()
    }
}
