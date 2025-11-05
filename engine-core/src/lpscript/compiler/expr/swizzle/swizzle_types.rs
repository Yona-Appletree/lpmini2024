/// Swizzle operation type checking
extern crate alloc;
use alloc::string::String;
use alloc::{format, vec};

use crate::lpscript::compiler::ast::Expr;
use crate::lpscript::compiler::typechecker::{FunctionTable, SymbolTable, TypeChecker};
use crate::lpscript::compiler::error::{TypeError, TypeErrorKind};
use crate::lpscript::shared::Type;
use alloc::boxed::Box;

impl TypeChecker {
    /// Type check swizzle operation
    ///
    /// Returns the result type based on component count.
    pub(crate) fn check_swizzle(
        base_expr: &mut Box<Expr>,
        components: &str,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
        span: crate::lpscript::shared::Span,
    ) -> Result<Type, TypeError> {
        Self::infer_type(base_expr, symbols, func_table)?;

        let base_ty = base_expr.ty.as_ref().unwrap();

        // Validate that base is a vector
        let base_size = match base_ty {
            Type::Vec2 => 2,
            Type::Vec3 => 3,
            Type::Vec4 => 4,
            _ => {
                return Err(TypeError {
                    kind: TypeErrorKind::InvalidOperation {
                        op: String::from("Swizzle can only be applied to vector types"),
                        types: vec![base_ty.clone()],
                    },
                    span,
                })
            }
        };

        // Validate components
        for c in components.chars() {
            let idx = match c {
                'x' | 'r' | 's' => 0,
                'y' | 'g' | 't' => 1,
                'z' | 'b' | 'p' => 2,
                'w' | 'a' | 'q' => 3,
                _ => {
                    return Err(TypeError {
                        kind: TypeErrorKind::InvalidOperation {
                            op: format!("Invalid swizzle component: {}", c),
                            types: vec![base_ty.clone()],
                        },
                        span,
                    })
                }
            };

            if idx >= base_size {
                return Err(TypeError {
                    kind: TypeErrorKind::InvalidOperation {
                        op: format!(
                            "Component {} out of range for type {}",
                            c,
                            type_to_string(base_ty)
                        ),
                        types: vec![base_ty.clone()],
                    },
                    span,
                });
            }
        }

        // Result type based on component count
        Ok(match components.len() {
            1 => Type::Fixed,
            2 => Type::Vec2,
            3 => Type::Vec3,
            4 => Type::Vec4,
            _ => {
                return Err(TypeError {
                    kind: TypeErrorKind::InvalidOperation {
                        op: String::from("Swizzle must have 1-4 components"),
                        types: vec![base_ty.clone()],
                    },
                    span,
                })
            }
        })
    }
}

fn type_to_string(ty: &Type) -> &str {
    match ty {
        Type::Fixed => "float",
        Type::Int32 => "int",
        Type::Bool => "bool",
        Type::Vec2 => "vec2",
        Type::Vec3 => "vec3",
        Type::Vec4 => "vec4",
        Type::Void => "void",
    }
}
