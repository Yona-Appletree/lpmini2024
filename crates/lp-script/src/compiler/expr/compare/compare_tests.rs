/// Integration tests for comparison operators using builder pattern test utilities
#[cfg(test)]
mod tests {
    use crate::compiler::expr::expr_test_util::ExprTest;
    use crate::fixed::ToFixed;
    use crate::vm::opcodes::LpsOpCode;

    // ========================================================================
    // Comprehensive tests - one per comparison operator
    // Each tests AST, opcodes, and execution together
    // ========================================================================

    #[test]
    fn test_less_than() -> Result<(), String> {
        // Test with literals: AST + opcodes + result
        ExprTest::new("1.0 < 2.0")
            .expect_ast(|b| {
                let left = b.num(1.0);
                let right = b.num(2.0);
                b.less(left, right)
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::LessFixed,
                LpsOpCode::Return,
            ])
            .expect_result_bool(true)
            .run()?;

        // Test with variable: true case
        ExprTest::new("x < 0.5")
            .with_x(0.3)
            .expect_result_bool(true)
            .run()?;

        // Test with variable: false case
        ExprTest::new("x < 0.5")
            .with_x(0.7)
            .expect_result_bool(false)
            .run()
    }

    #[test]
    fn test_greater_than() -> Result<(), String> {
        // Test with literals: AST + opcodes + result
        ExprTest::new("5.0 > 3.0")
            .expect_ast(|b| {
                let left = b.num(5.0);
                let right = b.num(3.0);
                b.greater(left, right)
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(5.0.to_fixed()),
                LpsOpCode::Push(3.0.to_fixed()),
                LpsOpCode::GreaterFixed,
                LpsOpCode::Return,
            ])
            .expect_result_bool(true)
            .run()?;

        // Test with variable: true case
        ExprTest::new("x > 0.5")
            .with_x(0.6)
            .expect_result_bool(true)
            .run()?;

        // Test with variable: false case
        ExprTest::new("x > 0.5")
            .with_x(0.4)
            .expect_result_bool(false)
            .run()
    }

    #[test]
    fn test_less_equal() -> Result<(), String> {
        // Test with literals: AST + opcodes + result
        ExprTest::new("2.0 <= 3.0")
            .expect_ast(|b| {
                let left = b.num(2.0);
                let right = b.num(3.0);
                b.less_eq(left, right)
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::Push(3.0.to_fixed()),
                LpsOpCode::LessEqFixed,
                LpsOpCode::Return,
            ])
            .expect_result_bool(true)
            .run()?;

        // Test with variable: equal case (should be true)
        ExprTest::new("x <= 0.5")
            .with_x(0.5)
            .expect_result_bool(true)
            .run()?;

        // Test with variable: less case (should be true)
        ExprTest::new("x <= 0.5")
            .with_x(0.3)
            .expect_result_bool(true)
            .run()?;

        // Test with variable: greater case (should be false)
        ExprTest::new("x <= 0.5")
            .with_x(0.7)
            .expect_result_bool(false)
            .run()
    }

    #[test]
    fn test_greater_equal() -> Result<(), String> {
        // Test with literals: AST + opcodes + result
        ExprTest::new("5.0 >= 3.0")
            .expect_ast(|b| {
                let left = b.num(5.0);
                let right = b.num(3.0);
                b.greater_eq(left, right)
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(5.0.to_fixed()),
                LpsOpCode::Push(3.0.to_fixed()),
                LpsOpCode::GreaterEqFixed,
                LpsOpCode::Return,
            ])
            .expect_result_bool(true)
            .run()?;

        // Test with variable: equal case (should be true)
        ExprTest::new("x >= 0.5")
            .with_x(0.5)
            .expect_result_bool(true)
            .run()?;

        // Test with variable: greater case (should be true)
        ExprTest::new("x >= 0.5")
            .with_x(0.7)
            .expect_result_bool(true)
            .run()?;

        // Test with variable: less case (should be false)
        ExprTest::new("x >= 0.5")
            .with_x(0.3)
            .expect_result_bool(false)
            .run()
    }

