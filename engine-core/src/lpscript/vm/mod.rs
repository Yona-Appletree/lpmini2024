pub mod error;
pub mod executor;
pub mod locals;
/// Virtual Machine for LightPlayer Script
pub mod opcodes;
pub mod program;
pub mod vm_stack;

pub use error::{RuntimeError, RuntimeErrorWithContext};
pub use executor::{execute_program_lps, LpsVm, VmLimits};
pub use locals::{LocalAccess, LocalDef, LocalType};
pub use opcodes::LpsOpCode;
pub use program::LpsProgram;
pub use vm_stack::Stack;
