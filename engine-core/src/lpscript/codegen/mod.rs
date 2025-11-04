/// Code generator: converts AST to VM opcodes
extern crate alloc;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

use super::ast::{Expr, Program};
use super::vm::opcodes::LpsOpCode;

mod local_allocator;
mod native_functions;
mod expr;
mod stmt;
mod functions;
mod program;

pub use native_functions::NativeFunction;
use local_allocator::LocalAllocator;

pub struct CodeGenerator;

impl CodeGenerator {
    /// Generate opcodes for an expression (expression mode)
    pub fn generate(expr: &Expr) -> Vec<LpsOpCode> {
        let mut code = Vec::new();
        let mut locals = LocalAllocator::new();
        let func_offsets = BTreeMap::new(); // Empty for expression mode
        expr::gen_expr(expr, &mut code, &mut locals, &func_offsets);
        code.push(LpsOpCode::Return);
        code
    }
    
    /// Generate opcodes for a program (script mode)
    /// Returns (opcodes, local_count) tuple
    pub fn generate_program(program: &Program) -> (Vec<LpsOpCode>, u32) {
        program::gen_program(program, |stmt, code, locals, func_offsets| {
            stmt::gen_stmt(stmt, code, locals, func_offsets, |expr, c, l, f| {
                expr::gen_expr(expr, c, l, f);
            });
        })
    }
}

