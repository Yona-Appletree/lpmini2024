/// Runtime errors (VM execution)
extern crate alloc;
use alloc::string::String;
use core::fmt;

#[derive(Debug)]
pub enum RuntimeError {
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
}

impl RuntimeError {
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
    pub error: RuntimeError,
    pub pc: usize,
    pub opcode: &'static str,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeError::StackUnderflow { required, actual } => {
                write!(
                    f,
                    "Stack underflow: need {} items, have {}",
                    required, actual
                )
            }
            RuntimeError::StackOverflow { sp } => {
                write!(f, "Stack overflow at sp={}", sp)
            }
            RuntimeError::LocalTypeMismatch {
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
            RuntimeError::LocalOutOfBounds { local_idx, max } => {
                write!(f, "Local index {} out of bounds (max {})", local_idx, max)
            }
            RuntimeError::DivisionByZero => {
                write!(f, "Division by zero")
            }
            RuntimeError::InvalidTextureCoords { u, v, texture_idx } => {
                write!(
                    f,
                    "Invalid texture coordinates ({}, {}) for texture {}",
                    u, v, texture_idx
                )
            }
            RuntimeError::InvalidArrayIndex { index, array_size } => {
                write!(
                    f,
                    "Array index {} out of bounds (size {})",
                    index, array_size
                )
            }
            RuntimeError::ProgramCounterOutOfBounds { pc, max } => {
                write!(f, "Program counter {} out of bounds (max {})", pc, max)
            }
            RuntimeError::TypeMismatch => {
                write!(f, "Type mismatch in operation")
            }
            RuntimeError::UnsupportedOpCode => {
                write!(f, "Unsupported opcode encountered")
            }
            RuntimeError::InstructionLimitExceeded => {
                write!(f, "Instruction limit exceeded (possible infinite loop)")
            }
            RuntimeError::CallStackOverflow { depth } => {
                write!(f, "Call stack overflow at depth {}", depth)
            }
            RuntimeError::InvalidFunctionIndex => {
                write!(f, "Invalid function index")
            }
        }
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

