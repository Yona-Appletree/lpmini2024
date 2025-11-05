/// Function call type checking
extern crate alloc;

use alloc::string::ToString;
use alloc::vec;

use crate::lpscript::compiler::ast::Expr;
use crate::lpscript::compiler::error::{TypeError, TypeErrorKind};
use crate::lpscript::compiler::typechecker::{FunctionTable, SymbolTable, TypeChecker};
use crate::lpscript::shared::{Span, Type};

impl TypeChecker {
    /// Type check function call
    ///
    /// Infers the return type based on the function signature.
    /// Handles both user-defined and built-in functions.
    pub(crate) fn check_function_call(
        name: &str,
        args: &mut [Expr],
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
        span: Span,
    ) -> Result<Type, TypeError> {
        // Type check all arguments first
        for arg in args.iter_mut() {
            Self::infer_type(arg, symbols, func_table)?;
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
            for (arg, expected_ty) in args.iter().zip(sig.params.iter()) {
                let arg_ty = arg.ty.as_ref().unwrap();
                if arg_ty != expected_ty {
                    return Err(TypeError {
                        kind: TypeErrorKind::Mismatch {
                            expected: expected_ty.clone(),
                            found: arg_ty.clone(),
                        },
                        span: arg.span,
                    });
                }
            }

            Ok(sig.return_type.clone())
        } else {
            // Built-in function
            Self::function_return_type(name, args)
        }
    }

