/// Return statement type checking
extern crate alloc;
use crate::compiler::ast::Expr;
use crate::compiler::typechecker::{FunctionTable, SymbolTable, TypeChecker};
use crate::compiler::error::TypeError;

impl TypeChecker {
    pub(crate) fn check_return(
        expr: &mut Expr,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
    ) -> Result<(), TypeError> {
        Self::infer_type(expr, symbols, func_table)?;
        Ok(())
    }
}
