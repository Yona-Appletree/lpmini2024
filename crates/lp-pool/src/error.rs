use core::fmt;

/// Errors that can occur during memory allocation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocError {
    /// Out of memory - requested more than available
    OutOfMemory {
        requested: usize,
        available: usize,
    },
    /// Pool exhausted - no more blocks available
    PoolExhausted,
    /// Invalid layout - alignment or size requirements invalid
    InvalidLayout,
}

impl fmt::Display for AllocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AllocError::OutOfMemory { requested, available } => {
                write!(f, "Out of memory: requested {} bytes, {} available", requested, available)
            }
            AllocError::PoolExhausted => {
                write!(f, "Pool exhausted - no more blocks available")
            }
            AllocError::InvalidLayout => {
                write!(f, "Invalid layout - alignment or size requirements invalid")
            }
        }
    }
}


