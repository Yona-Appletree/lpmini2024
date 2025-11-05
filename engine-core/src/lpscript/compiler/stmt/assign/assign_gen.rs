/// Assignment statement code generation
extern crate alloc;

use crate::lpscript::compiler::ast::Expr;
use crate::lpscript::compiler::codegen::CodeGenerator;
use crate::lpscript::shared::Type;
use crate::lpscript::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_assignment(&mut self, name: &str, value: &Expr) {
        // Generate code for value
        self.gen_expr(value);

        // Store in variable
        if let Some(index) = self.locals.get(name) {
            let ty = value.ty.as_ref().unwrap();
            match ty {
                Type::Fixed | Type::Bool => self.code.push(LpsOpCode::StoreLocalFixed(index)),
                Type::Int32 => self.code.push(LpsOpCode::StoreLocalInt32(index)),
                Type::Vec2 => self.code.push(LpsOpCode::StoreLocalVec2(index)),
                Type::Vec3 => self.code.push(LpsOpCode::StoreLocalVec3(index)),
                Type::Vec4 => self.code.push(LpsOpCode::StoreLocalVec4(index)),
                _ => {}
            }
        }
    }
}
