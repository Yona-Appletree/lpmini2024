/// Tests for the rendering pipeline
#[cfg(test)]
mod pipeline_tests {
    use crate::test_engine::{
        FxPipelineConfig, PipelineStep, BufferRef, BufferFormat, 
        OpCode, Palette, RuntimeOptions, FxPipeline,
    };
    use crate::math::{Fixed, ToFixed};
    
    #[test]
    fn test_simple_pipeline() {
        // Create a simple pipeline: generate white everywhere
        let program = vec![
            OpCode::Push(Fixed::ONE),
            OpCode::Return,
        ];
        
        let config = FxPipelineConfig::new(
            2,
            vec![
                PipelineStep::ExprStep {
                    program,
                    output: BufferRef::new(0, BufferFormat::ImageGrey),
                    params: vec![],
                },
            ],
        );
        
        let options = RuntimeOptions::new(4, 4);
        let mut pipeline = FxPipeline::new(config, options).expect("Valid config");
        
        pipeline.render(Fixed::ZERO).expect("Render should succeed");
        
        // Check buffer 0 has white values
        let buffer = pipeline.get_buffer(0).expect("Buffer should exist");
        for (i, &val) in buffer.data.iter().enumerate() {
            let f = Fixed(val).to_f32();
            assert!((f - 1.0).abs() < 0.01, 
                   "Pixel {} should be ~1.0, got {}", i, f);
        }
    }
    
    #[test]
    fn test_palette_step() {
        // Test that palette conversion works
        use crate::test_engine::vm::LoadSource;
        let program = vec![
            OpCode::Load(LoadSource::XNorm),
            OpCode::Return,
        ];
        
        let config = FxPipelineConfig::new(
            2,
            vec![
                PipelineStep::ExprStep {
                    program,
                    output: BufferRef::new(0, BufferFormat::ImageGrey),
                    params: vec![],
                },
                PipelineStep::PaletteStep {
                    input: BufferRef::new(0, BufferFormat::ImageGrey),
                    output: BufferRef::new(1, BufferFormat::ImageRgb),
                    palette: Palette::rainbow(),
                },
            ],
        );
        
        let options = RuntimeOptions::new(8, 8);
        let mut pipeline = FxPipeline::new(config, options).expect("Valid config");
        
        pipeline.render(Fixed::ZERO).expect("Render should succeed");
        
        // Buffer 1 should be RGB format
        let buffer = pipeline.get_buffer(1).expect("Buffer 1 should exist");
        assert_eq!(buffer.last_format, BufferFormat::ImageRgb);
        
        // Should have variation across x
        let first = buffer.data[0];
        let last = buffer.data[7];
        assert_ne!(first, last, "X gradient should produce different colors at edges");
    }
    
    #[test]
    fn test_extract_rgb_bytes() {
        let program = vec![
            OpCode::Push(0.5f32.to_fixed()),
            OpCode::Return,
        ];
        
        let config = FxPipelineConfig::new(
            2,
            vec![
                PipelineStep::ExprStep {
                    program,
                    output: BufferRef::new(0, BufferFormat::ImageGrey),
                    params: vec![],
                },
                PipelineStep::PaletteStep {
                    input: BufferRef::new(0, BufferFormat::ImageGrey),
                    output: BufferRef::new(1, BufferFormat::ImageRgb),
                    palette: Palette::grayscale(),
                },
            ],
        );
        
        let options = RuntimeOptions::new(4, 4);
        let mut pipeline = FxPipeline::new(config, options).expect("Valid config");
        pipeline.render(Fixed::ZERO).expect("Render should succeed");
        
        let mut rgb_bytes = vec![0u8; 4 * 4 * 3];
        pipeline.extract_rgb_bytes(1, &mut rgb_bytes);
        
        // All RGB bytes should be around 128 (0.5 * 255)
        for (i, &byte) in rgb_bytes.iter().enumerate() {
            assert!(byte > 50 && byte < 200, 
                   "Byte {} should be around 128, got {}", i, byte);
        }
    }
}

