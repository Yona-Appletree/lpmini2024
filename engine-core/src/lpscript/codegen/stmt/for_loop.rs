/// For loop code generation
extern crate alloc;
use alloc::boxed::Box;

use crate::lpscript::ast::{Expr, Stmt};
use crate::lpscript::vm::opcodes::LpsOpCode;
use super::super::CodeGenerator;

impl<'a> CodeGenerator<'a> {
    pub(in crate::lpscript::codegen::stmt) fn gen_for(
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
        
        // Generate condition (if present)
        if let Some(cond) = condition {
            self.gen_expr(cond);
            
            let jump_to_end_index = self.code.len();
            self.code.push(LpsOpCode::JumpIfZero(0)); // Placeholder
            
            self.gen_stmt(body);
            
            // Generate increment (if present)
            if let Some(inc) = increment {
                self.gen_expr(inc);
                self.code.push(LpsOpCode::Drop); // Discard increment result
            }
            
            // Jump back to condition
            let jump_back_offset = (loop_start as i32) - (self.code.len() as i32) - 1;
            self.code.push(LpsOpCode::Jump(jump_back_offset));
            
            // Patch JumpIfZero to point to end
            let end = self.code.len();
            if let LpsOpCode::JumpIfZero(ref mut offset) = self.code[jump_to_end_index] {
                *offset = (end as i32) - (jump_to_end_index as i32) - 1;
            }
        } else {
            // Infinite loop (no condition)
            self.gen_stmt(body);
            
            if let Some(inc) = increment {
                self.gen_expr(inc);
                self.code.push(LpsOpCode::Drop);
            }
            
            let jump_back_offset = (loop_start as i32) - (self.code.len() as i32) - 1;
            self.code.push(LpsOpCode::Jump(jump_back_offset));
        }
    }
}
