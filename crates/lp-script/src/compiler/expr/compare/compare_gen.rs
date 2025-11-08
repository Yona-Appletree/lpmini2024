/// Comparison operation code generation
extern crate alloc;

use crate::compiler::ast::Expr;
use crate::compiler::codegen::CodeGenerator;
use crate::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_less_id(&mut self, pool: &AstPool, left: Expr, right: ExprId) {
        self.gen_expr_id(pool, left);
        self.gen_expr_id(pool, right);
        self.code.push(LpsOpCode::LessFixed);
    }

    pub(crate) fn gen_greater_id(&mut self, pool: &AstPool, left: Expr, right: ExprId) {
        self.gen_expr_id(pool, left);
        self.gen_expr_id(pool, right);
        self.code.push(LpsOpCode::GreaterFixed);
    }

    pub(crate) fn gen_less_eq_id(&mut self, pool: &AstPool, left: Expr, right: ExprId) {
        self.gen_expr_id(pool, left);
        self.gen_expr_id(pool, right);
        self.code.push(LpsOpCode::LessEqFixed);
    }

    pub(crate) fn gen_greater_eq_id(&mut self, pool: &AstPool, left: Expr, right: ExprId) {
        self.gen_expr_id(pool, left);
        self.gen_expr_id(pool, right);
        self.code.push(LpsOpCode::GreaterEqFixed);
    }

    pub(crate) fn gen_eq_id(&mut self, pool: &AstPool, left: Expr, right: ExprId) {
        self.gen_expr_id(pool, left);
        self.gen_expr_id(pool, right);
        self.code.push(LpsOpCode::EqFixed);
    }

    pub(crate) fn gen_not_eq_id(&mut self, pool: &AstPool, left: Expr, right: ExprId) {
        self.gen_expr_id(pool, left);
        self.gen_expr_id(pool, right);
        self.code.push(LpsOpCode::NotEqFixed);
    }
}
