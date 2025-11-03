pub mod locals;
/// Virtual Machine for LightPlayer Script
pub mod opcodes;
pub mod program;

pub use locals::{LocalAccess, LocalDef, LocalType};
pub use opcodes::OpCode;
pub use program::LpsProgram;

// Note: Full VM executor implementation is ongoing.
// For now, the existing test_engine VM handles execution.
// This will be gradually migrated to the new LpsVm structure.
