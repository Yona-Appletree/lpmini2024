/// Vector constructor code generation
extern crate alloc;

use crate::compiler::ast::Expr;
use crate::compiler::codegen::CodeGenerator;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_vec_constructor(&mut self, args: &[Expr]) {
        // Generate code for each argument (leaves values on stack in order)
        for arg in args {
            self.gen_expr(arg);
        }
        // Vector constructors don't need a special opcode - args are already on stack
        // Vec2(x, y) leaves x, y on stack (that IS a vec2)
    }
}
