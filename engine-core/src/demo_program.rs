/// Demo program configuration
extern crate alloc;
use alloc::vec;

use crate::test_engine::{
    OpCode, LoadSource, fixed_from_f32, Palette, BufferFormat, BufferRef,
    PipelineStep, FxPipelineConfig, MappingConfig,
};
use crate::scene::SceneConfig;

/// Create the standard demo scene configuration
pub fn create_demo_scene(width: usize, height: usize, led_count: usize) -> SceneConfig {
    // Demo program: perlin noise with 3 octaves, zoom, and cosine smoothing
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

    let palette = Palette::rainbow();

    // Build pipeline configuration
    let pipeline_config = FxPipelineConfig::new(
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

    let mapping_config = MappingConfig::CircularPanel7Ring { width, height };

    SceneConfig::new(pipeline_config, mapping_config, led_count)
}

