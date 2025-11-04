/// Code generator: converts AST to VM opcodes
extern crate alloc;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use alloc::string::String;

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

pub struct CodeGenerator<'a> {
    code: &'a mut Vec<LpsOpCode>,
    locals: &'a mut LocalAllocator,
    func_offsets: &'a BTreeMap<String, u32>,
}

impl<'a> CodeGenerator<'a> {
    /// Create a new code generator instance
    fn new(
        code: &'a mut Vec<LpsOpCode>,
        locals: &'a mut LocalAllocator,
        func_offsets: &'a BTreeMap<String, u32>,
    ) -> Self {
        CodeGenerator {
            code,
            locals,
            func_offsets,
        }
    }
    
    /// Generate opcodes for an expression (expression mode)
    pub fn generate(expr: &Expr) -> Vec<LpsOpCode> {
        let mut code = Vec::new();
        let mut locals = LocalAllocator::new();
        let func_offsets = BTreeMap::new(); // Empty for expression mode
        
        let mut gen = CodeGenerator::new(&mut code, &mut locals, &func_offsets);
        gen.gen_expr(expr);
        gen.code.push(LpsOpCode::Return);
        
        code
    }
    
    /// Generate opcodes for a program (script mode)
    /// Returns (opcodes, local_count) tuple
    pub fn generate_program(program: &Program) -> (Vec<LpsOpCode>, u32) {
        program::gen_program(program, |stmt, code, locals, func_offsets| {
            let mut gen = CodeGenerator::new(code, locals, func_offsets);
            gen.gen_stmt(stmt);
        })
    }
}

