/// Logical operation code generation
extern crate alloc;

use crate::compiler::ast::Expr;
use crate::compiler::codegen::CodeGenerator;
use crate::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_and_id(&mut self, pool: &AstPool, left: Expr, right: ExprId) {
        self.gen_expr_id(pool, left);
        self.gen_expr_id(pool, right);
        self.code.push(LpsOpCode::AndFixed);
    }

    pub(crate) fn gen_or_id(&mut self, pool: &AstPool, left: Expr, right: ExprId) {
        self.gen_expr_id(pool, left);
        self.gen_expr_id(pool, right);
        self.code.push(LpsOpCode::OrFixed);
    }

    pub(crate) fn gen_not_id(&mut self, pool: &AstPool, operand: ExprId) {
        self.gen_expr_id(pool, operand);
        self.code.push(LpsOpCode::NotFixed);
    }
}
