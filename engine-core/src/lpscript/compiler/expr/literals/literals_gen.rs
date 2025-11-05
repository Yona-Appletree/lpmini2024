/// Literal expression code generation
extern crate alloc;

use crate::lpscript::vm::opcodes::LpsOpCode;
use crate::math::ToFixed;
use crate::lpscript::compiler::generator::CodeGenerator;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_number(&mut self, n: f32) {
        self.code.push(LpsOpCode::Push(n.to_fixed()));
    }

    pub(crate) fn gen_int_number(&mut self, n: i32) {
        // Use PushInt32 opcode for integer literals
        // The VM will convert to Fixed when needed, but this preserves integer semantics
        self.code.push(LpsOpCode::PushInt32(n));
    }
}