    #[test]
    fn test_equal() -> Result<(), String> {
        // Test with literals: AST + opcodes + result
        ExprTest::new("2.0 == 2.0")
            .expect_ast(|b| {
                let left = b.num(2.0);
                let right = b.num(2.0);
                b.eq(left, right)
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::EqFixed,
                LpsOpCode::Return,
            ])
            .expect_result_bool(true)
            .run()?;

        // Test with variable: equal case
        ExprTest::new("x == 0.5")
            .with_x(0.5)
            .expect_result_bool(true)
            .run()?;

        // Test with variable: not equal case
        ExprTest::new("x == 0.5")
            .with_x(0.3)
            .expect_result_bool(false)
            .run()
    }

    #[test]
    fn test_not_equal() -> Result<(), String> {
        // Test with literals: AST + opcodes + result
        ExprTest::new("2.0 != 3.0")
            .expect_ast(|b| {
                let left = b.num(2.0);
                let right = b.num(3.0);
                b.not_eq(left, right)
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::Push(3.0.to_fixed()),
                LpsOpCode::NotEqFixed,
                LpsOpCode::Return,
            ])
            .expect_result_bool(true)
            .run()?;

        // Test with variable: not equal case
        ExprTest::new("x != 0.5")
            .with_x(0.3)
            .expect_result_bool(true)
            .run()?;

        // Test with variable: equal case
        ExprTest::new("x != 0.5")
            .with_x(0.5)
            .expect_result_bool(false)
            .run()
    }

    // ========================================================================
    // Higher-order tests combining multiple comparisons
    // ========================================================================

    #[test]
    fn test_comparison_with_logical_and() -> Result<(), String> {
        // Test chained comparisons: x in range (0.3, 0.7)
        ExprTest::new("x > 0.3 && x < 0.7")
            .with_x(0.5)
            .expect_result_bool(true)
            .run()?;

        // Value below range
        ExprTest::new("x > 0.3 && x < 0.7")
            .with_x(0.2)
            .expect_result_bool(false)
            .run()?;

        // Value above range
        ExprTest::new("x > 0.3 && x < 0.7")
            .with_x(0.8)
            .expect_result_bool(false)
            .run()
    }

    #[test]
    fn test_comparison_with_ternary() -> Result<(), String> {
        // Comparison used in ternary condition
        ExprTest::new("x > 0.5 ? 1.0 : 0.0")
            .with_x(0.6)
            .expect_result_fixed(1.0)
            .run()?;

        ExprTest::new("x > 0.5 ? 1.0 : 0.0")
            .with_x(0.4)
            .expect_result_fixed(0.0)
            .run()?;

        // Complex: chained comparisons in ternary
        ExprTest::new("x > 0.3 && x < 0.7 ? 10.0 : -10.0")
            .with_x(0.5)
            .expect_result_fixed(10.0)
            .run()?;

        ExprTest::new("x > 0.3 && x < 0.7 ? 10.0 : -10.0")
            .with_x(0.2)
            .expect_result_fixed(-10.0)
            .run()
    }

    #[test]
    fn test_multiple_comparison_types() -> Result<(), String> {
        // Mix different comparison operators
        ExprTest::new("x >= 0.0 && x <= 1.0")
            .with_x(0.5)
            .expect_result_bool(true)
            .run()?;

        // Inequality with equality
        ExprTest::new("x != 0.0 && y == 1.0")
            .with_x(0.5)
            .with_y(1.0)
            .expect_result_bool(true)
            .run()?;

        ExprTest::new("x != 0.0 && y == 1.0")
            .with_x(0.0)
            .with_y(1.0)
            .expect_result_bool(false)
            .run()
    }
}
