/// Function definition code generation
extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use crate::lpscript::compiler::ast::{FunctionDef, Stmt};
use crate::lpscript::compiler::codegen::local_allocator::LocalAllocator;
use crate::lpscript::compiler::codegen::CodeGenerator;
use crate::lpscript::shared::Type;
use crate::lpscript::vm::opcodes::LpsOpCode;

/// Generate code for a single function definition
pub fn gen_function(
    func: &FunctionDef,
    code: &mut Vec<LpsOpCode>,
    function_offsets: &BTreeMap<String, u32>,
    _gen_stmt: impl Fn(&Stmt, &mut Vec<LpsOpCode>, &mut LocalAllocator, &BTreeMap<String, u32>),
) {
    let mut locals = LocalAllocator::new();

    // Allocate space for parameters (they're passed on stack)
    // First, allocate locals for all parameters to reserve their indices
    for param in func.params.iter() {
        locals.allocate_typed(param.name.clone(), param.ty.clone());
    }

    // Then, generate store instructions in REVERSE order
    // because parameters are on stack with last param on top
    for (i, param) in func.params.iter().enumerate().rev() {
        // Parameters are already on stack, need to store them
        match param.ty {
            Type::Bool | Type::Fixed => code.push(LpsOpCode::StoreLocalFixed(i as u32)),
            Type::Int32 => code.push(LpsOpCode::StoreLocalInt32(i as u32)),
            Type::Vec2 => code.push(LpsOpCode::StoreLocalVec2(i as u32)),
            Type::Vec3 => code.push(LpsOpCode::StoreLocalVec3(i as u32)),
            Type::Vec4 => code.push(LpsOpCode::StoreLocalVec4(i as u32)),
            Type::Void => {}
        }
    }

    // Generate function body using CodeGenerator
    let mut gen = CodeGenerator::new(code, &mut locals, function_offsets);
    for stmt in &func.body {
        gen.gen_stmt(stmt);
    }

    // If no explicit return, add a default one
    if !matches!(gen.code.last(), Some(LpsOpCode::Return)) {
        if func.return_type == Type::Void {
            gen.code.push(LpsOpCode::Return);
        } else {
            gen.code.push(LpsOpCode::Push(crate::math::Fixed::ZERO));
            gen.code.push(LpsOpCode::Return);
        }
    }
}
