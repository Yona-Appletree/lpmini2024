/// Assignment expression type checking
extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;

use crate::lp_script::compiler::ast::Expr;
use crate::lp_script::compiler::error::{TypeError, TypeErrorKind};
use crate::lp_script::compiler::typechecker::{FunctionTable, SymbolTable, TypeChecker};
use crate::lp_script::shared::Type;

impl TypeChecker {
    /// Type check assignment expression
    ///
    /// Returns the type of the assigned value.
    pub(crate) fn check_assign_expr(
        target: &str,
        value: &mut Box<Expr>,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
        span: crate::lp_script::shared::Span,
    ) -> Result<Type, TypeError> {
        // Check that variable exists
        let var_ty = symbols.lookup(target).ok_or_else(|| TypeError {
            kind: TypeErrorKind::UndefinedVariable(String::from(target)),
            span,
        })?;

        // Type check the value
        Self::infer_type(value, symbols, func_table)?;
        let value_ty = value.ty.as_ref().unwrap();

        // Check type matches
        if &var_ty != value_ty {
            return Err(TypeError {
                kind: TypeErrorKind::Mismatch {
                    expected: var_ty.clone(),
                    found: value_ty.clone(),
                },
                span: value.span,
            });
        }

        // Assignment expression returns the assigned value
        Ok(var_ty)
    }
}
