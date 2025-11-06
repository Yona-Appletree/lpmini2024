/// Block statement code generation
extern crate alloc;

use crate::lpscript::compiler::ast::{AstPool, StmtId};
use crate::lpscript::compiler::codegen::CodeGenerator;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_block_id(&mut self, pool: &AstPool, stmts: &[StmtId]) {
        self.locals.push_scope();
        for &stmt_id in stmts {
            self.gen_stmt_id(pool, stmt_id);
        }
        self.locals.pop_scope();
    }
}
