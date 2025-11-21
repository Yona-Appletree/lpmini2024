/// Tests for algebraic simplification optimization
#[cfg(test)]
mod algebraic_simplification_tests {
    use crate::compiler::optimize::ast::algebraic;
    use crate::compiler::optimize::ast_test_util::AstOptTest;

    // ============================================================================
    // Addition identities
    // ============================================================================

    #[test]
    fn test_x_plus_zero() {
        // x + 0 → x
        AstOptTest::new("time + 0.0")
            .with_pass(algebraic::simplify_expr)
            .expect_ast(|b| b.var("time"))
            .expect_semantics_preserved()
            .with_time(42.0)
            .run()
            .unwrap();
    }

    #[test]
    fn test_zero_plus_x() {
        // 0 + x → x
        AstOptTest::new("0.0 + time")
            .with_pass(algebraic::simplify_expr)
            .expect_ast(|b| b.var("time"))
            .expect_semantics_preserved()
            .with_time(42.0)
            .run()
            .unwrap();
    }

    #[test]
    fn test_time_plus_zero_int() {
        // time + 0 → time (with int literal)
        AstOptTest::new("time + 0")
            .with_pass(algebraic::simplify_expr)
            .expect_semantics_preserved()
            .with_time(5.0)
            .run()
            .unwrap();
    }

    // ============================================================================
    // Subtraction identities
    // ============================================================================

    #[test]
    fn test_x_minus_zero() {
        // x - 0 → x
        AstOptTest::new("time - 0.0")
            .with_pass(algebraic::simplify_expr)
            .expect_ast(|b| b.var("time"))
            .expect_semantics_preserved()
            .with_time(42.0)
            .run()
            .unwrap();
    }

    #[test]
    fn test_time_minus_zero_int() {
        // time - 0 → time (with int literal)
        AstOptTest::new("time - 0")
            .with_pass(algebraic::simplify_expr)
            .expect_semantics_preserved()
            .with_time(5.0)
            .run()
            .unwrap();
    }

    // ============================================================================
    // Multiplication identities
    // ============================================================================

    #[test]
    fn test_x_times_one() {
        // x * 1 → x
        AstOptTest::new("time * 1.0")
            .with_pass(algebraic::simplify_expr)
            .expect_ast(|b| b.var("time"))
            .expect_semantics_preserved()
            .with_time(42.0)
            .run()
            .unwrap();
    }

    #[test]
    fn test_one_times_x() {
        // 1 * x → x
        AstOptTest::new("1.0 * time")
            .with_pass(algebraic::simplify_expr)
            .expect_ast(|b| b.var("time"))
            .expect_semantics_preserved()
            .with_time(42.0)
            .run()
            .unwrap();
    }

    #[test]
    fn test_x_times_zero() {
        // x * 0 → 0
        AstOptTest::new("time * 0.0")
            .with_pass(algebraic::simplify_expr)
            .expect_ast(|b| b.num(0.0))
            .expect_semantics_preserved()
            .with_time(42.0)
            .run()
            .unwrap();
    }

    #[test]
    fn test_zero_times_x() {
        // 0 * x → 0
        AstOptTest::new("0.0 * time")
            .with_pass(algebraic::simplify_expr)
            .expect_ast(|b| b.num(0.0))
            .expect_semantics_preserved()
            .with_time(42.0)
            .run()
            .unwrap();
    }

    // NOTE: Skipping tests that mix Dec32 and Int32 types as they have
    // complex type coercion behavior that's not the focus of algebraic simplification

    #[test]
    fn test_time_times_zero_int() {
        // time * 0 → 0 (integer version)
        AstOptTest::new("time * 0")
            .with_pass(algebraic::simplify_expr)
            .expect_semantics_preserved()
            .with_time(5.0)
            .run()
            .unwrap();
    }

    // ============================================================================
    // Division identities
    // ============================================================================

    #[test]
    fn test_x_div_one() {
        // x / 1 → x
        AstOptTest::new("time / 1.0")
            .with_pass(algebraic::simplify_expr)
            .expect_ast(|b| b.var("time"))
            .expect_semantics_preserved()
            .with_time(42.0)
            .run()
            .unwrap();
    }

    // ============================================================================
    // Modulo identities
    // ============================================================================

    #[test]
    fn test_x_mod_one() {
        // x % 1 = frac(x) for fractional numbers, 0 for integers
        // We don't optimize this because the result depends on the input value
        // Just verify it doesn't break
        AstOptTest::new("time % 1.0")
            .with_pass(algebraic::simplify_expr)
            .expect_semantics_preserved()
            .with_time(42.5)
            .run()
            .unwrap();
    }

    // ============================================================================
    // Logical AND identities
    // ============================================================================

    #[test]
    fn test_x_and_true() {
        // x && true → x
        AstOptTest::new("time && 1.0")
            .with_pass(algebraic::simplify_expr)
            .expect_semantics_preserved()
            .with_time(1.0)
            .run()
            .unwrap();
    }

    #[test]
    fn test_true_and_x() {
        // true && x → x
        AstOptTest::new("1.0 && time")
            .with_pass(algebraic::simplify_expr)
            .expect_semantics_preserved()
            .with_time(1.0)
            .run()
            .unwrap();
    }

