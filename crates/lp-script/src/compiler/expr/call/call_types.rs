/// Function call type checking
extern crate alloc;

use alloc::string::ToString;

use crate::compiler::ast::Expr;
use crate::compiler::error::{TypeError, TypeErrorKind};
use crate::compiler::typechecker::{FunctionTable, SymbolTable, TypeChecker};
use crate::shared::Type;

use super::expand_componentwise;

/// Type check function call
///
/// Infers the return type based on the function signature.
/// Handles both user-defined and built-in functions.
/// May transform the expression via component-wise expansion.
pub(in crate::compiler) fn check_call_id(
    pool: &mut AstPool,
    name: &str,
    args: &[Expr],
    symbols: &mut SymbolTable,
    func_table: &FunctionTable,
    span: crate::shared::Span,
) -> Result<(Type, Option<Expr>), TypeError> {
    // Type check all arguments
    for &arg_id in args {
        TypeChecker::infer_type_id(pool, arg_id, symbols, func_table)?;
    }

    // Try component-wise expansion for built-in functions with vector args
    if expand_componentwise::is_componentwise_function(name) {
        if let Some(expanded_id) =
            expand_componentwise::expand_componentwise_call(pool, name, args, span)
        {
            // Recursively type-check the expanded expression
            TypeChecker::infer_type_id(pool, expanded_id, symbols, func_table)?;

            let return_ty = pool.expr(expanded_id).ty.clone().unwrap();
            // Return both the type and the expanded expression ID so caller can replace
            return Ok((return_ty, Some(expanded_id)));
        }
    }

    // Check if it's a user-defined function first
    if let Some(sig) = func_table.lookup(name) {
        // Validate argument count
        if args.len() != sig.params.len() {
            return Err(TypeError {
                kind: TypeErrorKind::InvalidArgumentCount {
                    expected: sig.params.len(),
                    found: args.len(),
                },
                span,
            });
        }

        // Validate argument types
        for (&arg_id, expected_ty) in args.iter().zip(sig.params.iter()) {
            let arg_ty = pool.expr(arg_id).ty.as_ref().unwrap();
            if arg_ty != expected_ty {
                return Err(TypeError {
                    kind: TypeErrorKind::Mismatch {
                        expected: expected_ty.clone(),
                        found: arg_ty.clone(),
                    },
                    span: pool.expr(arg_id).span,
                });
            }
        }

        Ok((sig.return_type.clone(), None))
    } else {
        // Built-in function - determine return type
        let ty = builtin_function_return_type_id(pool, name, args, span)?;
        Ok((ty, None))
    }
}

