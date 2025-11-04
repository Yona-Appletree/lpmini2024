/// Block statement code generation
extern crate alloc;

use crate::lpscript::ast::Stmt;
use super::super::CodeGenerator;

impl<'a> CodeGenerator<'a> {
    pub(in crate::lpscript::codegen::stmt) fn gen_block(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            self.gen_stmt(stmt);
        }
    }
}
