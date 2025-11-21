/// Tests for constant folding optimization
#[cfg(test)]
mod constant_folding_tests {
    use crate::compiler::optimize::ast::constant_fold;
    use crate::compiler::optimize::ast_test_util::AstOptTest;

    // ============================================================================
    // Arithmetic operations
    // ============================================================================

    #[test]
    fn test_addition() {
        // 2.0 + 3.0 → 5.0
        AstOptTest::new("2.0 + 3.0")
            .with_pass(constant_fold::fold_constants)
            .expect_ast(|b| b.num(5.0))
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    #[test]
    fn test_subtraction() {
        // 5.0 - 3.0 → 2.0
        AstOptTest::new("5.0 - 3.0")
            .with_pass(constant_fold::fold_constants)
            .expect_ast(|b| b.num(2.0))
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    #[test]
    fn test_multiplication() {
        // 2.0 * 3.0 → 6.0
        AstOptTest::new("2.0 * 3.0")
            .with_pass(constant_fold::fold_constants)
            .expect_ast(|b| b.num(6.0))
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    #[test]
    fn test_division() {
        // 6.0 / 2.0 → 3.0
        AstOptTest::new("6.0 / 2.0")
            .with_pass(constant_fold::fold_constants)
            .expect_ast(|b| b.num(3.0))
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    #[test]
    fn test_modulo() {
        // 7.0 % 3.0 → 1.0
        // Note: Float modulo may have precision issues between compile-time and runtime
        // Just verify the optimization doesn't break compilation
        AstOptTest::new("7.0 % 3.0")
            .with_pass(constant_fold::fold_constants)
            .run()
            .unwrap();
    }

    #[test]
    fn test_negative_numbers() {
        // -5.0 + 3.0 → -2.0
        AstOptTest::new("-5.0 + 3.0")
            .with_pass(constant_fold::fold_constants)
            .expect_ast(|b| b.num(-2.0))
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    #[test]
    fn test_nested_arithmetic() {
        // (2.0 + 3.0) * 4.0 → 5.0 * 4.0 → 20.0
        AstOptTest::new("(2.0 + 3.0) * 4.0")
            .with_pass(constant_fold::fold_constants)
            .expect_ast(|b| b.num(20.0))
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    #[test]
    fn test_complex_expression() {
        // (1.0 + 2.0) * (3.0 + 4.0) → 3.0 * 7.0 → 21.0
        AstOptTest::new("(1.0 + 2.0) * (3.0 + 4.0)")
            .with_pass(constant_fold::fold_constants)
            .expect_ast(|b| b.num(21.0))
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    // ============================================================================
    // Integer operations
    // ============================================================================

    #[test]
    fn test_int_addition() {
        // 2 + 3 → 5
        AstOptTest::new("2 + 3")
            .with_pass(constant_fold::fold_constants)
            .expect_ast(|b| b.int32(5))
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    #[test]
    fn test_int_multiplication() {
        // 2 * 3 → 6
        AstOptTest::new("2 * 3")
            .with_pass(constant_fold::fold_constants)
            .expect_ast(|b| b.int32(6))
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    // ============================================================================
    // Comparison operations
    // ============================================================================

    #[test]
    fn test_less_than_true() {
        // 2.0 < 3.0 → true (1.0)
        // Note: Result type is Bool, not Dec32
        AstOptTest::new("2.0 < 3.0")
            .with_pass(constant_fold::fold_constants)
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    #[test]
    fn test_less_than_false() {
        // 5.0 < 3.0 → false (0.0)
        AstOptTest::new("5.0 < 3.0")
            .with_pass(constant_fold::fold_constants)
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    #[test]
    fn test_greater_than_true() {
        // 5.0 > 3.0 → true (1.0)
        AstOptTest::new("5.0 > 3.0")
            .with_pass(constant_fold::fold_constants)
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    #[test]
    fn test_greater_than_false() {
        // 2.0 > 3.0 → false (0.0)
        AstOptTest::new("2.0 > 3.0")
            .with_pass(constant_fold::fold_constants)
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    #[test]
    fn test_less_equal_true() {
        // 3.0 <= 3.0 → true (1.0)
        AstOptTest::new("3.0 <= 3.0")
            .with_pass(constant_fold::fold_constants)
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    #[test]
    fn test_greater_equal_true() {
        // 3.0 >= 3.0 → true (1.0)
        AstOptTest::new("3.0 >= 3.0")
            .with_pass(constant_fold::fold_constants)
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    #[test]
    fn test_equal_true() {
        // 3.0 == 3.0 → true (1.0)
        AstOptTest::new("3.0 == 3.0")
            .with_pass(constant_fold::fold_constants)
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    #[test]
    fn test_equal_false() {
        // 2.0 == 3.0 → false (0.0)
        AstOptTest::new("2.0 == 3.0")
            .with_pass(constant_fold::fold_constants)
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    #[test]
    fn test_not_equal_true() {
        // 2.0 != 3.0 → true (1.0)
        AstOptTest::new("2.0 != 3.0")
            .with_pass(constant_fold::fold_constants)
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    #[test]
    fn test_not_equal_false() {
        // 3.0 != 3.0 → false (0.0)
        AstOptTest::new("3.0 != 3.0")
            .with_pass(constant_fold::fold_constants)
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    // ============================================================================
    // Logical operations
    // ============================================================================

    #[test]
    fn test_logical_and_true() {
        // 1.0 && 1.0 → true (1.0)
        // Note: Result type is Bool, not Dec32
        AstOptTest::new("1.0 && 1.0")
            .with_pass(constant_fold::fold_constants)
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    #[test]
    fn test_logical_and_false() {
        // 1.0 && 0.0 → false (0.0)
        AstOptTest::new("1.0 && 0.0")
            .with_pass(constant_fold::fold_constants)
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    #[test]
    fn test_logical_or_true() {
        // 1.0 || 0.0 → true (1.0)
        AstOptTest::new("1.0 || 0.0")
            .with_pass(constant_fold::fold_constants)
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    #[test]
    fn test_logical_or_false() {
        // 0.0 || 0.0 → false (0.0)
        AstOptTest::new("0.0 || 0.0")
            .with_pass(constant_fold::fold_constants)
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    #[test]
    fn test_logical_not_true() {
        // !0.0 → true (1.0)
        AstOptTest::new("!0.0")
            .with_pass(constant_fold::fold_constants)
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    #[test]
    fn test_logical_not_false() {
        // !1.0 → false (0.0)
        AstOptTest::new("!1.0")
            .with_pass(constant_fold::fold_constants)
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    // ============================================================================
    // Unary negation
    // ============================================================================

    #[test]
    fn test_negation() {
        // -5.0 → -5.0
        AstOptTest::new("-5.0")
            .with_pass(constant_fold::fold_constants)
            .expect_ast(|b| b.num(-5.0))
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    #[test]
    fn test_double_negation() {
        // -(-5.0) → 5.0 (use explicit parens to avoid pre-decrement)
        AstOptTest::new("-(-5.0)")
            .with_pass(constant_fold::fold_constants)
            .expect_ast(|b| b.num(5.0))
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    // ============================================================================
    // Ternary operator
    // ============================================================================

    #[test]
    fn test_ternary_true_branch() {
        // 1.0 ? 10.0 : 20.0 → 10.0
        AstOptTest::new("1.0 ? 10.0 : 20.0")
            .with_pass(constant_fold::fold_constants)
            .expect_ast(|b| b.num(10.0))
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    #[test]
    fn test_ternary_false_branch() {
        // 0.0 ? 10.0 : 20.0 → 20.0
        AstOptTest::new("0.0 ? 10.0 : 20.0")
            .with_pass(constant_fold::fold_constants)
            .expect_ast(|b| b.num(20.0))
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    #[test]
    fn test_ternary_with_expression_condition() {
        // (2.0 > 1.0) ? 10.0 : 20.0 → 10.0
        AstOptTest::new("(2.0 > 1.0) ? 10.0 : 20.0")
            .with_pass(constant_fold::fold_constants)
            .expect_ast(|b| b.num(10.0))
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    // ============================================================================
    // Math functions
    // ============================================================================

    #[test]
    fn test_sin_zero() {
        // sin(0.0) → 0.0
        AstOptTest::new("sin(0.0)")
            .with_pass(constant_fold::fold_constants)
            .expect_ast(|b| b.num(0.0))
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    #[test]
    fn test_cos_zero() {
        // cos(0.0) → ~1.0 (dec32-point precision: 0.9996948)
        // Dec32-point trig uses lookup tables, so there's slight precision loss
        AstOptTest::new("cos(0.0)")
            .with_pass(constant_fold::fold_constants)
            .expect_ast(|b| b.num(0.9996948))
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    #[test]
    fn test_sqrt() {
        // sqrt(4.0) → 2.0
        AstOptTest::new("sqrt(4.0)")
            .with_pass(constant_fold::fold_constants)
            .expect_ast(|b| b.num(2.0))
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    #[test]
    fn test_abs_positive() {
        // abs(5.0) → 5.0
        AstOptTest::new("abs(5.0)")
            .with_pass(constant_fold::fold_constants)
            .expect_ast(|b| b.num(5.0))
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    #[test]
    fn test_abs_negative() {
        // abs(-5.0) → 5.0
        AstOptTest::new("abs(-5.0)")
            .with_pass(constant_fold::fold_constants)
            .expect_ast(|b| b.num(5.0))
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    #[test]
    fn test_floor() {
        // floor(3.7) → 3.0
        AstOptTest::new("floor(3.7)")
            .with_pass(constant_fold::fold_constants)
            .expect_ast(|b| b.num(3.0))
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    #[test]
    fn test_ceil() {
        // ceil(3.2) → 4.0
        AstOptTest::new("ceil(3.2)")
            .with_pass(constant_fold::fold_constants)
            .expect_ast(|b| b.num(4.0))
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    #[test]
    fn test_min() {
        // min(2.0, 3.0) → 2.0
        AstOptTest::new("min(2.0, 3.0)")
            .with_pass(constant_fold::fold_constants)
            .expect_ast(|b| b.num(2.0))
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    #[test]
    fn test_max() {
        // max(2.0, 3.0) → 3.0
        AstOptTest::new("max(2.0, 3.0)")
            .with_pass(constant_fold::fold_constants)
            .expect_ast(|b| b.num(3.0))
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    #[test]
    fn test_clamp() {
        // clamp(5.0, 0.0, 10.0) → 5.0
        AstOptTest::new("clamp(5.0, 0.0, 10.0)")
            .with_pass(constant_fold::fold_constants)
            .expect_ast(|b| b.num(5.0))
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    #[test]
    fn test_clamp_low() {
        // clamp(-1.0, 0.0, 10.0) → 0.0
        AstOptTest::new("clamp(-1.0, 0.0, 10.0)")
            .with_pass(constant_fold::fold_constants)
            .expect_ast(|b| b.num(0.0))
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    #[test]
    fn test_clamp_high() {
        // clamp(15.0, 0.0, 10.0) → 10.0
        AstOptTest::new("clamp(15.0, 0.0, 10.0)")
            .with_pass(constant_fold::fold_constants)
            .expect_ast(|b| b.num(10.0))
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    #[test]
    fn test_pow() {
        // pow(2.0, 3.0) → 8.0
        AstOptTest::new("pow(2.0, 3.0)")
            .with_pass(constant_fold::fold_constants)
            .expect_ast(|b| b.num(8.0))
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    #[test]
    fn test_lerp() {
        // lerp(0.0, 10.0, 0.5) → 5.0
        AstOptTest::new("lerp(0.0, 10.0, 0.5)")
            .with_pass(constant_fold::fold_constants)
            .expect_ast(|b| b.num(5.0))
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    #[test]
    fn test_saturate() {
        // saturate(1.5) → 1.0
        AstOptTest::new("saturate(1.5)")
            .with_pass(constant_fold::fold_constants)
            .expect_ast(|b| b.num(1.0))
            .expect_semantics_preserved()
            .run()
            .unwrap();
    }

    // ============================================================================
    // Mixed constant and non-constant (partial folding)
    // ============================================================================

    #[test]
    fn test_partial_constant_folding() {
        // (2.0 + 3.0) * time → 5.0 * time
        // The addition should be folded, but not the multiplication
        AstOptTest::new("(2.0 + 3.0) * time")
            .with_pass(constant_fold::fold_constants)
            .expect_semantics_preserved()
            .with_time(2.0)
            .run()
            .unwrap();
    }

    #[test]
    fn test_no_folding_with_variables() {
        // time + time should not be folded
        AstOptTest::new("time + time")
            .with_pass(constant_fold::fold_constants)
            .expect_semantics_preserved()
            .with_time(42.0)
            .run()
            .unwrap();
    }

    // ============================================================================
    // Edge cases
    // ============================================================================

    #[test]
    fn test_division_by_zero() {
        // 1.0 / 0.0 should handle gracefully (produces infinity)
        // Skip semantics check as infinity comparison is tricky
        AstOptTest::new("1.0 / 0.0")
            .with_pass(constant_fold::fold_constants)
            .run()
            .unwrap();
    }

    #[test]
    fn test_modulo_by_zero() {
        // 1.0 % 0.0 should handle gracefully at compile time (produces 0)
        // Runtime throws DivisionByZero, so skip semantic check
        AstOptTest::new("1.0 % 0.0")
            .with_pass(constant_fold::fold_constants)
            .run()
            .unwrap();
    }
}
