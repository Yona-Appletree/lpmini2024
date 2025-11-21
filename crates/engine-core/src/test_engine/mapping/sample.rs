/// Image sampling with bilinear interpolation
use lp_gfx::lp_script::dec32::{Dec32, ToDec32};

/// Bilinear interpolation of a single channel
///
/// # Arguments
/// * `c00` - Top-left pixel value
/// * `c10` - Top-right pixel value
/// * `c01` - Bottom-left pixel value
/// * `c11` - Bottom-right pixel value
/// * `x_frac` - Horizontal fraction (0..Dec32::ONE)
/// * `y_frac` - Vertical fraction (0..Dec32::ONE)
///
/// # Returns
/// Interpolated value (0..255)
pub fn bilinear_interp_channel(
    c00: u8,
    c10: u8,
    c01: u8,
    c11: u8,
    x_frac: Dec32,
    y_frac: Dec32,
) -> u8 {
    // Convert u8 values to Dec32 for interpolation
    let c00_dec32: Dec32 = (c00 as i32).to_dec32();
    let c10_dec32: Dec32 = (c10 as i32).to_dec32();
    let c01_dec32: Dec32 = (c01 as i32).to_dec32();
    let c11_dec32: Dec32 = (c11 as i32).to_dec32();

    // Lerp in x direction
    let top: Dec32 = c00_dec32 + (c10_dec32 - c00_dec32) * x_frac;
    let bottom: Dec32 = c01_dec32 + (c11_dec32 - c01_dec32) * x_frac;

    // Lerp in y direction
    let result: Dec32 = top + (bottom - top) * y_frac;

    // Clamp and convert to u8
    let clamped = result.max(Dec32::ZERO).min(255i32.to_dec32());
    clamped.to_i32().clamp(0, 255) as u8
}

/// Bilinear interpolation of an RGB pixel
///
/// # Arguments
/// * `rgb00` - Top-left pixel [r, g, b]
/// * `rgb10` - Top-right pixel [r, g, b]
/// * `rgb01` - Bottom-left pixel [r, g, b]
/// * `rgb11` - Bottom-right pixel [r, g, b]
/// * `x_frac` - Horizontal fraction (0..Dec32::ONE)
/// * `y_frac` - Vertical fraction (0..Dec32::ONE)
///
/// # Returns
/// Interpolated RGB pixel [r, g, b]
pub fn bilinear_interp_rgb(
    rgb00: [u8; 3],
    rgb10: [u8; 3],
    rgb01: [u8; 3],
    rgb11: [u8; 3],
    x_frac: Dec32,
    y_frac: Dec32,
) -> [u8; 3] {
    [
        bilinear_interp_channel(rgb00[0], rgb10[0], rgb01[0], rgb11[0], x_frac, y_frac),
        bilinear_interp_channel(rgb00[1], rgb10[1], rgb01[1], rgb11[1], x_frac, y_frac),
        bilinear_interp_channel(rgb00[2], rgb10[2], rgb01[2], rgb11[2], x_frac, y_frac),
    ]
}

