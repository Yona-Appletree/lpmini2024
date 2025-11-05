/// Unary operation code generation
extern crate alloc;
use alloc::boxed::Box;

use crate::lpscript::compiler::ast::Expr;
use crate::lpscript::compiler::codegen::CodeGenerator;
use crate::lpscript::error::Type;
use crate::lpscript::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_neg(&mut self, operand: &Box<Expr>) {
        self.gen_expr(operand);

        // Use appropriate negation opcode based on type
        let ty = operand.ty.as_ref().unwrap();
        match ty {
            Type::Int32 => self.code.push(LpsOpCode::NegInt32),
            Type::Fixed => self.code.push(LpsOpCode::NegFixed),
            // For vector types, negate each component
            // This shouldn't happen since we optimize `-literal` during parsing,
            // but handle it just in case
            _ => self.code.push(LpsOpCode::NegFixed),
        }
    }
}
