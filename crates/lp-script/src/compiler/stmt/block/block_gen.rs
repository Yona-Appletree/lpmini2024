/// Block statement code generation
extern crate alloc;

use crate::compiler::ast::Stmt;
use crate::compiler::codegen::CodeGenerator;
use crate::compiler::error::CodegenError;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_block(&mut self, stmts: &[Stmt]) -> Result<(), CodegenError> {
        self.locals.push_scope();
        for stmt in stmts {
            self.gen_stmt(stmt)?;
        }
        self.locals.pop_scope();
        Ok(())
    }
}
