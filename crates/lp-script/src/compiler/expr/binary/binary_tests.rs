/// Binary arithmetic operator tests
#[cfg(test)]
mod tests {
    use crate::compiler::expr::expr_test_util::ExprTest;

    use crate::fixed::ToFixed;
    use crate::shared::Type;
    use crate::vm::opcodes::LpsOpCode;

    #[test]
    fn test_addition() -> Result<(), String> {
        ExprTest::new("1.0 + 2.0")
            .expect_ast(|b| {
                let left = b.num(1.0);
                let right = b.num(2.0);
                b.add(left, right, Type::Fixed)
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::AddFixed,
                LpsOpCode::Return,
            ])
            .expect_result_fixed(3.0)
            .run()
    }

    #[test]
    fn test_subtraction() -> Result<(), String> {
        ExprTest::new("5.0 - 2.0")
            .expect_ast(|b| {
                let left = b.num(5.0);
                let right = b.num(2.0);
                b.sub(left, right, Type::Fixed)
            })
            .expect_result_fixed(3.0)
            .run()
    }

    #[test]
    fn test_multiplication() -> Result<(), String> {
        ExprTest::new("3.0 * 4.0")
            .expect_ast(|b| {
                let left = b.num(3.0);
                let right = b.num(4.0);
                b.mul(left, right, Type::Fixed)
            })
            .expect_result_fixed(12.0)
            .run()
    }

    #[test]
    fn test_division() -> Result<(), String> {
        ExprTest::new("10.0 / 2.0")
            .expect_ast(|b| {
                let left = b.num(10.0);
                let right = b.num(2.0);
                b.div(left, right, Type::Fixed)
            })
            .expect_result_fixed(5.0)
            .run()
    }

    #[test]
    fn test_operator_precedence() -> Result<(), String> {
        // 1 + 2 * 3 should be 1 + (2 * 3) = 7
        ExprTest::new("1.0 + 2.0 * 3.0")
            .expect_ast(|b| {
                let left = b.num(1.0);
                let mul_left = b.num(2.0);
                let mul_right = b.num(3.0);
                let right = b.mul(mul_left, mul_right, Type::Fixed);
                b.add(left, right, Type::Fixed)
            })
            .expect_result_fixed(7.0)
            .run()
    }

    #[test]
    fn test_parenthesized_expression() -> Result<(), String> {
        // (1 + 2) * 3 should be 9
        ExprTest::new("(1.0 + 2.0) * 3.0")
            .expect_ast(|b| {
                let add_left = b.num(1.0);
                let add_right = b.num(2.0);
                let left = b.add(add_left, add_right, Type::Fixed);
                let right = b.num(3.0);
                b.mul(left, right, Type::Fixed)
            })
            .expect_result_fixed(9.0)
            .run()
    }

    #[test]
    fn test_int_float_promotion() -> Result<(), String> {
        ExprTest::new("1 + 2.0").expect_result_fixed(3.0).run()?;

        ExprTest::new("2.0 + 1").expect_result_fixed(3.0).run()
    }

    #[test]
    fn test_power_function() -> Result<(), String> {
        // Power operator (^) has been removed, use pow() function instead
        ExprTest::new("pow(2.0, 3.0)")
            .expect_opcodes(vec![
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::Push(3.0.to_fixed()),
                LpsOpCode::PowFixed,
                LpsOpCode::Return,
            ])
            .expect_result_fixed(8.0)
            .run()
    }

    // Type checking test (using ExprTest validates types automatically)
    #[test]
    fn test_arithmetic_typecheck() -> Result<(), String> {
        ExprTest::new("1.0 + 2.0").expect_result_fixed(3.0).run()
    }

    // ========================================================================
    // Vector Arithmetic Tests (GLSL-compatible)
    // ========================================================================

    // Vec2 + Vec2 (component-wise addition)
    #[test]
    fn test_vec2_addition() -> Result<(), String> {
        use crate::fixed::Vec2;
        ExprTest::new("vec2(1.0, 2.0) + vec2(3.0, 4.0)")
            .expect_result_vec2(Vec2::new(4.0.to_fixed(), 6.0.to_fixed()))
            .run()
    }

