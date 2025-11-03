/// Type checker for LightPlayer Script
/// 
/// Performs type inference and validation on the AST.
extern crate alloc;
use alloc::vec::Vec;
use alloc::string::ToString;

use crate::lpscript::ast::{Expr, ExprKind};
use crate::lpscript::error::{Span, Type, TypeError, TypeErrorKind};

pub struct TypeChecker;

impl TypeChecker {
    /// Type check an expression, returning a typed AST
    pub fn check(mut expr: Expr) -> Result<Expr, TypeError> {
        Self::infer_type(&mut expr)?;
        Ok(expr)
    }
    
    fn infer_type(expr: &mut Expr) -> Result<(), TypeError> {
        match &mut expr.kind {
            // Literals
            ExprKind::Number(_) => {
                expr.ty = Some(Type::Fixed);
            }
            
            ExprKind::IntNumber(_) => {
                expr.ty = Some(Type::Int32);
            }
            
            ExprKind::Variable(name) => {
                // Built-in variables are all Fixed (normalized coords, time, etc.)
                expr.ty = Some(Type::Fixed);
                
                // Validate variable exists
                match name.as_str() {
                    "x" | "xNorm" | "y" | "yNorm" | "time" | "t" | 
                    "timeNorm" | "centerAngle" | "angle" | "centerDist" | "dist" => {}
                    _ => {
                        return Err(TypeError {
                            kind: TypeErrorKind::UndefinedVariable(name.clone()),
                            span: expr.span,
                        });
                    }
                }
            }
            
            // Binary arithmetic operations
            ExprKind::Add(left, right) | ExprKind::Sub(left, right) | 
            ExprKind::Mul(left, right) | ExprKind::Div(left, right) |
            ExprKind::Mod(left, right) | ExprKind::Pow(left, right) => {
                Self::infer_type(left)?;
                Self::infer_type(right)?;
                
                let left_ty = left.ty.clone().unwrap();
                let right_ty = right.ty.clone().unwrap();
                
                // For now, both operands must match type
                if left_ty != right_ty {
                    // Allow int -> fixed promotion
                    if left_ty == Type::Int32 && right_ty == Type::Fixed {
                        left.ty = Some(Type::Fixed);
                    } else if left_ty == Type::Fixed && right_ty == Type::Int32 {
                        right.ty = Some(Type::Fixed);
                    } else {
                        return Err(TypeError {
                            kind: TypeErrorKind::Mismatch {
                                expected: left_ty.clone(),
                                found: right_ty.clone(),
                            },
                            span: expr.span,
                        });
                    }
                }
                
                expr.ty = Some(left_ty);
            }
            
            // Comparisons return Fixed (0 or 1)
            ExprKind::Less(left, right) | ExprKind::Greater(left, right) |
            ExprKind::LessEq(left, right) | ExprKind::GreaterEq(left, right) |
            ExprKind::Eq(left, right) | ExprKind::NotEq(left, right) => {
                Self::infer_type(left)?;
                Self::infer_type(right)?;
                
                // Comparisons always return Fixed (0 or 1)
                expr.ty = Some(Type::Fixed);
            }
            
            // Logical operations
            ExprKind::And(left, right) | ExprKind::Or(left, right) => {
                Self::infer_type(left)?;
                Self::infer_type(right)?;
                
                expr.ty = Some(Type::Fixed);
            }
            
            // Ternary
            ExprKind::Ternary { condition, true_expr, false_expr } => {
                Self::infer_type(condition)?;
                Self::infer_type(true_expr)?;
                Self::infer_type(false_expr)?;
                
                // Result type is the type of true_expr (must match false_expr)
                let true_ty = true_expr.ty.as_ref().unwrap();
                let false_ty = false_expr.ty.as_ref().unwrap();
                
                if true_ty != false_ty {
                    return Err(TypeError {
                        kind: TypeErrorKind::Mismatch {
                            expected: true_ty.clone(),
                            found: false_ty.clone(),
                        },
                        span: expr.span,
                    });
                }
                
                expr.ty = Some(true_ty.clone());
            }
            
            // Function calls
            ExprKind::Call { name, args } => {
                // Type check arguments
                for arg in args.iter_mut() {
                    Self::infer_type(arg)?;
                }
                
                // Determine return type based on function
                let return_ty = Self::function_return_type(name, args)?;
                expr.ty = Some(return_ty);
            }
            
            // Vector constructors
            ExprKind::Vec2Constructor(x, y) => {
                Self::infer_type(x)?;
                Self::infer_type(y)?;
                expr.ty = Some(Type::Vec2);
            }
            
            ExprKind::Vec3Constructor(x, y, z) => {
                Self::infer_type(x)?;
                Self::infer_type(y)?;
                Self::infer_type(z)?;
                expr.ty = Some(Type::Vec3);
            }
            
            ExprKind::Vec4Constructor(x, y, z, w) => {
                Self::infer_type(x)?;
                Self::infer_type(y)?;
                Self::infer_type(z)?;
                Self::infer_type(w)?;
                expr.ty = Some(Type::Vec4);
            }
        }
        
        Ok(())
    }
    
