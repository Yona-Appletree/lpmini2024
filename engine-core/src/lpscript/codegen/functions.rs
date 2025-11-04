/// Function definition code generation
extern crate alloc;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use alloc::string::String;

use crate::lpscript::ast::{FunctionDef, Stmt};
use crate::lpscript::error::Type;
use crate::lpscript::vm::opcodes::LpsOpCode;
use super::local_allocator::LocalAllocator;
use super::CodeGenerator;

/// Generate code for a single function definition
pub fn gen_function(
    func: &FunctionDef,
    code: &mut Vec<LpsOpCode>,
    function_offsets: &BTreeMap<String, u32>,
    gen_stmt: impl Fn(&Stmt, &mut Vec<LpsOpCode>, &mut LocalAllocator, &BTreeMap<String, u32>),
) {
    let mut locals = LocalAllocator::new();
    
    // Allocate space for parameters (they're passed on stack)
    for (i, param) in func.params.iter().enumerate() {
        locals.allocate(param.name.clone());
        // Parameters are already on stack, need to store them
        match param.ty {
            Type::Fixed | Type::Int32 => code.push(LpsOpCode::StoreLocalFixed(i as u32)),
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

