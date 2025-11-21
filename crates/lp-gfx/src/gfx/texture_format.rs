/// Texture format enumeration
///
/// Defines the pixel format for textures. Format must be specified at creation time
/// and cannot be changed later (especially important for GPU contexts using OpenGL).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextureFormat {
    /// 8 bits per channel (R, G, B, A) - 32 bits per pixel total
    RGBA8,
    /// 32-bit decimal greyscale (single channel, Dec32 format)
    Dec32,
    /// 1-bit per channel monochrome - 1 bit per pixel total
    Mono1,
}

impl TextureFormat {
    /// Get the number of bytes per pixel for this format
    pub fn bytes_per_pixel(&self) -> usize {
        match self {
            TextureFormat::RGBA8 => 4,
            TextureFormat::Dec32 => 4, // Dec32 is i32, which is 4 bytes
            TextureFormat::Mono1 => 1, // Packed, but we store as 1 byte per pixel for simplicity
        }
    }

    /// Get the number of channels for this format
    pub fn channel_count(&self) -> usize {
        match self {
            TextureFormat::RGBA8 => 4,
            TextureFormat::Dec32 => 1,
            TextureFormat::Mono1 => 1,
        }
    }
}
