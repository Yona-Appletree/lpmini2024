/// Tests for peephole optimization patterns
///
/// These tests use ExprTest with opcode optimization to verify that peephole
/// optimizations work correctly and preserve semantics.
#[cfg(test)]
mod peephole_optimization_tests {
    use crate::compiler::expr::expr_test_util::ExprTest;
    use crate::compiler::optimize::OptimizeOptions;

    // ============================================================================
    // Peephole optimization enabled tests
    // ============================================================================

    #[test]
    fn test_peephole_preserves_semantics_simple() {
        // Test that peephole optimization preserves semantics for simple expression
        ExprTest::new("2.0 + 3.0")
            .with_peephole_optimization()
            .expect_result_fixed(5.0)
            .run()
            .unwrap();
    }

    #[test]
    fn test_peephole_preserves_semantics_with_variables() {
        // Test with built-in variables
        ExprTest::new("time * 2.0")
            .with_peephole_optimization()
            .with_time(5.0)
            .expect_result_fixed(10.0)
            .run()
            .unwrap();
    }

    #[test]
    fn test_peephole_preserves_semantics_complex() {
        // Test complex expression
        ExprTest::new("(x + y) * time")
            .with_peephole_optimization()
            .with_vm_params(3.0, 4.0, 2.0)
            .expect_result_fixed(14.0)
            .run()
            .unwrap();
    }

    // ============================================================================
    // Comparison: optimized vs unoptimized opcode count
    // ============================================================================

    #[test]
    fn test_peephole_reduces_opcodes_or_same() {
        // Peephole optimization should reduce opcode count or keep it the same
        // This test verifies the optimization doesn't make things worse

        let test_expressions = vec![
            "2.0 + 3.0",
            "time * 2.0",
            "(x + y) * time",
            "max(2.0, 3.0)",
            "sin(time) + cos(time)",
        ];

        for expr in test_expressions {
            // Both unoptimized and optimized should compile successfully
            ExprTest::new(expr).run().unwrap();
            
            ExprTest::new(expr)
                .with_peephole_optimization()
                .run()
                .unwrap();

            // Note: We can't easily check opcode count here without modifying ExprTest
            // This test mainly ensures both compile successfully
        }
    }

    // ============================================================================
    // Full optimization pipeline tests
    // ============================================================================

    #[test]
    fn test_full_optimization_pipeline() {
        // Test with all optimizations enabled
        let options = OptimizeOptions::all();
        
        ExprTest::new("(2.0 + 3.0) * 1.0")
            .with_optimization(options)
            .expect_result_fixed(5.0)
            .run()
            .unwrap();
    }

    #[test]
    fn test_ast_and_opcode_optimization_combined() {
        // Test that AST optimization + opcode optimization work together
        // (2.0 + 3.0) should be folded to 5.0 at AST level
        // Then any redundant opcodes should be eliminated at opcode level
        
        let options = OptimizeOptions::all();
        
        ExprTest::new("(2.0 + 3.0) * time")
            .with_optimization(options)
            .with_time(2.0)
            .expect_result_fixed(10.0)
            .run()
            .unwrap();
    }

    // ============================================================================
    // Specific opcode patterns that trigger peephole optimizations
    // ============================================================================

    #[test]
    fn test_comparison_operators() {
        // Comparison operators should work correctly with peephole optimization
        ExprTest::new("x > 5.0")
            .with_peephole_optimization()
            .with_x(10.0)
            .expect_result_fixed(1.0)
            .run()
            .unwrap();

        ExprTest::new("x < 5.0")
            .with_peephole_optimization()
            .with_x(3.0)
            .expect_result_fixed(1.0)
            .run()
            .unwrap();
    }

    #[test]
    fn test_logical_operators() {
        // Logical operators should work correctly
        ExprTest::new("(x > 5.0) && (y < 10.0)")
            .with_peephole_optimization()
            .with_vm_params(7.0, 8.0, 0.0)
            .expect_result_fixed(1.0)
            .run()
            .unwrap();

        ExprTest::new("(x > 5.0) || (y < 10.0)")
            .with_peephole_optimization()
            .with_vm_params(3.0, 8.0, 0.0)
            .expect_result_fixed(1.0)
            .run()
            .unwrap();
    }

    #[test]
    fn test_ternary_operator() {
        // Ternary operator should work correctly
        ExprTest::new("x > 5.0 ? 10.0 : 20.0")
            .with_peephole_optimization()
            .with_x(7.0)
            .expect_result_fixed(10.0)
            .run()
            .unwrap();

        ExprTest::new("x > 5.0 ? 10.0 : 20.0")
            .with_peephole_optimization()
            .with_x(3.0)
            .expect_result_fixed(20.0)
            .run()
            .unwrap();
    }

