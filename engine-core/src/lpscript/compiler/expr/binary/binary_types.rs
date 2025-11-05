/// Binary arithmetic type checking
extern crate alloc;

use crate::lpscript::compiler::ast::Expr;
use crate::lpscript::compiler::typechecker::{FunctionTable, SymbolTable, TypeChecker};
use crate::lpscript::compiler::error::{TypeError, TypeErrorKind};
use crate::lpscript::shared::Type;
use alloc::boxed::Box;

impl TypeChecker {
    /// Type check binary arithmetic operators (+, -, *, /, %, ^)
    ///
    /// Handles scalar-scalar, vector-vector, and vector-scalar operations.
    /// Returns the result type.
    pub(crate) fn check_binary_arithmetic(
        left: &mut Box<Expr>,
        right: &mut Box<Expr>,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
        span: crate::lpscript::shared::Span,
    ) -> Result<Type, TypeError> {
        Self::infer_type(left, symbols, func_table)?;
        Self::infer_type(right, symbols, func_table)?;

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
}
