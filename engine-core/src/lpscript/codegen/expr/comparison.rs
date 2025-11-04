/// Comparison operation code generation
extern crate alloc;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::boxed::Box;

use crate::lpscript::ast::Expr;
use crate::lpscript::vm::opcodes::LpsOpCode;
use super::super::local_allocator::LocalAllocator;

pub fn gen_less(
    left: &Box<Expr>,
    right: &Box<Expr>,
    code: &mut Vec<LpsOpCode>,
    locals: &mut LocalAllocator,
    func_offsets: &BTreeMap<String, u32>,
    gen_expr: impl Fn(&Expr, &mut Vec<LpsOpCode>, &mut LocalAllocator, &BTreeMap<String, u32>) + Copy,
) {
    gen_expr(left, code, locals, func_offsets);
    gen_expr(right, code, locals, func_offsets);
    code.push(LpsOpCode::LessFixed);
}

pub fn gen_greater(
    left: &Box<Expr>,
    right: &Box<Expr>,
    code: &mut Vec<LpsOpCode>,
    locals: &mut LocalAllocator,
    func_offsets: &BTreeMap<String, u32>,
    gen_expr: impl Fn(&Expr, &mut Vec<LpsOpCode>, &mut LocalAllocator, &BTreeMap<String, u32>) + Copy,
) {
    gen_expr(left, code, locals, func_offsets);
    gen_expr(right, code, locals, func_offsets);
    code.push(LpsOpCode::GreaterFixed);
}

pub fn gen_less_eq(
    left: &Box<Expr>,
    right: &Box<Expr>,
    code: &mut Vec<LpsOpCode>,
    locals: &mut LocalAllocator,
    func_offsets: &BTreeMap<String, u32>,
    gen_expr: impl Fn(&Expr, &mut Vec<LpsOpCode>, &mut LocalAllocator, &BTreeMap<String, u32>) + Copy,
) {
    gen_expr(left, code, locals, func_offsets);
    gen_expr(right, code, locals, func_offsets);
    code.push(LpsOpCode::LessEqFixed);
}

pub fn gen_greater_eq(
    left: &Box<Expr>,
    right: &Box<Expr>,
    code: &mut Vec<LpsOpCode>,
    locals: &mut LocalAllocator,
    func_offsets: &BTreeMap<String, u32>,
    gen_expr: impl Fn(&Expr, &mut Vec<LpsOpCode>, &mut LocalAllocator, &BTreeMap<String, u32>) + Copy,
) {
    gen_expr(left, code, locals, func_offsets);
    gen_expr(right, code, locals, func_offsets);
    code.push(LpsOpCode::GreaterEqFixed);
}

pub fn gen_eq(
    left: &Box<Expr>,
    right: &Box<Expr>,
    code: &mut Vec<LpsOpCode>,
    locals: &mut LocalAllocator,
    func_offsets: &BTreeMap<String, u32>,
    gen_expr: impl Fn(&Expr, &mut Vec<LpsOpCode>, &mut LocalAllocator, &BTreeMap<String, u32>) + Copy,
) {
    gen_expr(left, code, locals, func_offsets);
    gen_expr(right, code, locals, func_offsets);
    code.push(LpsOpCode::EqFixed);
}

pub fn gen_not_eq(
    left: &Box<Expr>,
    right: &Box<Expr>,
    code: &mut Vec<LpsOpCode>,
    locals: &mut LocalAllocator,
    func_offsets: &BTreeMap<String, u32>,
    gen_expr: impl Fn(&Expr, &mut Vec<LpsOpCode>, &mut LocalAllocator, &BTreeMap<String, u32>) + Copy,
) {
    gen_expr(left, code, locals, func_offsets);
    gen_expr(right, code, locals, func_offsets);
    code.push(LpsOpCode::NotEqFixed);
}

