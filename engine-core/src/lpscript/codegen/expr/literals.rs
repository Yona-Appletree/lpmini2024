/// Literal expression code generation
extern crate alloc;
use alloc::vec::Vec;

use crate::lpscript::vm::opcodes::LpsOpCode;
use crate::math::ToFixed;

pub fn gen_number(n: f32, code: &mut Vec<LpsOpCode>) {
    code.push(LpsOpCode::Push(n.to_fixed()));
}

pub fn gen_int_number(n: i32, code: &mut Vec<LpsOpCode>) {
    // Convert int to fixed point for now (TODO: keep as int32)
    code.push(LpsOpCode::Push(n.to_fixed()));
}

