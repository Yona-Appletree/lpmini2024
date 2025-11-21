use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use lp_math::dec32::Dec32;

use crate::gfx::cpu::cpu_texture::CpuTexture;
use crate::gfx::gfx_context::GfxContext;
use crate::gfx::gfx_error::GfxError;
use crate::gfx::shader_ref::ShaderRef;
use crate::gfx::texture_format::TextureFormat;
use crate::gfx::texture_ref::TexRef;
use crate::lp_script::compile_script;
use crate::lp_script::vm::{execute_program_lps, execute_program_lps_vec3, LpsProgram};

/// CPU graphics context
///
/// Implements GfxContext using software rendering via lp-script VM.
/// All shader execution happens pixel-by-pixel on the CPU.
pub struct CpuContext {
    textures: BTreeMap<u32, CpuTexture>,
    shaders: BTreeMap<u32, LpsProgram>,
    next_texture_id: u32,
    next_shader_id: u32,
}

impl CpuContext {
    /// Create a new CPU graphics context
    pub fn new() -> Self {
        CpuContext {
            textures: BTreeMap::new(),
            shaders: BTreeMap::new(),
            next_texture_id: 1,
            next_shader_id: 1,
        }
    }
}

impl Default for CpuContext {
    fn default() -> Self {
        Self::new()
    }
}

