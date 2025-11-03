/// 2D to 1D LED mapping system
use crate::math::{Vec2, Fixed, FIXED_SHIFT, FIXED_ONE};

/// Single LED mapping entry with sub-pixel precision
#[derive(Debug, Clone, Copy)]
pub struct LedMap {
    pub pos: Vec2,
}

impl LedMap {
    pub const fn new(x: usize, y: usize) -> Self {
        LedMap {
            pos: Vec2::from_pixel(x, y),
        }
    }
    
    pub fn new_fixed(x: Fixed, y: Fixed) -> Self {
        LedMap {
            pos: Vec2::from_fixed(x, y),
        }
    }
}

/// LED mapping for the entire strip
pub struct LedMapping {
    maps: [LedMap; 128],
}

impl LedMapping {
    /// Create a new LED mapping from an array
    pub fn new(maps: [LedMap; 128]) -> Self {
        LedMapping { maps }
    }

    /// Create a simple grid mapping (for testing)
    /// Maps 128 LEDs to an 16x8 grid in row-major order
    pub fn grid_16x8() -> Self {
        let mut maps = [LedMap::new(0, 0); 128];
        for i in 0..128 {
            let x = i % 16;
            let y = i / 16;
            maps[i] = LedMap::new(x, y);
        }
        LedMapping { maps }
    }

    /// Create a serpentine/zigzag mapping (common for LED matrices)
    /// Even rows go left-to-right, odd rows go right-to-left
    pub fn serpentine_16x8() -> Self {
        let mut maps = [LedMap::new(0, 0); 128];
        for i in 0..128 {
            let y = i / 16;
            let x = if y % 2 == 0 { i % 16 } else { 15 - (i % 16) };
            maps[i] = LedMap::new(x, y);
        }
        LedMapping { maps }
    }

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
                let x_fixed = ((x * FIXED_ONE as f32) as i32).max(0).min((((width - 1) as i32) << FIXED_SHIFT) + FIXED_ONE);
                let y_fixed = ((y * FIXED_ONE as f32) as i32).max(0).min((((height - 1) as i32) << FIXED_SHIFT) + FIXED_ONE);
                maps[i] = LedMap::new_fixed(x_fixed, y_fixed);
            }

            #[cfg(not(feature = "use-libm"))]
            {
                let x = center_x + radius * angle.cos();
                let y = center_y + radius * angle.sin();
                let x_fixed = ((x * FIXED_ONE as f32) as i32).max(0).min((((width - 1) as i32) << FIXED_SHIFT) + FIXED_ONE);
                let y_fixed = ((y * FIXED_ONE as f32) as i32).max(0).min((((height - 1) as i32) << FIXED_SHIFT) + FIXED_ONE);
                maps[i] = LedMap::new_fixed(x_fixed, y_fixed);
            }
        }

        LedMapping { maps }
    }

    /// Create a 3-arm spiral (convenience function)
    pub fn spiral_3arm() -> Self {
        Self::spiral(3, 16, 16)
    }

    /// Get the mapping for a specific LED index
    #[inline(always)]
    pub fn get(&self, led_index: usize) -> Option<&LedMap> {
        self.maps.get(led_index)
    }
}

