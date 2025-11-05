/// Program-level code generation
extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use super::local_allocator::LocalAllocator;
use crate::lpscript::compiler::ast::{AstPool, Program, StmtId};
use crate::lpscript::vm::opcodes::LpsOpCode;

/// Generate opcodes for a program (script mode)
/// Returns (opcodes, local_count, local_types) tuple
pub fn gen_program(
    pool: &AstPool,
    program: &Program,
    gen_stmt: impl Fn(&AstPool, StmtId, &mut Vec<LpsOpCode>, &mut LocalAllocator, &BTreeMap<String, u32>) + Copy,
) -> (
    Vec<LpsOpCode>,
    u32,
    BTreeMap<u32, crate::lpscript::shared::Type>,
) {
    let mut code = Vec::new();
    let function_offsets = BTreeMap::new();

    // Note: Function generation is temporarily simplified
    // Full implementation would generate function code here

    // Generate main code using CodeGenerator
    let mut locals = LocalAllocator::new();
    let (local_count, local_types) = {
        let mut gen = super::CodeGenerator::new(&mut code, &mut locals, &function_offsets);
        for &stmt_id in &program.stmts {
            gen.gen_stmt_id(pool, stmt_id);
        }

        // If no explicit return, add one
        if !matches!(gen.code.last(), Some(LpsOpCode::Return)) {
            gen.code.push(LpsOpCode::Push(crate::math::Fixed::ZERO));
            gen.code.push(LpsOpCode::Return);
        }

        (gen.locals.next_index, gen.locals.local_types.clone())
    };

    // Return opcodes, local count, and types
    (code, local_count, local_types)
}
