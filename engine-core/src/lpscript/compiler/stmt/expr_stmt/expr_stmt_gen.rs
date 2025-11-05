/// Expression statement code generation
extern crate alloc;
use crate::lpscript::ast::Expr;
use crate::lpscript::compiler::generator::CodeGenerator;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_expr_stmt(&mut self, expr: &Expr) {
        self.gen_expr(expr);
        // Expression result is left on stack (typically for side effects)
    }
}
