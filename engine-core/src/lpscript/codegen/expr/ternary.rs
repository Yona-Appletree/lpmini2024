/// Ternary conditional code generation
extern crate alloc;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::boxed::Box;

use crate::lpscript::ast::Expr;
use crate::lpscript::vm::opcodes::LpsOpCode;
use super::super::local_allocator::LocalAllocator;

pub fn gen_ternary(
    condition: &Box<Expr>,
    true_expr: &Box<Expr>,
    false_expr: &Box<Expr>,
    code: &mut Vec<LpsOpCode>,
    locals: &mut LocalAllocator,
    func_offsets: &BTreeMap<String, u32>,
    gen_expr: impl Fn(&Expr, &mut Vec<LpsOpCode>, &mut LocalAllocator, &BTreeMap<String, u32>) + Copy,
) {
    gen_expr(condition, code, locals, func_offsets);
    gen_expr(true_expr, code, locals, func_offsets);
    gen_expr(false_expr, code, locals, func_offsets);
    code.push(LpsOpCode::Select);
}

