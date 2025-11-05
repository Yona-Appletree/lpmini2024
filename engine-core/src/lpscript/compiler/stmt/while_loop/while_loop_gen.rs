/// While loop code generation
extern crate alloc;
use alloc::boxed::Box;
use crate::lpscript::ast::{Expr, Stmt};
use crate::lpscript::vm::opcodes::LpsOpCode;
use crate::lpscript::compiler::generator::CodeGenerator;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_while(
        &mut self,
        condition: &Expr,
        body: &Box<Stmt>,
    ) {
        let loop_start = self.code.len();
        
        self.gen_expr(condition);
        
        let jump_to_end = self.code.len();
        self.code.push(LpsOpCode::JumpIfZero(0)); // Placeholder
        
        self.gen_stmt(body);
        
        self.code.push(LpsOpCode::Jump(loop_start as i32));
        
        let end_offset = self.code.len();
        self.code[jump_to_end] = LpsOpCode::JumpIfZero(end_offset as i32);
    }
}
