/// Vector constructor tests
#[cfg(test)]
mod tests {
    use crate::compiler::expr::expr_test_util::ExprTest;
    use crate::fixed::{ToFixed, Vec2, Vec3, Vec4};
    use crate::vm::opcodes::LpsOpCode;

    #[test]
    fn test_vec2_constructor() -> Result<(), String> {
        ExprTest::new("vec2(1.0, 2.0)")
            .expect_ast(|b| {
                let arg1 = b.num(1.0);
                let arg2 = b.num(2.0);
                b.vec2(vec![arg1, arg2])
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::Return,
            ])
            .expect_result_vec2(Vec2 {
                x: 1.0.to_fixed(),
                y: 2.0.to_fixed(),
            })
            .run()
    }

    #[test]
    fn test_vec3_constructor() -> Result<(), String> {
        ExprTest::new("vec3(1.0, 2.0, 3.0)")
            .expect_ast(|b| {
                let arg1 = b.num(1.0);
                let arg2 = b.num(2.0);
                let arg3 = b.num(3.0);
                b.vec3(vec![arg1, arg2, arg3])
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::Push(3.0.to_fixed()),
                LpsOpCode::Return,
            ])
            .expect_result_vec3(Vec3 {
                x: 1.0.to_fixed(),
                y: 2.0.to_fixed(),
                z: 3.0.to_fixed(),
            })
            .run()
    }

    #[test]
    fn test_vec4_constructor() -> Result<(), String> {
        ExprTest::new("vec4(1.0, 2.0, 3.0, 4.0)")
            .expect_ast(|b| {
                let arg1 = b.num(1.0);
                let arg2 = b.num(2.0);
                let arg3 = b.num(3.0);
                let arg4 = b.num(4.0);
                b.vec4(vec![arg1, arg2, arg3, arg4])
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::Push(3.0.to_fixed()),
                LpsOpCode::Push(4.0.to_fixed()),
                LpsOpCode::Return,
            ])
            .expect_result_vec4(Vec4 {
                x: 1.0.to_fixed(),
                y: 2.0.to_fixed(),
                z: 3.0.to_fixed(),
                w: 4.0.to_fixed(),
            })
            .run()
    }

    #[test]
    fn test_vec3_from_vec2_and_scalar() -> Result<(), String> {
        ExprTest::new("vec3(vec2(1.0, 2.0), 3.0)")
            .expect_result_vec3(Vec3 {
                x: 1.0.to_fixed(),
                y: 2.0.to_fixed(),
                z: 3.0.to_fixed(),
            })
            .run()
    }

    #[test]
    fn test_vec4_from_vec3_and_scalar() -> Result<(), String> {
        ExprTest::new("vec4(vec3(1.0, 2.0, 3.0), 4.0)")
            .expect_result_vec4(Vec4 {
                x: 1.0.to_fixed(),
                y: 2.0.to_fixed(),
                z: 3.0.to_fixed(),
                w: 4.0.to_fixed(),
            })
            .run()
    }

    #[test]
    fn test_vec4_from_two_vec2() -> Result<(), String> {
        ExprTest::new("vec4(vec2(1.0, 2.0), vec2(3.0, 4.0))")
            .expect_result_vec4(Vec4 {
                x: 1.0.to_fixed(),
                y: 2.0.to_fixed(),
                z: 3.0.to_fixed(),
                w: 4.0.to_fixed(),
            })
            .run()
    }

    #[test]
    fn test_vec2_from_single_scalar_each() -> Result<(), String> {
        ExprTest::new("vec2(1.0, 2.0)")
            .expect_result_vec2(Vec2 {
                x: 1.0.to_fixed(),
                y: 2.0.to_fixed(),
            })
            .run()
    }

    #[test]
    fn test_mat3_constructor() -> Result<(), String> {
        ExprTest::new("mat3(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0)")
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::Push(3.0.to_fixed()),
                LpsOpCode::Push(4.0.to_fixed()),
                LpsOpCode::Push(5.0.to_fixed()),
                LpsOpCode::Push(6.0.to_fixed()),
                LpsOpCode::Push(7.0.to_fixed()),
                LpsOpCode::Push(8.0.to_fixed()),
                LpsOpCode::Push(9.0.to_fixed()),
                LpsOpCode::Return,
            ])
            .expect_result_mat3(crate::fixed::Mat3::from_f32(
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0,
            ))
            .run()
    }

    #[test]
    fn test_mat3_from_three_vec3() -> Result<(), String> {
        ExprTest::new("mat3(vec3(1.0, 2.0, 3.0), vec3(4.0, 5.0, 6.0), vec3(7.0, 8.0, 9.0))")
            .expect_result_mat3(crate::fixed::Mat3::from_f32(
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0,
            ))
            .run()
    }

    #[test]
    fn test_mat3_identity() -> Result<(), String> {
        ExprTest::new("mat3(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0)")
            .expect_result_mat3(crate::fixed::Mat3::identity())
            .run()
    }

    #[test]
    fn test_mat3_zero() -> Result<(), String> {
        ExprTest::new("mat3(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)")
            .expect_result_mat3(crate::fixed::Mat3::zero())
            .run()
    }

    #[test]
    fn test_mat3_from_mixed_components() -> Result<(), String> {
        // Mat3 from vec2 + scalars
        ExprTest::new("mat3(vec2(1.0, 2.0), 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0)")
            .expect_result_mat3(crate::fixed::Mat3::from_f32(
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0,
            ))
            .run()?;

        // Mat3 from vec3 + vec3 + vec3 (already tested above)
        // Mat3 from 9 scalars (already tested above)
        Ok(())
    }

    #[test]
    fn test_mat3_negative_values() -> Result<(), String> {
        ExprTest::new("mat3(-1.0, -2.0, -3.0, -4.0, -5.0, -6.0, -7.0, -8.0, -9.0)")
            .expect_result_mat3(crate::fixed::Mat3::from_f32(
                -1.0, -2.0, -3.0, -4.0, -5.0, -6.0, -7.0, -8.0, -9.0,
            ))
            .run()
    }

    // Type checking tests (using ExprTest validates types automatically)
    // These tests already exist above and validate type checking through execution
}
