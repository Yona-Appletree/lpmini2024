/// Pipeline runtime with buffer management and step execution
extern crate alloc;
use alloc::vec::Vec;

use super::super::palette::Palette;
use super::config::FxPipelineConfig;
use super::rgb_utils::{i32_to_grey, pack_rgb};
use super::{Buffer, BufferFormat, BufferRef, PipelineError, PipelineStep, RuntimeOptions};
use lp_script::fixed::Fixed;

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
                PipelineStep::ExprStep {
                    program,
                    output,
                    params,
                } => {
                    self.execute_expr_step(program, output, params, time, step_idx)?;
                }

                PipelineStep::PaletteStep {
                    input,
                    output,
                    palette,
                } => {
                    self.execute_palette_step(input, output, palette, step_idx)?;
                }

                PipelineStep::BlurStep {
                    input,
                    output,
                    radius,
                } => {
                    self.execute_blur_step(input, output, *radius, step_idx)?;
                }
            }
        }

        Ok(())
    }

    /// Execute an expression step with type validation
    fn execute_expr_step(
        &mut self,
        program: &crate::lp_script::LpsProgram,
        output: &BufferRef,
        _params: &[BufferRef], // TODO: implement param buffer support
        time: Fixed,
        _step_idx: usize,
    ) -> Result<(), PipelineError> {
        let output_buf = &mut self.buffers[output.buffer_idx];

        // Use the new execute_expr_step from expr_step module
        super::expr_step::execute_expr_step(
            program,
            &mut output_buf.data,
            output.format,
            self.width,
            self.height,
            time,
        )?;

        // Update buffer format
        output_buf.set_format(output.format);

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
        for (i, &grey_val) in grey_values.iter().enumerate() {
            let rgb = palette.get_color(grey_val);
            output_buf.data[i] = pack_rgb(rgb.r, rgb.g, rgb.b);
        }
        output_buf.set_format(BufferFormat::ImageRgb);

        Ok(())
    }

    /// Execute a blur step (box blur approximation)
    fn execute_blur_step(
        &mut self,
        input: &BufferRef,
        output: &BufferRef,
        radius: Fixed,
        _step_idx: usize,
    ) -> Result<(), PipelineError> {
        let input_buf = &self.buffers[input.buffer_idx];
        let format = input_buf.last_format;

        // Clone input data for reading
        let input_data = input_buf.data.clone();

        // Convert radius from fixed-point to pixel radius (relative to image size)
        // radius is a fraction (e.g., 0.2 = 20% of image dimension)
        // Multiply by average dimension to get absolute pixels
        let avg_dimension = (self.width + self.height) / 2;
        let radius_pixels_fp = (radius.0 as i64 * avg_dimension as i64) >> 16; // Fixed-point multiply
        let radius_pixels = radius_pixels_fp.max(1) as usize; // Clamp to at least 1 pixel

        // Box blur (faster than Gaussian for embedded)
        let output_buf = &mut self.buffers[output.buffer_idx];

        match format {
            BufferFormat::ImageRgb => {
                // Blur RGB channels separately
                for y in 0..self.height {
                    for x in 0..self.width {
                        let mut sum_r = 0i64;
                        let mut sum_g = 0i64;
                        let mut sum_b = 0i64;
                        let mut count = 0i64;

                        // Sample within blur radius
                        for ky in -(radius_pixels as isize)..=(radius_pixels as isize) {
                            for kx in -(radius_pixels as isize)..=(radius_pixels as isize) {
                                let sx =
                                    (x as isize + kx).max(0).min(self.width as isize - 1) as usize;
                                let sy =
                                    (y as isize + ky).max(0).min(self.height as isize - 1) as usize;
                                let idx = sy * self.width + sx;

                                let (r, g, b) = super::rgb_utils::unpack_rgb(input_data[idx]);
                                sum_r += r as i64;
                                sum_g += g as i64;
                                sum_b += b as i64;
                                count += 1;
                            }
                        }

                        let avg_r = (sum_r / count) as u8;
                        let avg_g = (sum_g / count) as u8;
                        let avg_b = (sum_b / count) as u8;

                        let idx = y * self.width + x;
                        output_buf.data[idx] = pack_rgb(avg_r, avg_g, avg_b);
                    }
                }
            }
            BufferFormat::ImageGrey => {
                // Blur greyscale
                for y in 0..self.height {
                    for x in 0..self.width {
                        let mut sum = 0i64;
                        let mut count = 0i64;

                        for ky in -(radius_pixels as isize)..=(radius_pixels as isize) {
                            for kx in -(radius_pixels as isize)..=(radius_pixels as isize) {
                                let sx =
                                    (x as isize + kx).max(0).min(self.width as isize - 1) as usize;
                                let sy =
                                    (y as isize + ky).max(0).min(self.height as isize - 1) as usize;
                                let idx = sy * self.width + sx;

                                sum += input_data[idx] as i64;
                                count += 1;
                            }
                        }

                        let idx = y * self.width + x;
                        output_buf.data[idx] = (sum / count) as i32;
                    }
                }
            }
        }

        output_buf.set_format(format);
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
