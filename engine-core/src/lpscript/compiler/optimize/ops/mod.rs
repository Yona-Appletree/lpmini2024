/// Opcode-level optimizations
extern crate alloc;
use alloc::vec::Vec;

use crate::lpscript::vm::opcodes::LpsOpCode;

mod peephole;

/// Optimize opcodes using peephole patterns
pub fn optimize(opcodes: Vec<LpsOpCode>) -> Vec<LpsOpCode> {
    peephole::optimize(opcodes)
}