/// Sample an RGB pixel from a 2D buffer at dec32-point coordinates
///
/// # Arguments
/// * `buffer` - RGB buffer (width * height * 3 bytes)
/// * `x` - X coordinate in dec32-point
/// * `y` - Y coordinate in dec32-point
/// * `width` - Buffer width in pixels
/// * `height` - Buffer height in pixels
///
/// # Returns
/// Sampled RGB pixel [r, g, b], or [0, 0, 0] if out of bounds
pub fn sample_rgb_bilinear(
    buffer: &[u8],
    x: Dec32,
    y: Dec32,
    width: usize,
    height: usize,
) -> [u8; 3] {
    // Get integer and fractional parts
    let x_int = x.to_i32() as usize;
    let y_int = y.to_i32() as usize;
    let x_frac = x.frac();
    let y_frac = y.frac();

    // Bounds check - must be within the image
    if x_int >= width || y_int >= height {
        return [0, 0, 0];
    }

    // If we're exactly on a pixel (no fractional part), just return that pixel
    if x_frac.0 == 0 && y_frac.0 == 0 {
        let idx = (y_int * width + x_int) * 3;
        return [buffer[idx], buffer[idx + 1], buffer[idx + 2]];
    }

    // For bilinear interpolation, we need the neighboring pixels
    // If we're on the right or bottom edge, clamp to avoid going out of bounds
    let x_int_1 = if x_int + 1 < width { x_int + 1 } else { x_int };
    let y_int_1 = if y_int + 1 < height { y_int + 1 } else { y_int };

    // Sample 4 neighboring pixels (some may be the same if on edge)
    let idx_00 = (y_int * width + x_int) * 3;
    let idx_10 = (y_int * width + x_int_1) * 3;
    let idx_01 = (y_int_1 * width + x_int) * 3;
    let idx_11 = (y_int_1 * width + x_int_1) * 3;

    let rgb00 = [buffer[idx_00], buffer[idx_00 + 1], buffer[idx_00 + 2]];
    let rgb10 = [buffer[idx_10], buffer[idx_10 + 1], buffer[idx_10 + 2]];
    let rgb01 = [buffer[idx_01], buffer[idx_01 + 1], buffer[idx_01 + 2]];
    let rgb11 = [buffer[idx_11], buffer[idx_11 + 1], buffer[idx_11 + 2]];

    bilinear_interp_rgb(rgb00, rgb10, rgb01, rgb11, x_frac, y_frac)
}

#[cfg(all(test, not(feature = "use-libm")))]
mod tests {
    use lp_gfx::lp_script::dec32::ToDec32;

    use super::*;

    #[test]
    fn test_bilinear_interp_channel_corners() {
        // At (0, 0) - should return c00
        let result = bilinear_interp_channel(100, 200, 50, 150, Dec32::ZERO, Dec32::ZERO);
        assert_eq!(result, 100);

        // At (1, 0) - should return c10
        let result = bilinear_interp_channel(100, 200, 50, 150, Dec32::ONE, Dec32::ZERO);
        assert_eq!(result, 200);

        // At (0, 1) - should return c01
        let result = bilinear_interp_channel(100, 200, 50, 150, Dec32::ZERO, Dec32::ONE);
        assert_eq!(result, 50);

        // At (1, 1) - should return c11
        let result = bilinear_interp_channel(100, 200, 50, 150, Dec32::ONE, Dec32::ONE);
        assert_eq!(result, 150);
    }

    #[test]
    fn test_bilinear_interp_channel_center() {
        // At (0.5, 0.5) - should return average of all four
        let half = Dec32::HALF;
        let result = bilinear_interp_channel(0, 100, 100, 200, half, half);
        // (0 + 100) / 2 = 50 (top)
        // (100 + 200) / 2 = 150 (bottom)
        // (50 + 150) / 2 = 100 (center)
        assert_eq!(result, 100);
    }

    #[test]
    fn test_bilinear_interp_channel_exact_pixel_boundary() {
        // When exactly on a pixel boundary (x_frac = 0, y_frac = 0)
        // Should return the exact pixel value
        let result = bilinear_interp_channel(255, 0, 0, 0, Dec32::ZERO, Dec32::ZERO);
        assert_eq!(result, 255);
    }

    #[test]
    fn test_bilinear_interp_rgb() {
        let rgb00 = [255, 0, 0]; // Red
        let rgb10 = [0, 255, 0]; // Green
        let rgb01 = [0, 0, 255]; // Blue
        let rgb11 = [255, 255, 0]; // Yellow

        // At center (0.5, 0.5)
        let half = Dec32::HALF;
        let result = bilinear_interp_rgb(rgb00, rgb10, rgb01, rgb11, half, half);

        // Each channel should be averaged
        // R: (255 + 0 + 0 + 255) / 4 = 127.5 ≈ 127
        // G: (0 + 255 + 0 + 255) / 4 = 127.5 ≈ 127
        // B: (0 + 0 + 255 + 0) / 4 = 63.75 ≈ 63
        assert_eq!(result[0], 127); // R
        assert_eq!(result[1], 127); // G
        assert_eq!(result[2], 63); // B
    }

