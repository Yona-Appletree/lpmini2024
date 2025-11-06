/// Variable declaration code generation
extern crate alloc;
use alloc::string::ToString;

use crate::lpscript::compiler::ast::{AstPool, ExprId};
use crate::lpscript::compiler::codegen::CodeGenerator;
use crate::lpscript::shared::Type;
use crate::lpscript::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_var_decl_id(
        &mut self,
        pool: &AstPool,
        ty: &Type,
        name: &str,
        init: &Option<ExprId>,
    ) {
        // Allocate a local for this variable
        // This will allocate in the same order as the analyzer did
        let local_idx = self.locals.allocate_typed(name.to_string(), ty.clone());
        
        if let Some(init_id) = init {
            self.gen_expr_id(pool, *init_id);
            // Use type-specific StoreLocal opcode
            self.code.push(match ty {
                Type::Fixed | Type::Bool => LpsOpCode::StoreLocalFixed(local_idx),
                Type::Int32 => LpsOpCode::StoreLocalInt32(local_idx),
                Type::Vec2 => LpsOpCode::StoreLocalVec2(local_idx),
                Type::Vec3 => LpsOpCode::StoreLocalVec3(local_idx),
                Type::Vec4 => LpsOpCode::StoreLocalVec4(local_idx),
                _ => LpsOpCode::StoreLocalFixed(local_idx), // Fallback
            });
        }
    }
}
