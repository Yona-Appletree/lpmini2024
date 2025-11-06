/// Unary operation code generation
extern crate alloc;

use crate::lpscript::compiler::ast::{AstPool, ExprId};
use crate::lpscript::compiler::codegen::CodeGenerator;
use crate::lpscript::shared::Type;
use crate::lpscript::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_neg_id(&mut self, pool: &AstPool, operand: ExprId) {
        self.gen_expr_id(pool, operand);
        let operand_ty = pool.expr(operand).ty.as_ref();
        self.code.push(match operand_ty {
            Some(Type::Int32) => LpsOpCode::NegInt32,
            _ => LpsOpCode::NegFixed,
        });
    }
}
