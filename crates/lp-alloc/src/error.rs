/// Error type for allocation limit violations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocLimitError {
    /// Soft memory limit was exceeded
    SoftLimitExceeded,
}

impl core::fmt::Display for AllocLimitError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            AllocLimitError::SoftLimitExceeded => {
                write!(f, "Soft memory limit exceeded")
            }
        }
    }
}

#[cfg(any(feature = "std", test))]
impl std::error::Error for AllocLimitError {}

#[cfg(all(not(feature = "std"), not(test)))]
impl core::error::Error for AllocLimitError {}

/// Conversion from lp_pool::AllocError for migration compatibility.
/// This is only available when both crates are present during migration.
#[cfg(feature = "lp-pool-compat")]
impl From<lp_pool::AllocError> for AllocLimitError {
    fn from(err: lp_pool::AllocError) -> Self {
        match err {
            lp_pool::AllocError::PoolExhausted | lp_pool::AllocError::OutOfMemory { .. } => {
                AllocLimitError::SoftLimitExceeded
            }
            lp_pool::AllocError::InvalidLayout => {
                // InvalidLayout doesn't map cleanly, but treat as limit exceeded
                AllocLimitError::SoftLimitExceeded
            }
        }
    }
}
