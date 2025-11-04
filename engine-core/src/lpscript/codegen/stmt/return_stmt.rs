/// Return statement code generation
extern crate alloc;

use crate::lpscript::ast::Expr;
use crate::lpscript::vm::opcodes::LpsOpCode;
use super::super::CodeGenerator;

impl<'a> CodeGenerator<'a> {
    pub(in crate::lpscript::codegen::stmt) fn gen_return(&mut self, expr: &Expr) {
        self.gen_expr(expr);
        self.code.push(LpsOpCode::Return);
    }
}
