/// Expression statement type checking
extern crate alloc;
use crate::lp_script::compiler::ast::Expr;
use crate::lp_script::compiler::error::TypeError;
use crate::lp_script::compiler::typechecker::{FunctionTable, SymbolTable, TypeChecker};

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
