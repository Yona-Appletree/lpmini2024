use alloc::collections::BTreeMap;

use lp_math::dec32::Dec32;

use crate::gfx::gfx_context::GfxContext;
use crate::gfx::gfx_error::GfxError;
use crate::gfx::gpu::gpu_texture::GpuTexture;
use crate::gfx::shader_ref::ShaderRef;
use crate::gfx::texture_format::TextureFormat;
use crate::gfx::texture_ref::TexRef;

/// GPU graphics context
///
/// Implements GfxContext using GPU rendering via miniquad.
/// Shader execution happens on the GPU.
///
/// NOTE: This is a stub implementation. Full GPU support requires:
/// 1. lp-script to GLSL translation
/// 2. miniquad API integration for texture and shader management
/// 3. Render pass setup for shader execution
#[cfg(feature = "gpu")]
pub struct GpuContext {
    // TODO: Store miniquad context when API is integrated
    // For now, we can't create textures/shaders without the actual miniquad API
    textures: BTreeMap<u32, GpuTexture>,
    shaders: BTreeMap<u32, u32>, // Stub - would be miniquad::ShaderId
    #[allow(dead_code)] // Used in stub implementation
    next_texture_id: u32,
    #[allow(dead_code)] // Used in stub implementation
    next_shader_id: u32,
}

#[cfg(feature = "gpu")]
impl GpuContext {
    /// Create a new GPU graphics context
    ///
    /// NOTE: This is a stub. Full implementation requires miniquad context.
    pub fn new() -> Self {
        GpuContext {
            textures: BTreeMap::new(),
            shaders: BTreeMap::new(),
            next_texture_id: 1,
            next_shader_id: 1,
        }
    }
}

#[cfg(feature = "gpu")]
impl Default for GpuContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "gpu")]
impl GfxContext for GpuContext {
    fn create_texture(
        &mut self,
        _width: usize,
        _height: usize,
        _format: TextureFormat,
    ) -> Result<TexRef, GfxError> {
        // TODO: Implement GPU texture creation using miniquad API
        // This requires the actual miniquad context which is a trait object
        Err(GfxError::TextureCreationFailed(
            "GPU texture creation not yet implemented. miniquad API integration required.".into(),
        ))
    }

    fn delete_texture(&mut self, texture: TexRef) -> Result<(), GfxError> {
        if self.textures.remove(&texture.id()).is_some() {
            Ok(())
        } else {
            Err(GfxError::InvalidTextureRef(texture))
        }
    }

    fn get_texture_format(&self, texture: TexRef) -> Result<TextureFormat, GfxError> {
        self.textures
            .get(&texture.id())
            .map(|t| t.format())
            .ok_or(GfxError::InvalidTextureRef(texture))
    }

    fn get_texture_size(&self, texture: TexRef) -> Result<(usize, usize), GfxError> {
        self.textures
            .get(&texture.id())
            .map(|t| (t.width(), t.height()))
            .ok_or(GfxError::InvalidTextureRef(texture))
    }

    fn sample_texture(&self, _texture: TexRef, _u: Dec32, _v: Dec32) -> Result<Dec32, GfxError> {
        // GPU texture sampling happens in shaders, not via direct API
        // This method is not typically used for GPU contexts
        Err(GfxError::RuntimeError(
            "Direct texture sampling not supported for GPU context. Use shaders instead.".into(),
        ))
    }

    fn sample_texture_rgba(
        &self,
        _texture: TexRef,
        _u: Dec32,
        _v: Dec32,
    ) -> Result<(Dec32, Dec32, Dec32, Dec32), GfxError> {
        // GPU texture sampling happens in shaders, not via direct API
        Err(GfxError::RuntimeError(
            "Direct texture sampling not supported for GPU context. Use shaders instead.".into(),
        ))
    }

    fn compile_shader(&mut self, _source: &str) -> Result<ShaderRef, GfxError> {
        // TODO: Implement GPU shader compilation
        // This requires:
        // 1. Translating lp-script to GLSL (future work)
        // 2. Using miniquad shader API to compile GLSL
        // For now, return an error indicating it's not implemented
        Err(GfxError::ShaderCompilationFailed(
            "GPU shader compilation not yet implemented. lp-script to GLSL translation required."
                .into(),
        ))
    }

    fn delete_shader(&mut self, shader: ShaderRef) -> Result<(), GfxError> {
        if self.shaders.remove(&shader.id()).is_some() {
            Ok(())
        } else {
            Err(GfxError::InvalidShaderRef(shader))
        }
    }

    fn execute_shader(
        &mut self,
        _shader: ShaderRef,
        _output: TexRef,
        _inputs: &[TexRef],
        _time: Dec32,
    ) -> Result<(), GfxError> {
        // TODO: Implement GPU shader execution
        // This requires setting up a render pass, binding textures, etc.
        Err(GfxError::RuntimeError(
            "GPU shader execution not yet implemented".into(),
        ))
    }

    fn get_texture_data(&self, _texture: TexRef) -> Result<&[u8], GfxError> {
        Err(GfxError::RuntimeError(
            "Direct texture data access not available for GPU context. Use download_texture instead.".into(),
        ))
    }

    fn get_texture_data_mut(&mut self, _texture: TexRef) -> Result<&mut [u8], GfxError> {
        Err(GfxError::RuntimeError(
            "Direct texture data access not available for GPU context. Use download_texture instead.".into(),
        ))
    }

    fn download_texture(&self, texture: TexRef, output: &mut [u8]) -> Result<(), GfxError> {
        let tex = self
            .textures
            .get(&texture.id())
            .ok_or(GfxError::InvalidTextureRef(texture))?;

        // TODO: Implement texture download from GPU
        // This requires reading back texture data from GPU memory
        // For now, return an error indicating it's not implemented
        let expected_size = tex.width() * tex.height() * tex.format().bytes_per_pixel();
        if output.len() < expected_size {
            return Err(GfxError::BufferSizeMismatch {
                expected: expected_size,
                actual: output.len(),
            });
        }

        Err(GfxError::RuntimeError(
            "Texture download from GPU not yet implemented".into(),
        ))
    }
}
