/// Test utilities for AST-level optimizations
///
/// Provides a builder pattern for testing individual optimization passes
/// with both structural transformation and semantic preservation testing.
extern crate alloc;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::lpscript::compiler::ast::{AstPool, ExprId};
use crate::lpscript::compiler::expr::expr_test_util::ast_eq_ignore_spans_with_pool;
use crate::lpscript::compiler::{codegen, lexer, parser, typechecker};
use crate::lpscript::shared::Type;
use crate::lpscript::vm::{LpsVm, VmLimits, FunctionDef, LpsProgram};
use crate::math::{Fixed, ToFixed};

/// Type alias for optimization pass functions (pool-based)
/// Takes ownership of pool and returns both new ExprId and pool
pub type OptPassFn = fn(ExprId, AstPool) -> (ExprId, AstPool);

/// Builder for testing AST optimization passes
///
/// Supports two testing modes:
/// 1. **Structural**: Test that input expr transforms to expected output AST
/// 2. **Semantic**: Test that optimization preserves runtime result
///
/// # Example
/// ```
/// use engine_core::lpscript::compiler::optimize::ast::algebraic;
/// use engine_core::lpscript::compiler::optimize::ast_test_util::AstOptTest;
/// use engine_core::lpscript::compiler::test_ast::*;
///
/// // Test structural transformation
/// AstOptTest::new("x + 0.0")
///     .with_pass(algebraic::simplify_expr)
///     .expect_ast(var("x").with_type(Type::Fixed))
///     .run()
///     .unwrap();
///
/// // Test semantic preservation
/// AstOptTest::new("x * 1.0")
///     .with_pass(algebraic::simplify_expr)
///     .expect_semantics_preserved()
///     .run()
///     .unwrap();
/// ```
pub struct AstOptTest {
    input: String,
    pass: Option<OptPassFn>,
    expected_ast_builder: Option<Box<dyn FnOnce(&mut crate::lpscript::compiler::test_ast::AstBuilder) -> ExprId>>,
    check_semantics: bool,
    x: Fixed,
    y: Fixed,
    time: Fixed,
}

impl AstOptTest {
    /// Create a new test case with the given input expression
    pub fn new(input: &str) -> Self {
        AstOptTest {
            input: String::from(input),
            pass: None,
            expected_ast_builder: None,
            check_semantics: false,
            x: 0.5.to_fixed(),
            y: 0.5.to_fixed(),
            time: Fixed::ZERO,
        }
    }

    /// Specify which optimization pass to test
    pub fn with_pass(mut self, pass: OptPassFn) -> Self {
        self.pass = Some(pass);
        self
    }

    /// Expect a specific AST structure after optimization (using builder closure)
    pub fn expect_ast<F>(mut self, builder_fn: F) -> Self
    where
        F: FnOnce(&mut crate::lpscript::compiler::test_ast::AstBuilder) -> ExprId + 'static,
    {
        self.expected_ast_builder = Some(Box::new(builder_fn));
        self
    }

    /// Expect that optimization preserves runtime semantics
    /// (i.e., optimized code produces same result as unoptimized)
    pub fn expect_semantics_preserved(mut self) -> Self {
        self.check_semantics = true;
        self
    }

    /// Set x value for built-in `x` variable (default: 0.5)
    pub fn with_x(mut self, x: f32) -> Self {
        self.x = x.to_fixed();
        self
    }

    /// Set y value for built-in `y` variable (default: 0.5)
    pub fn with_y(mut self, y: f32) -> Self {
        self.y = y.to_fixed();
        self
    }

    /// Set time value for built-in (default: 0.0)
    pub fn with_time(mut self, time: f32) -> Self {
        self.time = time.to_fixed();
        self
    }

    /// Set VM run parameters (x, y, time) for built-in variables
    pub fn with_vm_params(mut self, x: f32, y: f32, time: f32) -> Self {
        self.x = x.to_fixed();
        self.y = y.to_fixed();
        self.time = time.to_fixed();
        self
    }