impl GfxContext for CpuContext {
    fn create_texture(
        &mut self,
        width: usize,
        height: usize,
        format: TextureFormat,
    ) -> Result<TexRef, GfxError> {
        let texture = CpuTexture::new(width, height, format)?;
        let id = self.next_texture_id;
        self.next_texture_id = self.next_texture_id.wrapping_add(1);
        self.textures.insert(id, texture);
        Ok(TexRef::new(id))
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

    fn sample_texture(&self, texture: TexRef, u: Dec32, v: Dec32) -> Result<Dec32, GfxError> {
        self.textures
            .get(&texture.id())
            .ok_or(GfxError::InvalidTextureRef(texture))?
            .sample(u, v)
    }

    fn sample_texture_rgba(
        &self,
        texture: TexRef,
        u: Dec32,
        v: Dec32,
    ) -> Result<(Dec32, Dec32, Dec32, Dec32), GfxError> {
        self.textures
            .get(&texture.id())
            .ok_or(GfxError::InvalidTextureRef(texture))?
            .sample_rgba(u, v)
    }

    fn compile_shader(&mut self, source: &str) -> Result<ShaderRef, GfxError> {
        let program = compile_script(source).map_err(|e| {
            // Convert CompileError to String
            // CompileError implements Display
            #[cfg(feature = "std")]
            {
                use alloc::string::ToString;
                let msg = e.to_string();
                GfxError::ShaderCompilationFailed(msg)
            }
            #[cfg(not(feature = "std"))]
            {
                use alloc::string::String;
                use core::fmt::Write;
                let mut msg = String::new();
                let _ = write!(msg, "{}", e);
                GfxError::ShaderCompilationFailed(msg)
            }
        })?;
        let id = self.next_shader_id;
        self.next_shader_id = self.next_shader_id.wrapping_add(1);
        self.shaders.insert(id, program);
        Ok(ShaderRef::new(id))
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
        shader: ShaderRef,
        output: TexRef,
        _inputs: &[TexRef],
        time: Dec32,
    ) -> Result<(), GfxError> {
        let program = self
            .shaders
            .get(&shader.id())
            .ok_or(GfxError::InvalidShaderRef(shader))?;

        let output_texture = self
            .textures
            .get_mut(&output.id())
            .ok_or(GfxError::InvalidTextureRef(output))?;

        let (width, height) = (output_texture.width(), output_texture.height());
        let format = output_texture.format();

        // TODO: Support input textures in shader execution
        // For now, we'll just execute the shader without texture sampling
        // This requires extending the VM to support texture sampling operations

        match format {
            TextureFormat::Dec32 => {
                let data = output_texture.data_dec32_mut()?;
                execute_program_lps(program, data, width, height, time);
            }
            TextureFormat::RGBA8 => {
                // For RGBA8, we need to execute as Vec3 (RGB) and then convert
                // For now, let's use a temporary buffer
                let pixel_count = width * height;
                let mut temp_buffer = Vec::new();
                temp_buffer.resize(pixel_count * 3, Dec32::ZERO);
                execute_program_lps_vec3(program, &mut temp_buffer, width, height, time);

                // Convert Vec3 output to RGBA8
                let data = output_texture.data_u8_mut()?;
                for i in 0..pixel_count {
                    let r = temp_buffer[i * 3];
                    let g = temp_buffer[i * 3 + 1];
                    let b = temp_buffer[i * 3 + 2];
                    // Clamp to [0, 255] and convert to u8
                    let r_u8 = (r.to_f32().clamp(0.0, 1.0) * 255.0) as u8;
                    let g_u8 = (g.to_f32().clamp(0.0, 1.0) * 255.0) as u8;
                    let b_u8 = (b.to_f32().clamp(0.0, 1.0) * 255.0) as u8;
                    data[i * 4] = r_u8;
                    data[i * 4 + 1] = g_u8;
                    data[i * 4 + 2] = b_u8;
                    data[i * 4 + 3] = 255; // Alpha
                }
            }
            TextureFormat::Mono1 => {
                // For Mono1, execute as scalar and convert to u8
                let pixel_count = width * height;
                let mut temp_buffer = Vec::new();
                temp_buffer.resize(pixel_count, Dec32::ZERO);
                execute_program_lps(program, &mut temp_buffer, width, height, time);

                // Convert Dec32 output to Mono1 (u8)
                let data = output_texture.data_u8_mut()?;
                for i in 0..pixel_count {
                    let value = temp_buffer[i];
                    // Clamp to [0, 1] and convert to u8
                    let value_u8 = (value.to_f32().clamp(0.0, 1.0) * 255.0) as u8;
                    data[i] = value_u8;
                }
            }
        }

        Ok(())
    }

    fn get_texture_data(&self, texture: TexRef) -> Result<&[u8], GfxError> {
        let tex = self
            .textures
            .get(&texture.id())
            .ok_or(GfxError::InvalidTextureRef(texture))?;

        match tex.format() {
            TextureFormat::Dec32 => {
                // For Dec32, we need to convert to u8 representation
                // This is a bit awkward - we'll return the raw bytes
                // Users should use get_texture_data_mut for Dec32 if they need the actual values
                Err(GfxError::FormatMismatch {
                    expected: TextureFormat::RGBA8,
                    actual: TextureFormat::Dec32,
                })
            }
            TextureFormat::RGBA8 | TextureFormat::Mono1 => tex.data_u8(),
        }
    }

    fn get_texture_data_mut(&mut self, texture: TexRef) -> Result<&mut [u8], GfxError> {
        let tex = self
            .textures
            .get_mut(&texture.id())
            .ok_or(GfxError::InvalidTextureRef(texture))?;

        match tex.format() {
            TextureFormat::Dec32 => {
                // For Dec32, we can't return &mut [u8] directly
                // Users should access via data_dec32_mut() instead
                Err(GfxError::FormatMismatch {
                    expected: TextureFormat::RGBA8,
                    actual: TextureFormat::Dec32,
                })
            }
            TextureFormat::RGBA8 | TextureFormat::Mono1 => tex.data_u8_mut(),
        }
    }

    fn download_texture(&self, texture: TexRef, output: &mut [u8]) -> Result<(), GfxError> {
        // For CPU context, download is just a copy
        let tex = self
            .textures
            .get(&texture.id())
            .ok_or(GfxError::InvalidTextureRef(texture))?;

        match tex.format() {
            TextureFormat::Dec32 => {
                // Convert Dec32 to u8 bytes
                let data = tex.data_dec32()?;
                let expected_size = data.len() * 4; // 4 bytes per Dec32
                if output.len() < expected_size {
                    return Err(GfxError::BufferSizeMismatch {
                        expected: expected_size,
                        actual: output.len(),
                    });
                }
                for (i, &dec32) in data.iter().enumerate() {
                    let bytes = dec32.to_dec32().to_le_bytes();
                    output[i * 4..i * 4 + 4].copy_from_slice(&bytes);
                }
            }
            TextureFormat::RGBA8 | TextureFormat::Mono1 => {
                let data = tex.data_u8()?;
                if output.len() < data.len() {
                    return Err(GfxError::BufferSizeMismatch {
                        expected: data.len(),
                        actual: output.len(),
                    });
                }
                output[..data.len()].copy_from_slice(data);
            }
        }
        Ok(())
    }
}
