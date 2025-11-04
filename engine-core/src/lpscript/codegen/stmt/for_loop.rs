/// For loop code generation
extern crate alloc;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::boxed::Box;

use crate::lpscript::ast::{Expr, Stmt};
use crate::lpscript::vm::opcodes::LpsOpCode;
use super::super::local_allocator::LocalAllocator;

pub fn gen_for(
    init: &Option<Box<Stmt>>,
    condition: &Option<Expr>,
    increment: &Option<Expr>,
    body: &Box<Stmt>,
    code: &mut Vec<LpsOpCode>,
    locals: &mut LocalAllocator,
    func_offsets: &BTreeMap<String, u32>,
    gen_expr: impl Fn(&Expr, &mut Vec<LpsOpCode>, &mut LocalAllocator, &BTreeMap<String, u32>) + Copy,
    gen_stmt: impl Fn(&Stmt, &mut Vec<LpsOpCode>, &mut LocalAllocator, &BTreeMap<String, u32>) + Copy,
) {
    // Generate init
    if let Some(init_stmt) = init {
        gen_stmt(init_stmt, code, locals, func_offsets);
    }
    
    let loop_start = code.len();
    
    // Generate condition (if present)
    if let Some(cond) = condition {
        gen_expr(cond, code, locals, func_offsets);
        
        let jump_to_end_index = code.len();
        code.push(LpsOpCode::JumpIfZero(0)); // Placeholder
        
        gen_stmt(body, code, locals, func_offsets);
        
        // Generate increment (if present)
        if let Some(inc) = increment {
            gen_expr(inc, code, locals, func_offsets);
            code.push(LpsOpCode::Drop); // Discard increment result
        }
        
        // Jump back to condition
        let jump_back_offset = (loop_start as i32) - (code.len() as i32) - 1;
        code.push(LpsOpCode::Jump(jump_back_offset));
        
        // Patch JumpIfZero to point to end
        let end = code.len();
        if let LpsOpCode::JumpIfZero(ref mut offset) = code[jump_to_end_index] {
            *offset = (end as i32) - (jump_to_end_index as i32) - 1;
        }
    } else {
        // Infinite loop (no condition)
        gen_stmt(body, code, locals, func_offsets);
        
        if let Some(inc) = increment {
            gen_expr(inc, code, locals, func_offsets);
            code.push(LpsOpCode::Drop);
        }
        
        let jump_back_offset = (loop_start as i32) - (code.len() as i32) - 1;
        code.push(LpsOpCode::Jump(jump_back_offset));
    }
}

