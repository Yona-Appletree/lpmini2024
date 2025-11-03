/// Test scene - shared between ESP32 and host
/// This defines the standard test program and scene configuration
extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

use crate::test_engine::{
    apply_2d_mapping, fixed_from_f32, BufferFormat, BufferRef, Fixed, FxPipeline, FxPipelineConfig,
    LedMapping, LoadSource, OpCode, Palette, PipelineStep, RuntimeOptions,
};

pub const WIDTH: usize = 16;
pub const HEIGHT: usize = 16;
pub const LED_COUNT: usize = 128;

/// Scene runtime state
pub struct SceneData {
    pub pipeline: FxPipeline,
    pub mapping: LedMapping,
    pub led_output: Vec<u8>,
}

impl SceneData {
    /// Create a new scene with the standard test configuration
    pub fn new() -> Self {
        // Create palette and mapping
        let palette = Palette::rainbow();
        let mapping = LedMapping::circular_panel_7ring(WIDTH, HEIGHT);

        // Test program: perlin noise with 3 octaves, zoom, and cosine smoothing
        let program = vec![
            OpCode::Load(LoadSource::XNorm),   // Normalized x (0..1)
            OpCode::Push(fixed_from_f32(0.3)), // Zoom factor
            OpCode::Mul,                       // Scale x down
            OpCode::Load(LoadSource::YNorm),   // Normalized y (0..1)
            OpCode::Push(fixed_from_f32(0.3)), // Zoom factor
            OpCode::Mul,                       // Scale y down
            OpCode::Load(LoadSource::Time),    // Time (scrolls the z-axis)
            OpCode::Perlin3(3),                // Generate perlin noise with 3 octaves
            OpCode::Cos,                       // Apply cosine (outputs 0..1)
            OpCode::Return,
        ];

        // Build pipeline configuration
        let config = FxPipelineConfig::new(
            2, // Two buffers: 0=greyscale, 1=RGB
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

        let options = RuntimeOptions::new(WIDTH, HEIGHT);
        let pipeline = FxPipeline::new(config, options).expect("Valid pipeline config");

        SceneData {
            pipeline,
            mapping,
            led_output: vec![0u8; LED_COUNT * 3],
        }
    }
}

/// Render a single frame of the test scene
#[inline(never)]
pub fn render_test_scene(scene: &mut SceneData, time: Fixed) {
    // Render the pipeline (generates greyscale in buffer 0, RGB in buffer 1)
    scene.pipeline.render(time).expect("Pipeline render failed");

    // Get RGB buffer 1 as bytes and apply 2D to 1D mapping
    let rgb_bytes = scene.pipeline.get_rgb_bytes(1);
    apply_2d_mapping(
        &rgb_bytes,
        &mut scene.led_output,
        &scene.mapping,
        WIDTH,
        HEIGHT,
    );
}