/// Apply 2D to 1D mapping with bilinear interpolation
///
/// # Arguments
/// * `rgb_2d` - Input RGB buffer in 2D format (width * height * 3 bytes)
/// * `led_output` - Output buffer for LED strip (led_count * 3 bytes)
/// * `mapping` - LED mapping configuration
/// * `width` - Width of the 2D buffer
/// * `height` - Height of the 2D buffer
pub fn apply_2d_mapping(rgb_2d: &[u8], led_output: &mut [u8], mapping: &LedMapping, width: usize, height: usize) {
    let led_count = led_output.len() / 3;
    assert!(led_count <= 128, "LED count exceeds maximum of 128");

    for led_idx in 0..led_count {
        if let Some(map) = mapping.get(led_idx) {
            // Get integer and fractional parts
            let x_int = (map.pos.x.0 >> FIXED_SHIFT) as usize;
            let y_int = (map.pos.y.0 >> FIXED_SHIFT) as usize;
            let x_frac = (map.pos.x.0 & (FIXED_ONE - 1)) as i64;
            let y_frac = (map.pos.y.0 & (FIXED_ONE - 1)) as i64;

            // Bounds check for bilinear sampling
            if x_int + 1 < width && y_int + 1 < height {
                // Sample 4 neighboring pixels
                let idx_00 = (y_int * width + x_int) * 3;
                let idx_10 = (y_int * width + x_int + 1) * 3;
                let idx_01 = ((y_int + 1) * width + x_int) * 3;
                let idx_11 = ((y_int + 1) * width + x_int + 1) * 3;

                // Bilinear interpolation for each channel
                let dst_idx = led_idx * 3;
                for c in 0..3 {
                    let c00 = rgb_2d[idx_00 + c] as i64;
                    let c10 = rgb_2d[idx_10 + c] as i64;
                    let c01 = rgb_2d[idx_01 + c] as i64;
                    let c11 = rgb_2d[idx_11 + c] as i64;

                    // Lerp in x direction
                    let top = c00 + ((c10 - c00) * x_frac >> FIXED_SHIFT);
                    let bottom = c01 + ((c11 - c01) * x_frac >> FIXED_SHIFT);

                    // Lerp in y direction
                    let result = top + ((bottom - top) * y_frac >> FIXED_SHIFT);
                    led_output[dst_idx + c] = result.clamp(0, 255) as u8;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_mapping() {
        let mapping = LedMapping::grid_16x8();

        // First LED should map to (0.5, 0.5) in fixed point
        let first = mapping.get(0).unwrap();
        assert_eq!(first.pos.x.0 >> FIXED_SHIFT, 0);
        assert_eq!(first.pos.y.0 >> FIXED_SHIFT, 0);

        // LED 16 should map to (0.5, 1.5) - start of second row
        let row2 = mapping.get(16).unwrap();
        assert_eq!(row2.pos.x.0 >> FIXED_SHIFT, 0);
        assert_eq!(row2.pos.y.0 >> FIXED_SHIFT, 1);

        // LED 127 should map to (15.5, 7.5) - last position
        let last = mapping.get(127).unwrap();
        assert_eq!(last.pos.x.0 >> FIXED_SHIFT, 15);
        assert_eq!(last.pos.y.0 >> FIXED_SHIFT, 7);
    }

    #[test]
    fn test_serpentine_mapping() {
        let mapping = LedMapping::serpentine_16x8();

        // First row: 0-15 maps to (0.5,0.5) through (15.5,0.5)
        let first = mapping.get(0).unwrap();
        assert_eq!(first.pos.x.0 >> FIXED_SHIFT, 0);
        assert_eq!(first.pos.y.0 >> FIXED_SHIFT, 0);

        let end_first_row = mapping.get(15).unwrap();
        assert_eq!(end_first_row.pos.x.0 >> FIXED_SHIFT, 15);
        assert_eq!(end_first_row.pos.y.0 >> FIXED_SHIFT, 0);

        // Second row: 16-31 maps to (15.5,1.5) through (0.5,1.5) (reversed)
        let start_second_row = mapping.get(16).unwrap();
        assert_eq!(start_second_row.pos.x.0 >> FIXED_SHIFT, 15);
        assert_eq!(start_second_row.pos.y.0 >> FIXED_SHIFT, 1);

        let end_second_row = mapping.get(31).unwrap();
        assert_eq!(end_second_row.pos.x.0 >> FIXED_SHIFT, 0);
        assert_eq!(end_second_row.pos.y.0 >> FIXED_SHIFT, 1);
    }

    #[test]
    fn test_apply_mapping() {
        // Create a 16x8 RGB buffer (width=16, height=8)
        let mut rgb_2d = vec![0u8; 16 * 8 * 3];

        // Set a 2x2 block to red for bilinear sampling
        for y in 3..5 {
            for x in 5..7 {
                let idx = (y * 16 + x) * 3;
                rgb_2d[idx] = 255; // R
                rgb_2d[idx + 1] = 0; // G
                rgb_2d[idx + 2] = 0; // B
            }
        }

        // Apply grid mapping (LED at (5,3) maps to pixel center (5.5, 3.5))
        let mapping = LedMapping::grid_16x8();
        let mut led_output = vec![0u8; 128 * 3];
        apply_2d_mapping(&rgb_2d, &mut led_output, &mapping, 16, 8);

        // LED at index (3 * 16 + 5) = 53 should be red (with bilinear it samples 4 red pixels)
        let led_idx = 53 * 3;
        assert_eq!(led_output[led_idx], 255);
        assert_eq!(led_output[led_idx + 1], 0);
        assert_eq!(led_output[led_idx + 2], 0);
    }
}
