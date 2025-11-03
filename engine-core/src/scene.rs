/// Scene configuration and runtime system
extern crate alloc;
use alloc::vec::Vec;

use crate::test_engine::{
    FxPipelineConfig, MappingConfig, LedMapping, Fixed,
    RuntimeOptions, FxPipeline, PipelineError, apply_2d_mapping,
};

/// Scene configuration (serializable, no runtime state)
#[derive(Clone)]
pub struct SceneConfig {
    pub pipeline_config: FxPipelineConfig,
    pub mapping_config: MappingConfig,
}

impl SceneConfig {
    pub fn new(
        pipeline_config: FxPipelineConfig,
        mapping_config: MappingConfig,
    ) -> Self {
        SceneConfig {
            pipeline_config,
            mapping_config,
        }
    }
    
    /// Get the LED count from the mapping config
    pub fn led_count(&self) -> usize {
        self.mapping_config.led_count()
    }
}

/// Scene runtime state
pub struct SceneRuntime {
    pub pipeline: FxPipeline,
    pub mapping: LedMapping,
    pub led_output: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl SceneRuntime {
    /// Create a new scene runtime from config
    pub fn new(config: SceneConfig, options: RuntimeOptions) -> Result<Self, PipelineError> {
        let led_count = config.led_count();
        let pipeline = FxPipeline::new(config.pipeline_config, options)?;
        let mapping = config.mapping_config.build();
        let led_output = alloc::vec![0u8; led_count * 3];
        
        Ok(SceneRuntime {
            pipeline,
            mapping,
            led_output,
            width: options.width,
            height: options.height,
        })
    }
    
    /// Get the LED count
    pub fn led_count(&self) -> usize {
        self.led_output.len() / 3
    }
    
    /// Render a single frame
    pub fn render(&mut self, time: Fixed, output_buffer_idx: usize) -> Result<(), PipelineError> {
        // Render the pipeline
        self.pipeline.render(time)?;
        
        // Get RGB buffer and apply 2D to 1D mapping
        let rgb_bytes = self.pipeline.get_rgb_bytes(output_buffer_idx);
        apply_2d_mapping(&rgb_bytes, &mut self.led_output, &self.mapping, self.width, self.height);
        
        Ok(())
    }
}

