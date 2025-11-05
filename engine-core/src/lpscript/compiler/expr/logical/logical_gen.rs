/// Logical operation code generation
extern crate alloc;
use alloc::boxed::Box;

use crate::lpscript::compiler::ast::Expr;
use crate::lpscript::compiler::codegen::CodeGenerator;
use crate::lpscript::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_and(&mut self, left: &Box<Expr>, right: &Box<Expr>) {
        self.gen_expr(left);
        self.gen_expr(right);
        self.code.push(LpsOpCode::AndFixed);
    }

    pub(crate) fn gen_or(&mut self, left: &Box<Expr>, right: &Box<Expr>) {
        self.gen_expr(left);
        self.gen_expr(right);
        self.code.push(LpsOpCode::OrFixed);
    }

    pub(crate) fn gen_not(&mut self, operand: &Box<Expr>) {
        self.gen_expr(operand);
        self.code.push(LpsOpCode::NotFixed);
    }
}
