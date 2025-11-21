use lp_gfx::{GfxContext, GfxError, TexRef, TextureFormat};
use lp_math::dec32::Dec32;

/// Runs a simple shader test on any context, returns output texture
///
/// Uses a checkerboard pattern shader (no texture sampling required).
/// The shader generates a checkerboard pattern using only built-in variables (`uv`) and math functions.
pub fn run_shader_test<C: GfxContext>(ctx: &mut C) -> Result<TexRef, GfxError> {
    // Create output texture (RGBA8 format, e.g., 64x64)
    let output = ctx.create_texture(64, 64, TextureFormat::RGBA8)?;

    // Compile checkerboard pattern shader
    // Uses mod and floor to create a checkerboard pattern
    let shader_source = r#"
        float c = mod(floor(uv.x * 8.0) + floor(uv.y * 8.0), 2.0);
        return vec3(c, c, c);
    "#;
    let shader = ctx.compile_shader(shader_source)?;

    // Execute shader with no input textures (empty inputs array)
    ctx.execute_shader(shader, output, &[], Dec32::ZERO)?;

    // Return output texture reference
    Ok(output)
}

/// Compares two textures from different contexts
///
/// Downloads GPU texture if needed, compares pixel-by-pixel.
/// Returns true if textures match within tolerance (for floating point).
pub fn compare_textures<C1: GfxContext, C2: GfxContext>(
    ctx1: &C1,
    ctx2: &C2,
    tex1: TexRef,
    tex2: TexRef,
) -> Result<bool, GfxError> {
    // Get texture sizes and formats
    let (w1, h1) = ctx1.get_texture_size(tex1)?;
    let (w2, h2) = ctx2.get_texture_size(tex2)?;

    if w1 != w2 || h1 != h2 {
        return Ok(false);
    }

    let format1 = ctx1.get_texture_format(tex1)?;
    let format2 = ctx2.get_texture_format(tex2)?;

    if format1 != format2 {
        return Ok(false);
    }

    // Download texture data
    let bytes_per_pixel = format1.bytes_per_pixel();
    let buffer_size = w1 * h1 * bytes_per_pixel;
    let mut buffer1 = vec![0u8; buffer_size];
    let mut buffer2 = vec![0u8; buffer_size];

    // For CPU context, we can use get_texture_data directly
    // For GPU context, we need to use download_texture
    // Try get_texture_data first, fall back to download_texture
    match ctx1.get_texture_data(tex1) {
        Ok(data) => buffer1[..data.len()].copy_from_slice(data),
        Err(_) => ctx1.download_texture(tex1, &mut buffer1)?,
    }

    match ctx2.get_texture_data(tex2) {
        Ok(data) => buffer2[..data.len()].copy_from_slice(data),
        Err(_) => ctx2.download_texture(tex2, &mut buffer2)?,
    }

    // Compare pixel-by-pixel with tolerance for floating point
    // For RGBA8, compare byte-by-byte
    // For Dec32, we'd need to decode and compare with tolerance, but for now just compare bytes
    Ok(buffer1 == buffer2)
}
