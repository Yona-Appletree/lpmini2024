use lp_math::fixed::{trig, Fixed};

use crate::nodes::lfo::lfo_waveform::LfoWaveform;

/// Calculates the offset needed to maintain phase when the period changes.
pub fn offset_to_maintain_phase(
    now_ms: i64,
    old_offset_ms: i64,
    old_period_ms: i64,
    new_period_ms: i64,
) -> i64 {
    let prev_phase = calc_phase_t(now_ms + old_offset_ms, old_period_ms);
    let new_phase = calc_phase_t(now_ms, new_period_ms);
    ((new_phase - prev_phase) * new_period_ms as f64).round() as i64
}

/// Calculates phase time in the range [0, 1) for a given time and period.
pub fn calc_phase_t(adjusted_ms: i64, period_ms: i64) -> f64 {
    if period_ms == 0 {
        return 0.0;
    }
    let phase = adjusted_ms % period_ms;
    let phase = if phase < 0 { phase + period_ms } else { phase };
    phase as f64 / period_ms as f64
}

/// Calculates the wave values for a given phase and waveform.
pub fn calc_wave_t(phase_unit: f64, waveform: LfoWaveform) -> Fixed {
    match waveform {
        LfoWaveform::Sine => {
            // Convert phase [0, 1) to radians [0, 2π)
            let radians = Fixed::from_f32(phase_unit as f32 * 2.0 * core::f32::consts::PI);
            trig::sin(radians)
        }
        LfoWaveform::Square => {
            if phase_unit < 0.5 {
                Fixed::ONE
            } else {
                -Fixed::ONE
            }
        }
        LfoWaveform::Triangle => {
            if phase_unit < 0.5 {
                Fixed::from_f32((phase_unit * 2.0) as f32)
            } else {
                Fixed::from_f32((2.0 - phase_unit * 2.0) as f32)
            }
        }
        LfoWaveform::Sawtooth => Fixed::from_f32((phase_unit * 2.0 - 1.0) as f32),
    }
}

