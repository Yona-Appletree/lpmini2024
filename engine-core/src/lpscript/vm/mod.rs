/// Virtual Machine for LightPlayer Script
pub mod opcodes;
pub mod locals;
pub mod program;
pub mod executor;
pub mod error;

pub use locals::{LocalAccess, LocalDef, LocalType};
pub use opcodes::LpsOpCode;
pub use program::LpsProgram;
pub use executor::{LpsVm, VmLimits, execute_program_lps};
pub use error::{RuntimeError, RuntimeErrorWithContext};
