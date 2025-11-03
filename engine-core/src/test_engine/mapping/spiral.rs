/// Spiral LED mappings
use super::{LedMap, LedMapping};
use crate::math::{fixed_from_int, Fixed, FIXED_ONE, FIXED_SHIFT};
use crate::test_engine::vm::{cos_fixed, sin_fixed};
use core::cmp::{max, min};

impl LedMapping {
    /// Create a spiral mapping with configurable number of arms
    ///
    /// # Arguments
    /// * `arms` - Number of spiral arms (1-8)
    /// * `width` - Width of the mapping area (default 16)
    /// * `height` - Height of the mapping area (default 16)
    pub fn spiral(arms: usize, width: usize, height: usize) -> Self {
        let mut maps = [LedMap::new(0, 0); 128];
        let center_x_px = width / 2;
        let center_y_px = height / 2;
        let max_radius_px = if width > height {
            width / 2
        } else {
            height / 2
        };

        for i in 0..128 {
            // Distribute LEDs across arms
            let arm = i % arms;
            let led_in_arm = i / arms;
            let total_leds_per_arm = (128 + arms - 1) / arms;

            // Calculate spiral parameters in fixed-point
            // t = led_in_arm / total_leds_per_arm (progress along arm, 0..1)
            let t_fixed = (led_in_arm as i32 * FIXED_ONE) / total_leds_per_arm as i32;

            // radius = t * max_radius
            let max_radius_fixed = (max_radius_px as i32) << FIXED_SHIFT;
            let radius_fixed = ((t_fixed as i64 * max_radius_fixed as i64) >> FIXED_SHIFT) as i32;

            // angle = (arm / arms) + (t * 4) in 0..1 range (represents rotations)
            let arm_angle = (arm as i32 * FIXED_ONE) / arms as i32;
            let spiral_angle = t_fixed << 2; // t * 4 (4 full rotations along spiral)
            let angle_fixed = arm_angle + spiral_angle;

            // Use fixed-point sin/cos (they return 0..1, we need -1..1)
            let cos_val = cos_fixed(angle_fixed);
            let sin_val = sin_fixed(angle_fixed);

            // Map from 0..1 to -1..1: (val * 2) - 1
            let cos_centered = (cos_val << 1) - FIXED_ONE;
            let sin_centered = (sin_val << 1) - FIXED_ONE;

            // x = center_x + radius * cos, y = center_y + radius * sin
            let center_x_fixed = (center_x_px as i32) << FIXED_SHIFT;
            let center_y_fixed = (center_y_px as i32) << FIXED_SHIFT;

            let x_offset = ((radius_fixed as i64 * cos_centered as i64) >> FIXED_SHIFT) as i32;
            let y_offset = ((radius_fixed as i64 * sin_centered as i64) >> FIXED_SHIFT) as i32;

            let x_fixed = min(
                max(0, center_x_fixed + x_offset),
                fixed_from_int(width as i32 - 1) + FIXED_ONE,
            );
            let y_fixed = min(
                max(0, center_y_fixed + y_offset),
                fixed_from_int(height as i32 - 1) + FIXED_ONE,
            );

            maps[i] = LedMap::new_fixed(x_fixed, y_fixed);
        }

        LedMapping { maps }
    }

    /// Create a 3-arm spiral (convenience function)
    pub fn spiral_3arm() -> Self {
        Self::spiral(3, 24, 24)
    }
}