    /// Run all expectations and return result
    pub fn run(mut self) -> Result<(), String> {
        let mut errors = Vec::new();

        // Parse the input
        let mut lexer = lexer::Lexer::new(&self.input);
        let tokens = lexer.tokenize();
        let mut parser = parser::Parser::new(tokens);
        let ast_id = match parser.parse() {
            Ok(id) => id,
            Err(e) => {
                errors.push(format!("Parse error: {}", e));
                return Err(errors.join("\n\n"));
            }
        };
        
        // Extract the pool from the parser after parsing
        let pool = parser.pool;

        // Type check
        let (typed_ast_id, mut pool) = match typechecker::TypeChecker::check(ast_id, pool) {
            Ok(result) => result,
            Err(e) => {
                errors.push(format!("Type check error: {}", e));
                return Err(errors.join("\n\n"));
            }
        };

        // Apply optimization pass if specified
        let (optimized_ast_id, optimized_pool) = if let Some(pass) = self.pass {
            pass(typed_ast_id, pool)
        } else {
            errors.push("No optimization pass specified - use .with_pass()".to_string());
            return Err(errors.join("\n\n"));
        };
        pool = optimized_pool;

        // Check AST structure if expected
        if let Some(builder_fn) = self.expected_ast_builder.take() {
            let mut expected_builder = crate::lpscript::compiler::test_ast::AstBuilder::new();
            let expected_id = builder_fn(&mut expected_builder);
            let expected_pool = expected_builder.into_pool();
            
            if !ast_eq_ignore_spans_with_pool(&pool, optimized_ast_id, &expected_pool, expected_id) {
                errors.push(format!(
                    "AST mismatch after optimization:\nExpected: {:?}\nActual:   {:?}",
                    expected_pool.expr(expected_id), pool.expr(optimized_ast_id)
                ));
            }
        }

        // Check semantic preservation if requested
        if self.check_semantics {
            match self.check_semantic_preservation(typed_ast_id, optimized_ast_id, &pool) {
                Ok(()) => {}
                Err(e) => errors.push(e),
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.join("\n\n"))
        }
    }

    /// Check that optimized AST produces same runtime result as original
    fn check_semantic_preservation(&self, original_id: ExprId, optimized_id: ExprId, pool: &AstPool) -> Result<(), String> {
        // Generate opcodes for both versions (no optimization at opcode level)
        let original_opcodes = codegen::CodeGenerator::generate(pool, original_id);
        let optimized_opcodes = codegen::CodeGenerator::generate(pool, optimized_id);

        // Create main functions for both programs
        let original_main = FunctionDef::new("main".into(), Type::Void)
            .with_opcodes(original_opcodes);
        let optimized_main = FunctionDef::new("main".into(), Type::Void)
            .with_opcodes(optimized_opcodes);
        
        let original_program = LpsProgram::new("original".into())
            .with_functions(vec![original_main]);
        let optimized_program = LpsProgram::new("optimized".into())
            .with_functions(vec![optimized_main]);

        // Create VMs
        let mut original_vm = LpsVm::new(&original_program, VmLimits::default())
            .map_err(|e| format!("Failed to create VM for original: {:?}", e))?;
        let mut optimized_vm = LpsVm::new(&optimized_program, VmLimits::default())
            .map_err(|e| format!("Failed to create VM for optimized: {:?}", e))?;

        // Run both and compare results
        let original_result = original_vm
            .run_scalar(self.x, self.y, self.time)
            .map_err(|e| format!("Runtime error in original: {:?}", e))?;
        let optimized_result = optimized_vm
            .run_scalar(self.x, self.y, self.time)
            .map_err(|e| format!("Runtime error in optimized: {:?}", e))?;

        // Compare with tolerance
        let original_f32 = original_result.to_f32();
        let optimized_f32 = optimized_result.to_f32();
        let diff = (original_f32 - optimized_f32).abs();

        if diff > 0.01 {
            Err(format!(
                "Semantic preservation failed:\nOriginal result:  {}\nOptimized result: {}\nDiff:             {}",
                original_f32, optimized_f32, diff
            ))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // TODO: Re-enable when algebraic optimizer is updated to use AstPool
    /*
    #[test]
    fn test_ast_transformation() {
        // Test that x + 0 simplifies to x
        AstOptTest::new("time + 0.0")
            .with_pass(algebraic::simplify_expr)
            .expect_ast({
                let mut expr = var("time");
                expr.ty = Some(Type::Fixed);
                expr
            })
            .run()
            .expect("Should simplify x + 0 to x");
    }

    #[test]
    fn test_semantic_preservation() {
        // Test that x * 1 produces same result as x
        AstOptTest::new("time * 1.0")
            .with_pass(algebraic::simplify_expr)
            .expect_semantics_preserved()
            .with_time(42.0)
            .run()
            .expect("Should preserve semantics");
    }

    #[test]
    fn test_both_ast_and_semantics() {
        // Test both structural transformation and semantic preservation
        AstOptTest::new("time * 1.0")
            .with_pass(algebraic::simplify_expr)
            .expect_ast({
                let mut expr = var("time");
                expr.ty = Some(Type::Fixed);
                expr
            })
            .expect_semantics_preserved()
            .with_time(42.0)
            .run()
            .expect("Should pass both checks");
    }
    */

    #[test]
    fn test_no_pass_specified() {
        // Should error if no pass is specified
        let result = AstOptTest::new("1.0 + 2.0").run();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("No optimization pass specified"));
    }

    #[test]
    fn test_with_vm_params() {
        // Test with custom VM parameters
        AstOptTest::new("x + y")
            .with_pass(|expr_id, pool| (expr_id, pool)) // Identity pass (no optimization)
            .expect_semantics_preserved()
            .with_vm_params(3.0, 4.0, 0.0)
            .run()
            .expect("Should work with custom VM params");
    }
}
