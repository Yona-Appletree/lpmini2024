use lp_script::dec32::trig::{cos, sin};
use lp_script::dec32::{Dec32, ToDec32};

/// Spiral LED mappings
use super::{LedMap, LedMapping};

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

        for (i, map) in maps.iter_mut().enumerate() {
            // Distribute LEDs across arms
            let arm = i % arms;
            let led_in_arm = i / arms;
            let total_leds_per_arm = 128_usize.div_ceil(arms);

            // Calculate spiral parameters in dec32-point
            // t = led_in_arm / total_leds_per_arm (progress along arm, 0..1)
            let t: Dec32 = led_in_arm.to_dec32() / total_leds_per_arm.to_dec32();

            // radius = t * max_radius
            let max_radius: Dec32 = max_radius_px.to_dec32();
            let radius: Dec32 = t * max_radius;

            // angle = (arm / arms) + (t * 4) in 0..1 range (represents rotations)
            let arm_angle: Dec32 = arm.to_dec32() / arms.to_dec32();
            let spiral_angle: Dec32 = t * 4.to_dec32(); // t * 4 (4 full rotations along spiral)
            let angle_normalized: Dec32 = arm_angle + spiral_angle;

            // Convert normalized angle (0..1) to radians (0..2π)
            let angle_radians: Dec32 = angle_normalized * Dec32::TAU;

            // Use dec32-point sin/cos (they expect radians, return -1..1 in Dec32)
            let cos_val: Dec32 = cos(angle_radians);
            let sin_val: Dec32 = sin(angle_radians);

            // sin/cos already return -1..1, use directly
            // x = center_x + radius * cos, y = center_y + radius * sin
            let center_x: Dec32 = center_x_px.to_dec32();
            let center_y: Dec32 = center_y_px.to_dec32();

            let x_offset: Dec32 = radius * cos_val;
            let y_offset: Dec32 = radius * sin_val;

            let x: Dec32 = (center_x + x_offset)
                .max(Dec32::ZERO)
                .min((width as i32 - 1).to_dec32() + Dec32::ONE);
            let y: Dec32 = (center_y + y_offset)
                .max(Dec32::ZERO)
                .min((height as i32 - 1).to_dec32() + Dec32::ONE);

            *map = LedMap::new_dec32(x, y);
        }

        LedMapping { maps }
    }

    /// Create a 3-arm spiral (convenience function)
    pub fn spiral_3arm() -> Self {
        Self::spiral(3, 24, 24)
    }
}
