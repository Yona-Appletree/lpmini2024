/// Return statement code generation
extern crate alloc;

use crate::compiler::ast::Expr;
use crate::compiler::codegen::CodeGenerator;
use crate::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_return(&mut self, expr: &Expr) {
        self.gen_expr(expr);
        self.code.push(LpsOpCode::Return);
    }
}
