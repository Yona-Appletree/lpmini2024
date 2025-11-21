/// Statement code generation dispatcher
extern crate alloc;

use super::CodeGenerator;
use crate::lp_script::compiler::ast::Stmt;
use crate::lp_script::compiler::error::CodegenError;

impl<'a> CodeGenerator<'a> {
    // Statement code generation - main dispatcher
    pub(crate) fn gen_stmt(&mut self, stmt: &Stmt) -> Result<(), CodegenError> {
        use crate::lp_script::compiler::ast::StmtKind;

        match &stmt.kind {
            StmtKind::VarDecl { ty, name, init } => {
                self.gen_var_decl(ty, name, init.as_ref())?;
            }
            StmtKind::Return(expr) => {
                self.gen_return(expr)?;
            }
            StmtKind::Expr(expr) => {
                self.gen_expr_stmt(expr)?;
            }
            StmtKind::Block(stmts) => {
                self.gen_block(stmts)?;
            }
            StmtKind::If {
                condition,
                then_stmt,
                else_stmt,
            } => {
                self.gen_if_stmt(
                    condition,
                    then_stmt.as_ref(),
                    else_stmt.as_ref().map(|s| s.as_ref()),
                )?;
            }
            StmtKind::While { condition, body } => {
                self.gen_while_stmt(condition, body.as_ref())?;
            }
            StmtKind::For {
                init,
                condition,
                increment,
                body,
            } => {
                self.gen_for_stmt(
                    init.as_ref().map(|s| s.as_ref()),
                    condition.as_ref(),
                    increment.as_ref(),
                    body.as_ref(),
                )?;
            }
        }
        Ok(())
    }
}
