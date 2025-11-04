/// Comprehensive VM opcode tests
#[cfg(test)]
mod vm_opcode_tests {
    use crate::math::{Fixed, ToFixed};
    use crate::test_engine::vm::{execute_program, OpCode};

    #[test]
    fn test_sin_output_range() {
        // VM sin/cos now use radians (GLSL-compatible)
        // Output is -1..1, which maps to 0..1 for use with palettes
        let input = vec![Fixed::ZERO; 1];
        let mut output = vec![Fixed::ZERO; 1];
        let width = 1;
        let height = 1;

        // sin(0) = 0 in -1..1, maps to 0.5 in 0..1
        let program = vec![OpCode::Push(Fixed::ZERO), OpCode::Sin, OpCode::Return];
        execute_program(&input, &mut output, &program, width, height, Fixed::ZERO);
        let result = output[0].to_f32();
        assert!(
            (result - 0.5).abs() < 0.03,
            "sin(0) should map to ~0.5, got {}",
            result
        );

        // sin(π/2) = 1.0 in -1..1, maps to 1.0 in 0..1
        let program = vec![
            OpCode::Push(Fixed::PI / Fixed::from_i32(2)),
            OpCode::Sin,
            OpCode::Return,
        ];
        execute_program(&input, &mut output, &program, width, height, Fixed::ZERO);
        let result = output[0].to_f32();
        assert!(
            (result - 1.0).abs() < 0.03,
            "sin(π/2) should map to ~1.0, got {}",
            result
        );
    }

    #[test]
    fn test_cos_output_range() {
        let input = vec![Fixed::ZERO; 1];
        let mut output = vec![Fixed::ZERO; 1];

        // cos(0) should be 1.0
        let program = vec![OpCode::Push(Fixed::ZERO), OpCode::Cos, OpCode::Return];
        execute_program(&input, &mut output, &program, 1, 1, Fixed::ZERO);
        let result = output[0].to_f32();
        assert!(
            (result - 1.0).abs() < 0.02,
            "cos(0) should be ~1.0, got {}",
            result
        );
    }

    #[test]
    fn test_perlin3_output_range() {
        let input = vec![Fixed::ZERO; 1];
        let mut output = vec![Fixed::ZERO; 1];

        // perlin3 should output normalized values
        let program = vec![
            OpCode::Push(0.5f32.to_fixed()),
            OpCode::Push(0.5f32.to_fixed()),
            OpCode::Push(Fixed::ZERO),
            OpCode::Perlin3(3),
            OpCode::Return,
        ];
        execute_program(&input, &mut output, &program, 1, 1, Fixed::ZERO);
        let result = output[0].to_f32();
        println!("perlin3(0.5, 0.5, 0, 3) = {}", result);
        // Perlin should be in a reasonable range
        assert!(
            result >= -2.0 && result <= 2.0,
            "perlin3 output {} out of expected range",
            result
        );
    }

    #[test]
    fn test_mul_div() {
        let input = vec![Fixed::ZERO; 1];
        let mut output = vec![Fixed::ZERO; 1];

        // 2 / 3 should be ~0.666
        let program = vec![
            OpCode::Push(2i32.to_fixed()),
            OpCode::Push(3i32.to_fixed()),
            OpCode::Div,
            OpCode::Return,
        ];
        execute_program(&input, &mut output, &program, 1, 1, Fixed::ZERO);
        let result = output[0].to_f32();
        assert!(
            (result - 0.666).abs() < 0.01,
            "2/3 should be ~0.666, got {}",
            result
        );

        // 2 * 3 should be 6
        let program = vec![
            OpCode::Push(2i32.to_fixed()),
            OpCode::Push(3i32.to_fixed()),
            OpCode::Mul,
            OpCode::Return,
        ];
        execute_program(&input, &mut output, &program, 1, 1, Fixed::ZERO);
        let result = output[0].to_f32();
        assert!(
            (result - 6.0).abs() < 0.01,
            "2*3 should be 6, got {}",
            result
        );
    }

