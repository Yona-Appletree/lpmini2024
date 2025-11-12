/// Expression statement code generation
extern crate alloc;

use crate::compiler::ast::Expr;
use crate::compiler::codegen::CodeGenerator;
use crate::compiler::error::CodegenError;
use crate::shared::Type;
use crate::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_expr_stmt(&mut self, expr: &Expr) -> Result<(), CodegenError> {
        self.gen_expr(expr)?;
        // Expression statements discard their result
        // Drop appropriate number of stack values based on expression type
        let expr_ty = expr.ty.as_ref();
        let drop_op = match expr_ty {
            Some(Type::Vec2) => LpsOpCode::Drop2,
            Some(Type::Vec3) => LpsOpCode::Drop3,
            Some(Type::Vec4) => LpsOpCode::Drop4,
            _ => LpsOpCode::Drop1,
        };
        self.code.push(drop_op);
        Ok(())
    }
}
