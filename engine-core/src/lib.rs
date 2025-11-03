#![cfg_attr(all(feature = "use-libm", not(test)), no_std)]

#[cfg(not(feature = "use-libm"))]
extern crate alloc;

/// Shared sine lookup table to avoid duplication
mod sin_table;

/// Stack-based VM for pixel operations (with instruction tests)
pub mod pixel_vm;

/// Test engine - modular rendering pipeline for LED effects
pub mod test_engine;

/// Standard test scene shared between ESP32 and host
pub mod test_scene;
