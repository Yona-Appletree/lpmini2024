/// Return statement code generation
extern crate alloc;
use crate::lpscript::ast::Expr;
use crate::lpscript::vm::opcodes::LpsOpCode;
use crate::lpscript::compiler::generator::CodeGenerator;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_return(&mut self, expr: &Expr) {
        self.gen_expr(expr);
        self.code.push(LpsOpCode::Return);
    }
}
