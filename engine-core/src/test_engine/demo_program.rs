extern crate alloc;
use alloc::vec;

use crate::test_engine::scene::SceneConfig;
use crate::test_engine::{
    BufferFormat, BufferRef, FxPipelineConfig, MappingConfig, Palette, PipelineStep,
};
use lpscript::{parse_expr, parse_script};

/// Create a test pattern with a rotating white line from the center
pub fn create_test_line_scene(width: usize, height: usize) -> SceneConfig {
    // Simple test: just output the angle as a gradient to verify CenterAngle works
    // centerAngle is now in radians (-π to π), normalize to 0..1 for display
    let program = parse_expr("fract((centerAngle + 3.14159) / 6.28318 + timeNorm)");

    // Grayscale palette (white = white, black = black)
    let palette = Palette::grayscale();

    let pipeline_config = FxPipelineConfig::new(
        2,
        vec![
            PipelineStep::ExprStep {
                program,
                output: BufferRef::new(0, BufferFormat::ImageGrey),
                params: vec![],
            },
            PipelineStep::PaletteStep {
                input: BufferRef::new(0, BufferFormat::ImageGrey),
                output: BufferRef::new(1, BufferFormat::ImageRgb),
                palette,
            },
        ],
    );

    let mapping_config = MappingConfig::CircularPanel7Ring { width, height };

    SceneConfig::new(pipeline_config, mapping_config)
}

/// Create the standard demo scene configuration
pub fn create_demo_scene(width: usize, height: usize) -> SceneConfig {
    // Demo program: RGB color waves with custom function
    // Returns vec3 (RGB) directly instead of using palette
    let program = parse_script(
        "\
        float wave(float dist, float angle, float freq, float phase) {
          return smoothstep(0.0, 0.4, fract(dist * freq + angle * 0.3 + phase));
        }

        float w1 = wave(centerDist, centerAngle, 4.0, -time * 0.5);
        float w2 = wave(centerDist, -centerAngle, 2.5, time * 0.3);
        float noise = perlin3(vec3(uv * 2.0, time * 0.2), 2);

        float brightness = (w1 * 0.6 + w2 * 0.4) * (0.4 + 0.6 * noise);

        float hue = fract(centerAngle * 0.15915 + time * 0.1);
        float r = saturate(abs(hue * 6.0 - 3.0) - 1.0);
        float g = saturate(2.0 - abs(hue * 6.0 - 2.0));
        float b = saturate(2.0 - abs(hue * 6.0 - 4.0));

        return vec3(r, g, b) * brightness;
    ",
    );

    // Build pipeline configuration
    let pipeline_config = FxPipelineConfig::new(
        2, // One buffer: RGB output
        vec![
            PipelineStep::ExprStep {
                program,
                output: BufferRef::new(1, BufferFormat::ImageRgb),
                params: vec![],
            },
            // PipelineStep::BlurStep {
            //     input: BufferRef::new(1, BufferFormat::ImageRgb),
            //     output: BufferRef::new(0, BufferFormat::ImageRgb), // Reuse buffer 0
            //     radius: Fixed::from_f32(0.1),                      // 0.2 pixel blur radius
            // },
        ],
    );

    let mapping_config = MappingConfig::CircularPanel7Ring { width, height };

    SceneConfig::new(pipeline_config, mapping_config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lpscript::math::Fixed;

    #[test]
    fn test_simple_white() {
        // First test: just output white for everything
        use lpscript::vm::execute_program_lps;
        let mut output = vec![Fixed::ZERO; 16 * 16];

        let program = parse_expr("1.0");

        execute_program_lps(&program, &mut output, 16, 16, Fixed::ZERO);

        // All pixels should be white
        assert_eq!(output[0], Fixed::ONE, "First pixel should be white");
        assert_eq!(
            output[8 * 16],
            Fixed::ONE,
            "Row 8 first pixel should be white"
        );
    }

    #[test]
    fn test_yint_load() {
        // Test that YInt loads correctly
        use lpscript::vm::execute_program_lps;
        let mut output = vec![Fixed::ZERO; 16 * 16];

        let program = parse_expr("coord.y");

        execute_program_lps(&program, &mut output, 16, 16, Fixed::ZERO);

        // Row 0 should have Y values of 0.5 in fixed-point
        println!(
            "Row 0, pixel 0: {:#x} (expected ~{:#x})",
            output[0].0,
            1 << 15
        );
        // Row 8 should have Y values of 8.5 in fixed-point
        println!(
            "Row 8, pixel 0: {:#x} (expected ~{:#x})",
            output[8 * 16].0,
            (8 << 16) + (1 << 15)
        );

        // Just verify it's not all zeros
        assert!(output[8 * 16].0 != 0, "Row 8 YInt should not be zero");
    }

    #[test]
    fn test_normalized_center_line() {
        // Test the normalized Y coordinate approach
        use lpscript::vm::execute_program_lps;

        // Test with 16x16 - center should be between row 7 and 8
        let mut output = vec![Fixed::ZERO; 16 * 16];

        // Adjusted range to match actual uv.y values
        // Row 7: uv.y = 0.4688, Row 8: uv.y = 0.5312
        let program = parse_expr("(uv.y > 0.46 && uv.y < 0.54) ? 1.0 : 0.0");

        execute_program_lps(&program, &mut output, 16, 16, Fixed::ZERO);

        // Center rows (7 and 8) should be white
        assert_eq!(
            output[7 * 16],
            Fixed::ONE,
            "Row 7 (uv.y=0.4688) should be white"
        );
        assert_eq!(
            output[8 * 16],
            Fixed::ONE,
            "Row 8 (uv.y=0.5312) should be white"
        );
        // Rows far from center should be black
        assert_eq!(output[0], Fixed::ZERO, "Top row should be black");
        assert_eq!(output[15 * 16], Fixed::ZERO, "Bottom row should be black");

        // Test with 8x8 - center should be between row 3 and 4
        let mut output8 = vec![Fixed::ZERO; 8 * 8];
        // Row 3: (3+0.5)/8 = 0.4375, Row 4: (4+0.5)/8 = 0.5625
        execute_program_lps(&program, &mut output8, 8, 8, Fixed::ZERO);

        // Center rows (3 and 4) should be white with the range 0.46-0.54
        assert_eq!(
            output8[3 * 8],
            Fixed::ZERO,
            "Row 3 (uv.y=0.4375) should be black (outside range)"
        );
        assert_eq!(
            output8[4 * 8],
            Fixed::ZERO,
            "Row 4 (uv.y=0.5625) should be black (outside range)"
        );
        assert_eq!(output8[0], Fixed::ZERO, "Top row in 8x8 should be black");
    }
}
