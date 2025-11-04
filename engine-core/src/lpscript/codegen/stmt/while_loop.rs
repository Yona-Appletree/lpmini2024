/// While loop code generation
extern crate alloc;
use alloc::boxed::Box;

use crate::lpscript::ast::{Expr, Stmt};
use crate::lpscript::vm::opcodes::LpsOpCode;
use super::super::CodeGenerator;

impl<'a> CodeGenerator<'a> {
    pub(in crate::lpscript::codegen::stmt) fn gen_while(
        &mut self,
        condition: &Expr,
        body: &Box<Stmt>,
    ) {
        // Generate: loop_start → condition → JumpIfZero(end) → body → Jump(loop_start)
        let loop_start = self.code.len();
        
        self.gen_expr(condition);
        
        let jump_to_end_index = self.code.len();
        self.code.push(LpsOpCode::JumpIfZero(0)); // Placeholder
        
        self.gen_stmt(body);
        
        // Jump back to loop start
        let jump_back_offset = (loop_start as i32) - (self.code.len() as i32) - 1;
        self.code.push(LpsOpCode::Jump(jump_back_offset));
        
        // Patch JumpIfZero to point to end
        let end = self.code.len();
        if let LpsOpCode::JumpIfZero(ref mut offset) = self.code[jump_to_end_index] {
            *offset = (end as i32) - (jump_to_end_index as i32) - 1;
        }
    }
}
