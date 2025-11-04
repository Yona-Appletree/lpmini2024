/// Logical operation code generation
extern crate alloc;
use alloc::boxed::Box;

use crate::lpscript::ast::Expr;
use crate::lpscript::vm::opcodes::LpsOpCode;
use super::super::CodeGenerator;

impl<'a> CodeGenerator<'a> {
    pub(in crate::lpscript::codegen::expr) fn gen_and(&mut self, left: &Box<Expr>, right: &Box<Expr>) {
        self.gen_expr(left);
        self.gen_expr(right);
        self.code.push(LpsOpCode::AndFixed);
    }

    pub(in crate::lpscript::codegen::expr) fn gen_or(&mut self, left: &Box<Expr>, right: &Box<Expr>) {
        self.gen_expr(left);
        self.gen_expr(right);
        self.code.push(LpsOpCode::OrFixed);
    }
}