/// Scales a value from the range [-1, 1] to a value in the range [min, max].
pub fn range_from_t(unit: Fixed, min: Fixed, max: Fixed) -> Fixed {
    // unit is in range [-1, 1], we need to map it to [min, max]
    // Map [-1, 1] -> [0, 1]: (unit + 1) / 2
    // Then scale [0, 1] -> [min, max]: unit_scaled * (max - min) + min
    let unit_scaled = (unit + Fixed::ONE) / Fixed::from_i32(2);
    unit_scaled * (max - min) + min
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 1e-5;

    fn assert_close_to(actual: Fixed, expected: f32) {
        let actual_f32 = actual.to_f32();
        assert!(
            (actual_f32 - expected).abs() < EPSILON,
            "expected {} but got {} (diff: {})",
            expected,
            actual_f32,
            (actual_f32 - expected).abs()
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
        assert!((calc_phase_t(250, 1000) - 0.25).abs() < 1e-8);
        // 1000 ms period, 1000 ms in: 0.0 (wraps)
        assert!((calc_phase_t(1000, 1000) - 0.0).abs() < 1e-8);
        // 1000 ms period, 1750 ms in: 0.75
        assert!((calc_phase_t(1750, 1000) - 0.75).abs() < 1e-8);
        // Negative adjusted_ms
        assert!((calc_phase_t(-250, 1000) - 0.75).abs() < 1e-8);
        // Zero period
        assert_eq!(calc_phase_t(100, 0), 0.0);
    }

    #[test]
    fn test_calc_phase_t_edge_cases() {
        // Very large periods
        assert!((calc_phase_t(1000000, 2000000) - 0.5).abs() < 1e-8);
        // Very small periods
        assert!((calc_phase_t(1, 2) - 0.5).abs() < 1e-8);
        // Exact multiples
        assert_eq!(calc_phase_t(2000, 1000), 0.0);
        assert_eq!(calc_phase_t(3000, 1000), 0.0);
        // Negative time with positive period
        assert!((calc_phase_t(-100, 1000) - 0.9).abs() < 1e-8);
    }

    #[test]
    fn test_calc_wave_t_sine() {
        // Sine at phase 0.0 should be 0.0
        assert_close_to(calc_wave_t(0.0, LfoWaveform::Sine), 0.0);
        // Sine at phase 0.25 should be approximately 1.0 (fixed-point precision)
        let sine_025 = calc_wave_t(0.25, LfoWaveform::Sine).to_f32();
        assert!(
            (sine_025 - 1.0).abs() < 0.01,
            "Expected sine(0.25) ≈ 1.0, got {}",
            sine_025
        );
        // Sine at phase 0.5 should be 0.0
        assert_close_to(calc_wave_t(0.5, LfoWaveform::Sine), 0.0);
        // Sine at phase 0.75 should be approximately -1.0
        let sine_075 = calc_wave_t(0.75, LfoWaveform::Sine).to_f32();
        assert!(
            (sine_075 - (-1.0)).abs() < 0.01,
            "Expected sine(0.75) ≈ -1.0, got {}",
            sine_075
        );
        // Sine at phase 1.0 should wrap to 0.0
        let sine_10 = calc_wave_t(1.0, LfoWaveform::Sine).to_f32();
        assert!(
            (sine_10 - 0.0).abs() < 0.01,
            "Expected sine(1.0) ≈ 0.0, got {}",
            sine_10
        );
    }

    #[test]
    fn test_calc_wave_t_square() {
        // Square: <0.5 is 1.0, >=0.5 is -1.0
        assert_eq!(calc_wave_t(0.0, LfoWaveform::Square), Fixed::ONE);
        assert_eq!(calc_wave_t(0.49, LfoWaveform::Square), Fixed::ONE);
        assert_eq!(calc_wave_t(0.5, LfoWaveform::Square), -Fixed::ONE);
        assert_eq!(calc_wave_t(0.99, LfoWaveform::Square), -Fixed::ONE);
        // Edge cases
        assert_eq!(calc_wave_t(0.499, LfoWaveform::Square), Fixed::ONE);
        assert_eq!(calc_wave_t(0.501, LfoWaveform::Square), -Fixed::ONE);
    }

    #[test]
    fn test_calc_wave_t_triangle() {
        // Triangle: at 0.0, should be 0.0
        assert_close_to(calc_wave_t(0.0, LfoWaveform::Triangle), 0.0);
        // At 0.25, should be 0.5
        assert_close_to(calc_wave_t(0.25, LfoWaveform::Triangle), 0.5);
        // At 0.5, should be 1.0
        assert_close_to(calc_wave_t(0.5, LfoWaveform::Triangle), 1.0);
        // At 0.75, should be 0.5
        assert_close_to(calc_wave_t(0.75, LfoWaveform::Triangle), 0.5);
        // At 1.0, should be 0.0
        assert_close_to(calc_wave_t(1.0, LfoWaveform::Triangle), 0.0);
        // Edge case: exactly at midpoint
        assert_close_to(calc_wave_t(0.5, LfoWaveform::Triangle), 1.0);
    }

    #[test]
    fn test_calc_wave_t_sawtooth() {
        // Sawtooth: 0.0 -> -1.0, 0.5 -> 0.0, 1.0 -> 1.0
        assert_close_to(calc_wave_t(0.0, LfoWaveform::Sawtooth), -1.0);
        assert_close_to(calc_wave_t(0.5, LfoWaveform::Sawtooth), 0.0);
        assert_close_to(calc_wave_t(1.0, LfoWaveform::Sawtooth), 1.0);
        // Intermediate values
        assert_close_to(calc_wave_t(0.25, LfoWaveform::Sawtooth), -0.5);
        assert_close_to(calc_wave_t(0.75, LfoWaveform::Sawtooth), 0.5);
    }

    #[test]
    fn test_range_from_t() {
        // t=-1.0 (min), min=2, max=4 => 2
        assert_close_to(
            range_from_t(-Fixed::ONE, Fixed::from_i32(2), Fixed::from_i32(4)),
            2.0,
        );
        // t=1.0 (max), min=2, max=4 => 4
        assert_close_to(
            range_from_t(Fixed::ONE, Fixed::from_i32(2), Fixed::from_i32(4)),
            4.0,
        );
        // t=0.0 (mid), min=2, max=4 => 3
        assert_close_to(
            range_from_t(Fixed::ZERO, Fixed::from_i32(2), Fixed::from_i32(4)),
            3.0,
        );
        // t=-0.5, min=-1, max=1 => -0.5
        assert_close_to(
            range_from_t(
                Fixed::from_f32(-0.5),
                Fixed::from_i32(-1),
                Fixed::from_i32(1),
            ),
            -0.5,
        );
    }

    #[test]
    fn test_range_from_t_edge_cases() {
        // Zero range
        assert_close_to(
            range_from_t(Fixed::ZERO, Fixed::from_i32(5), Fixed::from_i32(5)),
            5.0,
        );
        assert_close_to(
            range_from_t(Fixed::ONE, Fixed::from_i32(5), Fixed::from_i32(5)),
            5.0,
        );
        // Negative range
        assert_close_to(
            range_from_t(Fixed::ZERO, Fixed::from_i32(10), Fixed::from_i32(5)),
            7.5,
        );
        // Very small range (fixed-point precision may vary)
        let result = range_from_t(Fixed::ZERO, Fixed::from_f32(1.0), Fixed::from_f32(1.001));
        let result_f32 = result.to_f32();
        assert!(
            (result_f32 - 1.0005).abs() < 0.0001,
            "Expected result near 1.0005, got {}",
            result_f32
        );
    }
}
