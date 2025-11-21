// Core graphics types
pub mod gfx_context;
pub mod gfx_error;
pub mod shader_ref;
pub mod texture_format;
pub mod texture_ref;

// CPU implementation (always available with cpu feature)
#[cfg(feature = "cpu")]
pub mod cpu;
#[cfg(feature = "cpu")]
pub use cpu::CpuContext;

// GPU implementation (feature-gated)
#[cfg(feature = "gpu")]
pub mod gpu;
#[cfg(feature = "gpu")]
pub use gpu::GpuContext;
