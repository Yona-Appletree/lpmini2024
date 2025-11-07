/// Statement code generation dispatcher
extern crate alloc;

use super::CodeGenerator;
use crate::compiler::ast::{AstPool, Stmt, StmtId};

impl<'a> CodeGenerator<'a> {
    // Statement code generation by ID - main dispatcher
    pub(in crate) fn gen_stmt_id(&mut self, pool: &AstPool, stmt_id: StmtId) {
        use crate::compiler::ast::StmtKind;

        let stmt = pool.stmt(stmt_id);

        match &stmt.kind {
            StmtKind::VarDecl { ty, name, init } => {
                self.gen_var_decl_id(pool, ty, name, init);
            }
            StmtKind::Return(expr_id) => {
                self.gen_return_id(pool, *expr_id);
            }
            StmtKind::Expr(expr_id) => {
                self.gen_expr_stmt_id(pool, *expr_id);
            }
            StmtKind::Block(stmts) => {
                self.gen_block_id(pool, stmts);
            }
            StmtKind::If {
                condition,
                then_stmt,
                else_stmt,
            } => {
                self.gen_if_stmt_id(pool, *condition, *then_stmt, *else_stmt);
            }
            StmtKind::While { condition, body } => {
                self.gen_while_stmt_id(pool, *condition, *body);
            }
            StmtKind::For {
                init,
                condition,
                increment,
                body,
            } => {
                self.gen_for_stmt_id(pool, init, condition, increment, *body);
            }
        }
    }

    // Old gen_stmt method kept for compatibility with individual *_gen.rs files
    // TODO: Once all *_gen.rs files are updated to pool-based API, this can be removed
    #[allow(dead_code)]
    pub(in crate) fn gen_stmt(&mut self, _stmt: &Stmt) {
        // Stub - not used in pool-based code path
        // Individual *_gen.rs files still reference this but it's not called
    }
}
