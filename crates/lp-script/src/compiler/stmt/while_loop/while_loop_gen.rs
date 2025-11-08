/// While loop code generation
extern crate alloc;

use crate::compiler::ast::{Expr, Stmt};
use crate::compiler::codegen::CodeGenerator;
use crate::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_while_stmt(&mut self, condition: &Expr, body: &Stmt) {
        let loop_start = self.code.len();

        // Generate condition
        self.gen_expr(condition);

        // JumpIfZero to end
        let jump_to_end = self.code.len();
        self.code.push(LpsOpCode::JumpIfZero(0)); // Placeholder

        // Body
        self.gen_stmt(body);

        // Jump back to loop start
        let jump_back_idx = self.code.len();
        self.code.push(LpsOpCode::Jump(
            (loop_start as i32) - (jump_back_idx as i32) - 1,
        ));

        // Patch jump to end
        let end = self.code.len();
        if let LpsOpCode::JumpIfZero(ref mut offset) = self.code[jump_to_end] {
            *offset = (end as i32) - (jump_to_end as i32) - 1;
        }
    }
}
