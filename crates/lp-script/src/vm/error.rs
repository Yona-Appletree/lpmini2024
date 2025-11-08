/// Runtime errors (VM execution)
extern crate alloc;
use alloc::string::String;
use core::fmt;

#[derive(Debug)]
pub enum LpsVmError {
    StackUnderflow {
        required: usize,
        actual: usize,
    },
    StackOverflow {
        sp: usize,
    },
    LocalTypeMismatch {
        local_idx: usize,
        local_name: String,
        expected: &'static str,
        found: &'static str,
    },
    LocalOutOfBounds {
        local_idx: usize,
        max: usize,
    },
    DivisionByZero,
    InvalidTextureCoords {
        u: i32,
        v: i32,
        texture_idx: usize,
    },
    InvalidArrayIndex {
        index: i32,
        array_size: usize,
    },
    ProgramCounterOutOfBounds {
        pc: usize,
        max: usize,
    },
    TypeMismatch,
    UnsupportedOpCode,
    InstructionLimitExceeded,
    CallStackOverflow {
        depth: usize,
    },
    InvalidFunctionIndex,
    PoolAllocationFailed,
}

impl LpsVmError {
    /// Add execution context (PC, opcode name) to the error
    pub fn with_context(self, pc: usize, opcode: &'static str) -> RuntimeErrorWithContext {
        RuntimeErrorWithContext {
            error: self,
            pc,
            opcode,
        }
    }
}

/// Runtime error with execution context
#[derive(Debug)]
pub struct RuntimeErrorWithContext {
    pub error: LpsVmError,
    pub pc: usize,
    pub opcode: &'static str,
}

impl fmt::Display for LpsVmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LpsVmError::StackUnderflow { required, actual } => {
                write!(
                    f,
                    "Stack underflow: need {} items, have {}",
                    required, actual
                )
            }
            LpsVmError::StackOverflow { sp } => {
                write!(f, "Stack overflow at sp={}", sp)
            }
            LpsVmError::LocalTypeMismatch {
                local_idx,
                local_name,
                expected,
                found,
            } => {
                write!(
                    f,
                    "Local type mismatch for '{}' (index {}): expected {}, found {}",
                    local_name, local_idx, expected, found
                )
            }
            LpsVmError::LocalOutOfBounds { local_idx, max } => {
                write!(f, "Local index {} out of bounds (max {})", local_idx, max)
            }
            LpsVmError::DivisionByZero => {
                write!(f, "Division by zero")
            }
            LpsVmError::InvalidTextureCoords { u, v, texture_idx } => {
                write!(
                    f,
                    "Invalid texture coordinates ({}, {}) for texture {}",
                    u, v, texture_idx
                )
            }
            LpsVmError::InvalidArrayIndex { index, array_size } => {
                write!(
                    f,
                    "Array index {} out of bounds (size {})",
                    index, array_size
                )
            }
            LpsVmError::ProgramCounterOutOfBounds { pc, max } => {
                write!(f, "Program counter {} out of bounds (max {})", pc, max)
            }
            LpsVmError::TypeMismatch => {
                write!(f, "Type mismatch in operation")
            }
            LpsVmError::UnsupportedOpCode => {
                write!(f, "Unsupported opcode encountered")
            }
            LpsVmError::InstructionLimitExceeded => {
                write!(f, "Instruction limit exceeded (possible infinite loop)")
            }
            LpsVmError::CallStackOverflow { depth } => {
                write!(f, "Call stack overflow at depth {}", depth)
            }
            LpsVmError::InvalidFunctionIndex => {
                write!(f, "Invalid function index")
            }
            LpsVmError::PoolAllocationFailed => {
                write!(f, "Failed to allocate memory from LpPool")
            }
        }
    }
}

impl From<lp_pool::error::AllocError> for LpsVmError {
    fn from(_: lp_pool::error::AllocError) -> Self {
        LpsVmError::PoolAllocationFailed
    }
}

impl fmt::Display for RuntimeErrorWithContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Runtime error at PC {} ({}): {}",
            self.pc, self.opcode, self.error
        )
    }
}
