#![cfg_attr(not(any(test, feature = "profiling", feature = "std")), no_std)]

// Disabled: Custom allocator was causing SIGABRT during full test suite
// The VM now has built-in memory limits via VmLimits, making this unnecessary
// #[cfg(test)]
// mod test_allocator;
//
// #[cfg(test)]
// use test_allocator::LimitedAllocator;
//
// #[cfg(test)]
// #[global_allocator]
// static GLOBAL: LimitedAllocator = LimitedAllocator::new(8192); // 8GB limit

extern crate alloc;

/// Image types (grayscale, RGB)
#[allow(lp_pool_std_alloc)]
pub mod image;

/// Test engine - modular rendering pipeline for LED effects
#[allow(lp_pool_std_alloc)]
pub mod test_engine;

// Re-export lp-script and fixed for convenience
pub use lp_script;
