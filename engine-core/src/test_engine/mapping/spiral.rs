/// Spiral LED mappings
use super::{LedMap, LedMapping};
use crate::math::{FIXED_SHIFT, FIXED_ONE};
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
        let center_x = width as f32 / 2.0;
        let center_y = height as f32 / 2.0;
        let max_radius = if center_x > center_y {
            center_x
        } else {
            center_y
        };

        for i in 0..128 {
            // Distribute LEDs across arms
            let arm = i % arms;
            let led_in_arm = i / arms;
            let total_leds_per_arm = (128 + arms - 1) / arms;

            // Calculate spiral parameters
            let t = led_in_arm as f32 / total_leds_per_arm as f32;
            let radius = t * max_radius;
            let angle = (arm as f32 * 2.0 * 3.14159265 / arms as f32) + (t * 4.0 * 3.14159265);

            // Convert polar to cartesian using lookup-based approximation
            // For std environments, we could use angle.cos()/sin()
            // For no_std, we use libm
            #[cfg(feature = "use-libm")]
            {
                let x = center_x + radius * libm::cosf(angle);
                let y = center_y + radius * libm::sinf(angle);
                let x_fixed = min(max(0, (x * FIXED_ONE as f32) as i32), (((width - 1) as i32) << FIXED_SHIFT) + FIXED_ONE);
                let y_fixed = min(max(0, (y * FIXED_ONE as f32) as i32), (((height - 1) as i32) << FIXED_SHIFT) + FIXED_ONE);
                maps[i] = LedMap::new_fixed(x_fixed, y_fixed);
            }

            #[cfg(not(feature = "use-libm"))]
            {
                let x = center_x + radius * angle.cos();
                let y = center_y + radius * angle.sin();
                let x_fixed = min(max(0, (x * FIXED_ONE as f32) as i32), (((width - 1) as i32) << FIXED_SHIFT) + FIXED_ONE);
                let y_fixed = min(max(0, (y * FIXED_ONE as f32) as i32), (((height - 1) as i32) << FIXED_SHIFT) + FIXED_ONE);
                maps[i] = LedMap::new_fixed(x_fixed, y_fixed);
            }
        }

        LedMapping { maps }
    }

    /// Create a 3-arm spiral (convenience function)
    pub fn spiral_3arm() -> Self {
        Self::spiral(3, 24, 24)
    }
}

