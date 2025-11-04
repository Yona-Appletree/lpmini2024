/// Expression statement code generation
extern crate alloc;

use crate::lpscript::ast::Expr;
use crate::lpscript::vm::opcodes::LpsOpCode;
use super::super::CodeGenerator;

impl<'a> CodeGenerator<'a> {
    pub(in crate::lpscript::codegen::stmt) fn gen_expr_stmt(&mut self, expr: &Expr) {
        self.gen_expr(expr);
        // Pop the result (expression statement doesn't use the value)
        self.code.push(LpsOpCode::Drop);
    }
}
