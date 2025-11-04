/// Literal expression code generation
extern crate alloc;
use alloc::vec::Vec;

use crate::lpscript::vm::opcodes::LpsOpCode;
use crate::math::ToFixed;
use super::super::CodeGenerator;

impl<'a> CodeGenerator<'a> {
    pub(in crate::lpscript::codegen::expr) fn gen_number(&mut self, n: f32) {
        self.code.push(LpsOpCode::Push(n.to_fixed()));
    }

    pub(in crate::lpscript::codegen::expr) fn gen_int_number(&mut self, n: i32) {
        // Convert int to fixed point for now (TODO: keep as int32)
        self.code.push(LpsOpCode::Push(n.to_fixed()));
    }
}

