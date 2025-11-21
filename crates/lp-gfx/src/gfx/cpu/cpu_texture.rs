use alloc::vec;
use alloc::vec::Vec;

use lp_math::dec32::Dec32;

use crate::gfx::gfx_error::GfxError;
use crate::gfx::texture_format::TextureFormat;

/// CPU texture storage
///
/// Stores texture data in memory. For Dec32 format, uses Vec<Dec32> for efficiency.
/// For RGBA8 and Mono1, uses Vec<u8>.
pub struct CpuTexture {
    width: usize,
    height: usize,
    format: TextureFormat,
    // For Dec32 format, store as Vec<Dec32>
    data_dec32: Option<Vec<Dec32>>,
    // For RGBA8 and Mono1, store as Vec<u8>
    data_u8: Option<Vec<u8>>,
}

impl CpuTexture {
    /// Create a new CPU texture
    pub fn new(width: usize, height: usize, format: TextureFormat) -> Result<Self, GfxError> {
        if width == 0 || height == 0 {
            return Err(GfxError::InvalidSize { width, height });
        }

        let pixel_count = width * height;
        match format {
            TextureFormat::Dec32 => {
                let mut data = Vec::new();
                data.resize(pixel_count, Dec32::ZERO);
                Ok(CpuTexture {
                    width,
                    height,
                    format,
                    data_dec32: Some(data),
                    data_u8: None,
                })
            }
            TextureFormat::RGBA8 | TextureFormat::Mono1 => {
                let bytes_per_pixel = format.bytes_per_pixel();
                let data = vec![0u8; pixel_count * bytes_per_pixel];
                Ok(CpuTexture {
                    width,
                    height,
                    format,
                    data_dec32: None,
                    data_u8: Some(data),
                })
            }
        }
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

    /// Get texture data as u8 slice (for RGBA8 and Mono1)
    pub fn data_u8(&self) -> Result<&[u8], GfxError> {
        self.data_u8.as_deref().ok_or(GfxError::FormatMismatch {
            expected: TextureFormat::RGBA8, // Dummy, just for error
            actual: self.format,
        })
    }

    /// Get mutable texture data as u8 slice (for RGBA8 and Mono1)
    pub fn data_u8_mut(&mut self) -> Result<&mut [u8], GfxError> {
        self.data_u8.as_deref_mut().ok_or(GfxError::FormatMismatch {
            expected: TextureFormat::RGBA8, // Dummy, just for error
            actual: self.format,
        })
    }

    /// Get texture data as Dec32 slice (for Dec32 format)
    pub fn data_dec32(&self) -> Result<&[Dec32], GfxError> {
        self.data_dec32.as_deref().ok_or(GfxError::FormatMismatch {
            expected: TextureFormat::Dec32, // Dummy, just for error
            actual: self.format,
        })
    }

    /// Get mutable texture data as Dec32 slice (for Dec32 format)
    pub fn data_dec32_mut(&mut self) -> Result<&mut [Dec32], GfxError> {
        self.data_dec32
            .as_deref_mut()
            .ok_or(GfxError::FormatMismatch {
                expected: TextureFormat::Dec32, // Dummy, just for error
                actual: self.format,
            })
    }

    /// Sample texture at normalized UV coordinates with bilinear interpolation
    ///
    /// Returns a single channel value (Dec32). For RGBA textures, returns the red channel.
    pub fn sample(&self, u: Dec32, v: Dec32) -> Result<Dec32, GfxError> {
        // Clamp UV to [0, 1]
        let u = u.clamp(Dec32::ZERO, Dec32::ONE);
        let v = v.clamp(Dec32::ZERO, Dec32::ONE);

        // Convert to pixel coordinates
        let x_f = Dec32::from_i32(self.width as i32) * u;
        let y_f = Dec32::from_i32(self.height as i32) * v;

        // Get integer coordinates
        let x0 = (x_f.to_f32().floor() as usize).min(self.width.saturating_sub(1));
        let y0 = (y_f.to_f32().floor() as usize).min(self.height.saturating_sub(1));
        let x1 = (x0 + 1).min(self.width.saturating_sub(1));
        let y1 = (y0 + 1).min(self.height.saturating_sub(1));

        // Get fractional parts
        let fx = x_f - Dec32::from_i32(x0 as i32);
        let fy = y_f - Dec32::from_i32(y0 as i32);

        match self.format {
            TextureFormat::Dec32 => {
                let data = self.data_dec32()?;
                let v00 = data[y0 * self.width + x0];
                let v10 = data[y0 * self.width + x1];
                let v01 = data[y1 * self.width + x0];
                let v11 = data[y1 * self.width + x1];

                // Bilinear interpolation
                let v0 = v00 + (v10 - v00) * fx;
                let v1 = v01 + (v11 - v01) * fx;
                Ok(v0 + (v1 - v0) * fy)
            }
            TextureFormat::RGBA8 => {
                let data = self.data_u8()?;
                let idx00 = (y0 * self.width + x0) * 4;
                let idx10 = (y0 * self.width + x1) * 4;
                let idx01 = (y1 * self.width + x0) * 4;
                let idx11 = (y1 * self.width + x1) * 4;

                let r00 = Dec32::from_i32(data[idx00] as i32);
                let r10 = Dec32::from_i32(data[idx10] as i32);
                let r01 = Dec32::from_i32(data[idx01] as i32);
                let r11 = Dec32::from_i32(data[idx11] as i32);

                // Bilinear interpolation
                let r0 = r00 + (r10 - r00) * fx;
                let r1 = r01 + (r11 - r01) * fx;
                Ok(r0 + (r1 - r0) * fy)
            }
            TextureFormat::Mono1 => {
                let data = self.data_u8()?;
                let v00 = Dec32::from_i32(data[y0 * self.width + x0] as i32);
                let v10 = Dec32::from_i32(data[y0 * self.width + x1] as i32);
                let v01 = Dec32::from_i32(data[y1 * self.width + x0] as i32);
                let v11 = Dec32::from_i32(data[y1 * self.width + x1] as i32);

                // Bilinear interpolation
                let v0 = v00 + (v10 - v00) * fx;
                let v1 = v01 + (v11 - v01) * fx;
                Ok(v0 + (v1 - v0) * fy)
            }
        }
    }

    /// Sample texture at normalized UV coordinates with bilinear interpolation
    ///
    /// Returns RGBA values. For single-channel textures, returns (value, value, value, 1.0).
    pub fn sample_rgba(
        &self,
        u: Dec32,
        v: Dec32,
    ) -> Result<(Dec32, Dec32, Dec32, Dec32), GfxError> {
        match self.format {
            TextureFormat::RGBA8 => {
                // Clamp UV to [0, 1]
                let u = u.clamp(Dec32::ZERO, Dec32::ONE);
                let v = v.clamp(Dec32::ZERO, Dec32::ONE);

                // Convert to pixel coordinates
                let x_f = Dec32::from_i32(self.width as i32) * u;
                let y_f = Dec32::from_i32(self.height as i32) * v;

                // Get integer coordinates
                let x0 = (x_f.to_f32().floor() as usize).min(self.width.saturating_sub(1));
                let y0 = (y_f.to_f32().floor() as usize).min(self.height.saturating_sub(1));
                let x1 = (x0 + 1).min(self.width.saturating_sub(1));
                let y1 = (y0 + 1).min(self.height.saturating_sub(1));

                // Get fractional parts
                let fx = x_f - Dec32::from_i32(x0 as i32);
                let fy = y_f - Dec32::from_i32(y0 as i32);

                let data = self.data_u8()?;
                let idx00 = (y0 * self.width + x0) * 4;
                let idx10 = (y0 * self.width + x1) * 4;
                let idx01 = (y1 * self.width + x0) * 4;
                let idx11 = (y1 * self.width + x1) * 4;

                // Sample each channel
                let sample_channel = |offset: usize| -> Dec32 {
                    let v00 = Dec32::from_i32(data[idx00 + offset] as i32);
                    let v10 = Dec32::from_i32(data[idx10 + offset] as i32);
                    let v01 = Dec32::from_i32(data[idx01 + offset] as i32);
                    let v11 = Dec32::from_i32(data[idx11 + offset] as i32);

                    let v0 = v00 + (v10 - v00) * fx;
                    let v1 = v01 + (v11 - v01) * fx;
                    v0 + (v1 - v0) * fy
                };

                let r = sample_channel(0);
                let g = sample_channel(1);
                let b = sample_channel(2);
                let a = sample_channel(3);

                Ok((r, g, b, a))
            }
            TextureFormat::Dec32 | TextureFormat::Mono1 => {
                // Single channel - return (value, value, value, 1.0)
                let value = self.sample(u, v)?;
                Ok((value, value, value, Dec32::ONE))
            }
        }
    }
}
