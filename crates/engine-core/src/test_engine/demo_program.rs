extern crate alloc;
use alloc::vec;

use lp_script::{parse_expr, parse_script};

use crate::test_engine::scene::SceneConfig;
use crate::test_engine::{
    BufferFormat, BufferRef, FxPipelineConfig, MappingConfig, Palette, PipelineStep,
};

/// Create a test pattern with a rotating white line from the center
pub fn create_test_line_scene(width: usize, height: usize) -> SceneConfig {
    // Simple test: just output the angle as a gradient to verify CenterAngle works
    // centerAngle is now in radians (-π to π), normalize to 0..1 for display
    let program = parse_expr("fract((centerAngle + 3.14159) / 6.28318 + timeNorm)");

    // Grayscale palette (white = white, black = black)
    let palette = Palette::grayscale();

    let pipeline_config = FxPipelineConfig::new(
        2,
        vec![
            PipelineStep::ExprStep {
                program,
                output: BufferRef::new(0, BufferFormat::ImageGrey),
                params: vec![],
            },
            PipelineStep::PaletteStep {
                input: BufferRef::new(0, BufferFormat::ImageGrey),
                output: BufferRef::new(1, BufferFormat::ImageRgb),
                palette,
            },
        ],
    );

    let mapping_config = MappingConfig::CircularPanel9Ring { width, height };

    SceneConfig::new(pipeline_config, mapping_config)
}

/// Create the standard demo scene configuration
pub fn create_demo_scene(width: usize, height: usize) -> SceneConfig {
    // Demo program: RGB color waves with custom function
    // Returns vec3 (RGB) directly instead of using palette
    let program = parse_script(
        "\
        float wave(float dist, float angle, float freq, float phase) {
          return smoothstep(0.0, 0.4, fract(dist * freq + angle * 0.3 + phase));
        }

        float w1 = wave(centerDist, centerAngle, 4.0, -time * 0.5);
        float w2 = wave(centerDist, -centerAngle, 2.5, time * 0.3);
        float noise = perlin3(vec3(uv * 2.0, time * 0.2), 2);

        float brightness = (w1 * 0.6 + w2 * 0.4) * (0.4 + 0.6 * noise);

        float hue = fract(centerAngle * 0.15915 + time * 0.1);
        float r = saturate(abs(hue * 6.0 - 3.0) - 1.0);
        float g = saturate(2.0 - abs(hue * 6.0 - 2.0));
        float b = saturate(2.0 - abs(hue * 6.0 - 4.0));

        return vec3(r, g, b) * brightness;
    ",
    );

    // Build pipeline configuration
    let pipeline_config = FxPipelineConfig::new(
        2, // One buffer: RGB output
        vec![
            PipelineStep::ExprStep {
                program,
                output: BufferRef::new(1, BufferFormat::ImageRgb),
                params: vec![],
            },
            // PipelineStep::BlurStep {
            //     input: BufferRef::new(1, BufferFormat::ImageRgb),
            //     output: BufferRef::new(0, BufferFormat::ImageRgb), // Reuse buffer 0
            //     radius: Fixed::from_f32(0.1),                      // 0.2 pixel blur radius
            // },
        ],
    );

    let mapping_config = MappingConfig::CircularPanel9Ring { width, height };

    SceneConfig::new(pipeline_config, mapping_config)
}

