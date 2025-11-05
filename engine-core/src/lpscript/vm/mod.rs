pub mod call_stack;
pub mod error;
pub mod executor;
pub mod locals_storage;
/// Virtual Machine for LightPlayer Script
pub mod opcodes;
pub mod program;
pub mod vm_stack;

pub use call_stack::{CallFrame, CallStack};
pub use error::{RuntimeError, RuntimeErrorWithContext};
pub use executor::{execute_program_lps, LpsVm, VmLimits};
pub use locals_storage::LocalsStorage;
pub use opcodes::LpsOpCode;
pub use program::{FunctionDef, LocalVarDef, LpsProgram, ParamDef};
pub use vm_stack::Stack;
