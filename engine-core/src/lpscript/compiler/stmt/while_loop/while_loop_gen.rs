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
        
        // Jump back to loop start - calculate relative offset
        let jump_back_pos = self.code.len();
        let relative_offset = (loop_start as i32) - (jump_back_pos as i32) - 1;
        self.code.push(LpsOpCode::Jump(relative_offset));
        
        // Patch the jump to end - calculate relative offset
        let end_offset = self.code.len();
        let relative_offset = (end_offset as i32) - (jump_to_end as i32) - 1;
        self.code[jump_to_end] = LpsOpCode::JumpIfZero(relative_offset);
    }
}
