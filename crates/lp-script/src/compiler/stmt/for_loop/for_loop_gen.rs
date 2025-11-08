/// For loop code generation
extern crate alloc;

use crate::compiler::ast::{AstPool, ExprId, StmtId};
use crate::compiler::codegen::CodeGenerator;
use crate::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_for_stmt_id(
        &mut self,
        pool: &AstPool,
        init: &Option<StmtId>,
        condition: &Option<ExprId>,
        increment: &Option<ExprId>,
        body: StmtId,
    ) {
        self.locals.push_scope();

        // Init
        if let Some(init_id) = init {
            self.gen_stmt_id(pool, *init_id);
        }

        let loop_start = self.code.len();

        // Condition (defaults to true if omitted)
        let jump_to_end = if let Some(cond_id) = condition {
            self.gen_expr_id(pool, *cond_id);
            let jump_idx = self.code.len();
            self.code.push(LpsOpCode::JumpIfZero(0)); // Placeholder
            Some(jump_idx)
        } else {
            None
        };

        // Body
        self.gen_stmt_id(pool, body);

        // Increment
        if let Some(inc_id) = increment {
            self.gen_expr_id(pool, *inc_id);
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
