use super::mapping::{apply_2d_mapping, LedMapping};
use super::palette::{rgb_buffer_from_greyscale, Palette};
/// High-level rendering pipeline
use super::vm::{execute_program, Fixed, OpCode};

/// Complete frame rendering pipeline
///
/// This function orchestrates the full pipeline:
/// 1. Execute VM program on 2D grayscale buffer
/// 2. Convert grayscale to RGB using palette
/// 3. Map 2D RGB to 1D LED output
///
/// # Arguments
/// * `greyscale_buffer` - Working buffer for grayscale values (width * height)
/// * `input_buffer` - Input buffer for VM (width * height)
/// * `rgb_2d_buffer` - Working buffer for 2D RGB (width * height * 3)
/// * `led_output` - Final LED output buffer (led_count * 3)
/// * `program` - VM program to execute
/// * `palette` - Color palette for RGB conversion
/// * `mapping` - 2D to 1D LED mapping
/// * `width` - Width of 2D buffers
/// * `height` - Height of 2D buffers
/// * `time` - Animation time value
pub fn render_frame(
    greyscale_buffer: &mut [Fixed],
    input_buffer: &[Fixed],
    rgb_2d_buffer: &mut [u8],
    led_output: &mut [u8],
    program: &[OpCode],
    palette: &Palette,
    mapping: &LedMapping,
    width: usize,
    height: usize,
    time: Fixed,
) {
    // Step 1: Execute VM program to generate grayscale buffer
    execute_program(input_buffer, greyscale_buffer, program, width, height, time);

    // Step 2: Convert grayscale to RGB using palette
    rgb_buffer_from_greyscale(greyscale_buffer, rgb_2d_buffer, palette);

    // Step 3: Apply 2D to 1D mapping
    apply_2d_mapping(rgb_2d_buffer, led_output, mapping, width);
}

#[cfg(test)]
mod tests {
    use super::super::vm::{fixed_from_f32, fixed_to_f32, LoadSource, FIXED_SHIFT};
    use super::*;

    #[test]
    fn test_full_pipeline() {
        const WIDTH: usize = 16;
        const HEIGHT: usize = 8;
        const LED_COUNT: usize = 128;

        // Create buffers
        let mut greyscale_buffer = vec![0; WIDTH * HEIGHT];
        let input_buffer = vec![0; WIDTH * HEIGHT];
        let mut rgb_2d_buffer = vec![0u8; WIDTH * HEIGHT * 3];
        let mut led_output = vec![0u8; LED_COUNT * 3];

        // Simple program: set all pixels to 0.5
        let program = vec![
            OpCode::Push(1 << (FIXED_SHIFT - 1)), // 0.5 in fixed-point
            OpCode::Return,
        ];

        // Create palette and mapping
        let palette = Palette::rainbow();
        let mapping = LedMapping::grid_16x8();

        // Render frame
        render_frame(
            &mut greyscale_buffer,
            &input_buffer,
            &mut rgb_2d_buffer,
            &mut led_output,
            &program,
            &palette,
            &mapping,
            WIDTH,
            HEIGHT,
            0,
        );

        // Verify: all grayscale values should be 0.5
        for &val in greyscale_buffer.iter() {
            assert_eq!(val, 1 << (FIXED_SHIFT - 1));
        }

        // Verify: all RGB pixels should be the same (middle of palette)
        let first_r = rgb_2d_buffer[0];
        let first_g = rgb_2d_buffer[1];
        let first_b = rgb_2d_buffer[2];

        for i in 0..(WIDTH * HEIGHT) {
            assert_eq!(rgb_2d_buffer[i * 3], first_r);
            assert_eq!(rgb_2d_buffer[i * 3 + 1], first_g);
            assert_eq!(rgb_2d_buffer[i * 3 + 2], first_b);
        }

        // Verify: all LEDs should have the same color
        for i in 0..LED_COUNT {
            assert_eq!(led_output[i * 3], first_r);
            assert_eq!(led_output[i * 3 + 1], first_g);
            assert_eq!(led_output[i * 3 + 2], first_b);
        }
    }

