/// Bitwise operation code generation
extern crate alloc;

use crate::compiler::ast::Expr;
use crate::compiler::codegen::CodeGenerator;
use crate::compiler::error::CodegenError;
use crate::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_bitwise_and(
        &mut self,
        left: &Expr,
        right: &Expr,
    ) -> Result<(), CodegenError> {
        self.gen_expr(left)?;
        self.gen_expr(right)?;
        self.code.push(LpsOpCode::BitwiseAndInt32);
        Ok(())
    }

    pub(crate) fn gen_bitwise_or(&mut self, left: &Expr, right: &Expr) -> Result<(), CodegenError> {
        self.gen_expr(left)?;
        self.gen_expr(right)?;
        self.code.push(LpsOpCode::BitwiseOrInt32);
        Ok(())
    }

    pub(crate) fn gen_bitwise_xor(
        &mut self,
        left: &Expr,
        right: &Expr,
    ) -> Result<(), CodegenError> {
        self.gen_expr(left)?;
        self.gen_expr(right)?;
        self.code.push(LpsOpCode::BitwiseXorInt32);
        Ok(())
    }

    pub(crate) fn gen_bitwise_not(&mut self, operand: &Expr) -> Result<(), CodegenError> {
        self.gen_expr(operand)?;
        self.code.push(LpsOpCode::BitwiseNotInt32);
        Ok(())
    }

    pub(crate) fn gen_left_shift(&mut self, left: &Expr, right: &Expr) -> Result<(), CodegenError> {
        self.gen_expr(left)?;
        self.gen_expr(right)?;
        self.code.push(LpsOpCode::LeftShiftInt32);
        Ok(())
    }

    pub(crate) fn gen_right_shift(
        &mut self,
        left: &Expr,
        right: &Expr,
    ) -> Result<(), CodegenError> {
        self.gen_expr(left)?;
        self.gen_expr(right)?;
        self.code.push(LpsOpCode::RightShiftInt32);
        Ok(())
    }
}
