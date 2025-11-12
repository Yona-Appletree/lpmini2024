/// Function definition code generation
extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use crate::compiler::ast::FunctionDef as AstFunctionDef;
use crate::compiler::codegen::{CodeGenerator, LocalAllocator};
use crate::compiler::error::CodegenError;
use crate::compiler::func::{FunctionMetadata, FunctionTable};
use crate::shared::Type;
use crate::vm::opcodes::LpsOpCode;
use crate::vm::{FunctionDef as VmFunctionDef, LocalVarDef, ParamDef};

/// Generate code for a single user-defined function
///
/// This is the core function generation logic using the pool-based API.
/// It handles parameter allocation, function body code generation, and types conversion.
pub fn gen_user_function(
    ast_func: &AstFunctionDef,
    func_table: &FunctionTable,
    function_indices: &BTreeMap<String, u32>,
) -> Result<VmFunctionDef, CodegenError> {
    let mut func_code = Vec::new();

    // Get pre-analyzed types for this function
    let metadata: &FunctionMetadata = func_table
        .lookup(&ast_func.name)
        .expect("Function should have types from analysis pass");

    // Create a fresh LocalAllocator and allocate parameters
    // This must match the order used in the analyzer
    let mut locals = LocalAllocator::new();
    for param in &ast_func.params {
        locals.allocate_typed(param.name.clone(), param.ty.clone());
    }

    // Generate parameter store code (reverse order - last param on top of stack)
    for (i, param) in ast_func.params.iter().enumerate().rev() {
        match param.ty {
            Type::Bool | Type::Fixed => func_code.push(LpsOpCode::StoreLocalFixed(i as u32)),
            Type::Int32 => func_code.push(LpsOpCode::StoreLocalInt32(i as u32)),
            Type::Vec2 => func_code.push(LpsOpCode::StoreLocalVec2(i as u32)),
            Type::Vec3 => func_code.push(LpsOpCode::StoreLocalVec3(i as u32)),
            Type::Vec4 => func_code.push(LpsOpCode::StoreLocalVec4(i as u32)),
            Type::Mat3 => func_code.push(LpsOpCode::StoreLocalMat3(i as u32)),
            Type::Void => {}
        }
    }

    // Generate function body
    let mut gen = CodeGenerator::new(&mut func_code, &mut locals, function_indices);
    for stmt in &ast_func.body {
        gen.gen_stmt(stmt)?;
    }

    // Add return if missing
    if !matches!(func_code.last(), Some(LpsOpCode::Return)) {
        if ast_func.return_type == Type::Void {
            func_code.push(LpsOpCode::Return);
        } else {
            func_code.push(LpsOpCode::Push(crate::fixed::Fixed::ZERO));
            func_code.push(LpsOpCode::Return);
        }
    }

    // Convert to VmFunctionDef using types
    let params_defs: Vec<ParamDef> = ast_func
        .params
        .iter()
        .map(|p| ParamDef::new(p.name.clone(), p.ty.clone()))
        .collect();

    let local_defs: Vec<LocalVarDef> = metadata
        .locals
        .iter()
        .map(|local_info| LocalVarDef::new(local_info.name.clone(), local_info.ty.clone()))
        .collect();

    Ok(
        VmFunctionDef::new(ast_func.name.clone(), ast_func.return_type.clone())
            .with_params(params_defs)
            .with_locals(local_defs)
            .with_opcodes(func_code),
    )
}
