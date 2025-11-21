/// Literal expression code generation
extern crate alloc;

use crate::compiler::codegen::CodeGenerator;
use crate::dec32::ToDec32;
use crate::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_number(&mut self, n: f32) {
        self.code.push(LpsOpCode::Push(n.to_dec32()));
    }

    pub(crate) fn gen_int_number(&mut self, n: i32) {
        // Use PushInt32 opcode for integer literals
        // The VM will convert to Dec32 when needed, but this preserves integer semantics
        self.code.push(LpsOpCode::PushInt32(n));
    }
}
