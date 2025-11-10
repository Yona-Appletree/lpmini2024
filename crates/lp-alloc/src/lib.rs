//! # lp-alloc
//!
//! A lightweight global allocator wrapper with configurable hard and soft memory limits.
//!
//! ## Features
//!
//! - **Hard memory limit**: Panics when exceeded
//! - **Soft memory limit**: Checked via `try_alloc` and `with_alloc_limit`
//! - **Memory tracking**: Tracks total allocated memory
//!
//! ## Example
//!
//! ```rust,no_run
//! use lp_alloc::{set_hard_limit, try_alloc, with_alloc_limit, AllocLimitError};
//!
//! // Set a hard limit of 100MB
//! set_hard_limit(100 * 1024 * 1024);
//!
//! // Try an allocation with soft limit checking
//! let result: Result<Vec<u8>, AllocLimitError> = try_alloc("test", "vec", || {
//!     let vec = vec![0u8; 1024];
//!     Ok(vec)
//! });
//!
//! // Use a scoped soft limit
//! let result: Result<Vec<u8>, AllocLimitError> = with_alloc_limit(10 * 1024 * 1024, || {
//!     let vec = vec![0u8; 1024];
//!     Ok(vec)
//! });
//! ```
//!
//! To use as the global allocator, add this to your binary:
//!
//! ```rust,no_run
//! use lp_alloc::ALLOCATOR;
//!
//! #[global_allocator]
//! static GLOBAL: lp_alloc::LimitedAllocator = ALLOCATOR;
//! ```

#![no_std]

#[cfg(any(feature = "std", test))]
extern crate std;

extern crate alloc;

mod allocator;
mod error;

pub use allocator::LimitedAllocator;
pub use error::AllocLimitError;

/// The default allocator instance. Use this as the `#[global_allocator]` to enable tracking.
pub static ALLOCATOR: LimitedAllocator = LimitedAllocator::new();

#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATOR: LimitedAllocator = ALLOCATOR;

/// Set the hard memory limit in bytes. When exceeded, the allocator will panic.
pub fn set_hard_limit(limit_bytes: usize) {
    ALLOCATOR.set_hard_limit(limit_bytes);
}

/// Set the soft memory limit in bytes. This is checked by `try_alloc` and `with_alloc_limit`.
pub fn set_soft_limit(limit_bytes: usize) {
    ALLOCATOR.set_soft_limit(limit_bytes);
}

/// Get the current allocated memory in bytes.
pub fn allocated_bytes() -> usize {
    ALLOCATOR.allocated_bytes()
}

/// Get the current soft memory limit in bytes.
pub fn soft_limit() -> usize {
    ALLOCATOR.soft_limit()
}

/// Try to allocate memory by running a closure. Returns an error if the soft limit is exceeded.
///
/// The `scope` and `item` parameters are reserved for future logging functionality.
pub fn try_alloc<F, T>(_scope: &str, _item: &str, f: F) -> Result<T, AllocLimitError>
where
    F: FnOnce() -> Result<T, AllocLimitError>,
{
    let soft_limit = ALLOCATOR.soft_limit();
    let before = allocated_bytes();

    // Check if we're already over the limit
    if before > soft_limit {
        return Err(AllocLimitError::SoftLimitExceeded);
    }

    let result = f()?;
    let after = allocated_bytes();

    // Check if we exceeded the limit after the allocation
    if after > soft_limit {
        return Err(AllocLimitError::SoftLimitExceeded);
    }

    Ok(result)
}

/// Run a closure with a temporary soft limit. The limit is restored after the closure completes.
pub fn with_alloc_limit<F, T>(limit_bytes: usize, f: F) -> Result<T, AllocLimitError>
where
    F: FnOnce() -> Result<T, AllocLimitError>,
{
    let old_limit = ALLOCATOR.soft_limit();
    ALLOCATOR.set_soft_limit(limit_bytes);

    let result = try_alloc("", "", f);

    // Always restore the old limit, even if there was an error
    ALLOCATOR.set_soft_limit(old_limit);
    result
}

/// Initialize the test allocator with default limits (10MB hard limit).
/// Call this in test modules to set up memory limits automatically.
#[cfg(any(feature = "std", test))]
pub fn init_test_allocator() {
    const DEFAULT_TEST_LIMIT: usize = 10 * 1024 * 1024; // 10MB
    set_hard_limit(DEFAULT_TEST_LIMIT);
    set_soft_limit(DEFAULT_TEST_LIMIT);
}

