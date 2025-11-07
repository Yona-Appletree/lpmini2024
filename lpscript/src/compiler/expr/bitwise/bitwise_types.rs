/// Bitwise operator type checking
extern crate alloc;

use crate::compiler::ast::Expr;
use crate::compiler::error::{TypeError, TypeErrorKind};
use crate::compiler::typechecker::{FunctionTable, SymbolTable, TypeChecker};
use crate::shared::{Span, Type};
use alloc::boxed::Box;

impl TypeChecker {
    /// Type check binary bitwise operators (&, |, ^, <<, >>)
    ///
    /// These operators only work on Int32 types and return Int32.
    pub(crate) fn check_bitwise_binary(
        left: &mut Box<Expr>,
        right: &mut Box<Expr>,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
        span: Span,
    ) -> Result<Type, TypeError> {
        Self::infer_type(left, symbols, func_table)?;
        Self::infer_type(right, symbols, func_table)?;

        let left_ty = left.ty.clone().unwrap();
        let right_ty = right.ty.clone().unwrap();

        // Both operands must be Int32
        if !matches!(left_ty, Type::Int32) {
            return Err(TypeError {
                kind: TypeErrorKind::Mismatch {
                    expected: Type::Int32,
                    found: left_ty,
                },
                span,
            });
        }

        if !matches!(right_ty, Type::Int32) {
            return Err(TypeError {
                kind: TypeErrorKind::Mismatch {
                    expected: Type::Int32,
                    found: right_ty,
                },
                span,
            });
        }

        Ok(Type::Int32)
    }

    /// Type check unary bitwise NOT (~)
    ///
    /// Only works on Int32 and returns Int32.
    pub(crate) fn check_bitwise_not(
        operand: &mut Box<Expr>,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
        span: Span,
    ) -> Result<Type, TypeError> {
        Self::infer_type(operand, symbols, func_table)?;

        let operand_ty = operand.ty.clone().unwrap();

        // Operand must be Int32
        if !matches!(operand_ty, Type::Int32) {
            return Err(TypeError {
                kind: TypeErrorKind::Mismatch {
                    expected: Type::Int32,
                    found: operand_ty,
                },
                span,
            });
        }

        Ok(Type::Int32)
    }
}
