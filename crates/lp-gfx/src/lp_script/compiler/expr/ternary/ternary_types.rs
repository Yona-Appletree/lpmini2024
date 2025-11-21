/// Ternary expression type checking
extern crate alloc;

use alloc::boxed::Box;

use crate::lp_script::compiler::ast::Expr;
use crate::lp_script::compiler::error::{TypeError, TypeErrorKind};
use crate::lp_script::compiler::typechecker::{FunctionTable, SymbolTable, TypeChecker};
use crate::lp_script::shared::Type;

impl TypeChecker {
    /// Type check ternary operator (condition ? true_expr : false_expr)
    ///
    /// Result type is the type of true_expr (must match false_expr).
    pub(crate) fn check_ternary(
        condition: &mut Box<Expr>,
        true_expr: &mut Box<Expr>,
        false_expr: &mut Box<Expr>,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
        span: crate::lp_script::shared::Span,
    ) -> Result<Type, TypeError> {
        Self::infer_type(condition, symbols, func_table)?;
        Self::infer_type(true_expr, symbols, func_table)?;
        Self::infer_type(false_expr, symbols, func_table)?;

        // Result type is the type of true_expr (must match false_expr)
        let true_ty = true_expr.ty.as_ref().unwrap();
        let false_ty = false_expr.ty.as_ref().unwrap();

        if true_ty != false_ty {
            return Err(TypeError {
                kind: TypeErrorKind::Mismatch {
                    expected: true_ty.clone(),
                    found: false_ty.clone(),
                },
                span,
            });
        }

        Ok(true_ty.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::lp_script::compile_expr;
    use crate::lp_script::compiler::error::{CompileError, TypeErrorKind};

    // ========================================================================
    // Type Error Tests - Ternary Branch Type Mismatches
    // ========================================================================

    #[test]
    fn test_ternary_vec2_vs_vec3_branches() {
        let result = compile_expr("x > 0.5 ? vec2(1.0, 0.0) : vec3(0.0, 1.0, 2.0)");
        assert!(
            result.is_err(),
            "Ternary with vec2 and vec3 branches should be a type error"
        );

        if let Err(CompileError::TypeCheck(err)) = result {
            assert!(matches!(err.kind, TypeErrorKind::Mismatch { .. }));
        } else {
            panic!("Expected TypeCheck error");
        }
    }

    #[test]
    fn test_ternary_float_vs_vec2_branches() {
        let result = compile_expr("x > 0.5 ? 1.0 : vec2(1.0, 2.0)");
        assert!(
            result.is_err(),
            "Ternary with float and vec2 branches should be a type error"
        );

        if let Err(CompileError::TypeCheck(err)) = result {
            assert!(matches!(err.kind, TypeErrorKind::Mismatch { .. }));
        } else {
            panic!("Expected TypeCheck error");
        }
    }

    #[test]
    fn test_ternary_vec3_vs_float_branches() {
        let result = compile_expr("x > 0.5 ? vec3(1.0, 2.0, 3.0) : 5.0");
        assert!(
            result.is_err(),
            "Ternary with vec3 and float branches should be a type error"
        );

        if let Err(CompileError::TypeCheck(err)) = result {
            assert!(matches!(err.kind, TypeErrorKind::Mismatch { .. }));
        } else {
            panic!("Expected TypeCheck error");
        }
    }

    #[test]
    fn test_ternary_vec4_vs_vec2_branches() {
        let result = compile_expr("x > 0.5 ? vec4(1.0, 2.0, 3.0, 4.0) : vec2(1.0, 2.0)");
        assert!(
            result.is_err(),
            "Ternary with vec4 and vec2 branches should be a type error"
        );

        if let Err(CompileError::TypeCheck(err)) = result {
            assert!(matches!(err.kind, TypeErrorKind::Mismatch { .. }));
        } else {
            panic!("Expected TypeCheck error");
        }
    }
}
