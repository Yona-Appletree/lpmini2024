/// Binary arithmetic operator tests for Vec3 type
#[cfg(test)]
mod tests {
    use lp_math::dec32::{ToDec32, Vec3};

    use crate::lp_script::compiler::expr::expr_test_util::ExprTest;
    use crate::lp_script::vm::opcodes::LpsOpCode;

    // Vec3 + Vec3 (component-wise addition)
    #[test]
    fn test_vec3_addition() -> Result<(), String> {
        ExprTest::new("vec3(1.0, 2.0, 3.0) + vec3(4.0, 5.0, 6.0)")
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_dec32()),
                LpsOpCode::Push(2.0.to_dec32()),
                LpsOpCode::Push(3.0.to_dec32()),
                LpsOpCode::Push(4.0.to_dec32()),
                LpsOpCode::Push(5.0.to_dec32()),
                LpsOpCode::Push(6.0.to_dec32()),
                LpsOpCode::AddVec3,
                LpsOpCode::Return,
            ])
            .expect_result_vec3(Vec3::new(5.0.to_dec32(), 7.0.to_dec32(), 9.0.to_dec32()))
            .run()
    }

    // Vec3 - Vec3 (component-wise subtraction)
    #[test]
    fn test_vec3_subtraction() -> Result<(), String> {
        ExprTest::new("vec3(10.0, 20.0, 30.0) - vec3(1.0, 2.0, 3.0)")
            .expect_opcodes(vec![
                LpsOpCode::Push(10.0.to_dec32()),
                LpsOpCode::Push(20.0.to_dec32()),
                LpsOpCode::Push(30.0.to_dec32()),
                LpsOpCode::Push(1.0.to_dec32()),
                LpsOpCode::Push(2.0.to_dec32()),
                LpsOpCode::Push(3.0.to_dec32()),
                LpsOpCode::SubVec3,
                LpsOpCode::Return,
            ])
            .expect_result_vec3(Vec3::new(9.0.to_dec32(), 18.0.to_dec32(), 27.0.to_dec32()))
            .run()
    }

    // Vec3 * Vec3 (component-wise multiplication)
    #[test]
    fn test_vec3_multiplication() -> Result<(), String> {
        ExprTest::new("vec3(2.0, 3.0, 4.0) * vec3(5.0, 6.0, 7.0)")
            .expect_opcodes(vec![
                LpsOpCode::Push(2.0.to_dec32()),
                LpsOpCode::Push(3.0.to_dec32()),
                LpsOpCode::Push(4.0.to_dec32()),
                LpsOpCode::Push(5.0.to_dec32()),
                LpsOpCode::Push(6.0.to_dec32()),
                LpsOpCode::Push(7.0.to_dec32()),
                LpsOpCode::MulVec3,
                LpsOpCode::Return,
            ])
            .expect_result_vec3(Vec3::new(10.0.to_dec32(), 18.0.to_dec32(), 28.0.to_dec32()))
            .run()
    }

    // Vec3 / Vec3 (component-wise division)
    #[test]
    fn test_vec3_division() -> Result<(), String> {
        ExprTest::new("vec3(12.0, 18.0, 24.0) / vec3(3.0, 6.0, 8.0)")
            .expect_opcodes(vec![
                LpsOpCode::Push(12.0.to_dec32()),
                LpsOpCode::Push(18.0.to_dec32()),
                LpsOpCode::Push(24.0.to_dec32()),
                LpsOpCode::Push(3.0.to_dec32()),
                LpsOpCode::Push(6.0.to_dec32()),
                LpsOpCode::Push(8.0.to_dec32()),
                LpsOpCode::DivVec3,
                LpsOpCode::Return,
            ])
            .expect_result_vec3(Vec3::new(4.0.to_dec32(), 3.0.to_dec32(), 3.0.to_dec32()))
            .run()
    }

    // Vec3 % Vec3 (component-wise modulo)
    #[test]
    fn test_vec3_modulo() -> Result<(), String> {
        ExprTest::new("vec3(10.0, 7.0, 15.0) % vec3(3.0, 2.0, 4.0)")
            .expect_opcodes(vec![
                LpsOpCode::Push(10.0.to_dec32()),
                LpsOpCode::Push(7.0.to_dec32()),
                LpsOpCode::Push(15.0.to_dec32()),
                LpsOpCode::Push(3.0.to_dec32()),
                LpsOpCode::Push(2.0.to_dec32()),
                LpsOpCode::Push(4.0.to_dec32()),
                LpsOpCode::ModVec3,
                LpsOpCode::Return,
            ])
            .expect_result_vec3(Vec3::new(1.0.to_dec32(), 1.0.to_dec32(), 3.0.to_dec32()))
            .run()
    }

    // Vec3 * Scalar (broadcast scalar to all components)
    #[test]
    fn test_vec3_scalar_multiplication() -> Result<(), String> {
        ExprTest::new("vec3(2.0, 3.0, 4.0) * 2.0")
            .expect_opcodes(vec![
                LpsOpCode::Push(2.0.to_dec32()),
                LpsOpCode::Push(3.0.to_dec32()),
                LpsOpCode::Push(4.0.to_dec32()),
                LpsOpCode::Push(2.0.to_dec32()),
                LpsOpCode::MulVec3Scalar,
                LpsOpCode::Return,
            ])
            .expect_result_vec3(Vec3::new(4.0.to_dec32(), 6.0.to_dec32(), 8.0.to_dec32()))
            .run()
    }

    // Scalar * Vec3 (commutative - broadcast scalar to all components)
    #[test]
    fn test_scalar_vec3_multiplication() -> Result<(), String> {
        ExprTest::new("3.0 * vec3(1.0, 2.0, 3.0)")
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_dec32()),
                LpsOpCode::Push(2.0.to_dec32()),
                LpsOpCode::Push(3.0.to_dec32()),
                LpsOpCode::Push(3.0.to_dec32()),
                LpsOpCode::MulVec3Scalar,
                LpsOpCode::Return,
            ])
            .expect_result_vec3(Vec3::new(3.0.to_dec32(), 6.0.to_dec32(), 9.0.to_dec32()))
            .run()
    }

    // Vec3 / Scalar (broadcast scalar to all components)
    #[test]
    fn test_vec3_scalar_division() -> Result<(), String> {
        ExprTest::new("vec3(6.0, 9.0, 12.0) / 3.0")
            .expect_opcodes(vec![
                LpsOpCode::Push(6.0.to_dec32()),
                LpsOpCode::Push(9.0.to_dec32()),
                LpsOpCode::Push(12.0.to_dec32()),
                LpsOpCode::Push(3.0.to_dec32()),
                LpsOpCode::DivVec3Scalar,
                LpsOpCode::Return,
            ])
            .expect_result_vec3(Vec3::new(2.0.to_dec32(), 3.0.to_dec32(), 4.0.to_dec32()))
            .run()
    }

    // Vec3 * Int32 scalar (should promote Int32 to Dec32)
    #[test]
    fn test_vec3_int32_scalar_multiplication() -> Result<(), String> {
        ExprTest::new("vec3(1.0, 2.0, 3.0) * 2")
            .expect_result_vec3(Vec3::new(2.0.to_dec32(), 4.0.to_dec32(), 6.0.to_dec32()))
            .run()?;

        ExprTest::new("3 * vec3(1.0, 2.0, 3.0)")
            .expect_result_vec3(Vec3::new(3.0.to_dec32(), 6.0.to_dec32(), 9.0.to_dec32()))
            .run()
    }

    #[test]
    fn test_edge_cases() -> Result<(), String> {
        // Zero vector
        ExprTest::new("vec3(0.0, 0.0, 0.0) + vec3(1.0, 2.0, 3.0)")
            .expect_result_vec3(Vec3::new(1.0.to_dec32(), 2.0.to_dec32(), 3.0.to_dec32()))
            .run()?;

        // Negative components
        ExprTest::new("vec3(-1.0, -2.0, -3.0) + vec3(3.0, 4.0, 5.0)")
            .expect_result_vec3(Vec3::new(2.0.to_dec32(), 2.0.to_dec32(), 2.0.to_dec32()))
            .run()?;

        // Multiply by zero
        ExprTest::new("vec3(1.0, 2.0, 3.0) * 0.0")
            .expect_result_vec3(Vec3::new(0.0.to_dec32(), 0.0.to_dec32(), 0.0.to_dec32()))
            .run()
    }
}
