/// Function call tests for Vec3 type
#[cfg(test)]
mod tests {
    use crate::compiler::expr::expr_test_util::ExprTest;
    use crate::fixed::ToFixed;
    use crate::vm::opcodes::LpsOpCode;

    #[test]
    fn test_length() -> Result<(), String> {
        ExprTest::new("length(vec3(2.0, 3.0, 6.0))")
            .expect_opcodes(vec![
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::Push(3.0.to_fixed()),
                LpsOpCode::Push(6.0.to_fixed()),
                LpsOpCode::Length3,
                LpsOpCode::Return,
            ])
            .expect_result_fixed(7.0) // sqrt(4 + 9 + 36) = 7
            .run()
    }

    #[test]
    fn test_normalize() -> Result<(), String> {
        ExprTest::new("normalize(vec3(2.0, 0.0, 0.0)).x")
            .expect_result_fixed(1.0)
            .run()?;

        ExprTest::new("normalize(vec3(2.0, 0.0, 0.0)).y")
            .expect_result_fixed(0.0)
            .run()
    }

    #[test]
    fn test_dot() -> Result<(), String> {
        ExprTest::new("dot(vec3(1.0, 2.0, 3.0), vec3(4.0, 5.0, 6.0))")
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::Push(3.0.to_fixed()),
                LpsOpCode::Push(4.0.to_fixed()),
                LpsOpCode::Push(5.0.to_fixed()),
                LpsOpCode::Push(6.0.to_fixed()),
                LpsOpCode::Dot3,
                LpsOpCode::Return,
            ])
            .expect_result_fixed(32.0) // 1*4 + 2*5 + 3*6 = 32
            .run()
    }

    #[test]
    fn test_cross() -> Result<(), String> {
        ExprTest::new("cross(vec3(1.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0))")
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::Push(0.0.to_fixed()),
                LpsOpCode::Push(0.0.to_fixed()),
                LpsOpCode::Push(0.0.to_fixed()),
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::Push(0.0.to_fixed()),
                LpsOpCode::Cross3,
                LpsOpCode::Return,
            ])
            .expect_result_vec3(crate::fixed::Vec3::new(
                0.0.to_fixed(),
                0.0.to_fixed(),
                1.0.to_fixed(),
            ))
            .run()
    }

    #[test]
    fn test_distance() -> Result<(), String> {
        ExprTest::new("distance(vec3(0.0, 0.0, 0.0), vec3(2.0, 3.0, 6.0))")
            .expect_opcodes(vec![
                LpsOpCode::Push(0.0.to_fixed()),
                LpsOpCode::Push(0.0.to_fixed()),
                LpsOpCode::Push(0.0.to_fixed()),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::Push(3.0.to_fixed()),
                LpsOpCode::Push(6.0.to_fixed()),
                LpsOpCode::Distance3,
                LpsOpCode::Return,
            ])
            .expect_result_fixed(7.0)
            .run()
    }

    #[test]
    fn test_perlin3() -> Result<(), String> {
        // Tests perlin3 + cos nested function calls
        ExprTest::new("cos(perlin3(vec3(uv * 0.3, time), 3))")
            .with_x(0.5)
            .with_y(0.5)
            .with_time(1.0)
            .run()?;

        // Verify both Perlin3 and Cos opcodes are generated
        let program = crate::parse_expr("cos(perlin3(vec3(uv * 0.3, time), 3))");
        let opcodes = &program.main_function().unwrap().opcodes;
        let has_perlin = opcodes.iter().any(|op| matches!(op, LpsOpCode::Perlin3(_)));
        let has_cos = opcodes.iter().any(|op| matches!(op, LpsOpCode::CosFixed));
        assert!(has_perlin, "Should have Perlin3 opcode");
        assert!(has_cos, "Should have CosFixed opcode");
        Ok(())
    }

    #[test]
    fn test_perlin3_only_pushes_xyz() -> Result<(), String> {
        // Regression test for horizontal stripes bug!
        // perlin3(vec3) should only push 3 args to stack (x, y, z components of vec3)
        // Octaves should be extracted at compile time and embedded in the opcode
        let program = crate::parse_expr("perlin3(vec3(xNorm, yNorm, time), 3)");

        // Count Push/Load opcodes before Perlin3
        let mut push_count = 0;
        let opcodes = &program.main_function().unwrap().opcodes;
        for op in opcodes {
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
        let has_perlin = program.main_function().unwrap().opcodes.iter().any(|op| {
            if let LpsOpCode::Perlin3(octaves) = op {
                octaves == &3
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
}
