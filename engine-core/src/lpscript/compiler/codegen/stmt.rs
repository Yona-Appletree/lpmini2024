/// Statement code generation dispatcher
extern crate alloc;

use super::CodeGenerator;
use crate::lpscript::compiler::ast::{Stmt, StmtKind};

impl<'a> CodeGenerator<'a> {
    pub(in crate::lpscript) fn gen_stmt(&mut self, stmt: &Stmt) {
        match &stmt.kind {
            StmtKind::VarDecl { ty, name, init } => self.gen_var_decl(ty, name, init),
            StmtKind::Return(expr) => self.gen_return(expr),
            StmtKind::Expr(expr) => self.gen_expr_stmt(expr),
            StmtKind::Block(stmts) => self.gen_block(stmts),
            StmtKind::If {
                condition,
                then_stmt,
                else_stmt,
            } => self.gen_if(condition, then_stmt, else_stmt),
            StmtKind::While { condition, body } => self.gen_while(condition, body),
            StmtKind::For {
                init,
                condition,
                increment,
                body,
            } => self.gen_for(init, condition, increment, body),
        }
    }
}
