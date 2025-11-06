/// Program-level code generation
extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use super::local_allocator::LocalAllocator;
use crate::lpscript::compiler::ast::{AstPool, Program, StmtId};
use crate::lpscript::compiler::func::FunctionTable;
use crate::lpscript::shared::Type;
use crate::lpscript::vm::opcodes::LpsOpCode;
use crate::lpscript::vm::{FunctionDef as VmFunctionDef, LocalVarDef};

/// Generate a complete program with functions (new API with FunctionTable)
/// Returns a vector of FunctionDef with main at index 0
pub fn gen_program_with_functions(
    pool: &AstPool,
    program: &Program,
    func_table: &FunctionTable,
    _gen_stmt: impl Fn(&AstPool, StmtId, &mut Vec<LpsOpCode>, &mut LocalAllocator, &BTreeMap<String, u32>)
        + Copy,
) -> Vec<VmFunctionDef> {
    // Build function index map: function name -> final index in output
    // Main will always be at index 0, other functions follow in order (excluding main)
    let mut function_indices = BTreeMap::new();
    function_indices.insert(String::from("main"), 0);

    // Assign indices for non-main functions
    let mut next_index = 1;
    for func in &program.functions {
        if func.name != "main" {
            function_indices.insert(func.name.clone(), next_index);
            next_index += 1;
        }
    }

    let mut result_functions = Vec::new();

    // Generate user-defined functions first (we'll reorder later)
    for ast_func in &program.functions {
        let vm_func = crate::lpscript::compiler::func::func_gen::gen_user_function(
            pool,
            ast_func,
            func_table,
            &function_indices,
        );
        result_functions.push(vm_func);
    }

    // Generate main function from top-level statements
    let mut main_code = Vec::new();
    let mut main_locals = LocalAllocator::new();
    {
        let mut gen =
            super::CodeGenerator::new(&mut main_code, &mut main_locals, &function_indices);
        for &stmt_id in &program.stmts {
            gen.gen_stmt_id(pool, stmt_id);
        }

        // Add return if missing
        if !matches!(main_code.last(), Some(LpsOpCode::Return)) {
            main_code.push(LpsOpCode::Push(crate::math::Fixed::ZERO));
            main_code.push(LpsOpCode::Return);
        }
    }

    let main_local_defs: Vec<LocalVarDef> = (0..main_locals.next_index)
        .map(|i| {
            let ty = main_locals
                .local_types
                .get(&i)
                .cloned()
                .unwrap_or(Type::Fixed);
            LocalVarDef::new(alloc::format!("local_{}", i), ty)
        })
        .collect();

    let main_func = VmFunctionDef::new(String::from("main"), Type::Void)
        .with_locals(main_local_defs)
        .with_opcodes(main_code);

    // Ensure main is at index 0
    let mut final_functions = vec![main_func];

    // Add other functions (those that aren't "main")
    for func in result_functions {
        if func.name != "main" {
            final_functions.push(func);
        }
    }

    final_functions
}

/// Generate opcodes for a program (script mode) - Legacy API
/// Returns (opcodes, local_count, local_types) tuple
#[cfg(test)]
pub fn gen_program(
    pool: &AstPool,
    program: &Program,
    _gen_stmt: impl Fn(&AstPool, StmtId, &mut Vec<LpsOpCode>, &mut LocalAllocator, &BTreeMap<String, u32>)
        + Copy,
) -> (
    Vec<LpsOpCode>,
    u32,
    BTreeMap<u32, crate::lpscript::shared::Type>,
) {
    let mut code = Vec::new();
    let function_offsets = BTreeMap::new();

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