    #[test]
    fn test_coordinate_variation() {
        // First verify that XNorm actually varies across pixels
        use crate::test_engine::vm::LoadSource;
        let input = vec![Fixed::ZERO; 16];
        let mut output = vec![Fixed::ZERO; 16];

        let program = vec![OpCode::Load(LoadSource::XNorm), OpCode::Return];
        execute_program(&input, &mut output, &program, 4, 4, Fixed::ZERO);

        println!("XNorm values for 4x4:");
        for y in 0..4 {
            for x in 0..4 {
                print!("{:.3} ", output[y * 4 + x].to_f32());
            }
            println!();
        }

        // Should have variation
        let first = output[0];
        let has_variation = output.iter().any(|&v| v.0 != first.0);
        assert!(has_variation, "XNorm should vary across pixels");
    }

    #[test]
    fn test_perlin3_with_varying_coords() {
        // Test perlin3 directly with varying coordinates
        use crate::test_engine::vm::LoadSource;
        let input = vec![Fixed::ZERO; 16];
        let mut output = vec![Fixed::ZERO; 16];

        // First test what uv.x * 0.3 gives us
        let test_mult = vec![
            OpCode::Load(LoadSource::XNorm),
            OpCode::Push(0.3f32.to_fixed()),
            OpCode::Mul,
            OpCode::Return,
        ];

        execute_program(&input, &mut output, &test_mult, 4, 4, Fixed::ZERO);
        println!("XNorm * 0.3 values:");
        for y in 0..2 {
            for x in 0..2 {
                print!("{:.6} ", output[y * 4 + x].to_f32());
            }
            println!();
        }

        // perlin3(uv.x * 0.3, uv.y * 0.3, 0, 3)
        let program = vec![
            OpCode::Load(LoadSource::XNorm),
            OpCode::Push(0.3f32.to_fixed()),
            OpCode::Mul,
            OpCode::Load(LoadSource::YNorm),
            OpCode::Push(0.3f32.to_fixed()),
            OpCode::Mul,
            OpCode::Push(Fixed::ZERO), // time
            OpCode::Perlin3(3),
            OpCode::Return,
        ];

        execute_program(&input, &mut output, &program, 4, 4, Fixed::ZERO);

        println!("Perlin3 output for 4x4:");
        for y in 0..4 {
            for x in 0..4 {
                print!("{:.3} ", output[y * 4 + x].to_f32());
            }
            println!();
        }

        // Should have variation
        let first = output[0];
        let has_variation = output.iter().any(|&v| (v.0 - first.0).abs() > 100);
        assert!(
            has_variation,
            "Perlin3 output should vary, got all ~{}",
            first.to_f32()
        );
    }

    #[test]
    fn test_xnorm_mul_varies() {
        // Verify XNorm * 0.3 produces different values across row
        use crate::test_engine::vm::LoadSource;
        let input = vec![Fixed::ZERO; 16 * 16];
        let mut output = vec![Fixed::ZERO; 16 * 16];

        let program = vec![
            OpCode::Load(LoadSource::XNorm),
            OpCode::Push(0.3f32.to_fixed()),
            OpCode::Mul,
            OpCode::Return,
        ];
        execute_program(&input, &mut output, &program, 16, 16, Fixed::ZERO);

        println!("XNorm * 0.3 in first row (should vary):");
        for i in 0..8 {
            println!("  output[{}] (x={}) = {:.6}", i, i, output[i].to_f32());
        }

        // Check variation in first row
        let first_row_has_variation = (0..16).any(|x| output[x].0 != output[0].0);
        assert!(
            first_row_has_variation,
            "XNorm * 0.3 should vary across first row, got all {:.6}",
            output[0].to_f32()
        );
    }

    #[test]
    fn test_perlin3_x_variation() {
        // Test perlin3 directly without cos to see if it varies in X
        use crate::test_engine::vm::LoadSource;
        let input = vec![Fixed::ZERO; 16 * 16];
        let mut output = vec![Fixed::ZERO; 16 * 16];

        let program = vec![
            OpCode::Load(LoadSource::XNorm),
            OpCode::Push(0.3f32.to_fixed()),
            OpCode::Mul,
            OpCode::Load(LoadSource::YNorm),
            OpCode::Push(0.3f32.to_fixed()),
            OpCode::Mul,
            OpCode::Push(Fixed::ZERO), // time = 0
            OpCode::Perlin3(3),
            OpCode::Return,
        ];

        execute_program(&input, &mut output, &program, 16, 16, Fixed::ZERO);

        println!("Perlin3 output in first row (should vary in X):");
        for i in 0..8 {
            println!("  output[{}] (x={}) = {:.6}", i, i, output[i].to_f32());
        }

        // Check variation in first row
        let first_row_has_variation = (0..16).any(|x| (output[x].0 - output[0].0).abs() > 100);
        assert!(
            first_row_has_variation,
            "Perlin3 should vary in X across first row, got all ~{:.6}",
            output[0].to_f32()
        );
    }

