/// Unary operation code generation
extern crate alloc;

use crate::compiler::ast::Expr;
use crate::compiler::codegen::CodeGenerator;
use crate::shared::Type;
use crate::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_neg(&mut self, operand: &Expr) {
        self.gen_expr(operand);
        let operand_ty = operand.ty.as_ref();
        self.code.push(match operand_ty {
            Some(Type::Int32) => LpsOpCode::NegInt32,
            _ => LpsOpCode::NegFixed,
        });
    }
}
