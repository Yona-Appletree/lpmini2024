/// Test engine for pixel-based effects
/// 
/// This module provides a complete pipeline for generating LED effects:
/// - Stack-based VM for pixel operations
/// - Palette-based RGB conversion
/// - 2D to 1D LED mapping

pub mod vm;
pub mod palette;
pub mod mapping;
pub mod render;

// Re-export commonly used items
pub use vm::{OpCode, LoadSource, Fixed, FIXED_SHIFT, FIXED_ONE, fixed_from_f32, fixed_to_f32, execute_program};
pub use palette::{Palette, rgb_buffer_from_greyscale};
pub use mapping::{LedMapping, apply_2d_mapping};
pub use render::render_frame;