    #[test]
    fn test_vec3_addition() -> Result<(), String> {
        use crate::fixed::Vec3;
        ExprTest::new("vec3(1.0, 2.0, 3.0) + vec3(4.0, 5.0, 6.0)")
            .expect_result_vec3(Vec3::new(5.0.to_fixed(), 7.0.to_fixed(), 9.0.to_fixed()))
            .run()
    }

    #[test]
    fn test_vec4_addition() -> Result<(), String> {
        use crate::fixed::Vec4;
        ExprTest::new("vec4(1.0, 2.0, 3.0, 4.0) + vec4(1.0, 1.0, 1.0, 1.0)")
            .expect_result_vec4(Vec4::new(
                2.0.to_fixed(),
                3.0.to_fixed(),
                4.0.to_fixed(),
                5.0.to_fixed(),
            ))
            .run()
    }

    // Vec - Vec (component-wise subtraction)
    #[test]
    fn test_vec2_subtraction() -> Result<(), String> {
        use crate::fixed::Vec2;
        ExprTest::new("vec2(5.0, 8.0) - vec2(2.0, 3.0)")
            .expect_result_vec2(Vec2::new(3.0.to_fixed(), 5.0.to_fixed()))
            .run()
    }

    #[test]
    fn test_vec3_subtraction() -> Result<(), String> {
        use crate::fixed::Vec3;
        ExprTest::new("vec3(10.0, 20.0, 30.0) - vec3(1.0, 2.0, 3.0)")
            .expect_result_vec3(Vec3::new(9.0.to_fixed(), 18.0.to_fixed(), 27.0.to_fixed()))
            .run()
    }

    #[test]
    fn test_vec4_subtraction() -> Result<(), String> {
        use crate::fixed::Vec4;
        ExprTest::new("vec4(10.0, 9.0, 8.0, 7.0) - vec4(1.0, 2.0, 3.0, 4.0)")
            .expect_result_vec4(Vec4::new(
                9.0.to_fixed(),
                7.0.to_fixed(),
                5.0.to_fixed(),
                3.0.to_fixed(),
            ))
            .run()
    }

    // Vec * Vec (component-wise multiplication)
    #[test]
    fn test_vec2_multiplication() -> Result<(), String> {
        use crate::fixed::Vec2;
        ExprTest::new("vec2(2.0, 3.0) * vec2(4.0, 5.0)")
            .expect_result_vec2(Vec2::new(8.0.to_fixed(), 15.0.to_fixed()))
            .run()
    }

    #[test]
    fn test_vec3_multiplication() -> Result<(), String> {
        use crate::fixed::Vec3;
        ExprTest::new("vec3(2.0, 3.0, 4.0) * vec3(5.0, 6.0, 7.0)")
            .expect_result_vec3(Vec3::new(10.0.to_fixed(), 18.0.to_fixed(), 28.0.to_fixed()))
            .run()
    }

    #[test]
    fn test_vec4_multiplication() -> Result<(), String> {
        use crate::fixed::Vec4;
        ExprTest::new("vec4(1.0, 2.0, 3.0, 4.0) * vec4(2.0, 3.0, 4.0, 5.0)")
            .expect_result_vec4(Vec4::new(
                2.0.to_fixed(),
                6.0.to_fixed(),
                12.0.to_fixed(),
                20.0.to_fixed(),
            ))
            .run()
    }

    // Vec / Vec (component-wise division)
    #[test]
    fn test_vec2_division() -> Result<(), String> {
        use crate::fixed::Vec2;
        ExprTest::new("vec2(10.0, 20.0) / vec2(2.0, 4.0)")
            .expect_result_vec2(Vec2::new(5.0.to_fixed(), 5.0.to_fixed()))
            .run()
    }

    #[test]
    fn test_vec3_division() -> Result<(), String> {
        use crate::fixed::Vec3;
        ExprTest::new("vec3(12.0, 18.0, 24.0) / vec3(3.0, 6.0, 8.0)")
            .expect_result_vec3(Vec3::new(4.0.to_fixed(), 3.0.to_fixed(), 3.0.to_fixed()))
            .run()
    }

