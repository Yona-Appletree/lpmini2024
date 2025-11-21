use lp_script::fixed::trig::{cos, sin};
use lp_script::fixed::{Fixed, ToFixed};

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

            // Calculate spiral parameters in fixed-point
            // t = led_in_arm / total_leds_per_arm (progress along arm, 0..1)
            let t: Fixed = led_in_arm.to_fixed() / total_leds_per_arm.to_fixed();

            // radius = t * max_radius
            let max_radius: Fixed = max_radius_px.to_fixed();
            let radius: Fixed = t * max_radius;

            // angle = (arm / arms) + (t * 4) in 0..1 range (represents rotations)
            let arm_angle: Fixed = arm.to_fixed() / arms.to_fixed();
            let spiral_angle: Fixed = t * 4.to_fixed(); // t * 4 (4 full rotations along spiral)
            let angle_normalized: Fixed = arm_angle + spiral_angle;

            // Convert normalized angle (0..1) to radians (0..2π)
            let angle_radians: Fixed = angle_normalized * Fixed::TAU;

            // Use fixed-point sin/cos (they expect radians, return -1..1 in Fixed)
            let cos_val: Fixed = cos(angle_radians);
            let sin_val: Fixed = sin(angle_radians);

            // sin/cos already return -1..1, use directly
            // x = center_x + radius * cos, y = center_y + radius * sin
            let center_x: Fixed = center_x_px.to_fixed();
            let center_y: Fixed = center_y_px.to_fixed();

            let x_offset: Fixed = radius * cos_val;
            let y_offset: Fixed = radius * sin_val;

            let x: Fixed = (center_x + x_offset)
                .max(Fixed::ZERO)
                .min((width as i32 - 1).to_fixed() + Fixed::ONE);
            let y: Fixed = (center_y + y_offset)
                .max(Fixed::ZERO)
                .min((height as i32 - 1).to_fixed() + Fixed::ONE);

            *map = LedMap::new_fixed(x, y);
        }

        LedMapping { maps }
    }

    /// Create a 3-arm spiral (convenience function)
    pub fn spiral_3arm() -> Self {
        Self::spiral(3, 24, 24)
    }
}
