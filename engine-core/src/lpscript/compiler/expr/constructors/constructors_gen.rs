/// Vector constructor code generation
extern crate alloc;

use crate::lpscript::compiler::ast::{AstPool, ExprId};
use crate::lpscript::compiler::codegen::CodeGenerator;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_vec_constructor_id(&mut self, pool: &AstPool, args: &[ExprId]) {
        // Generate code for each argument (leaves values on stack in order)
        for arg_id in args {
            self.gen_expr_id(pool, *arg_id);
        }
        // Vector constructors don't need a special opcode - args are already on stack
        // Vec2(x, y) leaves x, y on stack (that IS a vec2)
    }
}
