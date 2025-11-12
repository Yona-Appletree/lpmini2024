/// Binary arithmetic operator tests for Vec4 type
#[cfg(test)]
mod tests {
    use crate::compiler::expr::expr_test_util::ExprTest;
    use crate::fixed::{ToFixed, Vec4};
    use crate::vm::opcodes::LpsOpCode;

    // Vec4 + Vec4 (component-wise addition)
    #[test]
    fn test_vec4_addition() -> Result<(), String> {
        ExprTest::new("vec4(1.0, 2.0, 3.0, 4.0) + vec4(1.0, 1.0, 1.0, 1.0)")
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::Push(3.0.to_fixed()),
                LpsOpCode::Push(4.0.to_fixed()),
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::AddVec4,
                LpsOpCode::Return,
            ])
            .expect_result_vec4(Vec4::new(
                2.0.to_fixed(),
                3.0.to_fixed(),
                4.0.to_fixed(),
                5.0.to_fixed(),
            ))
            .run()
    }

    // Vec4 - Vec4 (component-wise subtraction)
    #[test]
    fn test_vec4_subtraction() -> Result<(), String> {
        ExprTest::new("vec4(10.0, 9.0, 8.0, 7.0) - vec4(1.0, 2.0, 3.0, 4.0)")
            .expect_opcodes(vec![
                LpsOpCode::Push(10.0.to_fixed()),
                LpsOpCode::Push(9.0.to_fixed()),
                LpsOpCode::Push(8.0.to_fixed()),
                LpsOpCode::Push(7.0.to_fixed()),
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::Push(3.0.to_fixed()),
                LpsOpCode::Push(4.0.to_fixed()),
                LpsOpCode::SubVec4,
                LpsOpCode::Return,
            ])
            .expect_result_vec4(Vec4::new(
                9.0.to_fixed(),
                7.0.to_fixed(),
                5.0.to_fixed(),
                3.0.to_fixed(),
            ))
            .run()
    }

    // Vec4 * Vec4 (component-wise multiplication)
    #[test]
    fn test_vec4_multiplication() -> Result<(), String> {
        ExprTest::new("vec4(1.0, 2.0, 3.0, 4.0) * vec4(2.0, 3.0, 4.0, 5.0)")
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::Push(3.0.to_fixed()),
                LpsOpCode::Push(4.0.to_fixed()),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::Push(3.0.to_fixed()),
                LpsOpCode::Push(4.0.to_fixed()),
                LpsOpCode::Push(5.0.to_fixed()),
                LpsOpCode::MulVec4,
                LpsOpCode::Return,
            ])
            .expect_result_vec4(Vec4::new(
                2.0.to_fixed(),
                6.0.to_fixed(),
                12.0.to_fixed(),
                20.0.to_fixed(),
            ))
            .run()
    }

    // Vec4 / Vec4 (component-wise division)
    #[test]
    fn test_vec4_division() -> Result<(), String> {
        ExprTest::new("vec4(20.0, 30.0, 40.0, 50.0) / vec4(2.0, 3.0, 4.0, 5.0)")
            .expect_opcodes(vec![
                LpsOpCode::Push(20.0.to_fixed()),
                LpsOpCode::Push(30.0.to_fixed()),
                LpsOpCode::Push(40.0.to_fixed()),
                LpsOpCode::Push(50.0.to_fixed()),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::Push(3.0.to_fixed()),
                LpsOpCode::Push(4.0.to_fixed()),
                LpsOpCode::Push(5.0.to_fixed()),
                LpsOpCode::DivVec4,
                LpsOpCode::Return,
            ])
            .expect_result_vec4(Vec4::new(
                10.0.to_fixed(),
                10.0.to_fixed(),
                10.0.to_fixed(),
                10.0.to_fixed(),
            ))
            .run()
    }

    // Vec4 % Vec4 (component-wise modulo)
    #[test]
    fn test_vec4_modulo() -> Result<(), String> {
        ExprTest::new("vec4(10.0, 7.0, 15.0, 12.0) % vec4(3.0, 2.0, 4.0, 5.0)")
            .expect_opcodes(vec![
                LpsOpCode::Push(10.0.to_fixed()),
                LpsOpCode::Push(7.0.to_fixed()),
                LpsOpCode::Push(15.0.to_fixed()),
                LpsOpCode::Push(12.0.to_fixed()),
                LpsOpCode::Push(3.0.to_fixed()),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::Push(4.0.to_fixed()),
                LpsOpCode::Push(5.0.to_fixed()),
                LpsOpCode::ModVec4,
                LpsOpCode::Return,
            ])
            .expect_result_vec4(Vec4::new(
                1.0.to_fixed(),
                1.0.to_fixed(),
                3.0.to_fixed(),
                2.0.to_fixed(),
            ))
            .run()
    }

    // Vec4 * Scalar (broadcast scalar to all components)
    #[test]
    fn test_vec4_scalar_multiplication() -> Result<(), String> {
        ExprTest::new("vec4(1.0, 2.0, 3.0, 4.0) * 5.0")
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::Push(3.0.to_fixed()),
                LpsOpCode::Push(4.0.to_fixed()),
                LpsOpCode::Push(5.0.to_fixed()),
                LpsOpCode::MulVec4Scalar,
                LpsOpCode::Return,
            ])
            .expect_result_vec4(Vec4::new(
                5.0.to_fixed(),
                10.0.to_fixed(),
                15.0.to_fixed(),
                20.0.to_fixed(),
            ))
            .run()
    }

    // Scalar * Vec4 (commutative - broadcast scalar to all components)
    #[test]
    fn test_scalar_vec4_multiplication() -> Result<(), String> {
        ExprTest::new("2.0 * vec4(1.0, 2.0, 3.0, 4.0)")
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::Push(3.0.to_fixed()),
                LpsOpCode::Push(4.0.to_fixed()),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::MulVec4Scalar,
                LpsOpCode::Return,
            ])
            .expect_result_vec4(Vec4::new(
                2.0.to_fixed(),
                4.0.to_fixed(),
                6.0.to_fixed(),
                8.0.to_fixed(),
            ))
            .run()
    }

    // Vec4 / Scalar (broadcast scalar to all components)
    #[test]
    fn test_vec4_scalar_division() -> Result<(), String> {
        ExprTest::new("vec4(10.0, 20.0, 30.0, 40.0) / 10.0")
            .expect_opcodes(vec![
                LpsOpCode::Push(10.0.to_fixed()),
                LpsOpCode::Push(20.0.to_fixed()),
                LpsOpCode::Push(30.0.to_fixed()),
                LpsOpCode::Push(40.0.to_fixed()),
                LpsOpCode::Push(10.0.to_fixed()),
                LpsOpCode::DivVec4Scalar,
                LpsOpCode::Return,
            ])
            .expect_result_vec4(Vec4::new(
                1.0.to_fixed(),
                2.0.to_fixed(),
                3.0.to_fixed(),
                4.0.to_fixed(),
            ))
            .run()
    }

    // Vec4 * Int32 scalar (should promote Int32 to Fixed)
    #[test]
    fn test_vec4_int32_scalar_multiplication() -> Result<(), String> {
        ExprTest::new("vec4(1.0, 2.0, 3.0, 4.0) * 2")
            .expect_result_vec4(Vec4::new(
                2.0.to_fixed(),
                4.0.to_fixed(),
                6.0.to_fixed(),
                8.0.to_fixed(),
            ))
            .run()?;

        ExprTest::new("3 * vec4(1.0, 2.0, 3.0, 4.0)")
            .expect_result_vec4(Vec4::new(
                3.0.to_fixed(),
                6.0.to_fixed(),
                9.0.to_fixed(),
                12.0.to_fixed(),
            ))
            .run()
    }

    #[test]
    fn test_edge_cases() -> Result<(), String> {
        // Zero vector
        ExprTest::new("vec4(0.0, 0.0, 0.0, 0.0) + vec4(1.0, 2.0, 3.0, 4.0)")
            .expect_result_vec4(Vec4::new(
                1.0.to_fixed(),
                2.0.to_fixed(),
                3.0.to_fixed(),
                4.0.to_fixed(),
            ))
            .run()?;

        // Negative components
        ExprTest::new("vec4(-1.0, -2.0, -3.0, -4.0) + vec4(3.0, 4.0, 5.0, 6.0)")
            .expect_result_vec4(Vec4::new(
                2.0.to_fixed(),
                2.0.to_fixed(),
                2.0.to_fixed(),
                2.0.to_fixed(),
            ))
            .run()?;

        // Multiply by zero
        ExprTest::new("vec4(1.0, 2.0, 3.0, 4.0) * 0.0")
            .expect_result_vec4(Vec4::new(
                0.0.to_fixed(),
                0.0.to_fixed(),
                0.0.to_fixed(),
                0.0.to_fixed(),
            ))
            .run()
    }
}
