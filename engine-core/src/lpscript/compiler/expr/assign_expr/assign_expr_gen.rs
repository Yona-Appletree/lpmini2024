/// Assignment expression code generation
extern crate alloc;
use alloc::boxed::Box;

use crate::lpscript::ast::Expr;
use crate::lpscript::error::Type;
use crate::lpscript::vm::opcodes::LpsOpCode;
use crate::lpscript::compiler::generator::CodeGenerator;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_assign_expr(
        &mut self,
        target: &str,
        value: &Box<Expr>,
    ) {
        // Generate code for the value
        self.gen_expr(value);
        
        // Duplicate the value (assignment returns the value)
        self.code.push(LpsOpCode::Dup);
        
        // Store in the variable
        if let Some(index) = self.locals.get(target) {
            let ty = value.ty.as_ref().unwrap();
            match ty {
                Type::Fixed | Type::Int32 | Type::Bool => self.code.push(LpsOpCode::StoreLocalFixed(index)),
                Type::Vec2 => self.code.push(LpsOpCode::StoreLocalVec2(index)),
                Type::Vec3 => self.code.push(LpsOpCode::StoreLocalVec3(index)),
                Type::Vec4 => self.code.push(LpsOpCode::StoreLocalVec4(index)),
                _ => {}
            }
        }
        // Value is left on stack (assignment expression returns the value)
    }
}

