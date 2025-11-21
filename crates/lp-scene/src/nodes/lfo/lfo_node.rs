use lp_data::RuntimeError;
use lp_math::dec32::Dec32;

use super::lfo_input::LfoInput;
use super::lfo_math::{calc_phase_t, calc_wave_t, offset_to_maintain_phase, range_from_t};
use crate::node::{LpNode, NodeContext};
use crate::nodes::lfo::lfo_state::LfoState;

/// LFO node that generates oscillating values.
#[derive(
    Debug, Clone, PartialEq, lp_data_derive::RecordValue, serde::Serialize, serde::Deserialize,
)]
pub struct LfoNode {
    /// Input parameters.
    pub input: crate::nodes::lfo::lfo_input::LfoInput,

    /// Internal state.
    pub state: crate::nodes::lfo::lfo_state::LfoState,

    /// Output value (Dec32).
    pub output: Dec32,
}

impl Default for LfoNode {
    fn default() -> Self {
        Self {
            input: LfoInput::default(),
            state: LfoState::default(),
            output: Dec32::ZERO,
        }
    }
}

impl LfoNode {
    /// Create a new LFO node with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new LFO node with the given input.
    pub fn with_input(input: LfoInput) -> Self {
        Self {
            input,
            state: LfoState::default(),
            output: Dec32::ZERO,
        }
    }
}

impl LpNode for LfoNode {
    fn update(&mut self, context: &dyn NodeContext) -> Result<(), RuntimeError> {
        let now_ms = context.frame_time_ms();

        // Ensure phase angle is preserved when the period changes
        if self.state.prev_period_ms != self.input.period_ms {
            self.state.offset_ms = offset_to_maintain_phase(
                now_ms,
                self.state.offset_ms as i64,
                self.state.prev_period_ms as i64,
                self.input.period_ms as i64,
            ) as i32;
            self.state.prev_period_ms = self.input.period_ms;
        }

        let phase_unit = calc_phase_t(
            now_ms + self.state.offset_ms as i64,
            self.input.period_ms as i64,
        );
        let output_unit = calc_wave_t(phase_unit, self.input.shape);
        let output_scaled = range_from_t(output_unit, self.input.min, self.input.max);

        self.output = output_scaled;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lfo_node_update() {
        let mut node = LfoNode::new();
        node.input.period_ms = 1000;
        node.input.min = Dec32::ZERO;
        node.input.max = Dec32::ONE;

        struct TestContext {
            time_ms: i64,
        }

        impl NodeContext for TestContext {
            fn frame_time_ms(&self) -> i64 {
                self.time_ms
            }

            fn delta_ms(&self) -> Dec32 {
                Dec32::ZERO
            }
        }

        // Spot check: at time 0, sine is 0 (in [-1,1] range), which maps to 0.5 in [0,1] range
        let context = TestContext { time_ms: 0 };
        node.update(&context).unwrap();
        let output0 = node.output.to_f32();
        assert!(
            (output0 - 0.5).abs() < 0.1,
            "Expected output near 0.5 at time 0, got {}",
            output0
        );

        // Spot check: at 250ms with 1000ms period, phase is 0.25, sine should be near 1.0
        let context = TestContext { time_ms: 250 };
        node.update(&context).unwrap();
        let output1 = node.output.to_f32();
        assert!(
            (output1 - 1.0).abs() < 0.2,
            "Expected output near 1.0 at 250ms, got {}",
            output1
        );

        // Spot check: at 500ms, phase is 0.5, sine is 0, maps to 0.5
        let context = TestContext { time_ms: 500 };
        node.update(&context).unwrap();
        let output2 = node.output.to_f32();
        assert!(
            (output2 - 0.5).abs() < 0.1,
            "Expected output near 0.5 at 500ms, got {}",
            output2
        );
    }

    #[test]
    fn test_lfo_node_period_change() {
        let mut node = LfoNode::new();
        node.input.period_ms = 1000;
        node.input.min = Dec32::ZERO;
        node.input.max = Dec32::ONE;

        struct TestContext {
            time_ms: i64,
        }

        impl NodeContext for TestContext {
            fn frame_time_ms(&self) -> i64 {
                self.time_ms
            }

            fn delta_ms(&self) -> Dec32 {
                Dec32::ZERO
            }
        }

        // Update at 500ms (halfway through 1000ms period)
        let context = TestContext { time_ms: 500 };
        node.update(&context).unwrap();
        let output_before = node.output;

        // Change period to 2000ms - phase should be maintained
        node.input.period_ms = 2000;
        node.update(&context).unwrap();
        let output_after = node.output;

        // Output should be similar (phase maintained)
        let diff = (output_before.to_f32() - output_after.to_f32()).abs();
        assert!(
            diff < 0.2,
            "Phase not maintained: before={}, after={}",
            output_before.to_f32(),
            output_after.to_f32()
        );
    }
}
