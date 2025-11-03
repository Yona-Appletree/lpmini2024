/// Pipeline runtime with buffer management and step execution
extern crate alloc;
use alloc::vec::Vec;

use super::{Buffer, BufferFormat, PipelineStep, PipelineError, RuntimeOptions, BufferRef};
use super::rgb_utils::{grey_to_i32, i32_to_grey, pack_rgb};
use super::config::FxPipelineConfig;
use super::super::vm::{Fixed, OpCode, execute_program};
use super::super::palette::Palette;

/// Runtime pipeline state
pub struct FxPipeline {
    pub buffers: Vec<Buffer>,
    steps: Vec<PipelineStep>,
    width: usize,
    height: usize,
}

impl FxPipeline {
    /// Create a new pipeline from config
    pub fn new(config: FxPipelineConfig, options: RuntimeOptions) -> Result<Self, PipelineError> {
        // Validate configuration
        config.validate()?;
        
        // Create buffers
        let buffer_size = options.width * options.height;
        let mut buffers = Vec::new();
        for _ in 0..config.num_buffers {
            buffers.push(Buffer::new(buffer_size, BufferFormat::ImageGrey));
        }
        
        Ok(FxPipeline {
            buffers,
            steps: config.steps,
            width: options.width,
            height: options.height,
        })
    }
    
    /// Render a frame by executing all pipeline steps
    pub fn render(&mut self, time: Fixed) -> Result<(), PipelineError> {
        // Clone steps to avoid borrow checker issues
        let steps = self.steps.clone();
        
        for (step_idx, step) in steps.iter().enumerate() {
            match step {
                PipelineStep::ExprStep { program, output, params } => {
                    self.execute_expr_step(program, output, params, time, step_idx)?;
                }
                
                PipelineStep::PaletteStep { input, output, palette } => {
                    self.execute_palette_step(input, output, palette, step_idx)?;
                }
            }
        }
        
        Ok(())
    }
    
    /// Execute an expression step
    fn execute_expr_step(
        &mut self,
        program: &[OpCode],
        output: &BufferRef,
        _params: &[BufferRef],  // TODO: implement param buffer support
        time: Fixed,
        _step_idx: usize,
    ) -> Result<(), PipelineError> {
        // For now, use empty input buffer (params not yet implemented)
        let input_buffer: Vec<Fixed> = alloc::vec![0; self.width * self.height];
        
        // Execute VM program into a temporary greyscale buffer
        let mut temp_grey: Vec<Fixed> = alloc::vec![0; self.width * self.height];
        execute_program(&input_buffer, &mut temp_grey, program, self.width, self.height, time);
        
        // Write greyscale results to output buffer
        let output_buf = &mut self.buffers[output.buffer_idx];
        for i in 0..temp_grey.len() {
            output_buf.data[i] = grey_to_i32(temp_grey[i]);
        }
        output_buf.set_format(BufferFormat::ImageGrey);
        
        Ok(())
    }
    
    /// Execute a palette conversion step
    fn execute_palette_step(
        &mut self,
        input: &BufferRef,
        output: &BufferRef,
        palette: &Palette,
        _step_idx: usize,
    ) -> Result<(), PipelineError> {
        // Validate input format at runtime
        let input_buf = &self.buffers[input.buffer_idx];
        if input_buf.last_format != BufferFormat::ImageGrey {
            return Err(PipelineError::FormatMismatch {
                expected: BufferFormat::ImageGrey,
                actual: input_buf.last_format,
            });
        }
        
        // Extract greyscale values
        let grey_values: Vec<Fixed> = input_buf.data.iter().map(|&v| i32_to_grey(v)).collect();
        
        // Apply palette to each pixel
        let output_buf = &mut self.buffers[output.buffer_idx];
        for i in 0..grey_values.len() {
            let rgb = palette.get_color(grey_values[i]);
            output_buf.data[i] = pack_rgb(rgb.r, rgb.g, rgb.b);
        }
        output_buf.set_format(BufferFormat::ImageRgb);
        
        Ok(())
    }
    
    /// Get a buffer by index
    pub fn get_buffer(&self, idx: usize) -> Option<&Buffer> {
        self.buffers.get(idx)
    }
    
    /// Get RGB buffer as byte slice for mapping (extracts from i32 packed format)
    /// This allocates - prefer extract_rgb_bytes() for performance
    pub fn get_rgb_bytes(&self, buffer_idx: usize) -> Vec<u8> {
        if let Some(buf) = self.buffers.get(buffer_idx) {
            let mut rgb_bytes = alloc::vec![0u8; buf.data.len() * 3];
            for (i, &packed) in buf.data.iter().enumerate() {
                let (r, g, b) = super::rgb_utils::unpack_rgb(packed);
                rgb_bytes[i * 3] = r;
                rgb_bytes[i * 3 + 1] = g;
                rgb_bytes[i * 3 + 2] = b;
            }
            rgb_bytes
        } else {
            alloc::vec![]
        }
    }
    
    /// Extract RGB buffer into provided slice (no allocation)
    pub fn extract_rgb_bytes(&self, buffer_idx: usize, output: &mut [u8]) {
        if let Some(buf) = self.buffers.get(buffer_idx) {
            for (i, &packed) in buf.data.iter().enumerate() {
                if i * 3 + 2 < output.len() {
                    let (r, g, b) = super::rgb_utils::unpack_rgb(packed);
                    output[i * 3] = r;
                    output[i * 3 + 1] = g;
                    output[i * 3 + 2] = b;
                }
            }
        }
    }
    
    /// Get greyscale buffer as Fixed slice for visualization
    pub fn get_greyscale_fixed(&self, buffer_idx: usize) -> Vec<Fixed> {
        if let Some(buf) = self.buffers.get(buffer_idx) {
            buf.data.iter().map(|&v| i32_to_grey(v)).collect()
        } else {
            alloc::vec![]
        }
    }
}

