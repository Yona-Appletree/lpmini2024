/// Ternary conditional code generation
extern crate alloc;

use crate::compiler::ast::Expr;
use crate::compiler::codegen::CodeGenerator;
use crate::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_ternary_id(
        &mut self,
        pool: &AstPool,
        condition: Expr,
        true_expr: Expr,
        false_expr: Expr,
    ) {
        // Generate condition
        self.gen_expr_id(pool, condition);

        // JumpIfZero to false branch
        let jump_to_false = self.code.len();
        self.code.push(LpsOpCode::JumpIfZero(0)); // Placeholder

        // True branch
        self.gen_expr_id(pool, true_expr);
        let jump_to_end = self.code.len();
        self.code.push(LpsOpCode::Jump(0)); // Placeholder

        // Patch jump to false
        let false_start = self.code.len();
        if let LpsOpCode::JumpIfZero(ref mut offset) = self.code[jump_to_false] {
            *offset = (false_start as i32) - (jump_to_false as i32) - 1;
        }

        // False branch
        self.gen_expr_id(pool, false_expr);

        // Patch jump to end
        let end = self.code.len();
        if let LpsOpCode::Jump(ref mut offset) = self.code[jump_to_end] {
            *offset = (end as i32) - (jump_to_end as i32) - 1;
        }
    }
}
