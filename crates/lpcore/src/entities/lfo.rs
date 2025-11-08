use std::error::Error;

use schemars::{schema_for, JsonSchema, Schema};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::entity::entity_instance::{EntityInstance, UpdateContext};

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct LfoEntity {
    offset_ms: i64,
    prev_period_ms: i64,
}

impl LfoEntity {
    pub fn new() -> Self {
        Self {
            offset_ms: 0,
            prev_period_ms: 0,
        }
    }
}

impl EntityInstance for LfoEntity {
    fn update(&mut self, context: &dyn UpdateContext) -> Result<JsonValue, Box<dyn Error>> {
        let input: Input = serde_json::from_value(context.eval_input("")?)?;

        let now_ms = context.frame_info().now_ms;

        // ensure phase angle is preserved when the period changes
        if self.prev_period_ms != input.period_ms {
            self.offset_ms = offset_to_maintain_phase(
                now_ms,
                self.offset_ms,
                self.prev_period_ms,
                input.period_ms,
            );
            self.prev_period_ms = input.period_ms;
        }

        let phase_unit = calc_phase_t(now_ms + self.offset_ms, input.period_ms);
        let output_unit = calc_wave_t(phase_unit, input.shape);
        let output_scaled = range_from_t(output_unit, input.min, input.max);

        serde_json::to_value(output_scaled).map_err(Into::into)
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct Input {
    #[schemars(description = "Period of oscillation")]
    #[schemars(extend("ui" = "123"))]
    pub period_ms: i64,

    #[serde(default)]
    pub shape: Shape,

    #[serde(default)]
    pub min: f64,

    #[serde(default = "default_max")]
    pub max: f64,
}

pub fn schema() -> Schema {
    schema_for!(Input)
}

fn default_max() -> f64 {
    1.0
}

/// Waveforms for low frequency oscillators.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub enum Shape {
    Sine,
    Square,
    Triangle,
    Sawtooth,
}

impl Default for Shape {
    fn default() -> Self {
        Self::Sine
    }
}

/// Calculates the offset needed to maintain phase when the period changes.
fn offset_to_maintain_phase(
    now_ms: i64,
    old_offset_ms: i64,
    old_period_ms: i64,
    new_period_ms: i64,
) -> i64 {
    let prev_phase = calc_phase_t(now_ms + old_offset_ms, old_period_ms);
    let new_phase = calc_phase_t(now_ms, new_period_ms);
    println!("prev_phase: {}, new_phase: {}", prev_phase, new_phase);
    ((new_phase - prev_phase) * new_period_ms as f64).round() as i64
}

/// Calculates phase time in the range [0, 1) for a given time and period.
fn calc_phase_t(adjusted_ms: i64, period_ms: i64) -> f64 {
    let phase = adjusted_ms % period_ms;
    let phase = if phase < 0 { phase + period_ms } else { phase };
    phase as f64 / period_ms as f64
}

/// Calculates the wave values for a given phase and waveform.
fn calc_wave_t(phase_unit: f64, waveform: Shape) -> f64 {
    match waveform {
        Shape::Sine => (phase_unit * 2.0 * std::f64::consts::PI).sin(),
        Shape::Square => {
            if phase_unit < 0.5 {
                1.0
            } else {
                -1.0
            }
        }
        Shape::Triangle => {
            if phase_unit < 0.5 {
                phase_unit * 2.0
            } else {
                2.0 - phase_unit * 2.0
            }
        }
        Shape::Sawtooth => phase_unit * 2.0 - 1.0,
    }
}

/// Scales a values from the range [0, 1) to a values in the range [min, max).
fn range_from_t(unit: f64, min: f64, max: f64) -> f64 {
    unit * (max - min) + min
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-8;

    fn assert_close_to(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < EPSILON,
            "expected {} but got {} (diff: {})",
            expected,
            actual,
            (actual - expected).abs()
        );
    }

