extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

use crate::test_engine::{
    OpCode, LoadSource, fixed_from_f32, Palette, BufferFormat, BufferRef,
    PipelineStep, FxPipelineConfig, MappingConfig,
};
use crate::scene::SceneConfig;

/// Create a test pattern with a horizontal white line in the center
pub fn create_test_line_scene(width: usize, height: usize) -> SceneConfig {
    // YInt returns the integer pixel coordinate in fixed-point
    // For row 8, it returns 8.0 (0x80000), not 8.5
    let center_y = fixed_from_f32(8.0);  // Row 8
    
    let program = vec![
        OpCode::Load(LoadSource::YInt),           // 0: Get Y in fixed-point (0.0, 1.0, 2.0, ...)
        OpCode::Push(center_y),                   // 1: Push 8.0
        OpCode::JumpEq(3),                        // 2: If Y == 8.0, jump +3 to index 5
        OpCode::Push(0),                          // 3: Otherwise black
        OpCode::Return,                           // 4
        OpCode::Push(fixed_from_f32(1.0)),        // 5: White
        OpCode::Return,                           // 6
    ];

    // Grayscale palette (white = white, black = black)
    let palette = Palette::grayscale();

    let pipeline_config = FxPipelineConfig::new(
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
                palette,
            },
        ],
    );

    let mapping_config = MappingConfig::CircularPanel7Ring { width, height };

    SceneConfig::new(pipeline_config, mapping_config)
}

/// Create the standard demo scene configuration
pub fn create_demo_scene(width: usize, height: usize) -> SceneConfig {
    // Demo program: perlin noise with 3 octaves, zoom, and cosine smoothing
    let program = vec![
        OpCode::Load(LoadSource::XNorm),   // Normalized x (0..1)
        OpCode::Push(fixed_from_f32(0.3)), // Zoom factor
        OpCode::Mul,                       // Scale x down
        OpCode::Load(LoadSource::YNorm),   // Normalized y (0..1)
        OpCode::Push(fixed_from_f32(0.3)), // Zoom factor
        OpCode::Mul,                       // Scale y down
        OpCode::Load(LoadSource::Time),    // Time (scrolls the z-axis)
        OpCode::Perlin3(3),                // Generate perlin noise with 3 octaves
        OpCode::Cos,                       // Apply cosine (outputs 0..1)
        OpCode::Return,
    ];

    // Create palette
    let palette = Palette::rainbow();

    // Build pipeline configuration
    let pipeline_config = FxPipelineConfig::new(
        2, // Two buffers: 0=greyscale, 1=RGB
        vec![
            PipelineStep::ExprStep {
                program,
                output: BufferRef::new(0, BufferFormat::ImageGrey),
                params: vec![],
            },
            PipelineStep::PaletteStep {
                input: BufferRef::new(0, BufferFormat::ImageGrey),
                output: BufferRef::new(1, BufferFormat::ImageRgb),
                palette,
            },
        ],
    );

    let mapping_config = MappingConfig::CircularPanel7Ring { width, height };

    SceneConfig::new(pipeline_config, mapping_config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_engine::{Fixed, RuntimeOptions, FIXED_ONE};
    use crate::scene::SceneRuntime;

    #[test]
    fn test_simple_white() {
        // First test: just output white for everything
        use crate::test_engine::execute_program;
        let input = vec![0; 16 * 16];
        let mut output = vec![0; 16 * 16];
        
        let program = vec![
            OpCode::Push(FIXED_ONE),
            OpCode::Return,
        ];
        
        execute_program(&input, &mut output, &program, 16, 16, 0);
        
        // All pixels should be white
        assert_eq!(output[0], FIXED_ONE, "First pixel should be white");
        assert_eq!(output[8 * 16], FIXED_ONE, "Row 8 first pixel should be white");
    }

    #[test]
    fn test_yint_load() {
        // Test that YInt loads correctly
        use crate::test_engine::execute_program;
        let input = vec![0; 16 * 16];
        let mut output = vec![0; 16 * 16];
        
        let program = vec![
            OpCode::Load(LoadSource::YInt),
            OpCode::Return,
        ];
        
        execute_program(&input, &mut output, &program, 16, 16, 0);
        
        // Row 0 should have Y values of 0.5 in fixed-point
        println!("Row 0, pixel 0: {:#x} (expected ~{:#x})", output[0], 1 << 15);
        // Row 8 should have Y values of 8.5 in fixed-point  
        println!("Row 8, pixel 0: {:#x} (expected ~{:#x})", output[8 * 16], (8 << 16) + (1 << 15));
        
        // Just verify it's not all zeros
        assert!(output[8 * 16] != 0, "Row 8 YInt should not be zero");
    }
    
    #[test]
    fn test_jumpeq_directly() {
        // Test the exact program for row 8
        use crate::test_engine::execute_program;
        let input = vec![0; 16 * 16];
        let mut output = vec![0; 16 * 16];
        
        let center_y = fixed_from_f32(8.0);
        println!("Center Y value: {:#x}", center_y);
        
        let program = vec![
            OpCode::Load(LoadSource::YInt),       // 0
            OpCode::Push(center_y),               // 1
            OpCode::JumpEq(3),                    // 2: Jump +3 to index 5 if equal
            OpCode::Push(0),                      // 3: black
            OpCode::Return,                       // 4
            OpCode::Push(FIXED_ONE),              // 5: white
            OpCode::Return,                       // 6
        ];
        
        println!("Program has {} instructions", program.len());
        
        execute_program(&input, &mut output, &program, 16, 16, 0);
        
        println!("Row 8, Y should be {:#x}", 8 << 16);
        println!("Row 8 output: {:#x} (expected {:#x})", output[8 * 16], FIXED_ONE);
        println!("Row 7 output: {:#x} (expected 0)", output[7 * 16]);
        println!("Row 9 output: {:#x} (expected 0)", output[9 * 16]);
        
        assert_eq!(output[8 * 16], FIXED_ONE, "Row 8 should be white");
        assert_eq!(output[7 * 16], 0, "Row 7 should be black");
    }
    
    #[test]
    fn test_horizontal_line_pattern() {
        // Create a 16x16 test line scene
        let config = create_test_line_scene(16, 16);
        let options = RuntimeOptions::new(16, 16);
        let mut scene = SceneRuntime::new(config, options).expect("Valid config");
        
        // Render at time 0
        scene.render(0, 1).expect("Render failed");
        
        // Get the grayscale buffer
        let grey_buffer = scene.pipeline.get_buffer(0).expect("Buffer 0 should exist");
        
        // Debug: print some values to understand what's happening
        println!("Buffer data for row 8:");
        for x in 0..3 {
            let idx = 8 * 16 + x;
            println!("  Pixel ({}, 8) = {:#x} (expected {:#x})", x, grey_buffer.data[idx], FIXED_ONE);
        }
        
        // Check that only row 8 (center) has white pixels
        for y in 0..16 {
            for x in 0..16 {
                let idx = y * 16 + x;
                let value = grey_buffer.data[idx];
                
                if y == 8 {
                    // Center row should be white (FIXED_ONE)
                    assert_eq!(value, FIXED_ONE, "Pixel at ({}, {}) should be white", x, y);
                } else {
                    // All other rows should be black (0)
                    assert_eq!(value, 0, "Pixel at ({}, {}) should be black", x, y);
                }
            }
        }
    }
}
