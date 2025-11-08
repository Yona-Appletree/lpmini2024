/// Bitwise operation code generation
extern crate alloc;

use crate::compiler::ast::Expr;
use crate::compiler::codegen::CodeGenerator;
use crate::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_bitwise_and_id(&mut self, pool: &AstPool, left: Expr, right: ExprId) {
        self.gen_expr_id(pool, left);
        self.gen_expr_id(pool, right);
        self.code.push(LpsOpCode::BitwiseAndInt32);
    }

    pub(crate) fn gen_bitwise_or_id(&mut self, pool: &AstPool, left: Expr, right: ExprId) {
        self.gen_expr_id(pool, left);
        self.gen_expr_id(pool, right);
        self.code.push(LpsOpCode::BitwiseOrInt32);
    }

    pub(crate) fn gen_bitwise_xor_id(&mut self, pool: &AstPool, left: Expr, right: ExprId) {
        self.gen_expr_id(pool, left);
        self.gen_expr_id(pool, right);
        self.code.push(LpsOpCode::BitwiseXorInt32);
    }

    pub(crate) fn gen_bitwise_not_id(&mut self, pool: &AstPool, operand: ExprId) {
        self.gen_expr_id(pool, operand);
        self.code.push(LpsOpCode::BitwiseNotInt32);
    }

    pub(crate) fn gen_left_shift_id(&mut self, pool: &AstPool, left: Expr, right: ExprId) {
        self.gen_expr_id(pool, left);
        self.gen_expr_id(pool, right);
        self.code.push(LpsOpCode::LeftShiftInt32);
    }

    pub(crate) fn gen_right_shift_id(&mut self, pool: &AstPool, left: Expr, right: ExprId) {
        self.gen_expr_id(pool, left);
        self.gen_expr_id(pool, right);
        self.code.push(LpsOpCode::RightShiftInt32);
    }
}
