/// Test engine for pixel-based effects
/// 
/// This module provides a complete pipeline for generating LED effects:
/// - Stack-based VM for pixel operations
/// - Palette-based RGB conversion
/// - 2D to 1D LED mapping
/// - Flexible pipeline system

pub mod vm;
pub mod palette;
pub mod mapping;
pub mod pipeline;

#[cfg(test)]
mod vm_tests;
#[cfg(test)]
mod mapping_tests;
#[cfg(test)]
mod pipeline_tests;

// Re-export commonly used items
pub use vm::{OpCode, LoadSource, execute_program};
#[allow(deprecated)]
pub use crate::math::{Fixed, FIXED_SHIFT, FIXED_ONE, fixed_from_f32, fixed_to_f32, fixed_from_int};
pub use palette::{Palette, rgb_buffer_from_greyscale};
pub use mapping::{LedMapping, MappingConfig, apply_2d_mapping};
pub use pipeline::{FxPipeline, FxPipelineConfig, BufferFormat, BufferRef, PipelineStep, PipelineError, RuntimeOptions};

