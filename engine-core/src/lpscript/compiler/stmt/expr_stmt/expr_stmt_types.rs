/// Expression statement type checking
extern crate alloc;
use crate::lpscript::compiler::ast::Expr;
use crate::lpscript::compiler::typechecker::{FunctionTable, SymbolTable, TypeChecker};
use crate::lpscript::compiler::error::TypeError;

impl TypeChecker {
    pub(crate) fn check_expr_stmt(
        expr: &mut Expr,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
    ) -> Result<(), TypeError> {
        Self::infer_type(expr, symbols, func_table)?;
        Ok(())
    }
}
