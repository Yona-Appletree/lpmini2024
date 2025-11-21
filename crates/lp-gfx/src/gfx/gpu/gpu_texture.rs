use crate::gfx::gfx_error::GfxError;
use crate::gfx::texture_format::TextureFormat;

/// GPU texture storage
///
/// Manages GPU textures via miniquad.
/// Textures are stored on the GPU and cannot be directly accessed from CPU.
#[cfg(feature = "gpu")]
pub struct GpuTexture {
    width: usize,
    height: usize,
    format: TextureFormat,
    // miniquad texture handle
    texture: miniquad::TextureId,
}

#[cfg(feature = "gpu")]
impl GpuTexture {
    /// Create a new GPU texture
    ///
    /// NOTE: This is a stub. Full implementation requires miniquad context.
    pub fn new(
        _ctx: &mut (),
        width: usize,
        height: usize,
        _format: TextureFormat,
    ) -> Result<Self, GfxError> {
        if width == 0 || height == 0 {
            return Err(GfxError::InvalidSize { width, height });
        }

        // TODO: Implement GPU texture creation using miniquad API
        // The miniquad API needs to be checked for the correct way to create textures
        // For now, return an error indicating it's not implemented
        Err(GfxError::TextureCreationFailed(
            "GPU texture creation not yet implemented. miniquad API integration required.".into(),
        ))
    }

    /// Get texture width
    pub fn width(&self) -> usize {
        self.width
    }

    /// Get texture height
    pub fn height(&self) -> usize {
        self.height
    }

    /// Get texture format
    pub fn format(&self) -> TextureFormat {
        self.format
    }

    /// Get the miniquad texture ID
    pub fn texture_id(&self) -> miniquad::TextureId {
        self.texture
    }
}

// Stub implementation - actual GPU texture creation not yet implemented
#[cfg(not(feature = "gpu"))]
pub struct GpuTexture {
    width: usize,
    height: usize,
    format: TextureFormat,
}

#[cfg(not(feature = "gpu"))]
impl GpuTexture {
    pub fn new(
        _ctx: &mut (),
        width: usize,
        height: usize,
        format: TextureFormat,
    ) -> Result<Self, GfxError> {
        if width == 0 || height == 0 {
            return Err(GfxError::InvalidSize { width, height });
        }
        Ok(GpuTexture {
            width,
            height,
            format,
        })
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn format(&self) -> TextureFormat {
        self.format
    }
}
