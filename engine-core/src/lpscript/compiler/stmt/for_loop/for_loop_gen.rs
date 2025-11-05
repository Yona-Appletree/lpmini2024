/// For loop code generation
extern crate alloc;
use alloc::boxed::Box;
use crate::lpscript::ast::{Expr, Stmt};
use crate::lpscript::vm::opcodes::LpsOpCode;
use crate::lpscript::compiler::generator::CodeGenerator;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_for(
        &mut self,
        init: &Option<Box<Stmt>>,
        condition: &Option<Expr>,
        increment: &Option<Expr>,
        body: &Box<Stmt>,
    ) {
        // Generate init
        if let Some(init_stmt) = init {
            self.gen_stmt(init_stmt);
        }
        
        let loop_start = self.code.len();
        
        // Generate condition (default to true if none)
        let jump_to_end = if let Some(cond) = condition {
            self.gen_expr(cond);
            let jump = self.code.len();
            self.code.push(LpsOpCode::JumpIfZero(0)); // Placeholder
            jump
        } else {
            0 // No jump needed if no condition
        };
        
        // Generate body
        self.gen_stmt(body);
        
        // Generate increment
        if let Some(inc) = increment {
            self.gen_expr(inc);
            // Pop the result since we don't need it
            self.code.push(LpsOpCode::Drop1);
        }
        
        // Jump back to loop start - calculate relative offset
        let jump_back_pos = self.code.len();
        let relative_offset = (loop_start as i32) - (jump_back_pos as i32) - 1;
        self.code.push(LpsOpCode::Jump(relative_offset));
        
        // Fix jump_to_end if we have a condition - calculate relative offset
        if condition.is_some() {
            let end_offset = self.code.len();
            let relative_offset = (end_offset as i32) - (jump_to_end as i32) - 1;
            self.code[jump_to_end] = LpsOpCode::JumpIfZero(relative_offset);
        }
    }
}
