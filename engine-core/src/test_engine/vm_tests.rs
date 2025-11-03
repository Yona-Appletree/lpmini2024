/// Comprehensive VM opcode tests
#[cfg(test)]
mod vm_opcode_tests {
    use crate::test_engine::vm::{OpCode, LoadSource, execute_program, fixed_from_f32, fixed_to_f32, fixed_from_int};
    use crate::math::FIXED_ONE;
    
    #[test]
    fn test_sin_output_range() {
        // VM sin/cos map -1..1 to 0..1 for use with palettes
        let input = vec![0; 1];
        let mut output = vec![0; 1];
        let width = 1;
        let height = 1;
        
        // sin(0) = 0 in -1..1, maps to 0.5 in 0..1
        let program = vec![
            OpCode::Push(0),
            OpCode::Sin,
            OpCode::Return,
        ];
        execute_program(&input, &mut output, &program, width, height, 0);
        let result = fixed_to_f32(output[0]);
        assert!((result - 0.5).abs() < 0.01, "sin(0) should map to ~0.5, got {}", result);
        
        // sin(0.25) = 1.0 in -1..1, maps to 1.0 in 0..1
        let program = vec![
            OpCode::Push(fixed_from_f32(0.25)),
            OpCode::Sin,
            OpCode::Return,
        ];
        execute_program(&input, &mut output, &program, width, height, 0);
        let result = fixed_to_f32(output[0]);
        assert!((result - 1.0).abs() < 0.02, "sin(0.25) should map to ~1.0, got {}", result);
    }
    
    #[test]
    fn test_cos_output_range() {
        let input = vec![0; 1];
        let mut output = vec![0; 1];
        
        // cos(0) should be 1.0
        let program = vec![
            OpCode::Push(0),
            OpCode::Cos,
            OpCode::Return,
        ];
        execute_program(&input, &mut output, &program, 1, 1, 0);
        let result = fixed_to_f32(output[0]);
        assert!((result - 1.0).abs() < 0.02, "cos(0) should be ~1.0, got {}", result);
    }
    
    #[test]
    fn test_perlin3_output_range() {
        let input = vec![0; 1];
        let mut output = vec![0; 1];
        
        // perlin3 should output normalized values
        let program = vec![
            OpCode::Push(fixed_from_f32(0.5)),
            OpCode::Push(fixed_from_f32(0.5)),
            OpCode::Push(0),
            OpCode::Perlin3(3),
            OpCode::Return,
        ];
        execute_program(&input, &mut output, &program, 1, 1, 0);
        let result = fixed_to_f32(output[0]);
        println!("perlin3(0.5, 0.5, 0, 3) = {}", result);
        // Perlin should be in a reasonable range
        assert!(result >= -2.0 && result <= 2.0, "perlin3 output {} out of expected range", result);
    }
    
    #[test]
    fn test_mul_div() {
        let input = vec![0; 1];
        let mut output = vec![0; 1];
        
        // 2 / 3 should be ~0.666
        let program = vec![
            OpCode::Push(fixed_from_int(2)),
            OpCode::Push(fixed_from_int(3)),
            OpCode::Div,
            OpCode::Return,
        ];
        execute_program(&input, &mut output, &program, 1, 1, 0);
        let result = fixed_to_f32(output[0]);
        assert!((result - 0.666).abs() < 0.01, "2/3 should be ~0.666, got {}", result);
        
        // 2 * 3 should be 6
        let program = vec![
            OpCode::Push(fixed_from_int(2)),
            OpCode::Push(fixed_from_int(3)),
            OpCode::Mul,
            OpCode::Return,
        ];
        execute_program(&input, &mut output, &program, 1, 1, 0);
        let result = fixed_to_f32(output[0]);
        assert!((result - 6.0).abs() < 0.01, "2*3 should be 6, got {}", result);
    }
    
    #[test]
    fn test_demo_expression_output() {
        // Test the actual demo expression: cos(perlin3(xNorm*0.3, yNorm*0.3, time, 3))
        use crate::expr::parse_expr;
        
        let input = vec![0; 16 * 16];
        let mut output = vec![0; 16 * 16];
        
        let program = parse_expr("cos(perlin3(xNorm*0.3, yNorm*0.3, time, 3))");
        
        execute_program(&input, &mut output, &program, 16, 16, 0);
        
        // Check that we get varied output (not all the same value)
        let first = output[0];
        let has_variation = output.iter().any(|&v| (v - first).abs() > 1000);
        assert!(has_variation, "Demo expression should produce varied output, got all ~{}", fixed_to_f32(first));
        
        // Check that values are in a reasonable range for greyscale (we need 0..1 for palette)
        for (i, &val) in output.iter().enumerate() {
            let f = fixed_to_f32(val);
            assert!(f >= -2.0 && f <= 2.0, 
                "Pixel {} has value {} which is out of range", i, f);
        }
    }
}

