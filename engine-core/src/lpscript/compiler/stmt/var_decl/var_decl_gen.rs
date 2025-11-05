/// Variable declaration statement code generation
extern crate alloc;
use alloc::string::String;

use crate::lpscript::ast::Expr;
use crate::lpscript::error::Type;
use crate::lpscript::vm::opcodes::LpsOpCode;
use crate::lpscript::compiler::generator::CodeGenerator;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_var_decl(
        &mut self,
        ty: &Type,
        name: &str,
        init: &Option<Expr>,
    ) {
        let index = self.locals.allocate(String::from(name));
        
        if let Some(init_expr) = init {
            // Generate code to evaluate initializer
            self.gen_expr(init_expr);
            
            // Store in local variable
            match ty {
                Type::Fixed | Type::Int32 | Type::Bool => self.code.push(LpsOpCode::StoreLocalFixed(index)),
                Type::Vec2 => self.code.push(LpsOpCode::StoreLocalVec2(index)),
                Type::Vec3 => self.code.push(LpsOpCode::StoreLocalVec3(index)),
                Type::Vec4 => self.code.push(LpsOpCode::StoreLocalVec4(index)),
                _ => {}
            }
        }
    }
}

