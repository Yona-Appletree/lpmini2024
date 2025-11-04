/// Binary operation code generation
extern crate alloc;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::boxed::Box;

use crate::lpscript::ast::Expr;
use crate::lpscript::error::Type;
use crate::lpscript::vm::opcodes::LpsOpCode;
use super::super::local_allocator::LocalAllocator;

/// Binary operation types
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

pub fn gen_add(
    left: &Box<Expr>,
    right: &Box<Expr>,
    result_ty: &Type,
    code: &mut Vec<LpsOpCode>,
    locals: &mut LocalAllocator,
    func_offsets: &BTreeMap<String, u32>,
    gen_expr: impl Fn(&Expr, &mut Vec<LpsOpCode>, &mut LocalAllocator, &BTreeMap<String, u32>) + Copy,
) {
    gen_expr(left, code, locals, func_offsets);
    gen_expr(right, code, locals, func_offsets);
    gen_binary_op(
        BinaryOp::Add,
        left.ty.as_ref().unwrap(),
        right.ty.as_ref().unwrap(),
        result_ty,
        code,
    );
}

pub fn gen_sub(
    left: &Box<Expr>,
    right: &Box<Expr>,
    result_ty: &Type,
    code: &mut Vec<LpsOpCode>,
    locals: &mut LocalAllocator,
    func_offsets: &BTreeMap<String, u32>,
    gen_expr: impl Fn(&Expr, &mut Vec<LpsOpCode>, &mut LocalAllocator, &BTreeMap<String, u32>) + Copy,
) {
    gen_expr(left, code, locals, func_offsets);
    gen_expr(right, code, locals, func_offsets);
    gen_binary_op(
        BinaryOp::Sub,
        left.ty.as_ref().unwrap(),
        right.ty.as_ref().unwrap(),
        result_ty,
        code,
    );
}

pub fn gen_mul(
    left: &Box<Expr>,
    right: &Box<Expr>,
    result_ty: &Type,
    code: &mut Vec<LpsOpCode>,
    locals: &mut LocalAllocator,
    func_offsets: &BTreeMap<String, u32>,
    gen_expr: impl Fn(&Expr, &mut Vec<LpsOpCode>, &mut LocalAllocator, &BTreeMap<String, u32>) + Copy,
) {
    gen_expr(left, code, locals, func_offsets);
    gen_expr(right, code, locals, func_offsets);
    gen_binary_op(
        BinaryOp::Mul,
        left.ty.as_ref().unwrap(),
        right.ty.as_ref().unwrap(),
        result_ty,
        code,
    );
}

pub fn gen_div(
    left: &Box<Expr>,
    right: &Box<Expr>,
    result_ty: &Type,
    code: &mut Vec<LpsOpCode>,
    locals: &mut LocalAllocator,
    func_offsets: &BTreeMap<String, u32>,
    gen_expr: impl Fn(&Expr, &mut Vec<LpsOpCode>, &mut LocalAllocator, &BTreeMap<String, u32>) + Copy,
) {
    gen_expr(left, code, locals, func_offsets);
    gen_expr(right, code, locals, func_offsets);
    gen_binary_op(
        BinaryOp::Div,
        left.ty.as_ref().unwrap(),
        right.ty.as_ref().unwrap(),
        result_ty,
        code,
    );
}

pub fn gen_mod(
    left: &Box<Expr>,
    right: &Box<Expr>,
    result_ty: &Type,
    code: &mut Vec<LpsOpCode>,
    locals: &mut LocalAllocator,
    func_offsets: &BTreeMap<String, u32>,
    gen_expr: impl Fn(&Expr, &mut Vec<LpsOpCode>, &mut LocalAllocator, &BTreeMap<String, u32>) + Copy,
) {
    gen_expr(left, code, locals, func_offsets);
    gen_expr(right, code, locals, func_offsets);
    gen_binary_op(
        BinaryOp::Mod,
        left.ty.as_ref().unwrap(),
        right.ty.as_ref().unwrap(),
        result_ty,
        code,
    );
}

