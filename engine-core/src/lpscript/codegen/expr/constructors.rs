/// Vector constructor code generation
extern crate alloc;

use crate::lpscript::ast::Expr;
use super::super::CodeGenerator;

impl<'a> CodeGenerator<'a> {
    pub(in crate::lpscript::codegen::expr) fn gen_vec_constructor(&mut self, args: &[Expr]) {
        // Generate code for each argument, which pushes its components
        // Supports GLSL-style mixed args: vec3(vec2, float), vec4(vec3, float), etc.
        for arg in args {
            self.gen_expr(arg);
        }
        // Components are now on stack in the correct order
    }
}