    #[test]
    fn test_sample_rgb_bilinear_center() {
        // Create a 2x2 test image
        let buffer = [
            255, 0, 0, 0, 255, 0, // Row 0: Red, Green
            0, 0, 255, 255, 255, 0, // Row 1: Blue, Yellow
        ];

        // Sample at pixel (0, 0) - should be red
        let result = sample_rgb_bilinear(&buffer, Dec32::ZERO, Dec32::ZERO, 2, 2);
        assert_eq!(result, [255, 0, 0]);

        // Sample at center (0.5, 0.5)
        let half = Dec32::HALF;
        let result = sample_rgb_bilinear(&buffer, half, half, 2, 2);
        assert_eq!(result[0], 127); // R
        assert_eq!(result[1], 127); // G
        assert_eq!(result[2], 63); // B
    }

    #[test]
    fn test_sample_rgb_bilinear_exact_boundaries() {
        // Test the specific case where we're exactly on pixel boundaries
        // This mimics the issue with LEDs 81 and 89
        let buffer = [
            255, 0, 0, 0, 255, 0, 0, 0, 255, // Row 0
            0, 255, 0, 255, 255, 0, 255, 0, 255, // Row 1
            0, 0, 255, 255, 0, 255, 0, 255, 255, // Row 2
        ];

        // Sample at exact pixel (2, 0) - rightmost pixel of first row
        // When exactly on a pixel, should return that pixel's value
        let x = 2.0f32.to_dec32();
        let y = 0.0f32.to_dec32();
        let result = sample_rgb_bilinear(&buffer, x, y, 3, 3);
        assert_eq!(result, [0, 0, 255]); // Blue pixel at (2, 0)

        // Sample at exact pixel (0, 2) - bottommost pixel of first column
        let x = 0.0f32.to_dec32();
        let y = 2.0f32.to_dec32();
        let result = sample_rgb_bilinear(&buffer, x, y, 3, 3);
        assert_eq!(result, [0, 0, 255]); // Blue pixel at (0, 2)

        // Sample at exact pixel (1, 1) - center pixel
        let x = 1.0f32.to_dec32();
        let y = 1.0f32.to_dec32();
        let result = sample_rgb_bilinear(&buffer, x, y, 3, 3);
        assert_eq!(result, [255, 255, 0]); // Yellow pixel at (1, 1)

        // Sample at corner (0, 0)
        let x = 0.0f32.to_dec32();
        let y = 0.0f32.to_dec32();
        let result = sample_rgb_bilinear(&buffer, x, y, 3, 3);
        assert_eq!(result, [255, 0, 0]); // Red pixel at (0, 0)

        // Sample at bottom-right corner (2, 2)
        let x = 2.0f32.to_dec32();
        let y = 2.0f32.to_dec32();
        let result = sample_rgb_bilinear(&buffer, x, y, 3, 3);
        assert_eq!(result, [0, 255, 255]); // Cyan pixel at (2, 2)
    }

    #[test]
    fn test_sample_rgb_out_of_bounds() {
        let buffer = [255, 0, 0, 0, 255, 0]; // 2x1 image

        // Sample beyond width
        let result = sample_rgb_bilinear(&buffer, 2.0f32.to_dec32(), 0i32.to_dec32(), 2, 1);
        assert_eq!(result, [0, 0, 0]);

        // Sample beyond height
        let result = sample_rgb_bilinear(&buffer, 0i32.to_dec32(), 1.0f32.to_dec32(), 2, 1);
        assert_eq!(result, [0, 0, 0]);
    }