pub fn gen_pow(
    left: &Box<Expr>,
    right: &Box<Expr>,
    code: &mut Vec<LpsOpCode>,
    locals: &mut LocalAllocator,
    func_offsets: &BTreeMap<String, u32>,
    gen_expr: impl Fn(&Expr, &mut Vec<LpsOpCode>, &mut LocalAllocator, &BTreeMap<String, u32>) + Copy,
) {
    gen_expr(left, code, locals, func_offsets);
    gen_expr(right, code, locals, func_offsets);
    // Pow is always scalar for now
    // TODO: Add proper pow implementation
    code.push(LpsOpCode::Push(crate::math::Fixed::ONE)); // Placeholder
}

/// Generate typed binary operation based on operand and result types
fn gen_binary_op(op: BinaryOp, _left_ty: &Type, right_ty: &Type, result_ty: &Type, code: &mut Vec<LpsOpCode>) {
    match (op, result_ty) {
        // Fixed operations
        (BinaryOp::Add, Type::Fixed | Type::Int32) => code.push(LpsOpCode::AddFixed),
        (BinaryOp::Sub, Type::Fixed | Type::Int32) => code.push(LpsOpCode::SubFixed),
        (BinaryOp::Mul, Type::Fixed | Type::Int32) => code.push(LpsOpCode::MulFixed),
        (BinaryOp::Div, Type::Fixed | Type::Int32) => code.push(LpsOpCode::DivFixed),
        (BinaryOp::Mod, Type::Fixed | Type::Int32) => {
            // mod(x, y) = x - floor(x/y) * y
            // Stack has: [x, y]
            // We need: x - floor(x/y) * y
            // TODO: Implement properly - for now use placeholder
            code.push(LpsOpCode::DivFixed);
        }
        
        // Vec2 operations
        (BinaryOp::Add, Type::Vec2) => code.push(LpsOpCode::AddVec2),
        (BinaryOp::Sub, Type::Vec2) => code.push(LpsOpCode::SubVec2),
        (BinaryOp::Mul, Type::Vec2) => {
            // Check if it's vec * scalar or vec * vec
            if matches!(right_ty, Type::Fixed | Type::Int32) {
                code.push(LpsOpCode::MulVec2Scalar);
            } else {
                code.push(LpsOpCode::MulVec2);
            }
        }
        (BinaryOp::Div, Type::Vec2) => {
            if matches!(right_ty, Type::Fixed | Type::Int32) {
                code.push(LpsOpCode::DivVec2Scalar);
            } else {
                code.push(LpsOpCode::DivVec2);
            }
        }
        (BinaryOp::Mod, Type::Vec2) => code.push(LpsOpCode::MulVec2), // Placeholder
        
        // Vec3 operations
        (BinaryOp::Add, Type::Vec3) => code.push(LpsOpCode::AddVec3),
        (BinaryOp::Sub, Type::Vec3) => code.push(LpsOpCode::SubVec3),
        (BinaryOp::Mul, Type::Vec3) => {
            if matches!(right_ty, Type::Fixed | Type::Int32) {
                code.push(LpsOpCode::MulVec3Scalar);
            } else {
                code.push(LpsOpCode::MulVec3);
            }
        }
        (BinaryOp::Div, Type::Vec3) => {
            if matches!(right_ty, Type::Fixed | Type::Int32) {
                code.push(LpsOpCode::DivVec3Scalar);
            } else {
                code.push(LpsOpCode::DivVec3);
            }
        }
        (BinaryOp::Mod, Type::Vec3) => code.push(LpsOpCode::MulVec3), // Placeholder
        
        // Vec4 operations
        (BinaryOp::Add, Type::Vec4) => code.push(LpsOpCode::AddVec4),
        (BinaryOp::Sub, Type::Vec4) => code.push(LpsOpCode::SubVec4),
        (BinaryOp::Mul, Type::Vec4) => {
            if matches!(right_ty, Type::Fixed | Type::Int32) {
                code.push(LpsOpCode::MulVec4Scalar);
            } else {
                code.push(LpsOpCode::MulVec4);
            }
        }
        (BinaryOp::Div, Type::Vec4) => {
            if matches!(right_ty, Type::Fixed | Type::Int32) {
                code.push(LpsOpCode::DivVec4Scalar);
            } else {
                code.push(LpsOpCode::DivVec4);
            }
        }
        (BinaryOp::Mod, Type::Vec4) => code.push(LpsOpCode::MulVec4), // Placeholder
        
        _ => {} // Void or unsupported
    }
}

