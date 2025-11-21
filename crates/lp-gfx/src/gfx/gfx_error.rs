use alloc::string::String;

use crate::gfx::shader_ref::ShaderRef;
use crate::gfx::texture_format::TextureFormat;
use crate::gfx::texture_ref::TexRef;

/// Graphics context error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GfxError {
    /// Texture creation failed
    TextureCreationFailed(String),
    /// Texture deletion failed (e.g., invalid reference)
    TextureDeletionFailed(TexRef),
    /// Invalid texture reference
    InvalidTextureRef(TexRef),
    /// Shader compilation failed
    ShaderCompilationFailed(String),
    /// Shader deletion failed (e.g., invalid reference)
    ShaderDeletionFailed(ShaderRef),
    /// Invalid shader reference
    InvalidShaderRef(ShaderRef),
    /// Format mismatch (e.g., trying to sample RGBA texture as single channel)
    FormatMismatch {
        expected: TextureFormat,
        actual: TextureFormat,
    },
    /// Runtime execution error
    RuntimeError(String),
    /// Invalid texture size
    InvalidSize { width: usize, height: usize },
    /// Buffer size mismatch
    BufferSizeMismatch { expected: usize, actual: usize },
}

#[cfg(feature = "std")]
impl core::fmt::Display for GfxError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            GfxError::TextureCreationFailed(msg) => {
                write!(f, "Texture creation failed: {}", msg)
            }
            GfxError::TextureDeletionFailed(tex) => {
                write!(f, "Texture deletion failed for texture {}", tex.id())
            }
            GfxError::InvalidTextureRef(tex) => {
                write!(f, "Invalid texture reference: {}", tex.id())
            }
            GfxError::ShaderCompilationFailed(msg) => {
                write!(f, "Shader compilation failed: {}", msg)
            }
            GfxError::ShaderDeletionFailed(shader) => {
                write!(f, "Shader deletion failed for shader {}", shader.id())
            }
            GfxError::InvalidShaderRef(shader) => {
                write!(f, "Invalid shader reference: {}", shader.id())
            }
            GfxError::FormatMismatch { expected, actual } => {
                write!(
                    f,
                    "Format mismatch: expected {:?}, got {:?}",
                    expected, actual
                )
            }
            GfxError::RuntimeError(msg) => {
                write!(f, "Runtime error: {}", msg)
            }
            GfxError::InvalidSize { width, height } => {
                write!(f, "Invalid texture size: {}x{}", width, height)
            }
            GfxError::BufferSizeMismatch { expected, actual } => {
                write!(
                    f,
                    "Buffer size mismatch: expected {}, got {}",
                    expected, actual
                )
            }
        }
    }
}

#[cfg(feature = "std")]
extern crate std;
#[cfg(feature = "std")]
impl std::error::Error for GfxError {}
