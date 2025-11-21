/// Block statement type checking
extern crate alloc;
use crate::lp_script::compiler::ast::Stmt;
use crate::lp_script::compiler::error::TypeError;
use crate::lp_script::compiler::typechecker::{FunctionTable, SymbolTable, TypeChecker};

impl TypeChecker {
    pub(crate) fn check_block(
        stmts: &mut [Stmt],
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
    ) -> Result<(), TypeError> {
        symbols.push_scope();
        for stmt in stmts {
            Self::check_stmt(stmt, symbols, func_table)?;
        }
        symbols.pop_scope();
        Ok(())
    }
}