    fn function_return_type(name: &str, args: &[Expr]) -> Result<Type, TypeError> {
        match name {
            // Math functions: Fixed -> Fixed
            "sin" | "cos" | "abs" | "floor" | "ceil" | "sqrt" | "sign" |
            "frac" | "saturate" => {
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
            
            // Binary math functions
            "min" | "max" | "pow" | "step" => {
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
            
            // Perlin noise
            "perlin3" => {
                if args.len() < 3 || args.len() > 4 {
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
            
            _ => Err(TypeError {
                kind: TypeErrorKind::UndefinedFunction(name.to_string()),
                span: args.first().map(|e| e.span).unwrap_or(Span::new(0, 0)),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lpscript::parser::Parser;
    use crate::lpscript::lexer::Lexer;
    
    fn parse_and_check(input: &str) -> Result<Expr, TypeError> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        TypeChecker::check(ast)
    }
    
    #[test]
    fn test_simple_number() {
        let expr = parse_and_check("42.0").unwrap();
        assert_eq!(expr.ty, Some(Type::Fixed));
    }
    
    #[test]
    fn test_int_number() {
        let expr = parse_and_check("42").unwrap();
        assert_eq!(expr.ty, Some(Type::Int32));
    }
    
    #[test]
    fn test_variable() {
        let expr = parse_and_check("xNorm").unwrap();
        assert_eq!(expr.ty, Some(Type::Fixed));
    }
    
    #[test]
    fn test_undefined_variable() {
        let result = parse_and_check("undefined");
        assert!(matches!(result, Err(TypeError { kind: TypeErrorKind::UndefinedVariable(_), .. })));
    }
    
    #[test]
    fn test_arithmetic() {
        let expr = parse_and_check("1.0 + 2.0").unwrap();
        assert_eq!(expr.ty, Some(Type::Fixed));
    }
    
    #[test]
    fn test_function_call() {
        let expr = parse_and_check("sin(time)").unwrap();
        assert_eq!(expr.ty, Some(Type::Fixed));
    }
    
    #[test]
    fn test_undefined_function() {
        let result = parse_and_check("unknown(1.0)");
        assert!(matches!(result, Err(TypeError { kind: TypeErrorKind::UndefinedFunction(_), .. })));
    }
    
    #[test]
    fn test_vec2_constructor() {
        let expr = parse_and_check("vec2(1.0, 2.0)").unwrap();
        assert_eq!(expr.ty, Some(Type::Vec2));
    }
    
    #[test]
    fn test_vec3_constructor() {
        let expr = parse_and_check("vec3(1.0, 2.0, 3.0)").unwrap();
        assert_eq!(expr.ty, Some(Type::Vec3));
    }
    
    #[test]
    fn test_ternary_type_mismatch() {
        // true and false branches must match
        let result = parse_and_check("xNorm > 0.5 ? vec2(1.0, 0.0) : 1.0");
        assert!(matches!(result, Err(TypeError { kind: TypeErrorKind::Mismatch { .. }, .. })));
    }
}

