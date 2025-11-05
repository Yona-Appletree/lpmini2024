/// Block statement type checking
extern crate alloc;
use crate::lpscript::ast::Stmt;
use crate::lpscript::error::TypeError;
use crate::lpscript::typechecker::{TypeChecker, SymbolTable, FunctionTable};

impl TypeChecker {
    pub(crate) fn check_block(
        stmts: &mut [Stmt],
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
    ) -> Result<(), TypeError> {
        for stmt in stmts {
            Self::check_stmt(stmt, symbols, func_table)?;
        }
        Ok(())
    }
}