    #[test]
    fn test_x_and_false() {
        // x && false → false
        // Note: The result type will be Bool, not Dec32
        AstOptTest::new("time && 0.0")
            .with_pass(algebraic::simplify_expr)
            .expect_semantics_preserved()
            .with_time(1.0)
            .run()
            .unwrap();
    }

    #[test]
    fn test_false_and_x() {
        // false && x → false
        // Note: The result type will be Bool, not Dec32
        AstOptTest::new("0.0 && time")
            .with_pass(algebraic::simplify_expr)
            .expect_semantics_preserved()
            .with_time(1.0)
            .run()
            .unwrap();
    }

    // ============================================================================
    // Logical OR identities
    // ============================================================================

    #[test]
    fn test_x_or_true() {
        // x || true → true
        // Note: The result type will be Bool, not Dec32
        AstOptTest::new("time || 1.0")
            .with_pass(algebraic::simplify_expr)
            .expect_semantics_preserved()
            .with_time(0.0)
            .run()
            .unwrap();
    }

    #[test]
    fn test_true_or_x() {
        // true || x → true
        // Note: The result type will be Bool, not Dec32
        AstOptTest::new("1.0 || time")
            .with_pass(algebraic::simplify_expr)
            .expect_semantics_preserved()
            .with_time(0.0)
            .run()
            .unwrap();
    }

    #[test]
    fn test_x_or_false() {
        // x || false → x
        AstOptTest::new("time || 0.0")
            .with_pass(algebraic::simplify_expr)
            .expect_semantics_preserved()
            .with_time(1.0)
            .run()
            .unwrap();
    }

    #[test]
    fn test_false_or_x() {
        // false || x → x
        AstOptTest::new("0.0 || time")
            .with_pass(algebraic::simplify_expr)
            .expect_semantics_preserved()
            .with_time(1.0)
            .run()
            .unwrap();
    }

    // ============================================================================
    // Double negation
    // ============================================================================

    #[test]
    fn test_double_logical_negation() {
        // !(!x) → x
        AstOptTest::new("!!time")
            .with_pass(algebraic::simplify_expr)
            .expect_semantics_preserved()
            .with_time(1.0)
            .run()
            .unwrap();
    }

    #[test]
    fn test_double_arithmetic_negation() {
        // -(-x) → x
        // Note: --time parses as pre-decrement, not double negation
        // Use -(-time) for explicit double negation
        AstOptTest::new("-(-time)")
            .with_pass(algebraic::simplify_expr)
            .expect_ast(|b| b.var("time"))
            .expect_semantics_preserved()
            .with_time(42.0)
            .run()
            .unwrap();
    }

    // ============================================================================
    // Nested operations
    // ============================================================================

    #[test]
    fn test_nested_additions() {
        // (x + 0) + 0 → x (requires recursive simplification)
        AstOptTest::new("(time + 0.0) + 0.0")
            .with_pass(algebraic::simplify_expr)
            .expect_ast(|b| b.var("time"))
            .expect_semantics_preserved()
            .with_time(42.0)
            .run()
            .unwrap();
    }

    #[test]
    fn test_nested_multiplications() {
        // (x * 1) * 1 → x
        AstOptTest::new("(time * 1.0) * 1.0")
            .with_pass(algebraic::simplify_expr)
            .expect_ast(|b| b.var("time"))
            .expect_semantics_preserved()
            .with_time(42.0)
            .run()
            .unwrap();
    }

    #[test]
    fn test_zero_multiplication_in_addition() {
        // x + (y * 0) → x (y * 0 simplifies to 0, then x + 0 → x)
        // Note: This may require multiple passes in the full pipeline
        AstOptTest::new("time + (x * 0.0)")
            .with_pass(algebraic::simplify_expr)
            .expect_semantics_preserved()
            .with_vm_params(5.0, 0.0, 42.0)
            .run()
            .unwrap();
    }

    // ============================================================================
    // Special function optimizations
    // ============================================================================

    #[test]
    fn test_normalize_idempotence() {
        // normalize(normalize(x)) → normalize(x)
        // Note: normalize is for vec types, not scalars
        // Skip this test for now as it requires vec types
        // TODO: Add test with proper vec2/vec3 usage once supported
    }

    // ============================================================================
    // No-op tests (ensure we don't break valid expressions)
    // ============================================================================

    #[test]
    fn test_x_plus_two_unchanged() {
        // x + 2 should not be simplified (not an identity)
        AstOptTest::new("time + 2.0")
            .with_pass(algebraic::simplify_expr)
            .expect_semantics_preserved()
            .with_time(42.0)
            .run()
            .unwrap();
    }

    #[test]
    fn test_x_times_two_unchanged() {
        // x * 2 should not be simplified
        AstOptTest::new("time * 2.0")
            .with_pass(algebraic::simplify_expr)
            .expect_semantics_preserved()
            .with_time(42.0)
            .run()
            .unwrap();
    }

    #[test]
    fn test_x_div_two_unchanged() {
        // x / 2 should not be simplified
        AstOptTest::new("time / 2.0")
            .with_pass(algebraic::simplify_expr)
            .expect_semantics_preserved()
            .with_time(42.0)
            .run()
            .unwrap();
    }
}
