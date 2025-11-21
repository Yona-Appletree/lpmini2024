/// Binary arithmetic operator tests for Mat3 type
#[cfg(test)]
mod tests {
    use crate::compiler::expr::expr_test_util::ExprTest;
    use crate::dec32::Mat3;

    // Mat3 + Mat3 (component-wise addition)
    #[test]
    fn test_mat3_addition() -> Result<(), String> {
        ExprTest::new("mat3(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0) + mat3(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0)")
            .expect_result_mat3(Mat3::from_f32(
                2.0, 3.0, 4.0,
                5.0, 6.0, 7.0,
                8.0, 9.0, 10.0,
            ))
            .run()
    }

    // Mat3 - Mat3 (component-wise subtraction)
    #[test]
    fn test_mat3_subtraction() -> Result<(), String> {
        ExprTest::new("mat3(10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0) - mat3(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0)")
            .expect_result_mat3(Mat3::from_f32(
                9.0, 7.0, 5.0,
                3.0, 1.0, -1.0,
                -3.0, -5.0, -7.0,
            ))
            .run()
    }

    // Mat3 * Mat3 (matrix multiplication)
    #[test]
    fn test_mat3_matrix_multiplication() -> Result<(), String> {
        // Identity * Identity = Identity
        ExprTest::new("mat3(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0) * mat3(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0)")
            .expect_result_mat3(Mat3::identity())
            .run()
    }

    // Mat3 * Scalar (scalar multiplication)
    #[test]
    fn test_mat3_scalar_multiplication() -> Result<(), String> {
        ExprTest::new("mat3(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0) * 2.0")
            .expect_result_mat3(Mat3::from_f32(
                2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 18.0,
            ))
            .run()
    }

    // Scalar * Mat3 (scalar multiplication)
    #[test]
    fn test_scalar_mat3_multiplication() -> Result<(), String> {
        ExprTest::new("3.0 * mat3(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0)")
            .expect_result_mat3(Mat3::from_f32(
                3.0, 6.0, 9.0, 12.0, 15.0, 18.0, 21.0, 24.0, 27.0,
            ))
            .run()
    }

    // Mat3 / Scalar (scalar division)
    #[test]
    fn test_mat3_scalar_division() -> Result<(), String> {
        ExprTest::new("mat3(2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 18.0) / 2.0")
            .expect_result_mat3(Mat3::from_f32(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0))
            .run()
    }

    // Mat3 * Mat3 (more complex matrix multiplication)
    #[test]
    fn test_mat3_matrix_multiplication_complex() -> Result<(), String> {
        // Test non-identity multiplication
        ExprTest::new("mat3(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0) * mat3(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0)")
            .expect_result_mat3(Mat3::from_f32(
                1.0, 2.0, 3.0,
                4.0, 5.0, 6.0,
                7.0, 8.0, 9.0,
            ))
            .run()
    }

    // Mat3 / Scalar edge cases
    #[test]
    fn test_mat3_scalar_division_negative() -> Result<(), String> {
        ExprTest::new("mat3(-2.0, -4.0, -6.0, -8.0, -10.0, -12.0, -14.0, -16.0, -18.0) / -2.0")
            .expect_result_mat3(Mat3::from_f32(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0))
            .run()
    }

    // Mat3 * Scalar edge cases
    #[test]
    fn test_mat3_scalar_multiplication_negative() -> Result<(), String> {
        ExprTest::new("mat3(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0) * -2.0")
            .expect_result_mat3(Mat3::from_f32(
                -2.0, -4.0, -6.0, -8.0, -10.0, -12.0, -14.0, -16.0, -18.0,
            ))
            .run()
    }

    #[test]
    fn test_mat3_scalar_multiplication_zero() -> Result<(), String> {
        ExprTest::new("mat3(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0) * 0.0")
            .expect_result_mat3(Mat3::zero())
            .run()
    }

    #[test]
    fn test_mat3_scalar_multiplication_one() -> Result<(), String> {
        ExprTest::new("mat3(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0) * 1.0")
            .expect_result_mat3(Mat3::from_f32(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0))
            .run()
    }

    // Chained operations
    #[test]
    fn test_mat3_chained_operations() -> Result<(), String> {
        ExprTest::new("mat3(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0) + mat3(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0) - mat3(1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)")
            .expect_result_mat3(Mat3::from_f32(
                1.0, 3.0, 4.0,
                5.0, 6.0, 7.0,
                8.0, 9.0, 10.0,
            ))
            .run()
    }

    #[test]
    fn test_edge_cases() -> Result<(), String> {
        // Zero matrix
        ExprTest::new("mat3(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0) + mat3(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0)")
            .expect_result_mat3(Mat3::from_f32(
                1.0, 2.0, 3.0,
                4.0, 5.0, 6.0,
                7.0, 8.0, 9.0,
            ))
            .run()?;

        // Identity matrix
        ExprTest::new("mat3(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0) * 5.0")
            .expect_result_mat3(Mat3::from_f32(5.0, 0.0, 0.0, 0.0, 5.0, 0.0, 0.0, 0.0, 5.0))
            .run()?;

        // Negative matrix
        ExprTest::new("-mat3(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0) + mat3(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0)")
            .expect_result_mat3(Mat3::zero())
            .run()
    }
}
