/// Statement type checking
extern crate alloc;

use crate::compiler::ast::{AstPool, StmtId, StmtKind};
use crate::compiler::error::TypeError;
use crate::compiler::typechecker::{FunctionTable, SymbolTable, TypeChecker};

impl TypeChecker {
    /// Type check a statement by ID
    pub(crate) fn check_stmt_id(
        pool: &mut AstPool,
        stmt_id: StmtId,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
    ) -> Result<(), TypeError> {
        // Clone the statement kind to avoid borrow issues
        let stmt_kind = pool.stmt(stmt_id).kind.clone();

        match stmt_kind {
            StmtKind::VarDecl { ty, name, init } => {
                if let Some(init_id) = init {
                    Self::infer_type_id(pool, init_id, symbols, func_table)?;
                }
                let _ = symbols.declare(name, ty);
            }

            StmtKind::Return(expr_id) => {
                Self::infer_type_id(pool, expr_id, symbols, func_table)?;
            }

            StmtKind::Expr(expr_id) => {
                Self::infer_type_id(pool, expr_id, symbols, func_table)?;
            }

            StmtKind::Block(stmts) => {
                symbols.push_scope();
                for stmt_id in stmts {
                    Self::check_stmt_id(pool, stmt_id, symbols, func_table)?;
                }
                symbols.pop_scope();
            }

            StmtKind::If {
                condition,
                then_stmt,
                else_stmt,
            } => {
                Self::infer_type_id(pool, condition, symbols, func_table)?;
                Self::check_stmt_id(pool, then_stmt, symbols, func_table)?;
                if let Some(else_id) = else_stmt {
                    Self::check_stmt_id(pool, else_id, symbols, func_table)?;
                }
            }

            StmtKind::While { condition, body } => {
                Self::infer_type_id(pool, condition, symbols, func_table)?;
                Self::check_stmt_id(pool, body, symbols, func_table)?;
            }

            StmtKind::For {
                init,
                condition,
                increment,
                body,
            } => {
                symbols.push_scope();
                if let Some(init_id) = init {
                    Self::check_stmt_id(pool, init_id, symbols, func_table)?;
                }
                if let Some(cond_id) = condition {
                    Self::infer_type_id(pool, cond_id, symbols, func_table)?;
                }
                if let Some(inc_id) = increment {
                    Self::infer_type_id(pool, inc_id, symbols, func_table)?;
                }
                Self::check_stmt_id(pool, body, symbols, func_table)?;
                symbols.pop_scope();
            }
        }

        Ok(())
    }
}