    #[test]
    fn test_offset_to_maintain_phase() {
        // At 1500ms with a period of 1000 we should be t=0.5
        // Changing to a period of 2000 we should be t=0.75
        // So we need to offset by 500ms
        assert_eq!(offset_to_maintain_phase(1500, 0, 1000, 2000), 500);
        assert_eq!(offset_to_maintain_phase(1000, 500, 1000, 2000), 0);

        // Check negative case: if we try to maintain phase with negative ms, should still work
        // At 1900ms with a period of 1000 we should be t=0.9
        // Changing to a period of 500 we should be t=0.8
        // (0.8 - 0.9) * 500 = -50
        assert_eq!(offset_to_maintain_phase(1900, 0, 1000, 500), -50);
    }

    #[test]
    fn test_calc_phase_t_basic() {
        // 1000 ms period, 250 ms in: 0.25
        assert_close_to(calc_phase_t(250, 1000), 0.25);
        // 1000 ms period, 1000 ms in: 0.0 (wraps)
        assert_close_to(calc_phase_t(1000, 1000), 0.0);
        // 1000 ms period, 1750 ms in: 0.75
        assert_close_to(calc_phase_t(1750, 1000), 0.75);
        // Negative adjusted_ms
        assert_close_to(calc_phase_t(-250, 1000), 0.75);
    }

    #[test]
    fn test_calc_wave_t_sine() {
        // Sine at phase 0.0 should be 0.0
        assert_close_to(calc_wave_t(0.0, Shape::Sine), 0.0);
        // Sine at phase 0.25 should be 1.0
        assert_close_to(calc_wave_t(0.25, Shape::Sine), 1.0);
        // Sine at phase 0.5 should be 0.0
        assert_close_to(calc_wave_t(0.5, Shape::Sine), 0.0);
        // Sine at phase 0.75 should be -1.0
        assert_close_to(calc_wave_t(0.75, Shape::Sine), -1.0);
    }

    #[test]
    fn test_calc_wave_t_square() {
        // Square: <0.5 is 1.0, >=0.5 is -1.0
        assert_eq!(calc_wave_t(0.0, Shape::Square), 1.0);
        assert_eq!(calc_wave_t(0.49, Shape::Square), 1.0);
        assert_eq!(calc_wave_t(0.5, Shape::Square), -1.0);
        assert_eq!(calc_wave_t(0.99, Shape::Square), -1.0);
    }

    #[test]
    fn test_calc_wave_t_triangle() {
        // Triangle: at 0.0, should be 0.0
        assert_close_to(calc_wave_t(0.0, Shape::Triangle), 0.0);
        // At 0.25, should be 0.5
        assert_close_to(calc_wave_t(0.25, Shape::Triangle), 0.5);
        // At 0.5, should be 1.0
        assert_close_to(calc_wave_t(0.5, Shape::Triangle), 1.0);
        // At 0.75, should be 0.5
        assert_close_to(calc_wave_t(0.75, Shape::Triangle), 0.5);
        // At 1.0, should be 0.0
        assert_close_to(calc_wave_t(1.0, Shape::Triangle), 0.0);
    }

    #[test]
    fn test_calc_wave_t_sawtooth() {
        // Sawtooth: 0.0 -> -1.0, 0.5 -> 0.0, 1.0 -> 1.0
        assert_close_to(calc_wave_t(0.0, Shape::Sawtooth), -1.0);
        assert_close_to(calc_wave_t(0.5, Shape::Sawtooth), 0.0);
        assert_close_to(calc_wave_t(1.0, Shape::Sawtooth), 1.0);
    }

    #[test]
    fn test_range_from_t() {
        // t=0.0, min=2, max=4 => 2
        assert_close_to(range_from_t(0.0, 2.0, 4.0), 2.0);
        // t=1.0, min=2, max=4 => 4
        assert_close_to(range_from_t(1.0, 2.0, 4.0), 4.0);
        // t=0.5, min=2, max=4 => 3
        assert_close_to(range_from_t(0.5, 2.0, 4.0), 3.0);
        // t=0.25, min=-1, max=1 => -0.5
        assert_close_to(range_from_t(0.25, -1.0, 1.0), -0.5);
    }
}
