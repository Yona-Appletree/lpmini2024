/// Assignment expression code generation
extern crate alloc;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::boxed::Box;

use crate::lpscript::ast::Expr;
use crate::lpscript::error::Type;
use crate::lpscript::vm::opcodes::LpsOpCode;
use super::super::local_allocator::LocalAllocator;

pub fn gen_assign_expr(
    target: &str,
    value: &Box<Expr>,
    code: &mut Vec<LpsOpCode>,
    locals: &mut LocalAllocator,
    func_offsets: &BTreeMap<String, u32>,
    gen_expr: impl Fn(&Expr, &mut Vec<LpsOpCode>, &mut LocalAllocator, &BTreeMap<String, u32>) + Copy,
) {
    // Generate code for the value
    gen_expr(value, code, locals, func_offsets);
    
    // Duplicate the value (assignment returns the value)
    code.push(LpsOpCode::Dup);
    
    // Store in the variable
    if let Some(index) = locals.get(target) {
        let ty = value.ty.as_ref().unwrap();
        match ty {
            Type::Fixed | Type::Int32 => code.push(LpsOpCode::StoreLocalFixed(index)),
            Type::Vec2 => code.push(LpsOpCode::StoreLocalVec2(index)),
            Type::Vec3 => code.push(LpsOpCode::StoreLocalVec3(index)),
            Type::Vec4 => code.push(LpsOpCode::StoreLocalVec4(index)),
            _ => {}
        }
    }
    // Value is left on stack (assignment expression returns the value)
}