    #[test]
    fn test_circular_panel_led_81_and_89() {
        // Reproduce the exact issue: LEDs 81 and 89 in circular_panel_7ring mapping
        // are sampling at exact pixel boundaries and returning black
        extern crate alloc;
        use alloc::vec;

        use super::super::{apply_2d_mapping, LedMapping};

        const WIDTH: usize = 16;
        const HEIGHT: usize = 16;

        // Create the circular panel mapping (same as test_scene)
        let mapping = LedMapping::circular_panel_7ring(WIDTH, HEIGHT);

        // Create a simple gradient RGB buffer (red to green)
        let mut rgb_buffer = vec![0u8; WIDTH * HEIGHT * 3];
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let idx = (y * WIDTH + x) * 3;
                let brightness = (x * 255 / (WIDTH - 1)) as u8;
                rgb_buffer[idx] = 255 - brightness; // R: high on left
                rgb_buffer[idx + 1] = brightness; // G: high on right
                rgb_buffer[idx + 2] = 0; // B: zero
            }
        }

        // Create LED output buffer
        let mut led_output = vec![0u8; 128 * 3];

        // Apply the mapping
        apply_2d_mapping(&rgb_buffer, &mut led_output, &mapping, WIDTH, HEIGHT);

        // Check LED 81 - should NOT be black
        let led_81_idx = 81 * 3;
        let led_81_color = [
            led_output[led_81_idx],
            led_output[led_81_idx + 1],
            led_output[led_81_idx + 2],
        ];
        assert!(
            led_81_color[0] > 0 || led_81_color[1] > 0 || led_81_color[2] > 0,
            "LED 81 should not be black, got {:?}",
            led_81_color
        );

        // Check LED 89 - should NOT be black
        let led_89_idx = 89 * 3;
        let led_89_color = [
            led_output[led_89_idx],
            led_output[led_89_idx + 1],
            led_output[led_89_idx + 2],
        ];
        assert!(
            led_89_color[0] > 0 || led_89_color[1] > 0 || led_89_color[2] > 0,
            "LED 89 should not be black, got {:?}",
            led_89_color
        );

        // Also check their actual mapping positions for debugging
        if let core::option::Option::Some(map_81) = mapping.get(81) {
            let x_int = map_81.pos.x.to_i32() as usize;
            let y_int = map_81.pos.y.to_i32() as usize;
            let x_frac = map_81.pos.x.frac().0;
            let y_frac = map_81.pos.y.frac().0;

            // Direct sample to verify
            let direct_sample =
                sample_rgb_bilinear(&rgb_buffer, map_81.pos.x, map_81.pos.y, WIDTH, HEIGHT);
            assert!(
                direct_sample[0] > 0 || direct_sample[1] > 0 || direct_sample[2] > 0,
                "LED 81 at ({}.{}, {}.{}) should not sample black, got {:?}",
                x_int,
                x_frac,
                y_int,
                y_frac,
                direct_sample
            );
        }

        if let core::option::Option::Some(map_89) = mapping.get(89) {
            let x_int = map_89.pos.x.to_i32() as usize;
            let y_int = map_89.pos.y.to_i32() as usize;
            let x_frac = map_89.pos.x.frac().0;
            let y_frac = map_89.pos.y.frac().0;

            // Direct sample to verify
            let direct_sample =
                sample_rgb_bilinear(&rgb_buffer, map_89.pos.x, map_89.pos.y, WIDTH, HEIGHT);
            assert!(
                direct_sample[0] > 0 || direct_sample[1] > 0 || direct_sample[2] > 0,
                "LED 89 at ({}.{}, {}.{}) should not sample black, got {:?}",
                x_int,
                x_frac,
                y_int,
                y_frac,
                direct_sample
            );
        }
    }
}
