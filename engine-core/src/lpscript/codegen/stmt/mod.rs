/// Statement code generation
extern crate alloc;

use crate::lpscript::ast::{Stmt, StmtKind};
use super::CodeGenerator;

mod var_decl;
mod assign;
mod return_stmt;
mod expr_stmt;
mod block;
mod if_stmt;
mod while_loop;
mod for_loop;

impl<'a> CodeGenerator<'a> {
    /// Generate code for a statement
    pub(crate) fn gen_stmt(&mut self, stmt: &Stmt) {
        match &stmt.kind {
            StmtKind::VarDecl { ty, name, init } => {
                self.gen_var_decl(ty, name, init);
            }
            
            StmtKind::Assignment { name, value } => {
                self.gen_assignment(name, value);
            }
            
            StmtKind::Return(expr) => {
                self.gen_return(expr);
            }
            
            StmtKind::Expr(expr) => {
                self.gen_expr_stmt(expr);
            }
            
            StmtKind::Block(stmts) => {
                self.gen_block(stmts);
            }
            
            StmtKind::If { condition, then_stmt, else_stmt } => {
                self.gen_if(condition, then_stmt, else_stmt);
            }
            
            StmtKind::While { condition, body } => {
                self.gen_while(condition, body);
            }
            
            StmtKind::For { init, condition, increment, body } => {
                self.gen_for(init, condition, increment, body);
            }
        }
    }
}

