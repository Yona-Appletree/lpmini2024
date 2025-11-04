/// While loop code generation
extern crate alloc;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::boxed::Box;

use crate::lpscript::ast::{Expr, Stmt};
use crate::lpscript::vm::opcodes::LpsOpCode;
use super::super::local_allocator::LocalAllocator;

pub fn gen_while(
    condition: &Expr,
    body: &Box<Stmt>,
    code: &mut Vec<LpsOpCode>,
    locals: &mut LocalAllocator,
    func_offsets: &BTreeMap<String, u32>,
    gen_expr: impl Fn(&Expr, &mut Vec<LpsOpCode>, &mut LocalAllocator, &BTreeMap<String, u32>),
    gen_stmt: impl Fn(&Stmt, &mut Vec<LpsOpCode>, &mut LocalAllocator, &BTreeMap<String, u32>),
) {
    // Generate: loop_start → condition → JumpIfZero(end) → body → Jump(loop_start)
    let loop_start = code.len();
    
    gen_expr(condition, code, locals, func_offsets);
    
    let jump_to_end_index = code.len();
    code.push(LpsOpCode::JumpIfZero(0)); // Placeholder
    
    gen_stmt(body, code, locals, func_offsets);
    
    // Jump back to loop start
    let jump_back_offset = (loop_start as i32) - (code.len() as i32) - 1;
    code.push(LpsOpCode::Jump(jump_back_offset));
    
    // Patch JumpIfZero to point to end
    let end = code.len();
    if let LpsOpCode::JumpIfZero(ref mut offset) = code[jump_to_end_index] {
        *offset = (end as i32) - (jump_to_end_index as i32) - 1;
    }
}

