/// Statement type checking
extern crate alloc;

use crate::lpscript::compiler::ast::{Stmt, StmtKind};
use crate::lpscript::compiler::error::TypeError;
use crate::lpscript::compiler::typechecker::{FunctionTable, SymbolTable, TypeChecker};

impl TypeChecker {
    /// Type check a statement
    pub(crate) fn check_stmt(
        stmt: &mut Stmt,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
    ) -> Result<(), TypeError> {
        match &mut stmt.kind {
            StmtKind::VarDecl { ty, name, init } => {
                Self::check_var_decl(ty, name, init, symbols, func_table, stmt.span)?;
            }

            StmtKind::Assignment { name, value } => {
                Self::check_assignment(name, value, symbols, func_table, stmt.span)?;
            }

            StmtKind::Return(expr) => {
                Self::check_return(expr, symbols, func_table)?;
            }

            StmtKind::Expr(expr) => {
                Self::check_expr_stmt(expr, symbols, func_table)?;
            }

            StmtKind::Block(stmts) => {
                Self::check_block(stmts, symbols, func_table)?;
            }

            StmtKind::If {
                condition,
                then_stmt,
                else_stmt,
            } => {
                Self::check_if(condition, then_stmt, else_stmt, symbols, func_table)?;
            }

            StmtKind::While { condition, body } => {
                Self::check_while(condition, body, symbols, func_table)?;
            }

            StmtKind::For {
                init,
                condition,
                increment,
                body,
            } => {
                Self::check_for(init, condition, increment, body, symbols, func_table)?;
            }
        }

        Ok(())
    }
}