fn builtin_function_return_type_id(
    pool: &AstPool,
    name: &str,
    args: &[Expr],
    span: crate::shared::Span,
) -> Result<Type, TypeError> {
    match name {
        // Math functions: Fixed -> Fixed
        "sin" | "cos" | "tan" | "abs" | "floor" | "ceil" | "sqrt" | "sign" | "frac" | "fract"
        | "saturate" => {
            if args.len() != 1 {
                return Err(TypeError {
                    kind: TypeErrorKind::InvalidArgumentCount {
                        expected: 1,
                        found: args.len(),
                    },
                    span,
                });
            }
            Ok(Type::Fixed)
        }

        // atan: can take 1 or 2 args
        "atan" => {
            if args.is_empty() || args.len() > 2 {
                return Err(TypeError {
                    kind: TypeErrorKind::InvalidArgumentCount {
                        expected: 1,
                        found: args.len(),
                    },
                    span,
                });
            }
            Ok(Type::Fixed)
        }

        // Vector length: vec -> float
        "length" => {
            if args.len() != 1 {
                return Err(TypeError {
                    kind: TypeErrorKind::InvalidArgumentCount {
                        expected: 1,
                        found: args.len(),
                    },
                    span,
                });
            }
            let arg_ty = pool.expr(args[0]).ty.as_ref().unwrap();
            match arg_ty {
                Type::Vec2 | Type::Vec3 | Type::Vec4 => Ok(Type::Fixed),
                _ => Err(TypeError {
                    kind: TypeErrorKind::InvalidOperation {
                        op: "length".to_string(),
                        types: alloc::vec![arg_ty.clone()],
                    },
                    span: pool.expr(args[0]).span,
                }),
            }
        }

        // Normalize: vec -> vec (same type)
        "normalize" => {
            if args.len() != 1 {
                return Err(TypeError {
                    kind: TypeErrorKind::InvalidArgumentCount {
                        expected: 1,
                        found: args.len(),
                    },
                    span,
                });
            }
            let arg_ty = pool.expr(args[0]).ty.as_ref().unwrap();
            match arg_ty {
                Type::Vec2 | Type::Vec3 | Type::Vec4 => Ok(arg_ty.clone()),
                _ => Err(TypeError {
                    kind: TypeErrorKind::InvalidOperation {
                        op: "normalize".to_string(),
                        types: alloc::vec![arg_ty.clone()],
                    },
                    span: pool.expr(args[0]).span,
                }),
            }
        }

        // Dot product: vec x vec -> float
        "dot" => {
            if args.len() != 2 {
                return Err(TypeError {
                    kind: TypeErrorKind::InvalidArgumentCount {
                        expected: 2,
                        found: args.len(),
                    },
                    span,
                });
            }
            let left_ty = pool.expr(args[0]).ty.as_ref().unwrap();
            let right_ty = pool.expr(args[1]).ty.as_ref().unwrap();
            if left_ty != right_ty {
                return Err(TypeError {
                    kind: TypeErrorKind::Mismatch {
                        expected: left_ty.clone(),
                        found: right_ty.clone(),
                    },
                    span: pool.expr(args[1]).span,
                });
            }
            match left_ty {
                Type::Vec2 | Type::Vec3 | Type::Vec4 => Ok(Type::Fixed),
                _ => Err(TypeError {
                    kind: TypeErrorKind::InvalidOperation {
                        op: "dot".to_string(),
                        types: alloc::vec![left_ty.clone()],
                    },
                    span: pool.expr(args[0]).span,
                }),
            }
        }

        // Distance: vec x vec -> float
        "distance" => {
            if args.len() != 2 {
                return Err(TypeError {
                    kind: TypeErrorKind::InvalidArgumentCount {
                        expected: 2,
                        found: args.len(),
                    },
                    span,
                });
            }
            let left_ty = pool.expr(args[0]).ty.as_ref().unwrap();
            let right_ty = pool.expr(args[1]).ty.as_ref().unwrap();
            if left_ty != right_ty {
                return Err(TypeError {
                    kind: TypeErrorKind::Mismatch {
                        expected: left_ty.clone(),
                        found: right_ty.clone(),
                    },
                    span: pool.expr(args[1]).span,
                });
            }
            match left_ty {
                Type::Vec2 | Type::Vec3 | Type::Vec4 => Ok(Type::Fixed),
                _ => Err(TypeError {
                    kind: TypeErrorKind::InvalidOperation {
                        op: "distance".to_string(),
                        types: alloc::vec![left_ty.clone()],
                    },
                    span: pool.expr(args[0]).span,
                }),
            }
        }

        // Cross product: vec3 x vec3 -> vec3
        "cross" => {
            if args.len() != 2 {
                return Err(TypeError {
                    kind: TypeErrorKind::InvalidArgumentCount {
                        expected: 2,
                        found: args.len(),
                    },
                    span,
                });
            }
            let left_ty = pool.expr(args[0]).ty.as_ref().unwrap();
            let right_ty = pool.expr(args[1]).ty.as_ref().unwrap();
            if left_ty != &Type::Vec3 || right_ty != &Type::Vec3 {
                return Err(TypeError {
                    kind: TypeErrorKind::InvalidOperation {
                        op: "cross".to_string(),
                        types: alloc::vec![left_ty.clone(), right_ty.clone()],
                    },
                    span,
                });
            }
            Ok(Type::Vec3)
        }

        // Binary functions: Fixed x Fixed -> Fixed
        "pow" | "mod" | "min" | "max" | "step" => {
            if args.len() != 2 {
                return Err(TypeError {
                    kind: TypeErrorKind::InvalidArgumentCount {
                        expected: 2,
                        found: args.len(),
                    },
                    span,
                });
            }
            Ok(Type::Fixed)
        }

        // Ternary functions: Fixed x Fixed x Fixed -> Fixed
        "clamp" | "lerp" | "mix" | "smoothstep" => {
            if args.len() != 3 {
                return Err(TypeError {
                    kind: TypeErrorKind::InvalidArgumentCount {
                        expected: 3,
                        found: args.len(),
                    },
                    span,
                });
            }
            Ok(Type::Fixed)
        }

        // Perlin noise: vec3 -> float
        "perlin3" => {
            if args.len() < 1 || args.len() > 2 {
                return Err(TypeError {
                    kind: TypeErrorKind::InvalidArgumentCount {
                        expected: 1,
                        found: args.len(),
                    },
                    span,
                });
            }
            let arg_ty = pool.expr(args[0]).ty.as_ref().unwrap();
            if arg_ty != &Type::Vec3 {
                return Err(TypeError {
                    kind: TypeErrorKind::Mismatch {
                        expected: Type::Vec3,
                        found: arg_ty.clone(),
                    },
                    span: pool.expr(args[0]).span,
                });
            }
            Ok(Type::Fixed)
        }

        _ => Err(TypeError {
            kind: TypeErrorKind::UndefinedVariable(alloc::format!("Unknown function '{}'", name)),
            span,
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::compile_expr;
    use crate::compiler::error::{CompileError, TypeErrorKind};

    #[test]
    fn test_cross_with_vec2() {
        // cross() is vec3 only
        let result = compile_expr("cross(vec2(1.0, 2.0), vec2(3.0, 4.0))");
        assert!(result.is_err(), "cross() with vec2 should be a type error");

        if let Err(CompileError::TypeCheck(err)) = result {
            assert!(matches!(err.kind, TypeErrorKind::InvalidOperation { .. }));
        } else {
            panic!("Expected TypeCheck error");
        }
    }

    #[test]
    fn test_dot_mismatched_vector_sizes() {
        let result = compile_expr("dot(vec2(1.0, 2.0), vec3(3.0, 4.0, 5.0))");
        assert!(
            result.is_err(),
            "dot() with mismatched vector sizes should be a type error"
        );

        if let Err(CompileError::TypeCheck(err)) = result {
            assert!(matches!(err.kind, TypeErrorKind::Mismatch { .. }));
        } else {
            panic!("Expected TypeCheck error");
        }
    }

    #[test]
    fn test_length_with_wrong_arg_count() {
        let result = compile_expr("length(vec2(1.0, 2.0), vec2(3.0, 4.0))");
        assert!(
            result.is_err(),
            "length() with 2 arguments should be a type error"
        );

        if let Err(CompileError::TypeCheck(err)) = result {
            assert!(matches!(
                err.kind,
                TypeErrorKind::InvalidArgumentCount { .. }
            ));
        } else {
            panic!("Expected TypeCheck error");
        }
    }

    #[test]
    fn test_normalize_with_scalar() {
        let result = compile_expr("normalize(5.0)");
        assert!(
            result.is_err(),
            "normalize() with scalar should be a type error"
        );

        if let Err(CompileError::TypeCheck(err)) = result {
            assert!(matches!(err.kind, TypeErrorKind::InvalidOperation { .. }));
        } else {
            panic!("Expected TypeCheck error");
        }
    }

    #[test]
    fn test_perlin3_with_vec2() {
        let result = compile_expr("perlin3(vec2(1.0, 2.0))");
        assert!(
            result.is_err(),
            "perlin3() with vec2 should be a type error (expects vec3)"
        );

        if let Err(CompileError::TypeCheck(err)) = result {
            assert!(matches!(err.kind, TypeErrorKind::Mismatch { .. }));
        } else {
            panic!("Expected TypeCheck error");
        }
    }

    #[test]
    fn test_distance_mismatched_types() {
        let result = compile_expr("distance(vec3(1.0, 2.0, 3.0), vec2(4.0, 5.0))");
        assert!(
            result.is_err(),
            "distance() with mismatched vector sizes should be a type error"
        );

        if let Err(CompileError::TypeCheck(err)) = result {
            assert!(matches!(err.kind, TypeErrorKind::Mismatch { .. }));
        } else {
            panic!("Expected TypeCheck error");
        }
    }
}
