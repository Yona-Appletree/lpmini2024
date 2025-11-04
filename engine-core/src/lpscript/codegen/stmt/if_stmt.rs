/// If/else statement code generation
extern crate alloc;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::boxed::Box;

use crate::lpscript::ast::{Expr, Stmt};
use crate::lpscript::vm::opcodes::LpsOpCode;
use super::super::local_allocator::LocalAllocator;

pub fn gen_if(
    condition: &Expr,
    then_stmt: &Box<Stmt>,
    else_stmt: &Option<Box<Stmt>>,
    code: &mut Vec<LpsOpCode>,
    locals: &mut LocalAllocator,
    func_offsets: &BTreeMap<String, u32>,
    gen_expr: impl Fn(&Expr, &mut Vec<LpsOpCode>, &mut LocalAllocator, &BTreeMap<String, u32>),
    gen_stmt: impl Fn(&Stmt, &mut Vec<LpsOpCode>, &mut LocalAllocator, &BTreeMap<String, u32>),
) {
    // Generate: condition → JumpIfZero(else_offset) → then_block → Jump(end_offset) → else_block
    gen_expr(condition, code, locals, func_offsets);
    
    // Placeholder for JumpIfZero - we'll patch the offset later
    let jump_to_else_index = code.len();
    code.push(LpsOpCode::JumpIfZero(0)); // Placeholder offset
    
    // Generate then block
    gen_stmt(then_stmt, code, locals, func_offsets);
    
    if let Some(else_s) = else_stmt {
        // Placeholder for Jump past else block
        let jump_to_end_index = code.len();
        code.push(LpsOpCode::Jump(0)); // Placeholder offset
        
        // Patch the JumpIfZero to point here (start of else block)
        let else_start = code.len();
        if let LpsOpCode::JumpIfZero(ref mut offset) = code[jump_to_else_index] {
            *offset = (else_start as i32) - (jump_to_else_index as i32) - 1;
        }
        
        // Generate else block
        gen_stmt(else_s, code, locals, func_offsets);
        
        // Patch the Jump to point here (end)
        let end = code.len();
        if let LpsOpCode::Jump(ref mut offset) = code[jump_to_end_index] {
            *offset = (end as i32) - (jump_to_end_index as i32) - 1;
        }
    } else {
        // No else block - patch JumpIfZero to point to end
        let end = code.len();
        if let LpsOpCode::JumpIfZero(ref mut offset) = code[jump_to_else_index] {
            *offset = (end as i32) - (jump_to_else_index as i32) - 1;
        }
    }
}

