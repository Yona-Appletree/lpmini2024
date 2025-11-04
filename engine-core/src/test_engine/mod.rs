/// Test engine for pixel-based effects
/// 
/// This module provides a complete pipeline for generating LED effects:
/// - Palette-based RGB conversion
/// - 2D to 1D LED mapping
/// - Flexible pipeline system

pub mod palette;
pub mod mapping;
pub mod pipeline;

#[cfg(test)]
mod mapping_tests;
#[cfg(test)]
mod pipeline_tests;

// Re-export commonly used items
// LoadSource is still defined in the old vm module and used by lpscript
// TODO: Move LoadSource to a better location
mod vm;
pub use vm::LoadSource;

#[allow(deprecated)]
pub use crate::math::{Fixed, FIXED_SHIFT, FIXED_ONE, fixed_from_f32, fixed_to_f32, fixed_from_int};
pub use palette::{Palette, rgb_buffer_from_greyscale};
pub use mapping::{LedMapping, MappingConfig, apply_2d_mapping};
pub use pipeline::{FxPipeline, FxPipelineConfig, BufferFormat, BufferRef, PipelineStep, PipelineError, RuntimeOptions};

