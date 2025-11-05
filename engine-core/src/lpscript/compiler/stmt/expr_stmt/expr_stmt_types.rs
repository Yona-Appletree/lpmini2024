/// Expression statement type checking
extern crate alloc;
use crate::lpscript::ast::Expr;
use crate::lpscript::error::TypeError;
use crate::lpscript::typechecker::{TypeChecker, SymbolTable, FunctionTable};

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
