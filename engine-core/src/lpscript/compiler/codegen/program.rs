/// Program-level code generation
extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use super::local_allocator::LocalAllocator;
use crate::lpscript::compiler::ast::{Program, Stmt};
use crate::lpscript::compiler::func::gen_function;
use crate::lpscript::vm::opcodes::LpsOpCode;

/// Generate opcodes for a program (script mode)
/// Returns (opcodes, local_count) tuple
pub fn gen_program(
    program: &Program,
    gen_stmt: impl Fn(&Stmt, &mut Vec<LpsOpCode>, &mut LocalAllocator, &BTreeMap<String, u32>) + Copy,
) -> (Vec<LpsOpCode>, u32) {
    let mut code = Vec::new();
    let mut function_offsets = BTreeMap::new();

    // If there are functions, emit a jump to skip them (go to main code)
    let main_jump_index = if !program.functions.is_empty() {
        code.push(LpsOpCode::Jump(0)); // Placeholder, will patch later
        Some(0)
    } else {
        None
    };

    // Generate code for each function
    for func in &program.functions {
        let func_start = code.len();
        function_offsets.insert(func.name.clone(), func_start as u32);

        gen_function(func, &mut code, &function_offsets, gen_stmt);
    }

    // Patch the main jump to point here
    if let Some(jump_idx) = main_jump_index {
        let main_start = code.len();
        if let LpsOpCode::Jump(ref mut offset) = code[jump_idx] {
            *offset = (main_start as i32) - 1;
        }
    }

    // Generate main code using CodeGenerator
    let mut locals = LocalAllocator::new();
    let local_count = {
        let mut gen = super::CodeGenerator::new(&mut code, &mut locals, &function_offsets);
        for stmt in &program.stmts {
            gen.gen_stmt(stmt);
        }

        // If no explicit return, add one
        if !matches!(gen.code.last(), Some(LpsOpCode::Return)) {
            gen.code.push(LpsOpCode::Push(crate::math::Fixed::ZERO));
            gen.code.push(LpsOpCode::Return);
        }

        gen.locals.next_index
    };

    // Return opcodes and the total number of locals allocated
    (code, local_count)
}
