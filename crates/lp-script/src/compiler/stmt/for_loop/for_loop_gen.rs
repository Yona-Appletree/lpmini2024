/// For loop code generation
extern crate alloc;

use crate::compiler::ast::{Expr, Stmt};
use crate::compiler::codegen::CodeGenerator;
use crate::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_for_stmt(
        &mut self,
        init: Option<&Stmt>,
        condition: Option<&Expr>,
        increment: Option<&Expr>,
        body: &Stmt,
    ) {
        self.locals.push_scope();

        // Init
        if let Some(init_stmt) = init {
            self.gen_stmt(init_stmt);
        }

        let loop_start = self.code.len();

        // Condition (defaults to true if omitted)
        let jump_to_end = if let Some(cond) = condition {
            self.gen_expr(cond);
            let jump_idx = self.code.len();
            self.code.push(LpsOpCode::JumpIfZero(0)); // Placeholder
            Some(jump_idx)
        } else {
            None
        };

        // Body
        self.gen_stmt(body);

        // Increment
        if let Some(inc) = increment {
            self.gen_expr(inc);
            self.code.push(LpsOpCode::Drop1); // Discard result
        }

        // Jump back to loop start
        let jump_back_idx = self.code.len();
        self.code.push(LpsOpCode::Jump(
            (loop_start as i32) - (jump_back_idx as i32) - 1,
        ));

        // Patch jump to end
        if let Some(jump_idx) = jump_to_end {
            let end = self.code.len();
            if let LpsOpCode::JumpIfZero(ref mut offset) = self.code[jump_idx] {
                *offset = (end as i32) - (jump_idx as i32) - 1;
            }
        }

        self.locals.pop_scope();
    }
}
