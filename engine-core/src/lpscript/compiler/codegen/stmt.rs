/// Statement code generation dispatcher
extern crate alloc;

use super::CodeGenerator;
use crate::lpscript::compiler::ast::Stmt;

impl<'a> CodeGenerator<'a> {
    // Old gen_stmt method kept for compatibility with individual *_gen.rs files
    // TODO: Once all *_gen.rs files are updated to pool-based API, this can be removed
    #[allow(dead_code)]
    pub(in crate::lpscript) fn gen_stmt(&mut self, _stmt: &Stmt) {
        // Stub - not used in pool-based code path
        // Individual *_gen.rs files still reference this but it's not called
    }
}
