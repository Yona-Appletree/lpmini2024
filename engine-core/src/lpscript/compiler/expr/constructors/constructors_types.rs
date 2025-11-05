/// Vector constructor type checking
extern crate alloc;
use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use crate::lpscript::compiler::ast::Expr;
use crate::lpscript::compiler::typechecker::{FunctionTable, SymbolTable, TypeChecker};
use crate::lpscript::error::{Span, Type, TypeError, TypeErrorKind};

impl TypeChecker {
    /// Get the number of components in a type (for vector constructor validation)
    fn component_count(ty: &Type) -> usize {
        match ty {
            Type::Bool | Type::Fixed | Type::Int32 => 1,
            Type::Vec2 => 2,
            Type::Vec3 => 3,
            Type::Vec4 => 4,
            Type::Void => 0,
        }
    }

    /// Check vector constructor arguments and ensure total components match expected
    /// Alternative version that returns error with detailed message
    pub(crate) fn check_vector_constructor(
        args: &mut [Expr],
        expected_components: usize,
        name: &str,
        span: Span,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
    ) -> Result<(), TypeError> {
        // Type check all arguments
        for arg in args.iter_mut() {
            Self::infer_type(arg, symbols, func_table)?;
        }

        // Count total components provided
        let total: usize = args
            .iter()
            .map(|arg| Self::component_count(arg.ty.as_ref().unwrap()))
            .sum();

        if total != expected_components {
            let types: Vec<Type> = args.iter().map(|arg| arg.ty.clone().unwrap()).collect();

            return Err(TypeError {
                kind: TypeErrorKind::InvalidOperation {
                    op: format!(
                        "{} constructor expects {} components, got {}",
                        name, expected_components, total
                    ),
                    types,
                },
                span,
            });
        }

        Ok(())
    }

    /// Type check vector constructor
    ///
    /// Supports GLSL-style mixed args: vec3(vec2, float), vec4(vec3, float), etc.
    pub(crate) fn check_vec_constructor(
        args: &mut [Expr],
        expected_components: usize,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
        span: crate::lpscript::error::Span,
    ) -> Result<Type, TypeError> {
        // Type check all arguments
        for arg in args.iter_mut() {
            Self::infer_type(arg, symbols, func_table)?;
        }

        // Count total components provided
        let mut total_components = 0;
        for arg in args.iter() {
            let ty = arg.ty.as_ref().unwrap();
            total_components += match ty {
                Type::Fixed | Type::Int32 | Type::Bool => 1,
                Type::Vec2 => 2,
                Type::Vec3 => 3,
                Type::Vec4 => 4,
                _ => {
                    return Err(TypeError {
                        kind: TypeErrorKind::InvalidOperation {
                            op: String::from("Cannot use this type in vector constructor"),
                            types: vec![ty.clone()],
                        },
                        span: arg.span,
                    })
                }
            };
        }

        // Check that we have the right number of components
        if total_components != expected_components {
            return Err(TypeError {
                kind: TypeErrorKind::InvalidArgumentCount {
                    expected: expected_components,
                    found: total_components,
                },
                span,
            });
        }

        Ok(match expected_components {
            2 => Type::Vec2,
            3 => Type::Vec3,
            4 => Type::Vec4,
            _ => unreachable!(),
        })
    }
}
