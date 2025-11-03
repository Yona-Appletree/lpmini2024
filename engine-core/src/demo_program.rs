extern crate alloc;
use alloc::vec;

use crate::expr::parse_expr;
use crate::scene::SceneConfig;
use crate::test_engine::{
    fixed_from_f32, BufferFormat, BufferRef, FxPipelineConfig, LoadSource, MappingConfig, OpCode,
    Palette, PipelineStep,
};

/// Create a test pattern with a rotating white line from the center
pub fn create_test_line_scene(width: usize, height: usize) -> SceneConfig {
    // Simple test: just output the angle as a gradient to verify CenterAngle works
    let program = vec![
        OpCode::Load(LoadSource::CenterAngle), // Load angle (0-1)
        OpCode::Load(LoadSource::TimeNorm),
        OpCode::Add,
        OpCode::Frac,
        OpCode::Return, // Return it as brightness
    ];

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
    // Demo program: perlin noise with 3 octaves, zoom, and cosine smoothing
    // Using the expression parser instead of manually building opcodes!
    let program = parse_expr("cos(perlin3(xNorm*0.3, yNorm*0.3, time, 3))");

    // Create palette
    let palette = Palette::rainbow();

    // Build pipeline configuration
    let pipeline_config = FxPipelineConfig::new(
        2, // Two buffers: 0=greyscale/temp, 1=RGB
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
            PipelineStep::BlurStep {
                input: BufferRef::new(1, BufferFormat::ImageRgb),
                output: BufferRef::new(0, BufferFormat::ImageRgb), // Reuse buffer 0
                radius: fixed_from_f32(0.2),                       // 0.2 pixel blur radius
            },
            PipelineStep::BlurStep {
                input: BufferRef::new(0, BufferFormat::ImageRgb),
                output: BufferRef::new(1, BufferFormat::ImageRgb), // Back to buffer 1
                radius: fixed_from_f32(0.1),                       // Second pass for smoother blur
            },
        ],
    );

    let mapping_config = MappingConfig::CircularPanel7Ring { width, height };

    SceneConfig::new(pipeline_config, mapping_config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::SceneRuntime;
    use crate::test_engine::{RuntimeOptions, FIXED_ONE};

    #[test]
    fn test_simple_white() {
        // First test: just output white for everything
        use crate::test_engine::execute_program;
        let input = vec![0; 16 * 16];
        let mut output = vec![0; 16 * 16];

        let program = vec![OpCode::Push(FIXED_ONE), OpCode::Return];

        execute_program(&input, &mut output, &program, 16, 16, 0);

        // All pixels should be white
        assert_eq!(output[0], FIXED_ONE, "First pixel should be white");
        assert_eq!(
            output[8 * 16],
            FIXED_ONE,
            "Row 8 first pixel should be white"
        );
    }

    #[test]
    fn test_yint_load() {
        // Test that YInt loads correctly
        use crate::test_engine::execute_program;
        let input = vec![0; 16 * 16];
        let mut output = vec![0; 16 * 16];

        let program = vec![OpCode::Load(LoadSource::YInt), OpCode::Return];

        execute_program(&input, &mut output, &program, 16, 16, 0);

        // Row 0 should have Y values of 0.5 in fixed-point
        println!(
            "Row 0, pixel 0: {:#x} (expected ~{:#x})",
            output[0],
            1 << 15
        );
        // Row 8 should have Y values of 8.5 in fixed-point
        println!(
            "Row 8, pixel 0: {:#x} (expected ~{:#x})",
            output[8 * 16],
            (8 << 16) + (1 << 15)
        );

        // Just verify it's not all zeros
        assert!(output[8 * 16] != 0, "Row 8 YInt should not be zero");
    }

    #[test]
    fn test_normalized_center_line() {
        // Test the normalized Y coordinate approach
        use crate::test_engine::execute_program;

        // Test with 16x16 - center should be row 8
        let input = vec![0; 16 * 16];
        let mut output = vec![0; 16 * 16];

        let center_min = fixed_from_f32(0.48);
        let center_max = fixed_from_f32(0.52);

        let program = vec![
            OpCode::Load(LoadSource::YNorm),
            OpCode::Push(center_min),
            OpCode::JumpLt(6),
            OpCode::Load(LoadSource::YNorm),
            OpCode::Push(center_max),
            OpCode::JumpGt(3),
            OpCode::Push(FIXED_ONE),
            OpCode::Return,
            OpCode::Push(0),
            OpCode::Return,
        ];

        execute_program(&input, &mut output, &program, 16, 16, 0);

        // Center row (row 8, Y=0.5) should be white
        assert_eq!(output[8 * 16], FIXED_ONE, "Center row should be white");
        // Rows far from center should be black
        assert_eq!(output[0], 0, "Top row should be black");
        assert_eq!(output[15 * 16], 0, "Bottom row should be black");

        // Test with 8x8 - center should be row 4
        let input8 = vec![0; 8 * 8];
        let mut output8 = vec![0; 8 * 8];
        execute_program(&input8, &mut output8, &program, 8, 8, 0);

        // Center row (row 4, Y=0.5) should be white
        assert_eq!(
            output8[4 * 8],
            FIXED_ONE,
            "Center row in 8x8 should be white"
        );
        assert_eq!(output8[0], 0, "Top row in 8x8 should be black");
    }

    #[test]
    fn test_horizontal_line_pattern() {
        // Test with 16x16
        let config = create_test_line_scene(16, 16);
        let options = RuntimeOptions::new(16, 16);
        let mut scene = SceneRuntime::new(config, options).expect("Valid config");

        scene.render(0, 1).expect("Render failed");
        let grey_buffer = scene.pipeline.get_buffer(0).expect("Buffer 0 should exist");

        // Center row (row 8) should be white, others black
        assert_eq!(
            grey_buffer.data[8 * 16],
            FIXED_ONE,
            "Center row should be white"
        );
        assert_eq!(grey_buffer.data[0], 0, "Top row should be black");
        assert_eq!(grey_buffer.data[15 * 16], 0, "Bottom row should be black");

        // Test with 8x8
        let config8 = create_test_line_scene(8, 8);
        let options8 = RuntimeOptions::new(8, 8);
        let mut scene8 = SceneRuntime::new(config8, options8).expect("Valid config");

        scene8.render(0, 1).expect("Render failed");
        let grey_buffer8 = scene8
            .pipeline
            .get_buffer(0)
            .expect("Buffer 0 should exist");

        // Center row (row 4) should be white
        assert_eq!(
            grey_buffer8.data[4 * 8],
            FIXED_ONE,
            "Center row in 8x8 should be white"
        );
        assert_eq!(grey_buffer8.data[0], 0, "Top row in 8x8 should be black");
    }
}
