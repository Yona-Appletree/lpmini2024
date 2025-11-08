/// Binary arithmetic type checking
extern crate alloc;

use crate::compiler::ast::Expr;
use crate::compiler::error::{TypeError, TypeErrorKind};
use crate::compiler::typechecker::{FunctionTable, SymbolTable, TypeChecker};
use crate::shared::Type;

/// Type check binary arithmetic operators (+, -, *, /, %, ^)
///
/// Handles scalar-scalar, vector-vector, and vector-scalar operations.
/// Returns the result type.
pub(in crate::compiler) fn check_binary_arithmetic(
    left: &mut Expr,
    right: &mut Expr,
    symbols: &mut SymbolTable,
    func_table: &FunctionTable,
    span: crate::shared::Span,
) -> Result<Type, TypeError> {
    TypeChecker::infer_type(left, symbols, func_table)?;
    TypeChecker::infer_type(right, symbols, func_table)?;

    let left_ty = left.ty.clone().unwrap();
    let right_ty = right.ty.clone().unwrap();

    // Check for vector-scalar operations
    let result_ty = match (&left_ty, &right_ty) {
        // Both same type
        (l, r) if l == r => l.clone(),

        // Int -> Fixed promotion
        (Type::Int32, Type::Fixed) => {
            left.ty = Some(Type::Fixed);
            Type::Fixed
        }
        (Type::Fixed, Type::Int32) => {
            right.ty = Some(Type::Fixed);
            Type::Fixed
        }

        // Vector * Scalar (returns vector)
        (Type::Vec2, Type::Fixed | Type::Int32) => Type::Vec2,
        (Type::Vec3, Type::Fixed | Type::Int32) => Type::Vec3,
        (Type::Vec4, Type::Fixed | Type::Int32) => Type::Vec4,

        // Scalar * Vector (returns vector)
        (Type::Fixed | Type::Int32, Type::Vec2) => Type::Vec2,
        (Type::Fixed | Type::Int32, Type::Vec3) => Type::Vec3,
        (Type::Fixed | Type::Int32, Type::Vec4) => Type::Vec4,

        // Mismatch
        _ => {
            return Err(TypeError {
                kind: TypeErrorKind::Mismatch {
                    expected: left_ty.clone(),
                    found: right_ty.clone(),
                },
                span,
            })
        }
    };

    Ok(result_ty)
}

#[cfg(test)]
mod tests {
    use crate::compile_expr;
    use crate::compiler::error::{CompileError, TypeErrorKind};

    // ========================================================================
    // Type Error Tests - Mismatched Vector Sizes
    // ========================================================================

    #[test]
    fn test_vec2_plus_vec3_error() {
        let result = compile_expr("vec2(1.0, 2.0) + vec3(1.0, 2.0, 3.0)");
        assert!(result.is_err(), "vec2 + vec3 should be a type error");

        if let Err(CompileError::TypeCheck(err)) = result {
            assert!(matches!(err.kind, TypeErrorKind::Mismatch { .. }));
        } else {
            panic!("Expected TypeCheck error");
        }
    }

    #[test]
    fn test_vec3_mul_vec4_error() {
        let result = compile_expr("vec3(1.0, 2.0, 3.0) * vec4(1.0, 2.0, 3.0, 4.0)");
        assert!(result.is_err(), "vec3 * vec4 should be a type error");

        if let Err(CompileError::TypeCheck(err)) = result {
            assert!(matches!(err.kind, TypeErrorKind::Mismatch { .. }));
        } else {
            panic!("Expected TypeCheck error");
        }
    }

    #[test]
    fn test_vec2_minus_vec4_error() {
        let result = compile_expr("vec2(1.0, 2.0) - vec4(1.0, 2.0, 3.0, 4.0)");
        assert!(result.is_err(), "vec2 - vec4 should be a type error");

        if let Err(CompileError::TypeCheck(err)) = result {
            assert!(matches!(err.kind, TypeErrorKind::Mismatch { .. }));
        } else {
            panic!("Expected TypeCheck error");
        }
    }

    #[test]
    fn test_vec4_div_vec2_error() {
        let result = compile_expr("vec4(10.0, 20.0, 30.0, 40.0) / vec2(2.0, 4.0)");
        assert!(result.is_err(), "vec4 / vec2 should be a type error");

        if let Err(CompileError::TypeCheck(err)) = result {
            assert!(matches!(err.kind, TypeErrorKind::Mismatch { .. }));
        } else {
            panic!("Expected TypeCheck error");
        }
    }

    #[test]
    fn test_vec3_plus_vec2_error() {
        let result = compile_expr("vec3(1.0, 2.0, 3.0) + vec2(1.0, 2.0)");
        assert!(result.is_err(), "vec3 + vec2 should be a type error");

        if let Err(CompileError::TypeCheck(err)) = result {
            assert!(matches!(err.kind, TypeErrorKind::Mismatch { .. }));
        } else {
            panic!("Expected TypeCheck error");
        }
    }

    #[test]
    fn test_vec4_minus_vec3_error() {
        let result = compile_expr("vec4(10.0, 9.0, 8.0, 7.0) - vec3(1.0, 2.0, 3.0)");
        assert!(result.is_err(), "vec4 - vec3 should be a type error");

        if let Err(CompileError::TypeCheck(err)) = result {
            assert!(matches!(err.kind, TypeErrorKind::Mismatch { .. }));
        } else {
            panic!("Expected TypeCheck error");
        }
    }

    // ========================================================================
    // Type Error Tests - Bool/Void with Vectors
    // ========================================================================

    #[test]
    fn test_bool_plus_vec2_error() {
        let result = compile_expr("(1.0 > 0.5) + vec2(1.0, 2.0)");
        assert!(result.is_err(), "bool + vec2 should be a type error");

        if let Err(CompileError::TypeCheck(err)) = result {
            assert!(matches!(err.kind, TypeErrorKind::Mismatch { .. }));
        } else {
            panic!("Expected TypeCheck error");
        }
    }

    #[test]
    fn test_vec3_mul_bool_error() {
        let result = compile_expr("vec3(1.0, 2.0, 3.0) * (2.0 < 1.0)");
        assert!(result.is_err(), "vec3 * bool should be a type error");

        if let Err(CompileError::TypeCheck(err)) = result {
            assert!(matches!(err.kind, TypeErrorKind::Mismatch { .. }));
        } else {
            panic!("Expected TypeCheck error");
        }
    }
}
