/// If statement type checking
extern crate alloc;
use crate::compiler::ast::{Expr, Stmt};
use crate::compiler::typechecker::{FunctionTable, SymbolTable, TypeChecker};
use crate::compiler::error::TypeError;
use alloc::boxed::Box;

impl TypeChecker {
    pub(crate) fn check_if(
        condition: &mut Expr,
        then_stmt: &mut Box<Stmt>,
        else_stmt: &mut Option<Box<Stmt>>,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
    ) -> Result<(), TypeError> {
        Self::infer_type(condition, symbols, func_table)?;
        Self::check_stmt(then_stmt, symbols, func_table)?;

        if let Some(else_block) = else_stmt {
            Self::check_stmt(else_block, symbols, func_table)?;
        }

        Ok(())
    }
}
