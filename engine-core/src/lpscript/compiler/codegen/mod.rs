/// Code generator: converts AST to VM opcodes
extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use crate::lpscript::compiler::ast::{Expr, Program};
use crate::lpscript::LpsOpCode;

mod expr;
pub(crate) mod local_allocator;
mod native_functions;
mod program;
mod stmt;

pub(crate) use local_allocator::LocalAllocator;
pub use native_functions::NativeFunction;

pub struct CodeGenerator<'a> {
    pub(in crate::lpscript) code: &'a mut Vec<LpsOpCode>,
    pub(in crate::lpscript) locals: &'a mut LocalAllocator,
    pub(in crate::lpscript) func_offsets: &'a BTreeMap<String, u32>,
}

impl<'a> CodeGenerator<'a> {
    /// Create a new code generator instance
    pub(crate) fn new(
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
        Self::generate_with_locals(expr, Vec::new())
    }

    /// Generate opcodes for an expression with pre-declared local variables
    ///
    /// This is useful for testing assignment expressions which need mutable locals.
    /// The locals should be ordered by index (e.g., [("x", 0), ("y", 1), ...])
    pub fn generate_with_locals(expr: &Expr, predeclared: Vec<(String, u32)>) -> Vec<LpsOpCode> {
        let mut code = Vec::new();
        let mut locals = LocalAllocator::new();
        let func_offsets = BTreeMap::new(); // Empty for expression mode

        // Pre-allocate declared locals in order
        for (name, expected_index) in predeclared {
            let actual_index = locals.allocate(name);
            // Verify indices match (should always be true if called correctly)
            assert_eq!(
                actual_index, expected_index,
                "Local index mismatch during pre-allocation"
            );
        }

        let mut gen = CodeGenerator::new(&mut code, &mut locals, &func_offsets);
        gen.gen_expr(expr);
        gen.code.push(LpsOpCode::Return);

        code
    }

    /// Generate opcodes for a program (script mode)
    /// Returns (opcodes, local_count, local_types) tuple
    pub fn generate_program(
        program: &Program,
    ) -> (
        Vec<LpsOpCode>,
        u32,
        alloc::collections::BTreeMap<u32, crate::lpscript::shared::Type>,
    ) {
        program::gen_program(program, |stmt, code, locals, func_offsets| {
            let mut gen = CodeGenerator::new(code, locals, func_offsets);
            gen.gen_stmt(stmt);
        })
    }
}
