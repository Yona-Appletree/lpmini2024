/// Opcode-level optimizations
extern crate alloc;
use alloc::vec::Vec;

use crate::lp_script::vm::opcodes::LpsOpCode;

mod peephole;

#[cfg(test)]
mod peephole_tests;

/// Optimize opcodes using peephole patterns
pub fn optimize(opcodes: Vec<LpsOpCode>) -> Vec<LpsOpCode> {
    peephole::optimize(opcodes)
}
