/// Logical operation code generation
extern crate alloc;

use crate::compiler::ast::Expr;
use crate::compiler::codegen::CodeGenerator;
use crate::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_and(&mut self, left: &Expr, right: &Expr) {
        self.gen_expr(left);
        self.gen_expr(right);
        self.code.push(LpsOpCode::AndFixed);
    }

    pub(crate) fn gen_or(&mut self, left: &Expr, right: &Expr) {
        self.gen_expr(left);
        self.gen_expr(right);
        self.code.push(LpsOpCode::OrFixed);
    }

    pub(crate) fn gen_not(&mut self, operand: &Expr) {
        self.gen_expr(operand);
        self.code.push(LpsOpCode::NotFixed);
    }
}
