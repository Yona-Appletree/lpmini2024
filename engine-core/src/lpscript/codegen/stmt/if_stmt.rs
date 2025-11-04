/// If/else statement code generation
extern crate alloc;
use alloc::boxed::Box;

use crate::lpscript::ast::{Expr, Stmt};
use crate::lpscript::vm::opcodes::LpsOpCode;
use super::super::CodeGenerator;

impl<'a> CodeGenerator<'a> {
    pub(in crate::lpscript::codegen::stmt) fn gen_if(
        &mut self,
        condition: &Expr,
        then_stmt: &Box<Stmt>,
        else_stmt: &Option<Box<Stmt>>,
    ) {
        // Generate: condition → JumpIfZero(else_offset) → then_block → Jump(end_offset) → else_block
        self.gen_expr(condition);
        
        // Placeholder for JumpIfZero - we'll patch the offset later
        let jump_to_else_index = self.code.len();
        self.code.push(LpsOpCode::JumpIfZero(0)); // Placeholder offset
        
        // Generate then block
        self.gen_stmt(then_stmt);
        
        if let Some(else_s) = else_stmt {
            // Placeholder for Jump past else block
            let jump_to_end_index = self.code.len();
            self.code.push(LpsOpCode::Jump(0)); // Placeholder offset
            
            // Patch the JumpIfZero to point here (start of else block)
            let else_start = self.code.len();
            if let LpsOpCode::JumpIfZero(ref mut offset) = self.code[jump_to_else_index] {
                *offset = (else_start as i32) - (jump_to_else_index as i32) - 1;
            }
            
            // Generate else block
            self.gen_stmt(else_s);
            
            // Patch the Jump to point here (end)
            let end = self.code.len();
            if let LpsOpCode::Jump(ref mut offset) = self.code[jump_to_end_index] {
                *offset = (end as i32) - (jump_to_end_index as i32) - 1;
            }
        } else {
            // No else block - patch JumpIfZero to point to end
            let end = self.code.len();
            if let LpsOpCode::JumpIfZero(ref mut offset) = self.code[jump_to_else_index] {
                *offset = (end as i32) - (jump_to_else_index as i32) - 1;
            }
        }
    }
}
