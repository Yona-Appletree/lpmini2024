/// Variable declaration type checking
extern crate alloc;
use alloc::string::String;

use crate::compiler::ast::Expr;
use crate::compiler::error::{TypeError, TypeErrorKind};
use crate::compiler::typechecker::{FunctionTable, SymbolTable, TypeChecker};
use crate::shared::Type;

impl TypeChecker {
    /// Type check variable declaration
    pub(crate) fn check_var_decl(
        ty: &Type,
        name: &str,
        init: &mut Option<Expr>,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
        span: crate::shared::Span,
    ) -> Result<(), TypeError> {
        // If there's an initializer, type check it
        if let Some(init_expr) = init {
            Self::infer_type(init_expr, symbols, func_table)?;
            let init_ty = init_expr.ty.as_ref().unwrap();

            // Check type matches
            if ty != init_ty {
                // Allow int -> dec32 promotion
                if *ty == Type::Dec32 && *init_ty == Type::Int32 {
                    init_expr.ty = Some(Type::Dec32);
                } else {
                    return Err(TypeError {
                        kind: TypeErrorKind::Mismatch {
                            expected: ty.clone(),
                            found: init_ty.clone(),
                        },
                        span: init_expr.span,
                    });
                }
            }
        }

        // Add variable to symbol table
        symbols
            .declare(String::from(name), ty.clone())
            .map_err(|msg| TypeError {
                kind: TypeErrorKind::UndefinedVariable(msg),
                span,
            })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::compiler::error::{CompileError, TypeErrorKind};
    use crate::{compile_script_with_options, OptimizeOptions};

    // ========================================================================
    // Type Error Tests - Variable Declaration Type Mismatches
    // ========================================================================

    #[test]
    fn test_vec2_var_initialized_with_vec3() {
        let result = compile_script_with_options(
            "vec2 v = vec3(1.0, 2.0, 3.0); return v.x;",
            &OptimizeOptions::none(),
        );
        assert!(
            result.is_err(),
            "vec2 initialized with vec3 should be a type error"
        );

        if let Err(CompileError::TypeCheck(err)) = result {
            assert!(matches!(err.kind, TypeErrorKind::Mismatch { .. }));
        } else {
            panic!("Expected TypeCheck error");
        }
    }

    #[test]
    fn test_float_var_initialized_with_vec2() {
        let result = compile_script_with_options(
            "float x = vec2(1.0, 2.0); return x;",
            &OptimizeOptions::none(),
        );
        assert!(
            result.is_err(),
            "float initialized with vec2 should be a type error"
        );

        if let Err(CompileError::TypeCheck(err)) = result {
            assert!(matches!(err.kind, TypeErrorKind::Mismatch { .. }));
        } else {
            panic!("Expected TypeCheck error");
        }
    }

    #[test]
    fn test_vec3_var_initialized_with_vec2() {
        let result = compile_script_with_options(
            "vec3 v = vec2(1.0, 2.0); return v.x;",
            &OptimizeOptions::none(),
        );
        assert!(
            result.is_err(),
            "vec3 initialized with vec2 should be a type error"
        );

        if let Err(CompileError::TypeCheck(err)) = result {
            assert!(matches!(err.kind, TypeErrorKind::Mismatch { .. }));
        } else {
            panic!("Expected TypeCheck error");
        }
    }

    #[test]
    fn test_vec4_var_initialized_with_vec3() {
        let result = compile_script_with_options(
            "vec4 v = vec3(1.0, 2.0, 3.0); return v.x;",
            &OptimizeOptions::none(),
        );
        assert!(
            result.is_err(),
            "vec4 initialized with vec3 should be a type error"
        );

        if let Err(CompileError::TypeCheck(err)) = result {
            assert!(matches!(err.kind, TypeErrorKind::Mismatch { .. }));
        } else {
            panic!("Expected TypeCheck error");
        }
    }

    #[test]
    fn test_vec2_assigned_wrong_type() {
        let result = compile_script_with_options(
            "vec2 v; v = vec3(1.0, 2.0, 3.0); return v.x;",
            &OptimizeOptions::none(),
        );
        assert!(result.is_err(), "vec2 assigned vec3 should be a type error");

        // This will be caught by assignment type checking, not var decl
        if let Err(CompileError::TypeCheck(err)) = result {
            assert!(matches!(err.kind, TypeErrorKind::Mismatch { .. }));
        } else {
            panic!("Expected TypeCheck error");
        }
    }

    #[test]
    fn test_vec3_assigned_float() {
        let result =
            compile_script_with_options("vec3 v; v = 5.0; return v.x;", &OptimizeOptions::none());
        assert!(
            result.is_err(),
            "vec3 assigned float should be a type error"
        );

        if let Err(CompileError::TypeCheck(err)) = result {
            assert!(matches!(err.kind, TypeErrorKind::Mismatch { .. }));
        } else {
            panic!("Expected TypeCheck error");
        }
    }
}
