/// Assignment expression code generation
extern crate alloc;
use alloc::boxed::Box;

use crate::lpscript::compiler::ast::Expr;
use crate::lpscript::compiler::codegen::CodeGenerator;
use crate::lpscript::shared::Type;
use crate::lpscript::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_assign_expr(&mut self, target: &str, value: &Box<Expr>) {
        // Generate code for the value
        self.gen_expr(value);

        // Duplicate the value (assignment returns the value)
        // Use the appropriate Dup opcode based on the value type
        let ty = value.ty.as_ref().unwrap();
        match ty {
            Type::Fixed | Type::Int32 | Type::Bool => self.code.push(LpsOpCode::Dup1),
            Type::Vec2 => self.code.push(LpsOpCode::Dup2),
            Type::Vec3 => self.code.push(LpsOpCode::Dup3),
            Type::Vec4 => self.code.push(LpsOpCode::Dup4),
            _ => {}
        }

        // Store in the variable
        if let Some(index) = self.locals.get(target) {
            match ty {
                Type::Fixed | Type::Int32 | Type::Bool => {
                    self.code.push(LpsOpCode::StoreLocalFixed(index))
                }
                Type::Vec2 => self.code.push(LpsOpCode::StoreLocalVec2(index)),
                Type::Vec3 => self.code.push(LpsOpCode::StoreLocalVec3(index)),
                Type::Vec4 => self.code.push(LpsOpCode::StoreLocalVec4(index)),
                _ => {}
            }
        }
        // Value is left on stack (assignment expression returns the value)
    }
}
