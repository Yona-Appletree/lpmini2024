/// Function call type checking
extern crate alloc;

use alloc::string::ToString;
use alloc::vec;

use crate::lpscript::compiler::ast::Expr;
use crate::lpscript::compiler::typechecker::{FunctionTable, SymbolTable, TypeChecker};
use crate::lpscript::error::{Span, Type, TypeError, TypeErrorKind};

impl TypeChecker {
    /// Type check function call
    ///
    /// Infers the return type based on the function signature.
    pub(crate) fn check_function_call(
        name: &str,
        args: &mut [Expr],
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
    ) -> Result<Type, TypeError> {
        // Type check all arguments first
        for arg in args.iter_mut() {
            Self::infer_type(arg, symbols, func_table)?;
        }

        // Infer function return type (calls existing function_return_type method)
        Self::function_return_type(name, args)
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
