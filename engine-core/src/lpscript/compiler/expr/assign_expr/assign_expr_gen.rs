/// Assignment expression code generation
extern crate alloc;

use crate::lpscript::compiler::ast::{AstPool, ExprId};
use crate::lpscript::compiler::codegen::CodeGenerator;
use crate::lpscript::shared::Type;
use crate::lpscript::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_assign_expr_id(&mut self, pool: &AstPool, target: &str, value: ExprId) {
        self.gen_expr_id(pool, value);
        
        if let Some(local_idx) = self.locals.get(target) {
            let var_type = self.locals.get_type(local_idx).unwrap_or(&Type::Fixed);
            
            // Duplicate value based on type (assignment returns the assigned value)
            match var_type {
                Type::Vec2 => self.code.push(LpsOpCode::Dup2),
                Type::Vec3 => self.code.push(LpsOpCode::Dup3),
                Type::Vec4 => self.code.push(LpsOpCode::Dup4),
                _ => self.code.push(LpsOpCode::Dup1),
            }
            
            // Store using type-specific opcode
            self.code.push(match var_type {
                Type::Fixed | Type::Bool => LpsOpCode::StoreLocalFixed(local_idx),
                Type::Int32 => LpsOpCode::StoreLocalInt32(local_idx),
                Type::Vec2 => LpsOpCode::StoreLocalVec2(local_idx),
                Type::Vec3 => LpsOpCode::StoreLocalVec3(local_idx),
                Type::Vec4 => LpsOpCode::StoreLocalVec4(local_idx),
                _ => LpsOpCode::StoreLocalFixed(local_idx),
            });
        }
    }
}
