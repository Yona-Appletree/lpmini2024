/// Assignment statement code generation
extern crate alloc;

use crate::lpscript::ast::Expr;
use crate::lpscript::error::Type;
use crate::lpscript::vm::opcodes::LpsOpCode;
use super::super::CodeGenerator;

impl<'a> CodeGenerator<'a> {
    pub(in crate::lpscript::codegen::stmt) fn gen_assignment(
        &mut self,
        name: &str,
        value: &Expr,
    ) {
        // Generate code for value
        self.gen_expr(value);
        
        // Store in variable
        if let Some(index) = self.locals.get(name) {
            let ty = value.ty.as_ref().unwrap();
            match ty {
                Type::Fixed | Type::Int32 => self.code.push(LpsOpCode::StoreLocalFixed(index)),
                Type::Vec2 => self.code.push(LpsOpCode::StoreLocalVec2(index)),
                Type::Vec3 => self.code.push(LpsOpCode::StoreLocalVec3(index)),
                Type::Vec4 => self.code.push(LpsOpCode::StoreLocalVec4(index)),
                _ => {}
            }
        }
    }
}
