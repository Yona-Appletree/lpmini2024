/// Unary operation code generation
extern crate alloc;
use alloc::boxed::Box;

use crate::lpscript::compiler::ast::Expr;
use crate::lpscript::compiler::codegen::CodeGenerator;
use crate::lpscript::shared::Type;
use crate::lpscript::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_neg(&mut self, operand: &Box<Expr>) {
        self.gen_expr(operand);

        // Use appropriate negation opcode based on type
        let ty = operand.ty.as_ref().unwrap();
        match ty {
            Type::Int32 => self.code.push(LpsOpCode::NegInt32),
            Type::Fixed => self.code.push(LpsOpCode::NegFixed),
            Type::Vec2 => self.code.push(LpsOpCode::NegVec2),
            Type::Vec3 => self.code.push(LpsOpCode::NegVec3),
            Type::Vec4 => self.code.push(LpsOpCode::NegVec4),
            _ => {} // Bool or Void - shouldn't happen
        }
    }
}
