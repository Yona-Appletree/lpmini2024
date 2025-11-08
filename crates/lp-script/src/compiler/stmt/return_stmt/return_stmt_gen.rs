/// Return statement code generation
extern crate alloc;

use crate::compiler::ast::Expr;
use crate::compiler::codegen::CodeGenerator;
use crate::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_return_id(&mut self, pool: &AstPool, expr_id: ExprId) {
        self.gen_expr_id(pool, expr_id);
        self.code.push(LpsOpCode::Return);
    }
}
