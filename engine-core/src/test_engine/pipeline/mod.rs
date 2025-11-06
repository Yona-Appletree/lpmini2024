/// Flexible pipeline system for LED effects
extern crate alloc;
use alloc::vec::Vec;

use crate::math::Fixed;
use super::palette::Palette;
use crate::power_limit::PowerLimitConfig;
use crate::lpscript::LpsProgram;

pub mod rgb_utils;
pub mod config;
pub mod runtime;
pub mod expr_step;

pub use rgb_utils::{pack_rgb, unpack_rgb, grey_to_i32, i32_to_grey};
pub use config::FxPipelineConfig;
pub use runtime::FxPipeline;
pub use expr_step::{execute_expr_step, validate_expr_program_type};

/// Buffer format identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferFormat {
    ImageGrey,  // Single greyscale value per pixel (stored in lower 32 bits)
    ImageRgb,   // RGB packed as 0x00RRGGBB
}

/// Reference to a buffer with expected format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferRef {
    pub buffer_idx: usize,
    pub format: BufferFormat,
}

impl BufferRef {
    pub const fn new(buffer_idx: usize, format: BufferFormat) -> Self {
        BufferRef { buffer_idx, format }
    }
}

/// Runtime buffer with format tracking
pub struct Buffer {
    pub data: Vec<i32>,
    pub last_format: BufferFormat,
}

impl Buffer {
    pub fn new(size: usize, format: BufferFormat) -> Self {
        Buffer {
            data: alloc::vec![0; size],
            last_format: format,
        }
    }
    
    pub fn set_format(&mut self, format: BufferFormat) {
        self.last_format = format;
    }
}

/// Pipeline execution step
#[derive(Clone)]
pub enum PipelineStep {
    /// Execute expression program
    ExprStep {
        program: LpsProgram,
        output: BufferRef,
        params: Vec<BufferRef>,
    },
    
    /// Apply palette to convert greyscale to RGB
    PaletteStep {
        input: BufferRef,
        output: BufferRef,
        palette: Palette,
    },
    
    /// Apply Gaussian blur
    BlurStep {
        input: BufferRef,
        output: BufferRef,
        radius: Fixed, // Blur radius in fixed-point (pixels)
    },
}

/// Pipeline validation and execution errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipelineError {
    InvalidBufferRef { buffer_idx: usize, num_buffers: usize },
    FormatMismatch { expected: BufferFormat, actual: BufferFormat },
    InvalidStep { step_idx: usize },
    TypeMismatch {
        expected: crate::lpscript::shared::Type,
        actual: crate::lpscript::shared::Type,
        context: alloc::string::String,
    },
    InvalidProgram(alloc::string::String),
    Unimplemented(alloc::string::String),
}

/// Runtime options for pipeline execution
#[derive(Debug, Clone, Copy)]
pub struct RuntimeOptions {
    pub width: usize,
    pub height: usize,
    pub power_config: PowerLimitConfig,
}

impl RuntimeOptions {
    pub const fn new(width: usize, height: usize) -> Self {
        RuntimeOptions { 
            width, 
            height,
            power_config: PowerLimitConfig {
                brightness_256: 256,
                power_budget_ma: 1000,
                led_white_power_ma: 50,
                led_idle_power_ma: 1,
            }
        }
    }
    
    pub const fn with_power_config(width: usize, height: usize, power_config: PowerLimitConfig) -> Self {
        RuntimeOptions { 
            width, 
            height,
            power_config,
        }
    }
}

