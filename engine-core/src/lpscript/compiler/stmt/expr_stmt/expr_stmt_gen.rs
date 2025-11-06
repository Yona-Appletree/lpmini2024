/// Expression statement code generation
extern crate alloc;

use crate::lpscript::compiler::ast::{AstPool, ExprId};
use crate::lpscript::compiler::codegen::CodeGenerator;
use crate::lpscript::shared::Type;
use crate::lpscript::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_expr_stmt_id(&mut self, pool: &AstPool, expr_id: ExprId) {
        self.gen_expr_id(pool, expr_id);
        // Expression statements discard their result
        // Drop appropriate number of stack values based on expression type
        let expr_ty = pool.expr(expr_id).ty.as_ref();
        let drop_op = match expr_ty {
            Some(Type::Vec2) => LpsOpCode::Drop2,
            Some(Type::Vec3) => LpsOpCode::Drop3,
            Some(Type::Vec4) => LpsOpCode::Drop4,
            _ => LpsOpCode::Drop1,
        };
        self.code.push(drop_op);
    }
}
