/// Virtual Machine for LightPlayer Script
pub mod opcodes;
pub mod locals;
pub mod program;
pub mod executor;

pub use locals::{LocalAccess, LocalDef, LocalType};
// Note: OpCode is NOT exported yet - we're still using test_engine::OpCode during migration
// pub use opcodes::OpCode;
pub use program::LpsProgram;
pub use executor::LpsVm;

// Note: Full VM executor implementation is ongoing.
// For now, the existing test_engine VM handles execution.
// This will be gradually migrated to the new LpsVm structure.
