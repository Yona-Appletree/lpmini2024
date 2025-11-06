/// If statement code generation
extern crate alloc;

use crate::lpscript::compiler::ast::{AstPool, ExprId, StmtId};
use crate::lpscript::compiler::codegen::CodeGenerator;
use crate::lpscript::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_if_stmt_id(
        &mut self,
        pool: &AstPool,
        condition: ExprId,
        then_stmt: StmtId,
        else_stmt: Option<StmtId>,
    ) {
        // Generate condition
        self.gen_expr_id(pool, condition);
        
        // JumpIfZero to else/end
        let jump_to_else = self.code.len();
        self.code.push(LpsOpCode::JumpIfZero(0)); // Placeholder
        
        // Then branch
        self.gen_stmt_id(pool, then_stmt);
        
        if let Some(else_id) = else_stmt {
            // Jump over else
            let jump_to_end = self.code.len();
            self.code.push(LpsOpCode::Jump(0)); // Placeholder
            
            // Patch jump to else
            let else_start = self.code.len();
            if let LpsOpCode::JumpIfZero(ref mut offset) = self.code[jump_to_else] {
                *offset = (else_start as i32) - (jump_to_else as i32) - 1;
            }
            
            // Else branch
            self.gen_stmt_id(pool, else_id);
            
            // Patch jump to end
            let end = self.code.len();
            if let LpsOpCode::Jump(ref mut offset) = self.code[jump_to_end] {
                *offset = (end as i32) - (jump_to_end as i32) - 1;
            }
        } else {
            // No else, patch jump to end
            let end = self.code.len();
            if let LpsOpCode::JumpIfZero(ref mut offset) = self.code[jump_to_else] {
                *offset = (end as i32) - (jump_to_else as i32) - 1;
            }
        }
    }
}
