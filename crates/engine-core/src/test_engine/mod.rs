pub mod mapping;
/// Test engine for pixel-based effects
///
/// This module provides a complete pipeline for generating LED effects:
/// - Palette-based RGB conversion
/// - 2D to 1D LED mapping
/// - Flexible pipeline system
pub mod palette;
pub mod pipeline;

/// Demo program configuration
pub mod demo_program;
#[cfg(test)]
mod mapping_tests;
#[cfg(test)]
mod pipeline_tests;
/// Scene configuration and runtime system
pub mod scene;
/// Standard test scene shared between ESP32 and host
pub mod test_scene;

/// Power limiting and brightness control
pub mod power_limit;

// Re-export commonly used items
// LoadSource is now defined in lp-gfx::vm::opcodes::load
#[allow(deprecated)]
pub use lp_gfx::lp_script::dec32::{fixed_from_f32, fixed_from_int, fixed_to_f32, Dec32};
pub use lp_gfx::lp_script::vm::opcodes::load::LoadSource;
pub use mapping::{apply_2d_mapping, LedMapping, MappingConfig};
pub use palette::{rgb_buffer_from_greyscale, Palette};
pub use pipeline::{
    BufferFormat, BufferRef, FxPipeline, FxPipelineConfig, PipelineError, PipelineStep,
    RuntimeOptions,
};
