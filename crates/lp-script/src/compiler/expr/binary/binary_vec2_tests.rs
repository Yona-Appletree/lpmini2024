/// Binary arithmetic operator tests for Vec2 type
#[cfg(test)]
mod tests {
    use crate::compiler::expr::expr_test_util::ExprTest;
    use crate::dec32::{ToDec32, Vec2};
    use crate::vm::opcodes::LpsOpCode;

    // Vec2 + Vec2 (component-wise addition)
    #[test]
    fn test_vec2_addition() -> Result<(), String> {
        ExprTest::new("vec2(1.0, 2.0) + vec2(3.0, 4.0)")
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_dec32()),
                LpsOpCode::Push(2.0.to_dec32()),
                LpsOpCode::Push(3.0.to_dec32()),
                LpsOpCode::Push(4.0.to_dec32()),
                LpsOpCode::AddVec2,
                LpsOpCode::Return,
            ])
            .expect_result_vec2(Vec2::new(4.0.to_dec32(), 6.0.to_dec32()))
            .run()
    }

    // Vec2 - Vec2 (component-wise subtraction)
    #[test]
    fn test_vec2_subtraction() -> Result<(), String> {
        ExprTest::new("vec2(5.0, 8.0) - vec2(2.0, 3.0)")
            .expect_opcodes(vec![
                LpsOpCode::Push(5.0.to_dec32()),
                LpsOpCode::Push(8.0.to_dec32()),
                LpsOpCode::Push(2.0.to_dec32()),
                LpsOpCode::Push(3.0.to_dec32()),
                LpsOpCode::SubVec2,
                LpsOpCode::Return,
            ])
            .expect_result_vec2(Vec2::new(3.0.to_dec32(), 5.0.to_dec32()))
            .run()
    }

    // Vec2 * Vec2 (component-wise multiplication)
    #[test]
    fn test_vec2_multiplication() -> Result<(), String> {
        ExprTest::new("vec2(2.0, 3.0) * vec2(4.0, 5.0)")
            .expect_opcodes(vec![
                LpsOpCode::Push(2.0.to_dec32()),
                LpsOpCode::Push(3.0.to_dec32()),
                LpsOpCode::Push(4.0.to_dec32()),
                LpsOpCode::Push(5.0.to_dec32()),
                LpsOpCode::MulVec2,
                LpsOpCode::Return,
            ])
            .expect_result_vec2(Vec2::new(8.0.to_dec32(), 15.0.to_dec32()))
            .run()
    }

    // Vec2 / Vec2 (component-wise division)
    #[test]
    fn test_vec2_division() -> Result<(), String> {
        ExprTest::new("vec2(10.0, 20.0) / vec2(2.0, 4.0)")
            .expect_opcodes(vec![
                LpsOpCode::Push(10.0.to_dec32()),
                LpsOpCode::Push(20.0.to_dec32()),
                LpsOpCode::Push(2.0.to_dec32()),
                LpsOpCode::Push(4.0.to_dec32()),
                LpsOpCode::DivVec2,
                LpsOpCode::Return,
            ])
            .expect_result_vec2(Vec2::new(5.0.to_dec32(), 5.0.to_dec32()))
            .run()
    }

    // Vec2 % Vec2 (component-wise modulo)
    #[test]
    fn test_vec2_modulo() -> Result<(), String> {
        ExprTest::new("vec2(10.0, 7.0) % vec2(3.0, 2.0)")
            .expect_opcodes(vec![
                LpsOpCode::Push(10.0.to_dec32()),
                LpsOpCode::Push(7.0.to_dec32()),
                LpsOpCode::Push(3.0.to_dec32()),
                LpsOpCode::Push(2.0.to_dec32()),
                LpsOpCode::ModVec2,
                LpsOpCode::Return,
            ])
            .expect_result_vec2(Vec2::new(1.0.to_dec32(), 1.0.to_dec32()))
            .run()
    }

    // Vec2 * Scalar (broadcast scalar to all components)
    #[test]
    fn test_vec2_scalar_multiplication() -> Result<(), String> {
        ExprTest::new("vec2(1.0, 2.0) * 3.0")
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_dec32()),
                LpsOpCode::Push(2.0.to_dec32()),
                LpsOpCode::Push(3.0.to_dec32()),
                LpsOpCode::MulVec2Scalar,
                LpsOpCode::Return,
            ])
            .expect_result_vec2(Vec2::new(3.0.to_dec32(), 6.0.to_dec32()))
            .run()
    }

    // Scalar * Vec2 (commutative - broadcast scalar to all components)
    #[test]
    fn test_scalar_vec2_multiplication() -> Result<(), String> {
        ExprTest::new("2.0 * vec2(3.0, 4.0)")
            .expect_opcodes(vec![
                LpsOpCode::Push(3.0.to_dec32()),
                LpsOpCode::Push(4.0.to_dec32()),
                LpsOpCode::Push(2.0.to_dec32()),
                LpsOpCode::MulVec2Scalar,
                LpsOpCode::Return,
            ])
            .expect_result_vec2(Vec2::new(6.0.to_dec32(), 8.0.to_dec32()))
            .run()
    }

    // Vec2 / Scalar (broadcast scalar to all components)
    #[test]
    fn test_vec2_scalar_division() -> Result<(), String> {
        ExprTest::new("vec2(10.0, 20.0) / 2.0")
            .expect_opcodes(vec![
                LpsOpCode::Push(10.0.to_dec32()),
                LpsOpCode::Push(20.0.to_dec32()),
                LpsOpCode::Push(2.0.to_dec32()),
                LpsOpCode::DivVec2Scalar,
                LpsOpCode::Return,
            ])
            .expect_result_vec2(Vec2::new(5.0.to_dec32(), 10.0.to_dec32()))
            .run()
    }

    // Vec2 * Int32 scalar (should promote Int32 to Dec32)
    #[test]
    fn test_vec2_int32_scalar_multiplication() -> Result<(), String> {
        ExprTest::new("vec2(1.0, 2.0) * 3")
            .expect_result_vec2(Vec2::new(3.0.to_dec32(), 6.0.to_dec32()))
            .run()?;

        ExprTest::new("2 * vec2(3.0, 4.0)")
            .expect_result_vec2(Vec2::new(6.0.to_dec32(), 8.0.to_dec32()))
            .run()
    }

    #[test]
    fn test_edge_cases() -> Result<(), String> {
        // Zero vector
        ExprTest::new("vec2(0.0, 0.0) + vec2(1.0, 2.0)")
            .expect_result_vec2(Vec2::new(1.0.to_dec32(), 2.0.to_dec32()))
            .run()?;

        // Negative components
        ExprTest::new("vec2(-1.0, -2.0) + vec2(3.0, 4.0)")
            .expect_result_vec2(Vec2::new(2.0.to_dec32(), 2.0.to_dec32()))
            .run()?;

        // Multiply by zero
        ExprTest::new("vec2(1.0, 2.0) * 0.0")
            .expect_result_vec2(Vec2::new(0.0.to_dec32(), 0.0.to_dec32()))
            .run()
    }
}