    #[test]
    fn test_vec4_division() -> Result<(), String> {
        use crate::fixed::Vec4;
        ExprTest::new("vec4(20.0, 30.0, 40.0, 50.0) / vec4(2.0, 3.0, 4.0, 5.0)")
            .expect_result_vec4(Vec4::new(
                10.0.to_fixed(),
                10.0.to_fixed(),
                10.0.to_fixed(),
                10.0.to_fixed(),
            ))
            .run()
    }

    // Vec * Scalar (broadcast scalar to all components)
    #[test]
    fn test_vec2_scalar_multiplication() -> Result<(), String> {
        use crate::fixed::Vec2;
        ExprTest::new("vec2(1.0, 2.0) * 3.0")
            .expect_result_vec2(Vec2::new(3.0.to_fixed(), 6.0.to_fixed()))
            .run()
    }

    #[test]
    fn test_vec3_scalar_multiplication() -> Result<(), String> {
        use crate::fixed::Vec3;
        ExprTest::new("vec3(2.0, 3.0, 4.0) * 2.0")
            .expect_result_vec3(Vec3::new(4.0.to_fixed(), 6.0.to_fixed(), 8.0.to_fixed()))
            .run()
    }

    #[test]
    fn test_vec4_scalar_multiplication() -> Result<(), String> {
        use crate::fixed::Vec4;
        ExprTest::new("vec4(1.0, 2.0, 3.0, 4.0) * 5.0")
            .expect_result_vec4(Vec4::new(
                5.0.to_fixed(),
                10.0.to_fixed(),
                15.0.to_fixed(),
                20.0.to_fixed(),
            ))
            .run()
    }

    // Scalar * Vec (commutative - broadcast scalar to all components)
    #[test]
    fn test_scalar_vec2_multiplication() -> Result<(), String> {
        use crate::fixed::Vec2;
        ExprTest::new("2.0 * vec2(3.0, 4.0)")
            .expect_result_vec2(Vec2::new(6.0.to_fixed(), 8.0.to_fixed()))
            .run()
    }

    #[test]
    fn test_scalar_vec3_multiplication() -> Result<(), String> {
        use crate::fixed::Vec3;
        ExprTest::new("3.0 * vec3(1.0, 2.0, 3.0)")
            .expect_result_vec3(Vec3::new(3.0.to_fixed(), 6.0.to_fixed(), 9.0.to_fixed()))
            .run()
    }

    #[test]
    fn test_scalar_vec4_multiplication() -> Result<(), String> {
        use crate::fixed::Vec4;
        ExprTest::new("2.0 * vec4(1.0, 2.0, 3.0, 4.0)")
            .expect_result_vec4(Vec4::new(
                2.0.to_fixed(),
                4.0.to_fixed(),
                6.0.to_fixed(),
                8.0.to_fixed(),
            ))
            .run()
    }

    // Vec / Scalar (broadcast scalar to all components)
    #[test]
    fn test_vec2_scalar_division() -> Result<(), String> {
        use crate::fixed::Vec2;
        ExprTest::new("vec2(10.0, 20.0) / 2.0")
            .expect_result_vec2(Vec2::new(5.0.to_fixed(), 10.0.to_fixed()))
            .run()
    }

    #[test]
    fn test_vec3_scalar_division() -> Result<(), String> {
        use crate::fixed::Vec3;
        ExprTest::new("vec3(6.0, 9.0, 12.0) / 3.0")
            .expect_result_vec3(Vec3::new(2.0.to_fixed(), 3.0.to_fixed(), 4.0.to_fixed()))
            .run()
    }

    #[test]
    fn test_vec4_scalar_division() -> Result<(), String> {
        use crate::fixed::Vec4;
        ExprTest::new("vec4(10.0, 20.0, 30.0, 40.0) / 10.0")
            .expect_result_vec4(Vec4::new(
                1.0.to_fixed(),
                2.0.to_fixed(),
                3.0.to_fixed(),
                4.0.to_fixed(),
            ))
            .run()
    }
}