    #[test]
    fn test_gradient_pipeline() {
        const WIDTH: usize = 16;
        const HEIGHT: usize = 8;
        const LED_COUNT: usize = 128;

        // Create buffers
        let mut greyscale_buffer = vec![0; WIDTH * HEIGHT];
        let input_buffer = vec![0; WIDTH * HEIGHT];
        let mut rgb_2d_buffer = vec![0u8; WIDTH * HEIGHT * 3];
        let mut led_output = vec![0u8; LED_COUNT * 3];

        // Program: horizontal gradient based on x position
        // Using XNorm which is already normalized to 0..1
        let program = vec![
            OpCode::Load(LoadSource::XNorm),
            OpCode::Return,
        ];

        // Create palette and mapping
        let palette = Palette::rainbow();
        let mapping = LedMapping::grid_16x8();

        // Render frame
        render_frame(
            &mut greyscale_buffer,
            &input_buffer,
            &mut rgb_2d_buffer,
            &mut led_output,
            &program,
            &palette,
            &mapping,
            WIDTH,
            HEIGHT,
            0,
        );

        // Verify: grayscale values should increase from left to right
        // First pixel (x=0) should be 0
        assert_eq!(greyscale_buffer[0], 0);

        // Last pixel in first row (x=15) should be close to 1.0
        let last_in_row = greyscale_buffer[15];
        assert!(last_in_row > (1 << (FIXED_SHIFT - 1))); // > 0.5

        // Verify: middle pixel should have different color than edges
        let first_color = (rgb_2d_buffer[0], rgb_2d_buffer[1], rgb_2d_buffer[2]);
        let middle_idx = 7 * 3;
        let middle_color = (
            rgb_2d_buffer[middle_idx],
            rgb_2d_buffer[middle_idx + 1],
            rgb_2d_buffer[middle_idx + 2],
        );

        // At least one channel should be different between first and middle
        let colors_different = first_color.0 != middle_color.0
            || first_color.1 != middle_color.1
            || first_color.2 != middle_color.2;
        assert!(colors_different, "Gradient should produce different colors");
    }

    #[test]
    fn test_horizontal_gradient_with_mapping() {
        const WIDTH: usize = 16;
        const HEIGHT: usize = 16;
        const LED_COUNT: usize = 128;

        // Create buffers
        let mut greyscale_buffer = vec![0; WIDTH * HEIGHT];
        let input_buffer = vec![0; WIDTH * HEIGHT];
        let mut rgb_2d_buffer = vec![0u8; WIDTH * HEIGHT * 3];
        let mut led_output = vec![0u8; LED_COUNT * 3];

        // Simple horizontal gradient: value = x normalized to 0..1
        let program = vec![
            OpCode::Load(LoadSource::XNorm),
            OpCode::Return,
        ];

        // Create palette and serpentine mapping
        let palette = Palette::rainbow();
        let mapping = LedMapping::serpentine_16x8();

        // Render frame
        render_frame(
            &mut greyscale_buffer,
            &input_buffer,
            &mut rgb_2d_buffer,
            &mut led_output,
            &program,
            &palette,
            &mapping,
            WIDTH,
            HEIGHT,
            0,
        );

        // Verify greyscale gradient
        // First column (x=0) should be ~0
        for y in 0..HEIGHT {
            let val = greyscale_buffer[y * WIDTH];
            assert!(
                val < (1 << (FIXED_SHIFT - 4)),
                "x=0 should be near 0, got {}",
                fixed_to_f32(val)
            );
        }

        // Last column (x=15) should be ~1
        for y in 0..HEIGHT {
            let val = greyscale_buffer[y * WIDTH + 15];
            let f = fixed_to_f32(val);
            assert!(f > 0.9, "x=15 should be near 1, got {}", f);
        }

        // Verify RGB gradient changes from left to right
        let left_color = (rgb_2d_buffer[0], rgb_2d_buffer[1], rgb_2d_buffer[2]);
        let right_idx = 15 * 3;
        let right_color = (
            rgb_2d_buffer[right_idx],
            rgb_2d_buffer[right_idx + 1],
            rgb_2d_buffer[right_idx + 2],
        );

        // Left and right should be different colors
        assert_ne!(
            left_color, right_color,
            "Gradient should change from left to right"
        );

        // Verify LED mapping
        // LED 0 should map to position (0,0) with serpentine
        let led0_color = (led_output[0], led_output[1], led_output[2]);
        assert_eq!(led0_color, left_color, "LED 0 should match source position");

        // LED 15 should map to position (15,0)
        let led15_idx = 15 * 3;
        let led15_color = (
            led_output[led15_idx],
            led_output[led15_idx + 1],
            led_output[led15_idx + 2],
        );
        assert_eq!(
            led15_color, right_color,
            "LED 15 should match source position"
        );

        // LED 16 should map to position (15,1) (serpentine reverses second row)
        let source_15_1_idx = (1 * WIDTH + 15) * 3;
        let source_15_1 = (
            rgb_2d_buffer[source_15_1_idx],
            rgb_2d_buffer[source_15_1_idx + 1],
            rgb_2d_buffer[source_15_1_idx + 2],
        );
        let led16_idx = 16 * 3;
        let led16_color = (
            led_output[led16_idx],
            led_output[led16_idx + 1],
            led_output[led16_idx + 2],
        );
        assert_eq!(
            led16_color, source_15_1,
            "LED 16 should map to (15,1) in serpentine"
        );
    }
}
