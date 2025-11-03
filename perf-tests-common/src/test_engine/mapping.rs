/// 2D to 1D LED mapping system

/// Single LED mapping entry
#[derive(Debug, Clone, Copy)]
pub struct LedMap {
    pub x: usize,
    pub y: usize,
}

impl LedMap {
    pub const fn new(x: usize, y: usize) -> Self {
        LedMap { x, y }
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
                let x_clamped = if x as usize > width - 1 {
                    width - 1
                } else {
                    x as usize
                };
                let y_clamped = if y as usize > height - 1 {
                    height - 1
                } else {
                    y as usize
                };
                maps[i] = LedMap::new(x_clamped, y_clamped);
            }

            #[cfg(not(feature = "use-libm"))]
            {
                let x = center_x + radius * angle.cos();
                let y = center_y + radius * angle.sin();
                let x_clamped = if x as usize > width - 1 {
                    width - 1
                } else {
                    x as usize
                };
                let y_clamped = if y as usize > height - 1 {
                    height - 1
                } else {
                    y as usize
                };
                maps[i] = LedMap::new(x_clamped, y_clamped);
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

/// Apply 2D to 1D mapping to convert an RGB buffer to LED output
///
/// # Arguments
/// * `rgb_2d` - Input RGB buffer in 2D format (width * height * 3 bytes)
/// * `led_output` - Output buffer for LED strip (led_count * 3 bytes)
/// * `mapping` - LED mapping configuration
/// * `width` - Width of the 2D buffer
pub fn apply_2d_mapping(rgb_2d: &[u8], led_output: &mut [u8], mapping: &LedMapping, width: usize) {
    let led_count = led_output.len() / 3;
    assert!(led_count <= 128, "LED count exceeds maximum of 128");

    for led_idx in 0..led_count {
        if let Some(map) = mapping.get(led_idx) {
            let src_idx = (map.y * width + map.x) * 3;
            let dst_idx = led_idx * 3;

            // Bounds check
            if src_idx + 2 < rgb_2d.len() && dst_idx + 2 < led_output.len() {
                led_output[dst_idx] = rgb_2d[src_idx];
                led_output[dst_idx + 1] = rgb_2d[src_idx + 1];
                led_output[dst_idx + 2] = rgb_2d[src_idx + 2];
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

        // First LED should map to (0, 0)
        let first = mapping.get(0).unwrap();
        assert_eq!(first.x, 0);
        assert_eq!(first.y, 0);

        // LED 16 should map to (0, 1) - start of second row
        let row2 = mapping.get(16).unwrap();
        assert_eq!(row2.x, 0);
        assert_eq!(row2.y, 1);

        // LED 127 should map to (15, 7) - last position
        let last = mapping.get(127).unwrap();
        assert_eq!(last.x, 15);
        assert_eq!(last.y, 7);
    }

    #[test]
    fn test_serpentine_mapping() {
        let mapping = LedMapping::serpentine_16x8();

        // First row: 0-15 maps to (0,0) through (15,0)
        let first = mapping.get(0).unwrap();
        assert_eq!(first.x, 0);
        assert_eq!(first.y, 0);

        let end_first_row = mapping.get(15).unwrap();
        assert_eq!(end_first_row.x, 15);
        assert_eq!(end_first_row.y, 0);

        // Second row: 16-31 maps to (15,1) through (0,1) (reversed)
        let start_second_row = mapping.get(16).unwrap();
        assert_eq!(start_second_row.x, 15);
        assert_eq!(start_second_row.y, 1);

        let end_second_row = mapping.get(31).unwrap();
        assert_eq!(end_second_row.x, 0);
        assert_eq!(end_second_row.y, 1);
    }

    #[test]
    fn test_apply_mapping() {
        // Create a 16x8 RGB buffer (width=16, height=8)
        let mut rgb_2d = vec![0u8; 16 * 8 * 3];

        // Set pixel (5, 3) to red
        let idx = (3 * 16 + 5) * 3;
        rgb_2d[idx] = 255; // R
        rgb_2d[idx + 1] = 0; // G
        rgb_2d[idx + 2] = 0; // B

        // Apply grid mapping
        let mapping = LedMapping::grid_16x8();
        let mut led_output = vec![0u8; 128 * 3];
        apply_2d_mapping(&rgb_2d, &mut led_output, &mapping, 16);

        // LED at index (3 * 16 + 5) = 53 should be red
        let led_idx = 53 * 3;
        assert_eq!(led_output[led_idx], 255);
        assert_eq!(led_output[led_idx + 1], 0);
        assert_eq!(led_output[led_idx + 2], 0);
    }
}
