/// Vector constructor type checking
extern crate alloc;
use alloc::vec;
use alloc::string::String;

use crate::lpscript::ast::Expr;
use crate::lpscript::error::{Type, TypeError, TypeErrorKind};
use crate::lpscript::typechecker::{TypeChecker, SymbolTable, FunctionTable};

impl TypeChecker {
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
                _ => return Err(TypeError {
                    kind: TypeErrorKind::InvalidOperation {
                        op: String::from("Cannot use this type in vector constructor"),
                        types: vec![ty.clone()],
                    },
                    span: arg.span,
                }),
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

