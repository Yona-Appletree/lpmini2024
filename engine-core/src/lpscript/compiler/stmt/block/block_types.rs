/// Block statement type checking
extern crate alloc;
use crate::lpscript::compiler::ast::Stmt;
use crate::lpscript::compiler::typechecker::{FunctionTable, SymbolTable, TypeChecker};
use crate::lpscript::compiler::error::TypeError;

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