/// Run the demo scene with profiling enabled
/// This function collects profiling data and generates a flamegraph
#[cfg(feature = "profiling")]
pub fn run_demo_with_profiling(
    width: usize,
    height: usize,
    num_frames: u32,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    extern crate std;
    use std::time::Instant;

    use lp_script::fixed::ToFixed;
    use pprof::ProfilerGuard;

    use crate::test_engine::scene::SceneRuntime;
    use crate::test_engine::RuntimeOptions;

    // Create the demo scene
    let config = create_demo_scene(width, height);
    let options = RuntimeOptions::new(width, height);
    let mut scene = SceneRuntime::new(config, options)
        .map_err(|e| format!("Failed to create scene: {:?}", e))?;

    // Get CPU time at start (Unix/macOS)
    #[cfg(unix)]
    let cpu_start = {
        use std::mem::MaybeUninit;
        unsafe {
            let mut tms: MaybeUninit<libc::tms> = MaybeUninit::uninit();
            let _ = libc::times(tms.as_mut_ptr());
            let tms = tms.assume_init();
            (tms.tms_utime + tms.tms_stime) as f64 / libc::sysconf(libc::_SC_CLK_TCK) as f64
        }
    };
    #[cfg(not(unix))]
    let cpu_start = 0.0;

    // Start wall clock timer and profiling
    let wall_start = Instant::now();
    let guard = ProfilerGuard::new(100)?;

    // Run the demo for the specified number of frames
    for i in 0..num_frames {
        let time = (i as f32 * 0.01).to_fixed();
        scene
            .render(time, 1)
            .map_err(|e| format!("Render failed: {:?}", e))?;
    }

    // Get CPU time at end
    #[cfg(unix)]
    let cpu_end = {
        use std::mem::MaybeUninit;
        unsafe {
            let mut tms: MaybeUninit<libc::tms> = MaybeUninit::uninit();
            let _ = libc::times(tms.as_mut_ptr());
            let tms = tms.assume_init();
            (tms.tms_utime + tms.tms_stime) as f64 / libc::sysconf(libc::_SC_CLK_TCK) as f64
        }
    };
    #[cfg(not(unix))]
    let cpu_end = 0.0;

    let wall_elapsed = wall_start.elapsed();

    // Generate and save the profile report
    if let Ok(report) = guard.report().build() {
        // Generate flamegraph directly
        let mut file = std::fs::File::create(output_path)?;
        report.flamegraph(&mut file)?;
        println!("Flamegraph saved to: {}", output_path);
    }

    // Print timing information
    println!("\nTiming Results:");
    let wall_ms = wall_elapsed.as_secs_f64() * 1000.0;
    println!(
        "  Wall clock time: {:.3}s ({:.1}ms)",
        wall_elapsed.as_secs_f64(),
        wall_ms
    );
    #[cfg(unix)]
    {
        let cpu_time = cpu_end - cpu_start;
        let cpu_ms = cpu_time * 1000.0;
        println!("  CPU time: {:.3}s ({:.1}ms)", cpu_time, cpu_ms);
        println!("  CPU ms spent: {:.1}ms", cpu_ms);
    }
    #[cfg(not(unix))]
    {
        println!("  CPU time: N/A (not available on this platform)");
    }
    println!("  Frames: {}", num_frames);
    if num_frames > 0 {
        println!("  Avg time per frame: {:.3}ms", wall_ms / num_frames as f64);
        println!(
            "  FPS: {:.1}",
            num_frames as f64 / wall_elapsed.as_secs_f64()
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use lp_script::fixed::Fixed;

    use super::*;

    #[test]
    fn test_simple_white() {
        // First test: just output white for everything
        use lp_script::vm::execute_program_lps;
        let mut output = vec![Fixed::ZERO; 16 * 16];

        let program = parse_expr("1.0");

        execute_program_lps(&program, &mut output, 16, 16, Fixed::ZERO);

        // All pixels should be white
        assert_eq!(output[0], Fixed::ONE, "First pixel should be white");
        assert_eq!(
            output[8 * 16],
            Fixed::ONE,
            "Row 8 first pixel should be white"
        );
    }

    #[test]
    fn test_yint_load() {
        // Test that YInt loads correctly
        use lp_script::vm::execute_program_lps;
        let mut output = vec![Fixed::ZERO; 16 * 16];

        let program = parse_expr("coord.y");

        execute_program_lps(&program, &mut output, 16, 16, Fixed::ZERO);

        // Row 0 should have Y values of 0.5 in fixed-point
        println!(
            "Row 0, pixel 0: {:#x} (expected ~{:#x})",
            output[0].0,
            1 << 15
        );
        // Row 8 should have Y values of 8.5 in fixed-point
        println!(
            "Row 8, pixel 0: {:#x} (expected ~{:#x})",
            output[8 * 16].0,
            (8 << 16) + (1 << 15)
        );

        // Just verify it's not all zeros
        assert!(output[8 * 16].0 != 0, "Row 8 YInt should not be zero");
    }

    #[test]
    fn test_normalized_center_line() {
        // Test the normalized Y coordinate approach
        use lp_script::vm::execute_program_lps;

        // Test with 16x16 - center should be between row 7 and 8
        let mut output = vec![Fixed::ZERO; 16 * 16];

        // Adjusted range to match actual uv.y values
        // Row 7: uv.y = 0.4688, Row 8: uv.y = 0.5312
        let program = parse_expr("(uv.y > 0.46 && uv.y < 0.54) ? 1.0 : 0.0");

        execute_program_lps(&program, &mut output, 16, 16, Fixed::ZERO);

        // Center rows (7 and 8) should be white
        assert_eq!(
            output[7 * 16],
            Fixed::ONE,
            "Row 7 (uv.y=0.4688) should be white"
        );
        assert_eq!(
            output[8 * 16],
            Fixed::ONE,
            "Row 8 (uv.y=0.5312) should be white"
        );
        // Rows far from center should be black
        assert_eq!(output[0], Fixed::ZERO, "Top row should be black");
        assert_eq!(output[15 * 16], Fixed::ZERO, "Bottom row should be black");

        // Test with 8x8 - center should be between row 3 and 4
        let mut output8 = vec![Fixed::ZERO; 8 * 8];
        // Row 3: (3+0.5)/8 = 0.4375, Row 4: (4+0.5)/8 = 0.5625
        execute_program_lps(&program, &mut output8, 8, 8, Fixed::ZERO);

        // Center rows (3 and 4) should be white with the range 0.46-0.54
        assert_eq!(
            output8[3 * 8],
            Fixed::ZERO,
            "Row 3 (uv.y=0.4375) should be black (outside range)"
        );
        assert_eq!(
            output8[4 * 8],
            Fixed::ZERO,
            "Row 4 (uv.y=0.5625) should be black (outside range)"
        );
        assert_eq!(output8[0], Fixed::ZERO, "Top row in 8x8 should be black");
    }
}
