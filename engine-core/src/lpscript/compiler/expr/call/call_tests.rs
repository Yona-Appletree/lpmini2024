/// Function call tests
#[cfg(test)]
mod tests {
    use crate::lpscript::compiler::expr::expr_test_util::ExprTest;
    use crate::lpscript::compiler::test_ast::*;
    use crate::lpscript::shared::Type;
    use crate::lpscript::vm::opcodes::LpsOpCode;
    use crate::math::ToFixed;

    #[test]
    fn test_function_call_sin() -> Result<(), String> {
        ExprTest::new("sin(0.0)")
            .expect_ast(call("sin", vec![num(0.0)], Type::Fixed))
            .expect_opcodes(vec![
                LpsOpCode::Push(0.0.to_fixed()),
                LpsOpCode::SinFixed,
                LpsOpCode::Return,
            ])
            .expect_result_fixed(0.0)
            .run()
    }

    #[test]
    fn test_function_call_cos() -> Result<(), String> {
        ExprTest::new("cos(0.0)")
            .expect_ast(call("cos", vec![num(0.0)], Type::Fixed))
            .expect_opcodes(vec![
                LpsOpCode::Push(0.0.to_fixed()),
                LpsOpCode::CosFixed,
                LpsOpCode::Return,
            ])
            .expect_result_fixed(1.0)
            .run()
    }

    #[test]
    fn test_function_call_min() -> Result<(), String> {
        ExprTest::new("min(1.0, 2.0)")
            .expect_ast(call("min", vec![num(1.0), num(2.0)], Type::Fixed))
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::MinFixed,
                LpsOpCode::Return,
            ])
            .expect_result_fixed(1.0)
            .run()?;

        ExprTest::new("min(5.0, 3.0)")
            .expect_result_fixed(3.0)
            .run()
    }

    #[test]
    fn test_function_call_max() -> Result<(), String> {
        ExprTest::new("max(1.0, 2.0)")
            .expect_ast(call("max", vec![num(1.0), num(2.0)], Type::Fixed))
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::MaxFixed,
                LpsOpCode::Return,
            ])
            .expect_result_fixed(2.0)
            .run()?;

        ExprTest::new("max(5.0, 3.0)")
            .expect_result_fixed(5.0)
            .run()
    }

    #[test]
    fn test_function_call_abs() -> Result<(), String> {
        ExprTest::new("abs(-5.0)")
            .expect_ast(call("abs", vec![num(-5.0)], Type::Fixed))
            .expect_opcodes(vec![
                LpsOpCode::Push((-5.0).to_fixed()),
                LpsOpCode::AbsFixed,
                LpsOpCode::Return,
            ])
            .expect_result_fixed(5.0)
            .run()?;

        ExprTest::new("abs(3.0)").expect_result_fixed(3.0).run()
    }

    #[test]
    fn test_function_call_floor() -> Result<(), String> {
        ExprTest::new("floor(2.7)")
            .expect_ast(call("floor", vec![num(2.7)], Type::Fixed))
            .expect_opcodes(vec![
                LpsOpCode::Push(2.7.to_fixed()),
                LpsOpCode::FloorFixed,
                LpsOpCode::Return,
            ])
            .expect_result_fixed(2.0)
            .run()
    }

    #[test]
    fn test_function_call_ceil() -> Result<(), String> {
        ExprTest::new("ceil(2.3)")
            .expect_ast(call("ceil", vec![num(2.3)], Type::Fixed))
            .expect_opcodes(vec![
                LpsOpCode::Push(2.3.to_fixed()),
                LpsOpCode::CeilFixed,
                LpsOpCode::Return,
            ])
            .expect_result_fixed(3.0)
            .run()
    }

    #[test]
    fn test_function_call_nested() -> Result<(), String> {
        // Test function calls with expressions as arguments
        ExprTest::new("sin(1.0 + 2.0)")
            .expect_result_fixed((3.0f32).sin())
            .run()?;

        // Nested function calls
        ExprTest::new("abs(sin(0.0))")
            .expect_result_fixed(0.0)
            .run()
    }

    #[test]
    fn test_complex_perlin() -> Result<(), String> {
        // Tests perlin3 + cos nested function calls
        ExprTest::new("cos(perlin3(vec3(uv * 0.3, time), 3))")
            .with_x(0.5)
            .with_y(0.5)
            .with_time(1.0)
            .run()?;

        // Verify both Perlin3 and Cos opcodes are generated
        let program = crate::lpscript::parse_expr("cos(perlin3(vec3(uv * 0.3, time), 3))");
        let has_perlin = program
            .opcodes
            .iter()
            .any(|op| matches!(op, LpsOpCode::Perlin3(_)));
        let has_cos = program
            .opcodes
            .iter()
            .any(|op| matches!(op, LpsOpCode::CosFixed));
        assert!(has_perlin, "Should have Perlin3 opcode");
        assert!(has_cos, "Should have CosFixed opcode");
        Ok(())
    }

    #[test]
    fn test_perlin3_only_pushes_xyz() -> Result<(), String> {
        // Regression test for horizontal stripes bug!
        // perlin3(vec3) should only push 3 args to stack (x, y, z components of vec3)
        // Octaves should be extracted at compile time and embedded in the opcode
        let program = crate::lpscript::parse_expr("perlin3(vec3(xNorm, yNorm, time), 3)");

        // Count Push/Load opcodes before Perlin3
        let mut push_count = 0;
        for op in &program.opcodes {
            if matches!(op, LpsOpCode::Perlin3(_)) {
                break;
            }
            if matches!(op, LpsOpCode::Push(_) | LpsOpCode::Load(_)) {
                push_count += 1;
            }
        }

        // CRITICAL: Should be exactly 3 (xNorm, yNorm, time)
        // If it's 4, the VM will pop them as (z=octaves, y=z, x=y)
        // causing only Y to vary (horizontal stripes)!
        assert_eq!(
            push_count, 3,
            "BUG: perlin3 pushed {} args but VM expects 3. This causes horizontal stripes!",
            push_count
        );

        // Verify octaves is embedded in opcode
        let has_perlin = program.opcodes.iter().any(|op| {
            if let LpsOpCode::Perlin3(octaves) = op {
                *octaves == 3
            } else {
                false
            }
        });
        assert!(
            has_perlin,
            "Should have Perlin3(3) opcode with octaves=3 embedded"
        );

        Ok(())
    }

    // Type checking tests (using ExprTest validates types automatically)
    #[test]
    fn test_function_call_typecheck() -> Result<(), String> {
        ExprTest::new("sin(time)")
            .with_time(0.0)
            .expect_result_fixed(0.0)
            .run()
    }

    #[test]
    fn test_vector_functions_typecheck() -> Result<(), String> {
        // normalize returns same vector type
        ExprTest::new("normalize(vec2(3.0, 4.0)).x")
            .expect_result_fixed(0.6)
            .run()?;

        // length returns scalar
        ExprTest::new("length(vec2(3.0, 4.0))")
            .expect_result_fixed(5.0)
            .run()
    }
}
