/// Pipeline configuration with validation
extern crate alloc;
use alloc::vec::Vec;

use super::{BufferFormat, PipelineError, PipelineStep};

/// Pipeline configuration
#[derive(Clone)]
pub struct FxPipelineConfig {
    pub num_buffers: usize,
    pub steps: Vec<PipelineStep>,
}

impl FxPipelineConfig {
    pub fn new(num_buffers: usize, steps: Vec<PipelineStep>) -> Self {
        FxPipelineConfig { num_buffers, steps }
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), PipelineError> {
        for (_step_idx, step) in self.steps.iter().enumerate() {
            match step {
                PipelineStep::ExprStep { output, params, .. } => {
                    // Validate output buffer
                    if output.buffer_idx >= self.num_buffers {
                        return Err(PipelineError::InvalidBufferRef {
                            buffer_idx: output.buffer_idx,
                            num_buffers: self.num_buffers,
                        });
                    }

                    // Validate param buffers
                    for param in params {
                        if param.buffer_idx >= self.num_buffers {
                            return Err(PipelineError::InvalidBufferRef {
                                buffer_idx: param.buffer_idx,
                                num_buffers: self.num_buffers,
                            });
                        }
                    }
                }

                PipelineStep::PaletteStep { input, output, .. } => {
                    // Validate input buffer
                    if input.buffer_idx >= self.num_buffers {
                        return Err(PipelineError::InvalidBufferRef {
                            buffer_idx: input.buffer_idx,
                            num_buffers: self.num_buffers,
                        });
                    }

                    // Validate output buffer
                    if output.buffer_idx >= self.num_buffers {
                        return Err(PipelineError::InvalidBufferRef {
                            buffer_idx: output.buffer_idx,
                            num_buffers: self.num_buffers,
                        });
                    }

                    // Validate format compatibility (palette needs greyscale input)
                    if input.format != BufferFormat::ImageGrey {
                        return Err(PipelineError::FormatMismatch {
                            expected: BufferFormat::ImageGrey,
                            actual: input.format,
                        });
                    }
                }

                PipelineStep::BlurStep { input, output, .. } => {
                    // Validate input buffer
                    if input.buffer_idx >= self.num_buffers {
                        return Err(PipelineError::InvalidBufferRef {
                            buffer_idx: input.buffer_idx,
                            num_buffers: self.num_buffers,
                        });
                    }

                    // Validate output buffer
                    if output.buffer_idx >= self.num_buffers {
                        return Err(PipelineError::InvalidBufferRef {
                            buffer_idx: output.buffer_idx,
                            num_buffers: self.num_buffers,
                        });
                    }

                    // Blur accepts any format (no format validation needed)
                }
            }
        }

        Ok(())
    }
}