    #[test]
    fn test_cos_on_varying_values() {
        // Test if cos preserves variation
        let input = vec![Fixed::ZERO; 16];
        let mut output = vec![Fixed::ZERO; 16];

        // cos(uv.x * 0.3)
        use crate::test_engine::vm::LoadSource;
        let program = vec![
            OpCode::Load(LoadSource::XNorm),
            OpCode::Push(0.3f32.to_fixed()),
            OpCode::Mul,
            OpCode::Cos,
            OpCode::Return,
        ];

        execute_program(&input, &mut output, &program, 4, 4, Fixed::ZERO);

        println!("cos(XNorm * 0.3) in first row:");
        for i in 0..4 {
            println!("  output[{}] (x={}) = {:.6}", i, i, output[i].to_f32());
        }

        // Should have variation
        let first_row_has_variation = (0..4).any(|x| (output[x].0 - output[0].0).abs() > 100);
        assert!(
            first_row_has_variation,
            "cos(XNorm * 0.3) should vary, got all ~{:.6}",
            output[0].to_f32()
        );
    }

    #[test]
    fn test_cos_perlin_combination() {
        // Test cos(perlin3(...)) manually built to see if variation is preserved
        use crate::test_engine::vm::LoadSource;
        let input = vec![Fixed::ZERO; 16];
        let mut output = vec![Fixed::ZERO; 16];

        let program = vec![
            OpCode::Load(LoadSource::XNorm),
            OpCode::Push(0.3f32.to_fixed()),
            OpCode::Mul,
            OpCode::Load(LoadSource::YNorm),
            OpCode::Push(0.3f32.to_fixed()),
            OpCode::Mul,
            OpCode::Push(Fixed::ZERO), // time = 0
            OpCode::Perlin3(3),
            OpCode::Cos,
            OpCode::Return,
        ];

        execute_program(&input, &mut output, &program, 4, 4, Fixed::ZERO);

        println!("cos(perlin3(...)) in first row:");
        for i in 0..4 {
            println!("  output[{}] (x={}) = {:.6}", i, i, output[i].to_f32());
        }

        // Should have variation
        let first_row_has_variation = (0..4).any(|x| (output[x].0 - output[0].0).abs() > 100);
        assert!(
            first_row_has_variation,
            "cos(perlin3(...)) should vary in X, got all ~{:.6}",
            output[0].to_f32()
        );
    }

    #[test]
    fn test_demo_expression_output() {
        // Test the actual demo expression: cos(perlin3(vec3(uv * 0.3, time), 3))
        use crate::lpscript::parse_expr;

        let input = vec![Fixed::ZERO; 16 * 16];
        let mut output = vec![Fixed::ZERO; 16 * 16];

        let program = parse_expr("cos(perlin3(vec3(uv * 0.3, time), 3))").to_legacy_opcodes();

        println!("Parsed opcodes:");
        for (i, op) in program.iter().enumerate() {
            println!("  [{}] {:?}", i, op);
        }

        execute_program(&input, &mut output, &program, 16, 16, Fixed::ZERO);

        println!("Demo expression output sample (first row, should vary in X):");
        for i in 0..8 {
            println!("  output[{}] (x={},y=0) = {:.6}", i, i, output[i].to_f32());
        }
        println!("Demo expression output sample (first column):");
        for i in 0..5 {
            println!(
                "  output[{}] (x=0,y={}) = {:.6}",
                i * 16,
                i,
                output[i * 16].to_f32()
            );
        }

        // Check that we get varied output (not all the same value)
        // With radian-based trig, perlin output (-1..1) to cos() gives values near 1.0
        // So we check for smaller variation than before
        let first = output[0];
        let has_variation = output.iter().any(|&v| (v.0 - first.0).abs() > 200);
        assert!(
            has_variation,
            "Demo expression should produce varied output, got all ~{}",
            first.to_f32()
        );

        // Check that values are in a reasonable range for greyscale (we need 0..1 for palette)
        for (i, &val) in output.iter().enumerate() {
            let f = val.to_f32();
            assert!(
                f >= -2.0 && f <= 2.0,
                "Pixel {} has value {} which is out of range",
                i,
                f
            );
        }
    }
}
