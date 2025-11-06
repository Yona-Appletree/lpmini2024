pub mod mapping;
/// Test engine for pixel-based effects
///
/// This module provides a complete pipeline for generating LED effects:
/// - Palette-based RGB conversion
/// - 2D to 1D LED mapping
/// - Flexible pipeline system
pub mod palette;
pub mod pipeline;

#[cfg(test)]
mod mapping_tests;
#[cfg(test)]
mod pipeline_tests;

// Re-export commonly used items
// LoadSource is now defined in lpscript::vm::opcodes::load
pub use crate::lpscript::vm::opcodes::LoadSource;

#[allow(deprecated)]
pub use crate::math::{
    fixed_from_f32, fixed_from_int, fixed_to_f32, Fixed, FIXED_ONE, FIXED_SHIFT,
};
pub use mapping::{apply_2d_mapping, LedMapping, MappingConfig};
pub use palette::{rgb_buffer_from_greyscale, Palette};
pub use pipeline::{
    BufferFormat, BufferRef, FxPipeline, FxPipelineConfig, PipelineError, PipelineStep,
    RuntimeOptions,
};
