#[cfg(all(feature = "std", feature = "cpu"))]
mod tests {
    use lp_gfx::{CpuContext, GfxContext, GfxError, TexRef, TextureFormat};
    use lp_math::dec32::Dec32;

    // Test utilities (inline for now)
    fn run_shader_test(ctx: &mut CpuContext) -> Result<TexRef, GfxError> {
        let output = ctx.create_texture(64, 64, TextureFormat::RGBA8)?;
        let shader_source = r#"
            float c = mod(floor(uv.x * 8.0) + floor(uv.y * 8.0), 2.0);
            return vec3(c, c, c);
        "#;
        let shader = ctx.compile_shader(shader_source)?;
        ctx.execute_shader(shader, output, &[], Dec32::ZERO)?;
        Ok(output)
    }

    #[test]
    fn test_cpu_shader_execution() {
        // Create CPU context
        let mut cpu_ctx = CpuContext::new();

        // Run shader test
        let result = run_shader_test(&mut cpu_ctx);
        assert!(result.is_ok(), "Shader execution should succeed");

        let texture = result.unwrap();
        let (width, height) = cpu_ctx.get_texture_size(texture).unwrap();
        assert_eq!(width, 64);
        assert_eq!(height, 64);

        let format = cpu_ctx.get_texture_format(texture).unwrap();
        assert_eq!(format, TextureFormat::RGBA8);
    }

    #[test]
    fn test_texture_creation() {
        let mut ctx = CpuContext::new();

        // Test RGBA8 texture
        let tex1 = ctx.create_texture(256, 256, TextureFormat::RGBA8).unwrap();
        let (w, h) = ctx.get_texture_size(tex1).unwrap();
        assert_eq!(w, 256);
        assert_eq!(h, 256);
        assert_eq!(ctx.get_texture_format(tex1).unwrap(), TextureFormat::RGBA8);

        // Test Dec32 texture
        let tex2 = ctx.create_texture(128, 128, TextureFormat::Dec32).unwrap();
        assert_eq!(ctx.get_texture_format(tex2).unwrap(), TextureFormat::Dec32);

        // Test Mono1 texture
        let tex3 = ctx.create_texture(64, 64, TextureFormat::Mono1).unwrap();
        assert_eq!(ctx.get_texture_format(tex3).unwrap(), TextureFormat::Mono1);
    }

    #[test]
    fn test_texture_deletion() {
        let mut ctx = CpuContext::new();
        let tex = ctx.create_texture(64, 64, TextureFormat::RGBA8).unwrap();
        assert!(ctx.delete_texture(tex).is_ok());
        // Trying to use deleted texture should fail
        assert!(ctx.get_texture_size(tex).is_err());
    }

    #[cfg(feature = "gpu")]
    #[test]
    fn test_cpu_gpu_shader_match() {
        // Create CPU context
        let mut cpu_ctx = CpuContext::new();

        // Create GPU context (stub - will fail until GPU implementation is complete)
        // let mut gpu_ctx = GpuContext::new();

        // Run same shader on both
        let cpu_result = run_shader_test(&mut cpu_ctx).unwrap();
        // let gpu_result = run_shader_test(&mut gpu_ctx).unwrap();

        // Compare results
        // assert!(compare_textures(&cpu_ctx, &gpu_ctx, cpu_result, gpu_result).unwrap());

        // For now, just verify CPU works
        assert!(cpu_result.id() > 0);
    }
}
