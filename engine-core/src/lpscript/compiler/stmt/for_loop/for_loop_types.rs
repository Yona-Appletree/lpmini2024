/// For loop type checking
extern crate alloc;
use alloc::boxed::Box;
use crate::lpscript::ast::{Expr, Stmt};
use crate::lpscript::error::TypeError;
use crate::lpscript::typechecker::{TypeChecker, SymbolTable, FunctionTable};

impl TypeChecker {
    pub(crate) fn check_for(
        init: &mut Option<Box<Stmt>>,
        condition: &mut Option<Expr>,
        increment: &mut Option<Expr>,
        body: &mut Box<Stmt>,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
    ) -> Result<(), TypeError> {
        if let Some(init_stmt) = init {
            Self::check_stmt(init_stmt, symbols, func_table)?;
        }
        
        if let Some(cond) = condition {
            Self::infer_type(cond, symbols, func_table)?;
        }
        
        if let Some(inc) = increment {
            Self::infer_type(inc, symbols, func_table)?;
        }
        
        Self::check_stmt(body, symbols, func_table)?;
        
        Ok(())
    }
}
