/// Logical operation code generation
extern crate alloc;

use crate::lp_script::compiler::ast::Expr;
use crate::lp_script::compiler::codegen::CodeGenerator;
use crate::lp_script::compiler::error::CodegenError;
use crate::lp_script::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_and(&mut self, left: &Expr, right: &Expr) -> Result<(), CodegenError> {
        self.gen_expr(left)?;
        self.gen_expr(right)?;
        self.code.push(LpsOpCode::AndDec32);
        Ok(())
    }

    pub(crate) fn gen_or(&mut self, left: &Expr, right: &Expr) -> Result<(), CodegenError> {
        self.gen_expr(left)?;
        self.gen_expr(right)?;
        self.code.push(LpsOpCode::OrDec32);
        Ok(())
    }

    pub(crate) fn gen_not(&mut self, operand: &Expr) -> Result<(), CodegenError> {
        self.gen_expr(operand)?;
        self.code.push(LpsOpCode::NotDec32);
        Ok(())
    }
}
