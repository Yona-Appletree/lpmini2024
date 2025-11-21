/// Program-level code generation
extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use super::local_allocator::LocalAllocator;
use crate::lp_script::compiler::ast::{Program, Stmt, StmtKind};
use crate::lp_script::compiler::error::CodegenError;
use crate::lp_script::compiler::func::FunctionTable;
use crate::lp_script::shared::Type;
use crate::lp_script::vm::opcodes::LpsOpCode;
use crate::lp_script::vm::{FunctionDef as VmFunctionDef, LocalVarDef};

/// Infer return type from typed statements (after type checking)
fn infer_main_return_type(stmts: &[Stmt]) -> Type {
    // Look for the last return statement to determine type
    for stmt in stmts.iter().rev() {
        if let StmtKind::Return(expr) = &stmt.kind {
            // After type checking, expr.ty should be Some
            if let Some(ty) = &expr.ty {
                return ty.clone();
            }
        }
    }

    // No explicit return found - default to Void
    Type::Void
}

/// Generate a complete program with functions (new API with FunctionTable)
/// Returns a vector of FunctionDef with main at index 0
pub fn gen_program_with_functions(
    program: &Program,
    func_table: &FunctionTable,
    _gen_stmt: impl Fn(
            &Stmt,
            &mut Vec<LpsOpCode>,
            &mut LocalAllocator,
            &BTreeMap<String, u32>,
        ) -> Result<(), CodegenError>
        + Copy,
) -> Result<Vec<VmFunctionDef>, CodegenError> {
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
        let vm_func = crate::lp_script::compiler::func::func_gen::gen_user_function(
            ast_func,
            func_table,
            &function_indices,
        )?;
        result_functions.push(vm_func);
    }

    // Generate main function from top-level statements
    let mut main_code = Vec::new();
    let mut main_locals = LocalAllocator::new();
    for stmt in &program.stmts {
        _gen_stmt(stmt, &mut main_code, &mut main_locals, &function_indices)?;
    }

    // Add return if missing
    if !matches!(main_code.last(), Some(LpsOpCode::Return)) {
        main_code.push(LpsOpCode::Push(lp_math::dec32::Dec32::ZERO));
        main_code.push(LpsOpCode::Return);
    }

    let main_local_defs: Vec<LocalVarDef> = (0..main_locals.next_index)
        .map(|i| {
            let ty = main_locals
                .local_types
                .get(&i)
                .cloned()
                .unwrap_or(Type::Dec32);
            LocalVarDef::new(alloc::format!("local_{}", i), ty)
        })
        .collect();

    // Get return type - first try function table, then infer from typed statements
    let main_return_type = func_table
        .lookup("main")
        .map(|meta| meta.return_type.clone())
        .unwrap_or_else(|| infer_main_return_type(&program.stmts));

    let main_func = VmFunctionDef::new(String::from("main"), main_return_type)
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

    Ok(final_functions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lp_script::compile_script;

    #[test]
    fn test_infer_main_return_type_from_script() {
        // Test that we correctly infer Vec3 return type from a script
        let program = compile_script(
            "float r = xNorm; \
             float g = yNorm; \
             float b = 0.5; \
             return vec3(r, g, b);",
        )
        .unwrap();

        let main_func = program.main_function().expect("Should have main");
        println!("Main function return type: {:?}", main_func.return_type);

        assert_eq!(
            main_func.return_type,
            Type::Vec3,
            "Should infer Vec3 return type from script"
        );
    }
}