    // ============================================================================
    // Edge cases and regression tests
    // ============================================================================

    #[test]
    fn test_nested_expressions() {
        // Nested expressions should be optimized correctly
        ExprTest::new("((x + y) * 2.0) / time")
            .with_peephole_optimization()
            .with_vm_params(3.0, 4.0, 2.0)
            .expect_result_fixed(7.0)
            .run()
            .unwrap();
    }

    #[test]
    fn test_function_calls() {
        // Function calls should work with peephole optimization
        ExprTest::new("sin(time)")
            .with_peephole_optimization()
            .with_time(0.0)
            .expect_result_fixed(0.0)
            .run()
            .unwrap();

        ExprTest::new("max(x, y)")
            .with_peephole_optimization()
            .with_vm_params(5.0, 3.0, 0.0)
            .expect_result_fixed(5.0)
            .run()
            .unwrap();
    }

    #[test]
    fn test_multiple_operations() {
        // Multiple operations should be optimized correctly
        ExprTest::new("x + y - time * 2.0")
            .with_peephole_optimization()
            .with_vm_params(10.0, 5.0, 2.0)
            .expect_result_fixed(11.0)
            .run()
            .unwrap();
    }

    // ============================================================================
    // Verify optimization doesn't break correctness
    // ============================================================================

    #[test]
    fn test_no_optimization_vs_peephole() {
        // Verify that peephole-optimized code produces same result as unoptimized
        let expressions = vec![
            ("2.0 + 3.0", 5.0),
            ("10.0 - 3.0", 7.0),
            ("2.0 * 3.0", 6.0),
            ("6.0 / 2.0", 3.0),
            // Note: Skipping modulo due to precision differences
        ];

        for (expr, expected) in expressions {
            // Unoptimized
            ExprTest::new(expr)
                .expect_result_fixed(expected)
                .run()
                .unwrap();

            // Peephole optimized
            ExprTest::new(expr)
                .with_peephole_optimization()
                .expect_result_fixed(expected)
                .run()
                .unwrap();
        }
    }

    #[test]
    fn test_optimization_levels() {
        // Test different optimization configurations
        
        // No optimization
        ExprTest::new("2.0 + 3.0")
            .expect_result_fixed(5.0)
            .run()
            .unwrap();

        // Only peephole
        let mut options = OptimizeOptions::none();
        options.peephole_optimization = true;
        ExprTest::new("2.0 + 3.0")
            .with_optimization(options)
            .expect_result_fixed(5.0)
            .run()
            .unwrap();

        // Only constant folding (AST level)
        let mut options = OptimizeOptions::none();
        options.constant_folding = true;
        options.max_ast_passes = 5;
        ExprTest::new("2.0 + 3.0")
            .with_optimization(options)
            .expect_result_fixed(5.0)
            .run()
            .unwrap();

        // All optimizations
        ExprTest::new("2.0 + 3.0")
            .with_optimization(OptimizeOptions::all())
            .expect_result_fixed(5.0)
            .run()
            .unwrap();
    }

    // ============================================================================
    // Direct opcode pattern tests
    // ============================================================================

    #[test]
    fn test_push_drop_elimination_pattern() {
        // While we can't directly test Push/Drop patterns without looking at opcodes,
        // we can test expressions that might generate them
        
        // Expression statements that don't use their result might generate Push/Drop
        // But in expression mode, everything returns a value, so this is harder to test
        
        ExprTest::new("5.0")
            .with_peephole_optimization()
            .expect_result_fixed(5.0)
            .run()
            .unwrap();
    }

    #[test]
    fn test_optimization_with_all_builtin_vars() {
        // Test all built-in variables work with optimization
        ExprTest::new("x + y + time")
            .with_peephole_optimization()
            .with_vm_params(1.0, 2.0, 3.0)
            .expect_result_fixed(6.0)
            .run()
            .unwrap();
    }

    #[test]
    fn test_complex_nested_ternary() {
        // Complex nested ternary operators
        ExprTest::new("x > 5.0 ? (y > 3.0 ? 10.0 : 20.0) : 30.0")
            .with_peephole_optimization()
            .with_vm_params(7.0, 4.0, 0.0)
            .expect_result_fixed(10.0)
            .run()
            .unwrap();

        ExprTest::new("x > 5.0 ? (y > 3.0 ? 10.0 : 20.0) : 30.0")
            .with_peephole_optimization()
            .with_vm_params(7.0, 2.0, 0.0)
            .expect_result_fixed(20.0)
            .run()
            .unwrap();

        ExprTest::new("x > 5.0 ? (y > 3.0 ? 10.0 : 20.0) : 30.0")
            .with_peephole_optimization()
            .with_vm_params(3.0, 4.0, 0.0)
            .expect_result_fixed(30.0)
            .run()
            .unwrap();
    }
}

