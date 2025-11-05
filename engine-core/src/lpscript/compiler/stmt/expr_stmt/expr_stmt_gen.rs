/// Expression statement code generation
extern crate alloc;
use crate::lpscript::ast::Expr;
use crate::lpscript::compiler::generator::CodeGenerator;
use crate::lpscript::error::Type;
use crate::lpscript::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_expr_stmt(&mut self, expr: &Expr) {
        self.gen_expr(expr);
        
        // Drop the result since it's not used (expression statement for side effects)
        // Number of values to drop depends on the expression type
        let drop_count = match expr.ty.as_ref().unwrap() {
            Type::Fixed | Type::Bool | Type::Int32 => 1,
            Type::Vec2 => 2,
            Type::Vec3 => 3,
            Type::Vec4 => 4,
            Type::Void => 0, // No result to drop
        };
        
        for _ in 0..drop_count {
            self.code.push(LpsOpCode::Drop1);
        }
    }
}
