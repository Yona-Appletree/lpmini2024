/// Unary negation tests for Mat3 type
#[cfg(test)]
mod tests {
    use crate::compiler::expr::expr_test_util::ExprTest;
    use crate::fixed::{Mat3, ToFixed};
    use crate::shared::Type;
    use crate::vm::opcodes::LpsOpCode;

    #[test]
    fn test_negation() -> Result<(), String> {
        ExprTest::new("-mat3(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0)")
            .expect_ast(|b| {
                let args: Vec<_> = (1..=9).map(|i| b.num(i as f32)).collect();
                let operand = b.mat3(args);
                b.neg(operand, Type::Mat3)
            })
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
                LpsOpCode::NegMat3,
                LpsOpCode::Return,
            ])
            .expect_result_mat3(Mat3::from_f32(
                -1.0, -2.0, -3.0, -4.0, -5.0, -6.0, -7.0, -8.0, -9.0,
            ))
            .run()
    }

    #[test]
    fn test_negation_zero() -> Result<(), String> {
        ExprTest::new("-mat3(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)")
            .expect_result_mat3(Mat3::zero())
            .run()
    }

    #[test]
    fn test_negation_identity() -> Result<(), String> {
        ExprTest::new("-mat3(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0)")
            .expect_result_mat3(Mat3::from_f32(
                -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0,
            ))
            .run()
    }

    #[test]
    fn test_negation_double() -> Result<(), String> {
        ExprTest::new("--mat3(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0)")
            .expect_result_mat3(Mat3::from_f32(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0))
            .run()
    }

    #[test]
    fn test_negation_negative_values() -> Result<(), String> {
        ExprTest::new("-mat3(-1.0, -2.0, -3.0, -4.0, -5.0, -6.0, -7.0, -8.0, -9.0)")
            .expect_result_mat3(Mat3::from_f32(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0))
            .run()
    }

    #[test]
    fn test_negation_in_expression() -> Result<(), String> {
        // Negation combined with addition
        ExprTest::new("-mat3(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0) + mat3(2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 18.0)")
            .expect_result_mat3(Mat3::from_f32(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0))
            .run()
    }
}