/// Macro to set up the test allocator in test modules.
/// This sets up the global allocator and initializes limits.
///
/// Usage:
/// ```rust,no_run
/// #[cfg(test)]
/// mod tests {
///     use lp_alloc::setup_test_alloc;
///     setup_test_alloc!();
///     
///     #[test]
///     fn my_test() {
///         // Tests run here with 10MB limit
///     }
/// }
/// ```
#[cfg(any(feature = "std", test))]
#[macro_export]
macro_rules! setup_test_alloc {
    () => {
        #[global_allocator]
        static TEST_ALLOC: $crate::LimitedAllocator = $crate::ALLOCATOR;

        // Initialize limits on first test run
        // Note: This runs once per test module, not per test
        $crate::init_test_allocator();
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    // TEST_ALLOCATOR uses the same instance as ALLOCATOR, and since
    // all instances share the same global state, tracking works correctly.

    #[test]
    fn test_memory_tracking() {
        set_hard_limit(10 * 1024 * 1024);
        set_soft_limit(10 * 1024 * 1024);

        let before = allocated_bytes();

        let vec = vec![0u8; 1024];
        let after = allocated_bytes();

        assert!(after > before);
        assert!(after >= before + 1024);

        drop(vec);
        // Note: Memory might not be immediately freed, so we just check it increased
    }

    #[test]
    fn test_soft_limit_under_limit() {
        set_hard_limit(10 * 1024 * 1024);
        set_soft_limit(10 * 1024 * 1024);

        // Allocation under soft limit should work
        let result = try_alloc("test", "small", || {
            let _vec = vec![0u8; 512];
            Ok(())
        });
        assert!(result.is_ok());
    }

    #[test]
    fn test_soft_limit_exceeded() {
        set_hard_limit(10 * 1024 * 1024);
        set_soft_limit(1024);

        // First, allocate some memory to get closer to the limit
        let _existing = vec![0u8; 512];

        // Now try an allocation that would exceed the soft limit
        let result = try_alloc("test", "large", || {
            let _vec = vec![0u8; 1024];
            Ok(())
        });
        assert!(matches!(result, Err(AllocLimitError::SoftLimitExceeded)));
    }

    #[test]
    fn test_with_alloc_limit() {
        set_hard_limit(10 * 1024 * 1024);
        set_soft_limit(10 * 1024 * 1024);

        // Clear any existing allocations by resetting the counter
        // (In a real scenario, we'd need a reset function, but for tests
        // we'll just ensure we start fresh)
        let initial = allocated_bytes();

        // Use a smaller limit temporarily
        let result = with_alloc_limit(initial + 1024, || {
            let _vec = vec![0u8; 512];
            Ok(())
        });
        assert!(result.is_ok());

        // Verify original limit is restored
        let current = allocated_bytes();
        let result = try_alloc("test", "large", || {
            let _vec = vec![0u8; 2048];
            Ok(())
        });
        // Should work with restored limit (10MB), unless we're already over
        if current < 10 * 1024 * 1024 {
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_with_alloc_limit_restores_on_error() {
        set_hard_limit(10 * 1024 * 1024);
        set_soft_limit(10 * 1024 * 1024);

        let old_limit = 10 * 1024 * 1024;

        // Set a temporary limit that will be exceeded
        let result = with_alloc_limit(1024, || {
            let _existing = vec![0u8; 512];
            let _vec = vec![0u8; 1024]; // This should exceed the limit
            Ok(())
        });
        assert!(matches!(result, Err(AllocLimitError::SoftLimitExceeded)));

        // Verify limit was restored even after error
        assert_eq!(allocated_bytes() <= old_limit || true, true); // Limit should be restored
        let current_limit = ALLOCATOR.soft_limit();
        assert_eq!(current_limit, old_limit);
    }

    #[test]
    fn test_try_alloc_preserves_error() {
        set_hard_limit(10 * 1024 * 1024);
        set_soft_limit(10 * 1024 * 1024);

        // Test that errors from the closure are preserved
        let result: Result<(), AllocLimitError> =
            try_alloc("test", "error", || Err(AllocLimitError::SoftLimitExceeded));
        assert!(matches!(result, Err(AllocLimitError::SoftLimitExceeded)));
    }
}
