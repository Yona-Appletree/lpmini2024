#![cfg_attr(not(test), no_std)]

#[cfg(test)]
mod test_allocator;

// NOTE: Custom allocator available in test_allocator.rs to enforce memory limits
// Disabled by default as it needs tuning to avoid false positives
// To enable: uncomment the lines below and adjust limit as needed
// #[cfg(test)]
// use test_allocator::LimitedAllocator;
// #[cfg(test)]
// #[global_allocator]
// static GLOBAL: LimitedAllocator = LimitedAllocator::new(8192); // 8GB limit

extern crate alloc;

/// Shared sine lookup table to avoid duplication
mod sin_table;

/// Math utilities (fixed-point, vectors)
pub mod math;

/// Image types (grayscale, RGB)
pub mod image;

/// Test engine - modular rendering pipeline for LED effects
pub mod test_engine;

/// Scene configuration and runtime system
pub mod scene;

/// Demo program configuration
pub mod demo_program;

/// Standard test scene shared between ESP32 and host
pub mod test_scene;

/// Power limiting and brightness control
pub mod power_limit;

/// Expression parser for generating VM opcodes
pub mod lpscript;
