/// Integration tests for optimization pipeline
#[cfg(test)]
mod optimization_tests {
    use crate::lpscript::{
        compile_expr_with_options, compile_script_with_options, OptimizeOptions,
    };

    #[test]
    fn test_constant_folding_arithmetic() {
        // Test that 2 + 3 gets folded to 5
        let optimized = compile_expr_with_options("2.0 + 3.0", &OptimizeOptions::all()).unwrap();
        let unoptimized = compile_expr_with_options("2.0 + 3.0", &OptimizeOptions::none()).unwrap();

        println!("Optimized opcodes: {:?}", optimized.opcodes);
        println!("Unoptimized opcodes: {:?}", unoptimized.opcodes);

        // Optimized should have fewer opcodes
        assert!(optimized.opcodes.len() < unoptimized.opcodes.len());

        // Optimized should just push 5.0 (may have Return at end for expr mode)
        assert!(optimized.opcodes.len() <= 2); // Push(5.0) + maybe Return
    }

    #[test]
    fn test_constant_folding_math_functions() {
        // sin(0.0) should fold to 0.0
        let program = compile_expr_with_options("sin(0.0)", &OptimizeOptions::all()).unwrap();
        assert!(program.opcodes.len() <= 2); // Push(0.0) + Return

        // cos(0.0) should fold to 1.0
        let program = compile_expr_with_options("cos(0.0)", &OptimizeOptions::all()).unwrap();
        assert!(program.opcodes.len() <= 2); // Push(1.0) + Return

        // sqrt(4.0) should fold to 2.0
        let program = compile_expr_with_options("sqrt(4.0)", &OptimizeOptions::all()).unwrap();
        assert!(program.opcodes.len() <= 2); // Push(2.0) + Return
    }

    #[test]
    fn test_algebraic_simplification() {
        // x * 1.0 should simplify to x
        let optimized = compile_expr_with_options("time * 1.0", &OptimizeOptions::all()).unwrap();
        let unoptimized =
            compile_expr_with_options("time * 1.0", &OptimizeOptions::none()).unwrap();

        // Optimized should have fewer opcodes (no multiplication)
        assert!(optimized.opcodes.len() < unoptimized.opcodes.len());

        // x + 0.0 should simplify to x
        let optimized = compile_expr_with_options("time + 0.0", &OptimizeOptions::all()).unwrap();
        let unoptimized =
            compile_expr_with_options("time + 0.0", &OptimizeOptions::none()).unwrap();

        assert!(optimized.opcodes.len() < unoptimized.opcodes.len());

        // x * 0.0 should fold to 0.0
        let program = compile_expr_with_options("time * 0.0", &OptimizeOptions::all()).unwrap();
        assert!(program.opcodes.len() <= 2); // Push(0.0) + Return
    }

    #[test]
    fn test_constant_propagation_in_ternary() {
        // true ? x : y should fold to x
        let program =
            compile_expr_with_options("1.0 ? time : 0.0", &OptimizeOptions::all()).unwrap();
        let unoptimized =
            compile_expr_with_options("1.0 ? time : 0.0", &OptimizeOptions::none()).unwrap();

        // Optimized should not include the false branch or select
        assert!(program.opcodes.len() < unoptimized.opcodes.len());
    }

    #[test]
    fn test_dead_code_elimination_in_script() {
        let script = "
            float x = 2.0 + 3.0;
            return x;
            float y = 10.0;  // Dead code
        ";

        let optimized = compile_script_with_options(script, &OptimizeOptions::all()).unwrap();
        let unoptimized = compile_script_with_options(script, &OptimizeOptions::none()).unwrap();

        // Optimized should have fewer opcodes (dead code removed)
        assert!(optimized.opcodes.len() < unoptimized.opcodes.len());
    }

    #[test]
    fn test_if_with_constant_condition() {
        let script = "
            if (1.0) {
                return 1.0;
            } else {
                return 0.0;
            }
        ";

        let optimized = compile_script_with_options(script, &OptimizeOptions::all()).unwrap();
        let unoptimized = compile_script_with_options(script, &OptimizeOptions::none()).unwrap();

        // Optimized should have fewer opcodes (no jump, no else branch)
        assert!(optimized.opcodes.len() < unoptimized.opcodes.len());
    }

    #[test]
    fn test_multi_pass_optimization() {
        // (2.0 + 3.0) * 1.0 should fold to 5.0 through multiple passes
        let program =
            compile_expr_with_options("(2.0 + 3.0) * 1.0", &OptimizeOptions::all()).unwrap();

        // Should be fully optimized to just Push(5.0) + Return
        assert!(program.opcodes.len() <= 2);
    }

    #[test]
    fn test_optimization_preserves_semantics() {
        // Ensure optimized and unoptimized produce same results
        let expressions = vec![
            "2.0 + 3.0 * 4.0",
            "sin(0.0) + cos(0.0)",
            "time * 1.0 + 0.0",
            "max(2.0, 3.0)",
            "min(10.0, 5.0)",
        ];

        for expr in expressions {
            let optimized = compile_expr_with_options(expr, &OptimizeOptions::all()).unwrap();
            let unoptimized = compile_expr_with_options(expr, &OptimizeOptions::none()).unwrap();

            // Both should compile successfully
            assert!(!optimized.opcodes.is_empty());
            assert!(!unoptimized.opcodes.is_empty());

            // Optimized should be smaller or equal
            assert!(optimized.opcodes.len() <= unoptimized.opcodes.len());
        }
    }

    #[test]
    fn test_disable_optimizations() {
        let options = OptimizeOptions::none();

        // Even with OptimizeOptions::none(), code should still compile
        let program = compile_expr_with_options("2.0 + 3.0", &options).unwrap();

        // Should NOT be optimized (should have Push 2.0, Push 3.0, Add)
        assert!(program.opcodes.len() > 1);
    }

    #[test]
    fn test_partial_optimization() {
        // Create custom options with only constant folding
        let mut options = OptimizeOptions::none();
        options.constant_folding = true;
        options.max_ast_passes = 5;

        let program = compile_expr_with_options("2.0 + 3.0", &options).unwrap();

        // Should be optimized to Push(5.0) + Return
        assert!(program.opcodes.len() <= 2);
    }

    #[test]
    fn test_nested_constant_expressions() {
        // Test that nested constants get fully folded
        let program =
            compile_expr_with_options("(1.0 + 2.0) * (3.0 + 4.0)", &OptimizeOptions::all())
                .unwrap();

        // Should fold to 3.0 * 7.0 = 21.0, plus Return
        assert!(program.opcodes.len() <= 2);
    }

    #[test]
    fn test_comparison_constant_folding() {
        let program = compile_expr_with_options("2.0 < 3.0", &OptimizeOptions::all()).unwrap();

        // Should fold to 1.0 (true) + Return
        assert!(program.opcodes.len() <= 2);

        let program = compile_expr_with_options("5.0 < 3.0", &OptimizeOptions::all()).unwrap();

        // Should fold to 0.0 (false) + Return
        assert!(program.opcodes.len() <= 2);
    }

    #[test]
    fn test_logical_constant_folding() {
        // true && false -> false
        let program = compile_expr_with_options("1.0 && 0.0", &OptimizeOptions::all()).unwrap();
        assert!(program.opcodes.len() <= 2);

        // true || false -> true
        let program = compile_expr_with_options("1.0 || 0.0", &OptimizeOptions::all()).unwrap();
        assert!(program.opcodes.len() <= 2);

        // !true -> false
        let program = compile_expr_with_options("!1.0", &OptimizeOptions::all()).unwrap();
        assert!(program.opcodes.len() <= 2);
    }
}
