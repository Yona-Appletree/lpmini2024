#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

// Re-export lp_data::kind as crate::kind for proc macro compatibility
pub mod kind {
    pub use lp_data::kind::*;
}

// Re-export RuntimeError for proc macro compatibility
pub use lp_data::RuntimeError;

pub mod node;
pub mod nodes;
pub mod scene;
pub mod scene_config;

#[cfg(test)]
mod scene_test;
