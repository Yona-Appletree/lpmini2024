/// Assignment statement type checking
extern crate alloc;
use alloc::string::String;

use crate::lpscript::compiler::ast::Expr;
use crate::lpscript::compiler::typechecker::{FunctionTable, SymbolTable, TypeChecker};
use crate::lpscript::error::{TypeError, TypeErrorKind};

impl TypeChecker {
    /// Type check assignment statement
    pub(crate) fn check_assignment(
        name: &str,
        value: &mut Expr,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
        span: crate::lpscript::error::Span,
    ) -> Result<(), TypeError> {
        // Check that variable exists
        let var_ty = symbols.lookup(name).ok_or_else(|| TypeError {
            kind: TypeErrorKind::UndefinedVariable(String::from(name)),
            span,
        })?;

        // Type check the value
        Self::infer_type(value, symbols, func_table)?;
        let value_ty = value.ty.as_ref().unwrap();

        // Check type matches
        if &var_ty != value_ty {
            return Err(TypeError {
                kind: TypeErrorKind::Mismatch {
                    expected: var_ty,
                    found: value_ty.clone(),
                },
                span: value.span,
            });
        }

        Ok(())
    }
}
