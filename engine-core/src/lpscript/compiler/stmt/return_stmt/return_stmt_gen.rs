/// Return statement code generation
extern crate alloc;
use crate::lpscript::compiler::ast::Expr;
use crate::lpscript::compiler::codegen::CodeGenerator;
use crate::lpscript::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_return(&mut self, expr: &Expr) {
        self.gen_expr(expr);
        self.code.push(LpsOpCode::Return);
    }
}
