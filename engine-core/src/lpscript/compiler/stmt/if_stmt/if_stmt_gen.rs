/// If statement code generation
extern crate alloc;
use alloc::boxed::Box;
use crate::lpscript::ast::{Expr, Stmt};
use crate::lpscript::vm::opcodes::LpsOpCode;
use crate::lpscript::compiler::generator::CodeGenerator;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_if(
        &mut self,
        condition: &Expr,
        then_stmt: &Box<Stmt>,
        else_stmt: &Option<Box<Stmt>>,
    ) {
        self.gen_expr(condition);
        
        let jump_to_else = self.code.len();
        self.code.push(LpsOpCode::JumpIfZero(0)); // Placeholder
        
        self.gen_stmt(then_stmt);
        
        if let Some(else_block) = else_stmt {
            let jump_to_end = self.code.len();
            self.code.push(LpsOpCode::Jump(0)); // Placeholder
            
            let else_offset = self.code.len();
            self.code[jump_to_else] = LpsOpCode::JumpIfZero(else_offset as i32);
            
            self.gen_stmt(else_block);
            
            let end_offset = self.code.len();
            self.code[jump_to_end] = LpsOpCode::Jump(end_offset as i32);
        } else {
            let end_offset = self.code.len();
            self.code[jump_to_else] = LpsOpCode::JumpIfZero(end_offset as i32);
        }
    }
}
