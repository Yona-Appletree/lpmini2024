/// While loop type checking
extern crate alloc;
use crate::compiler::ast::{Expr, Stmt};
use crate::compiler::typechecker::{FunctionTable, SymbolTable, TypeChecker};
use crate::compiler::error::TypeError;
use alloc::boxed::Box;

impl TypeChecker {
    pub(crate) fn check_while(
        condition: &mut Expr,
        body: &mut Box<Stmt>,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
    ) -> Result<(), TypeError> {
        Self::infer_type(condition, symbols, func_table)?;
        Self::check_stmt(body, symbols, func_table)?;
        Ok(())
    }
}
