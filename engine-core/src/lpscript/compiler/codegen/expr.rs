/// Expression code generation dispatcher
extern crate alloc;

use super::CodeGenerator;
use crate::lpscript::compiler::ast::{Expr, ExprKind};

impl<'a> CodeGenerator<'a> {
    // Old gen_expr method kept for compatibility with individual *_gen.rs files
    // TODO: Once all *_gen.rs files are updated to pool-based API, this can be removed
    #[allow(dead_code)]
    pub(in crate::lpscript) fn gen_expr(&mut self, _expr: &Expr) {
        // Stub - not used in pool-based code path
        // Individual *_gen.rs files still reference this but it's not called
    }
}
