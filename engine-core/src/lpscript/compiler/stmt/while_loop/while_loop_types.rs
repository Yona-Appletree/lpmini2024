/// While loop type checking
extern crate alloc;
use alloc::boxed::Box;
use crate::lpscript::ast::{Expr, Stmt};
use crate::lpscript::error::TypeError;
use crate::lpscript::typechecker::{TypeChecker, SymbolTable, FunctionTable};

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
