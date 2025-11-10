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
