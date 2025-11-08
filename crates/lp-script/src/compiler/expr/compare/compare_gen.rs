/// Comparison operation code generation
extern crate alloc;

use crate::compiler::ast::Expr;
use crate::compiler::codegen::CodeGenerator;
use crate::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_less(&mut self, left: &Expr, right: &Expr) {
        self.gen_expr(left);
        self.gen_expr(right);
        self.code.push(LpsOpCode::LessFixed);
    }

    pub(crate) fn gen_greater(&mut self, left: &Expr, right: &Expr) {
        self.gen_expr(left);
        self.gen_expr(right);
        self.code.push(LpsOpCode::GreaterFixed);
    }

    pub(crate) fn gen_less_eq(&mut self, left: &Expr, right: &Expr) {
        self.gen_expr(left);
        self.gen_expr(right);
        self.code.push(LpsOpCode::LessEqFixed);
    }

    pub(crate) fn gen_greater_eq(&mut self, left: &Expr, right: &Expr) {
        self.gen_expr(left);
        self.gen_expr(right);
        self.code.push(LpsOpCode::GreaterEqFixed);
    }

    pub(crate) fn gen_eq(&mut self, left: &Expr, right: &Expr) {
        self.gen_expr(left);
        self.gen_expr(right);
        self.code.push(LpsOpCode::EqFixed);
    }

    pub(crate) fn gen_not_eq(&mut self, left: &Expr, right: &Expr) {
        self.gen_expr(left);
        self.gen_expr(right);
        self.code.push(LpsOpCode::NotEqFixed);
    }
}