    /// Determine return type of built-in functions
    pub(crate) fn function_return_type(name: &str, args: &[Expr]) -> Result<Type, TypeError> {
        match name {
            // Math functions: Fixed -> Fixed
            "sin" | "cos" | "tan" | "abs" | "floor" | "ceil" | "sqrt" | "sign" | "frac"
            | "fract" | "saturate" => {
                if args.len() != 1 {
                    return Err(TypeError {
                        kind: TypeErrorKind::InvalidArgumentCount {
                            expected: 1,
                            found: args.len(),
                        },
                        span: args.first().map(|e| e.span).unwrap_or(Span::new(0, 0)),
                    });
                }
                Ok(Type::Fixed)
            }

            // atan: can take 1 or 2 args (atan(y) or atan(y, x))
            "atan" => {
                if args.is_empty() || args.len() > 2 {
                    return Err(TypeError {
                        kind: TypeErrorKind::InvalidArgumentCount {
                            expected: 1,
                            found: args.len(),
                        },
                        span: args.first().map(|e| e.span).unwrap_or(Span::new(0, 0)),
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
                        span: args.first().map(|e| e.span).unwrap_or(Span::new(0, 0)),
                    });
                }
                let arg_ty = args[0].ty.as_ref().unwrap();
                match arg_ty {
                    Type::Vec2 | Type::Vec3 | Type::Vec4 => Ok(Type::Fixed),
                    _ => Err(TypeError {
                        kind: TypeErrorKind::InvalidOperation {
                            op: "length".to_string(),
                            types: vec![arg_ty.clone()],
                        },
                        span: args[0].span,
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
                        span: args.first().map(|e| e.span).unwrap_or(Span::new(0, 0)),
                    });
                }
                let arg_ty = args[0].ty.as_ref().unwrap();
                match arg_ty {
                    Type::Vec2 | Type::Vec3 | Type::Vec4 => Ok(arg_ty.clone()),
                    _ => Err(TypeError {
                        kind: TypeErrorKind::InvalidOperation {
                            op: "normalize".to_string(),
                            types: vec![arg_ty.clone()],
                        },
                        span: args[0].span,
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
                        span: args.first().map(|e| e.span).unwrap_or(Span::new(0, 0)),
                    });
                }
                // Both args must be same vector type
                let left_ty = args[0].ty.as_ref().unwrap();
                let right_ty = args[1].ty.as_ref().unwrap();
                if left_ty != right_ty {
                    return Err(TypeError {
                        kind: TypeErrorKind::Mismatch {
                            expected: left_ty.clone(),
                            found: right_ty.clone(),
                        },
                        span: args[1].span,
                    });
                }
                match left_ty {
                    Type::Vec2 | Type::Vec3 | Type::Vec4 => Ok(Type::Fixed),
                    _ => Err(TypeError {
                        kind: TypeErrorKind::InvalidOperation {
                            op: "dot".to_string(),
                            types: vec![left_ty.clone()],
                        },
                        span: args[0].span,
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
                        span: args.first().map(|e| e.span).unwrap_or(Span::new(0, 0)),
                    });
                }
                let left_ty = args[0].ty.as_ref().unwrap();
                let right_ty = args[1].ty.as_ref().unwrap();
                if left_ty != right_ty {
                    return Err(TypeError {
                        kind: TypeErrorKind::Mismatch {
                            expected: left_ty.clone(),
                            found: right_ty.clone(),
                        },
                        span: args[1].span,
                    });
                }
                match left_ty {
                    Type::Vec2 | Type::Vec3 | Type::Vec4 => Ok(Type::Fixed),
                    _ => Err(TypeError {
                        kind: TypeErrorKind::InvalidOperation {
                            op: "distance".to_string(),
                            types: vec![left_ty.clone()],
                        },
                        span: args[0].span,
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
                        span: args.first().map(|e| e.span).unwrap_or(Span::new(0, 0)),
                    });
                }
                // Both must be vec3
                let left_ty = args[0].ty.as_ref().unwrap();
                let right_ty = args[1].ty.as_ref().unwrap();
                if left_ty != &Type::Vec3 || right_ty != &Type::Vec3 {
                    return Err(TypeError {
                        kind: TypeErrorKind::InvalidOperation {
                            op: "cross requires vec3 arguments".to_string(),
                            types: vec![left_ty.clone(), right_ty.clone()],
                        },
                        span: args[0].span,
                    });
                }
                Ok(Type::Vec3)
            }

            // Binary math functions
            "min" | "max" | "pow" | "step" | "mod" => {
                if args.len() != 2 {
                    return Err(TypeError {
                        kind: TypeErrorKind::InvalidArgumentCount {
                            expected: 2,
                            found: args.len(),
                        },
                        span: args.first().map(|e| e.span).unwrap_or(Span::new(0, 0)),
                    });
                }
                Ok(Type::Fixed)
            }

            // Ternary functions
            "clamp" | "lerp" | "mix" | "smoothstep" => {
                if args.len() != 3 {
                    return Err(TypeError {
                        kind: TypeErrorKind::InvalidArgumentCount {
                            expected: 3,
                            found: args.len(),
                        },
                        span: args.first().map(|e| e.span).unwrap_or(Span::new(0, 0)),
                    });
                }
                Ok(Type::Fixed)
            }

            // Perlin noise: perlin3(vec3) or perlin3(vec3, octaves)
            "perlin3" => {
                if args.is_empty() || args.len() > 2 {
                    return Err(TypeError {
                        kind: TypeErrorKind::InvalidArgumentCount {
                            expected: 1,
                            found: args.len(),
                        },
                        span: args.first().map(|e| e.span).unwrap_or(Span::new(0, 0)),
                    });
                }

                // First arg must be vec3
                let first_ty = args[0].ty.as_ref().unwrap();
                if first_ty != &Type::Vec3 {
                    return Err(TypeError {
                        kind: TypeErrorKind::Mismatch {
                            expected: Type::Vec3,
                            found: first_ty.clone(),
                        },
                        span: args[0].span,
                    });
                }

                // Second arg (if present) should be int (octaves), but we're lenient
                Ok(Type::Fixed)
            }

            _ => Err(TypeError {
                kind: TypeErrorKind::UndefinedFunction(name.to_string()),
                span: args.first().map(|e| e.span).unwrap_or(Span::new(0, 0)),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lpscript::compile_expr;
    use crate::lpscript::compiler::error::{CompileError, TypeErrorKind};

    // ========================================================================
    // Type Error Tests - Built-in Function Calls
    // ========================================================================

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
