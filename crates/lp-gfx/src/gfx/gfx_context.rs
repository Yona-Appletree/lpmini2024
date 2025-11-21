use lp_math::dec32::Dec32;

use crate::gfx::gfx_error::GfxError;
use crate::gfx::shader_ref::ShaderRef;
use crate::gfx::texture_format::TextureFormat;
use crate::gfx::texture_ref::TexRef;

/// Graphics context trait
///
/// Provides a unified interface for managing textures and executing shaders.
/// Implementations include CpuContext (software rendering) and GpuContext (GPU rendering).
pub trait GfxContext {
    /// Create a new texture with the specified dimensions and format
    fn create_texture(
        &mut self,
        width: usize,
        height: usize,
        format: TextureFormat,
    ) -> Result<TexRef, GfxError>;

    /// Delete a texture and free its resources
    fn delete_texture(&mut self, texture: TexRef) -> Result<(), GfxError>;

    /// Get the format of a texture
    fn get_texture_format(&self, texture: TexRef) -> Result<TextureFormat, GfxError>;

    /// Get the size of a texture (width, height)
    fn get_texture_size(&self, texture: TexRef) -> Result<(usize, usize), GfxError>;

    /// Sample a texture at normalized UV coordinates (0.0-1.0)
    ///
    /// Returns a single channel value (Dec32). For RGBA textures, returns the red channel.
    fn sample_texture(&self, texture: TexRef, u: Dec32, v: Dec32) -> Result<Dec32, GfxError>;

    /// Sample a texture at normalized UV coordinates (0.0-1.0)
    ///
    /// Returns RGBA values. For single-channel textures, returns (value, value, value, 1.0).
    fn sample_texture_rgba(
        &self,
        texture: TexRef,
        u: Dec32,
        v: Dec32,
    ) -> Result<(Dec32, Dec32, Dec32, Dec32), GfxError>;

    /// Compile a shader from source code
    ///
    /// The source code is lp-script shader code. Returns a ShaderRef that can be reused.
    fn compile_shader(&mut self, source: &str) -> Result<ShaderRef, GfxError>;

    /// Delete a compiled shader and free its resources
    fn delete_shader(&mut self, shader: ShaderRef) -> Result<(), GfxError>;

    /// Execute a compiled shader
    ///
    /// Executes the shader on the output texture, using the provided input textures.
    /// The shader receives built-in variables: `uv` (vec2), `coord` (vec2), `time` (float).
    fn execute_shader(
        &mut self,
        shader: ShaderRef,
        output: TexRef,
        inputs: &[TexRef],
        time: Dec32,
    ) -> Result<(), GfxError>;

    /// Get direct access to texture data (CPU context only)
    ///
    /// Returns a reference to the underlying texture data buffer.
    /// This is only available for CPU contexts - GPU contexts should use `download_texture`.
    fn get_texture_data(&self, texture: TexRef) -> Result<&[u8], GfxError>;

    /// Get mutable access to texture data (CPU context only)
    ///
    /// Returns a mutable reference to the underlying texture data buffer.
    /// This is only available for CPU contexts - GPU contexts should use `download_texture`.
    fn get_texture_data_mut(&mut self, texture: TexRef) -> Result<&mut [u8], GfxError>;

    /// Download texture data from GPU to CPU (GPU context only)
    ///
    /// Copies texture data from GPU memory to the provided buffer.
    /// The buffer must be sized correctly for the texture format and dimensions.
    /// Returns raw texture data in the texture's format.
    fn download_texture(&self, texture: TexRef, output: &mut [u8]) -> Result<(), GfxError>;
}
