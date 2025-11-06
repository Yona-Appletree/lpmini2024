/// Return statement code generation
extern crate alloc;

use crate::lpscript::compiler::ast::{AstPool, ExprId};
use crate::lpscript::compiler::codegen::CodeGenerator;
use crate::lpscript::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_return_id(&mut self, pool: &AstPool, expr_id: ExprId) {
        self.gen_expr_id(pool, expr_id);
        self.code.push(LpsOpCode::Return);
    }
}
