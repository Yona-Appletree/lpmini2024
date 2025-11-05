/// Ternary conditional code generation
extern crate alloc;
use alloc::boxed::Box;

use crate::lpscript::compiler::ast::Expr;
use crate::lpscript::compiler::codegen::CodeGenerator;
use crate::lpscript::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_ternary(
        &mut self,
        condition: &Box<Expr>,
        true_expr: &Box<Expr>,
        false_expr: &Box<Expr>,
    ) {
        self.gen_expr(condition);
        self.gen_expr(true_expr);
        self.gen_expr(false_expr);
        self.code.push(LpsOpCode::Select);
    }
}
