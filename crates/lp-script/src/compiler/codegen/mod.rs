/// Code generator: converts AST to VM opcodes
extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use crate::compiler::ast::{Expr, Program};
use crate::LpsOpCode;

mod expr;
pub(crate) mod local_allocator;
mod native_functions;
mod program;
mod stmt;

pub(crate) use local_allocator::LocalAllocator;
pub use native_functions::NativeFunction;

pub struct CodeGenerator<'a> {
    pub(crate) code: &'a mut Vec<LpsOpCode>,
    pub(crate) locals: &'a mut LocalAllocator,
    pub(crate) func_offsets: &'a BTreeMap<String, u32>,
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
    pub fn generate(expr: &Expr) -> Result<Vec<LpsOpCode>, crate::compiler::error::CodegenError> {
        Self::generate_with_locals(expr, Vec::new())
    }

    /// Generate opcodes for an expression with pre-declared local variables
    ///
    /// This is useful for testing assignment expressions which need mutable locals.
    /// The locals should be ordered by index (e.g., [("x", 0), ("y", 1), ...])
    pub fn generate_with_locals(
        expr: &Expr,
        predeclared: Vec<(String, u32, crate::shared::Type)>,
    ) -> Result<Vec<LpsOpCode>, crate::compiler::error::CodegenError> {
        let mut code = Vec::new();
        let mut locals = LocalAllocator::new();
        let func_offsets = BTreeMap::new(); // Empty for expression mode

        // Pre-allocate declared locals in order with their types
        for (name, expected_index, ty) in predeclared {
            let actual_index = locals.allocate_typed(name, ty);
            // Verify indices match (should always be true if called correctly)
            assert_eq!(
                actual_index, expected_index,
                "Local index mismatch during pre-allocation"
            );
        }

        let mut gen = CodeGenerator::new(&mut code, &mut locals, &func_offsets);
        gen.gen_expr(expr)?;
        gen.code.push(LpsOpCode::Return);

        Ok(code)
    }

    /// Generate functions for a program (new API with FunctionTable)
    pub fn generate_program_with_functions(
        program: &Program,
        func_table: &crate::compiler::func::FunctionTable,
    ) -> Result<Vec<crate::vm::FunctionDef>, crate::compiler::error::CodegenError> {
        program::gen_program_with_functions(
            program,
            func_table,
            |stmt, code, locals, func_offsets| {
                let mut gen = CodeGenerator::new(code, locals, func_offsets);
                gen.gen_stmt(stmt)
            },
        )
    }

    // Codegen methods are implemented in their respective organized modules
    // Expression dispatcher: codegen/expr.rs
    // Statement dispatcher: codegen/stmt.rs
    // Individual generators: expr/*/..._gen.rs, stmt/*/..._gen.rs
}
