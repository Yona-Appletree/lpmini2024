/// Statement type checking
extern crate alloc;

use crate::compiler::ast::{Stmt, StmtKind};
use crate::compiler::error::TypeError;
use crate::compiler::typechecker::{FunctionTable, SymbolTable, TypeChecker};

impl TypeChecker {
    /// Type check a statement
    pub(crate) fn check_stmt(
        stmt: &mut Stmt,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
    ) -> Result<(), TypeError> {
        match &mut stmt.kind {
            StmtKind::VarDecl { ty, name, init } => {
                if let Some(init_expr) = init {
                    Self::infer_type(init_expr, symbols, func_table)?;
                }
                let _ = symbols.declare(name.clone(), ty.clone());
            }

            StmtKind::Return(expr) => {
                Self::infer_type(expr, symbols, func_table)?;
            }

            StmtKind::Expr(expr) => {
                Self::infer_type(expr, symbols, func_table)?;
            }

            StmtKind::Block(stmts) => {
                symbols.push_scope();
                for stmt in stmts.iter_mut() {
                    Self::check_stmt(stmt, symbols, func_table)?;
                }
                symbols.pop_scope();
            }

            StmtKind::If {
                condition,
                then_stmt,
                else_stmt,
            } => {
                Self::infer_type(condition, symbols, func_table)?;
                Self::check_stmt(then_stmt.as_mut(), symbols, func_table)?;
                if let Some(else_s) = else_stmt {
                    Self::check_stmt(else_s.as_mut(), symbols, func_table)?;
                }
            }

            StmtKind::While { condition, body } => {
                Self::infer_type(condition, symbols, func_table)?;
                Self::check_stmt(body.as_mut(), symbols, func_table)?;
            }

            StmtKind::For {
                init,
                condition,
                increment,
                body,
            } => {
                symbols.push_scope();
                if let Some(init_stmt) = init {
                    Self::check_stmt(init_stmt.as_mut(), symbols, func_table)?;
                }
                if let Some(cond) = condition {
                    Self::infer_type(cond, symbols, func_table)?;
                }
                if let Some(inc) = increment {
                    Self::infer_type(inc, symbols, func_table)?;
                }
                Self::check_stmt(body.as_mut(), symbols, func_table)?;
                symbols.pop_scope();
            }
        }

        Ok(())
    }
}
