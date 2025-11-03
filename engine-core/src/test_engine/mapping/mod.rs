/// 2D to 1D LED mapping system
use crate::math::{Vec2, Fixed, FIXED_SHIFT, FIXED_ONE};

mod grid;
mod spiral;
mod circular;
mod sample;
pub mod config;

pub use sample::{bilinear_interp_channel, bilinear_interp_rgb, sample_rgb_bilinear};
pub use config::MappingConfig;

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

    /// Get the mapping for a specific LED index
    #[inline(always)]
    pub fn get(&self, led_index: usize) -> core::option::Option<&LedMap> {
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
    #[cfg(not(feature = "use-libm"))]
    assert!(led_count <= 128, "LED count exceeds maximum of 128");

    for led_idx in 0..led_count {
        if let core::option::Option::Some(map) = mapping.get(led_idx) {
            let rgb = sample_rgb_bilinear(rgb_2d, map.pos.x.0, map.pos.y.0, width, height);
            let dst_idx = led_idx * 3;
            led_output[dst_idx] = rgb[0];
            led_output[dst_idx + 1] = rgb[1];
            led_output[dst_idx + 2] = rgb[2];
        }
    }
}

#[cfg(all(test, not(feature = "use-libm")))]
mod tests {
    extern crate alloc;
    use alloc::vec;
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

