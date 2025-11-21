/// Function call tests for Mat3 type
#[cfg(test)]
mod tests {
    use crate::compiler::expr::expr_test_util::ExprTest;
    use crate::dec32::{Mat3, ToDec32};
    use crate::vm::opcodes::LpsOpCode;

    #[test]
    fn test_transpose() -> Result<(), String> {
        // Transpose of identity is identity
        ExprTest::new("transpose(mat3(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0))")
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_dec32()),
                LpsOpCode::Push(0.0.to_dec32()),
                LpsOpCode::Push(0.0.to_dec32()),
                LpsOpCode::Push(0.0.to_dec32()),
                LpsOpCode::Push(1.0.to_dec32()),
                LpsOpCode::Push(0.0.to_dec32()),
                LpsOpCode::Push(0.0.to_dec32()),
                LpsOpCode::Push(0.0.to_dec32()),
                LpsOpCode::Push(1.0.to_dec32()),
                LpsOpCode::TransposeMat3,
                LpsOpCode::Return,
            ])
            .expect_result_mat3(Mat3::identity())
            .run()?;

        // Transpose of a non-symmetric matrix
        ExprTest::new("transpose(mat3(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0))")
            .expect_result_mat3(Mat3::from_f32(1.0, 4.0, 7.0, 2.0, 5.0, 8.0, 3.0, 6.0, 9.0))
            .run()
    }

    #[test]
    fn test_determinant() -> Result<(), String> {
        // Determinant of identity is 1
        ExprTest::new("determinant(mat3(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0))")
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_dec32()),
                LpsOpCode::Push(0.0.to_dec32()),
                LpsOpCode::Push(0.0.to_dec32()),
                LpsOpCode::Push(0.0.to_dec32()),
                LpsOpCode::Push(1.0.to_dec32()),
                LpsOpCode::Push(0.0.to_dec32()),
                LpsOpCode::Push(0.0.to_dec32()),
                LpsOpCode::Push(0.0.to_dec32()),
                LpsOpCode::Push(1.0.to_dec32()),
                LpsOpCode::DeterminantMat3,
                LpsOpCode::Return,
            ])
            .expect_result_dec32(1.0)
            .run()?;

        // Determinant of zero matrix is 0
        ExprTest::new("determinant(mat3(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0))")
            .expect_result_dec32(0.0)
            .run()
    }

    #[test]
    fn test_inverse() -> Result<(), String> {
        // Inverse of identity is identity
        ExprTest::new("inverse(mat3(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0))")
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_dec32()),
                LpsOpCode::Push(0.0.to_dec32()),
                LpsOpCode::Push(0.0.to_dec32()),
                LpsOpCode::Push(0.0.to_dec32()),
                LpsOpCode::Push(1.0.to_dec32()),
                LpsOpCode::Push(0.0.to_dec32()),
                LpsOpCode::Push(0.0.to_dec32()),
                LpsOpCode::Push(0.0.to_dec32()),
                LpsOpCode::Push(1.0.to_dec32()),
                LpsOpCode::InverseMat3,
                LpsOpCode::Return,
            ])
            .expect_result_mat3(Mat3::identity())
            .run()
    }

    #[test]
    fn test_transpose_symmetric() -> Result<(), String> {
        // Transpose of symmetric matrix is itself
        ExprTest::new("transpose(mat3(1.0, 2.0, 3.0, 2.0, 4.0, 5.0, 3.0, 5.0, 6.0))")
            .expect_result_mat3(Mat3::from_f32(1.0, 2.0, 3.0, 2.0, 4.0, 5.0, 3.0, 5.0, 6.0))
            .run()
    }

    #[test]
    fn test_determinant_non_zero() -> Result<(), String> {
        // Determinant of a simple matrix
        ExprTest::new("determinant(mat3(2.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 2.0))")
            .expect_result_dec32(8.0)
            .run()
    }

    #[test]
    fn test_determinant_negative() -> Result<(), String> {
        // Determinant can be negative
        ExprTest::new("determinant(mat3(-1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0))")
            .expect_result_dec32(-1.0)
            .run()
    }

    #[test]
    fn test_chained_matrix_operations() -> Result<(), String> {
        // Transpose then multiply
        ExprTest::new("transpose(mat3(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0)) * mat3(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0)")
            .expect_result_mat3(Mat3::from_f32(
                1.0, 4.0, 7.0,
                2.0, 5.0, 8.0,
                3.0, 6.0, 9.0,
            ))
            .run()
    }

    #[test]
    fn test_determinant_after_operations() -> Result<(), String> {
        // Determinant of scaled identity
        ExprTest::new("determinant(mat3(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0) * 2.0)")
            .expect_result_dec32(8.0)
            .run()
    }
}
