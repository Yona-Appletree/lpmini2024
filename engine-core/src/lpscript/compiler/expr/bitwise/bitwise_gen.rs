/// Bitwise operation code generation
extern crate alloc;
use alloc::boxed::Box;

use crate::lpscript::compiler::ast::Expr;
use crate::lpscript::compiler::codegen::CodeGenerator;
use crate::lpscript::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_bitwise_and(&mut self, left: &Box<Expr>, right: &Box<Expr>) {
        self.gen_expr(left);
        self.gen_expr(right);
        self.code.push(LpsOpCode::BitwiseAndInt32);
    }

    pub(crate) fn gen_bitwise_or(&mut self, left: &Box<Expr>, right: &Box<Expr>) {
        self.gen_expr(left);
        self.gen_expr(right);
        self.code.push(LpsOpCode::BitwiseOrInt32);
    }

    pub(crate) fn gen_bitwise_xor(&mut self, left: &Box<Expr>, right: &Box<Expr>) {
        self.gen_expr(left);
        self.gen_expr(right);
        self.code.push(LpsOpCode::BitwiseXorInt32);
    }

    pub(crate) fn gen_bitwise_not(&mut self, operand: &Box<Expr>) {
        self.gen_expr(operand);
        self.code.push(LpsOpCode::BitwiseNotInt32);
    }

    pub(crate) fn gen_left_shift(&mut self, left: &Box<Expr>, right: &Box<Expr>) {
        self.gen_expr(left);
        self.gen_expr(right);
        self.code.push(LpsOpCode::LeftShiftInt32);
    }

    pub(crate) fn gen_right_shift(&mut self, left: &Box<Expr>, right: &Box<Expr>) {
        self.gen_expr(left);
        self.gen_expr(right);
        self.code.push(LpsOpCode::RightShiftInt32);
    }
}

