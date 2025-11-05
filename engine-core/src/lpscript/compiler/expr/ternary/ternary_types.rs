/// Ternary expression type checking
extern crate alloc;

use crate::lpscript::ast::Expr;
use crate::lpscript::error::{Type, TypeError, TypeErrorKind};
use crate::lpscript::typechecker::{TypeChecker, SymbolTable, FunctionTable};
use alloc::boxed::Box;

impl TypeChecker {
    /// Type check ternary operator (condition ? true_expr : false_expr)
    /// 
    /// Result type is the type of true_expr (must match false_expr).
    pub(crate) fn check_ternary(
        condition: &mut Box<Expr>,
        true_expr: &mut Box<Expr>,
        false_expr: &mut Box<Expr>,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
        span: crate::lpscript::error::Span,
    ) -> Result<Type, TypeError> {
        Self::infer_type(condition, symbols, func_table)?;
        Self::infer_type(true_expr, symbols, func_table)?;
        Self::infer_type(false_expr, symbols, func_table)?;

        // Result type is the type of true_expr (must match false_expr)
        let true_ty = true_expr.ty.as_ref().unwrap();
        let false_ty = false_expr.ty.as_ref().unwrap();

        if true_ty != false_ty {
            return Err(TypeError {
                kind: TypeErrorKind::Mismatch {
                    expected: true_ty.clone(),
                    found: false_ty.clone(),
                },
                span,
            });
        }

        Ok(true_ty.clone())
    }
}

