/// Return statement code generation
extern crate alloc;

use crate::lp_script::compiler::ast::Expr;
use crate::lp_script::compiler::codegen::CodeGenerator;
use crate::lp_script::compiler::error::CodegenError;
use crate::lp_script::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_return(&mut self, expr: &Expr) -> Result<(), CodegenError> {
        self.gen_expr(expr)?;
        self.code.push(LpsOpCode::Return);
        Ok(())
    }
}
