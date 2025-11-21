/// Expression step execution with type validation
extern crate alloc;
use alloc::vec;

use lp_gfx::lp_script::dec32::Dec32;
use lp_gfx::lp_script::{execute_program_lps, execute_program_lps_vec3, LpsProgram, Type};

use super::rgb_utils::grey_to_i32;
use super::{BufferFormat, PipelineError};

/// Validate that the program's return type matches the expected buffer format
pub fn validate_expr_program_type(
    program: &LpsProgram,
    expected_format: BufferFormat,
) -> Result<(), PipelineError> {
    let main_func = program
        .main_function()
        .ok_or(PipelineError::InvalidProgram(
            "No main function found".into(),
        ))?;

    let expected_type = match expected_format {
        BufferFormat::ImageGrey => Type::Dec32,
        BufferFormat::ImageRgb => Type::Vec3,
    };

    if main_func.return_type != expected_type {
        return Err(PipelineError::TypeMismatch {
            expected: expected_type,
            actual: main_func.return_type.clone(),
            context: "Expression step output".into(),
        });
    }

    Ok(())
}

/// Execute an expression step with type validation
pub fn execute_expr_step(
    program: &LpsProgram,
    output_data: &mut [i32],
    output_format: BufferFormat,
    width: usize,
    height: usize,
    time: Dec32,
) -> Result<(), PipelineError> {
    // Validate program return type matches buffer format
    validate_expr_program_type(program, output_format)?;

    match output_format {
        BufferFormat::ImageGrey => {
            // Execute VM program into a temporary greyscale buffer
            let mut temp_grey: vec::Vec<Dec32> = vec![Dec32::ZERO; width * height];
            execute_program_lps(program, &mut temp_grey, width, height, time);

            // Write greyscale results to output buffer
            for i in 0..temp_grey.len() {
                output_data[i] = grey_to_i32(temp_grey[i]);
            }

            Ok(())
        }
        BufferFormat::ImageRgb => {
            // Execute VM program into a temporary Vec3 buffer
            // Vec3 outputs are 3x the size (r, g, b per pixel)
            let mut temp_vec3: vec::Vec<Dec32> = vec![Dec32::ZERO; width * height * 3];
            execute_program_lps_vec3(program, &mut temp_vec3, width, height, time);

            // Pack RGB triplets into output buffer
            for i in 0..(width * height) {
                let r_dec32 = temp_vec3[i * 3];
                let g_dec32 = temp_vec3[i * 3 + 1];
                let b_dec32 = temp_vec3[i * 3 + 2];

                // Convert Dec32 (0..1) to u8 (0..255)
                let r = (r_dec32.to_f32().clamp(0.0, 1.0) * 255.0) as u8;
                let g = (g_dec32.to_f32().clamp(0.0, 1.0) * 255.0) as u8;
                let b = (b_dec32.to_f32().clamp(0.0, 1.0) * 255.0) as u8;

                // Pack into i32
                output_data[i] = super::rgb_utils::pack_rgb(r, g, b);
            }

            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use lp_gfx::lp_script::parse_expr;

    use super::*;

    #[test]
    fn test_validate_type_dec32_to_grey_succeeds() {
        // Expression returning Dec32 should work with ImageGrey
        let program = parse_expr("0.5");

        let result = validate_expr_program_type(&program, BufferFormat::ImageGrey);
        assert!(
            result.is_ok(),
            "Dec32 return type should be valid for ImageGrey"
        );
    }

    #[test]
    fn test_validate_type_vec3_to_rgb_succeeds() {
        // Expression returning Vec3 should work with ImageRgb
        let program = parse_expr("vec3(1.0, 0.5, 0.0)");

        let result = validate_expr_program_type(&program, BufferFormat::ImageRgb);
        assert!(
            result.is_ok(),
            "Vec3 return type should be valid for ImageRgb"
        );
    }

    #[test]
    fn test_validate_type_dec32_to_rgb_fails() {
        // Expression returning Dec32 should FAIL with ImageRgb
        let program = parse_expr("0.5");

        let result = validate_expr_program_type(&program, BufferFormat::ImageRgb);
        assert!(
            result.is_err(),
            "Dec32 return type should NOT be valid for ImageRgb"
        );

        match result {
            Err(PipelineError::TypeMismatch {
                expected, actual, ..
            }) => {
                assert_eq!(expected, Type::Vec3);
                assert_eq!(actual, Type::Dec32);
            }
            _ => panic!("Expected TypeMismatch error"),
        }
    }

    #[test]
    fn test_validate_type_vec3_to_grey_fails() {
        // Expression returning Vec3 should FAIL with ImageGrey
        let program = parse_expr("vec3(1.0, 0.5, 0.0)");

        let result = validate_expr_program_type(&program, BufferFormat::ImageGrey);
        assert!(
            result.is_err(),
            "Vec3 return type should NOT be valid for ImageGrey"
        );

        match result {
            Err(PipelineError::TypeMismatch {
                expected, actual, ..
            }) => {
                assert_eq!(expected, Type::Dec32);
                assert_eq!(actual, Type::Vec3);
            }
            _ => panic!("Expected TypeMismatch error"),
        }
    }

    #[test]
    fn test_execute_grey_expression() {
        // Test executing a greyscale expression
        let program = parse_expr("xNorm");
        let mut output = vec![0i32; 4 * 4];

        let result = execute_expr_step(
            &program,
            &mut output,
            BufferFormat::ImageGrey,
            4,
            4,
            Dec32::ZERO,
        );

        assert!(result.is_ok());
        // First pixel should be (0+0.5)/4 = 0.125 in dec32 point
        assert!(output[0] != 0, "Output should have non-zero values");
    }

    #[test]
    fn test_execute_wrong_type_fails() {
        // Test that executing Vec3 program with ImageGrey fails early
        let program = parse_expr("vec3(1.0, 0.5, 0.0)");
        let mut output = vec![0i32; 4 * 4];

        let result = execute_expr_step(
            &program,
            &mut output,
            BufferFormat::ImageGrey,
            4,
            4,
            Dec32::ZERO,
        );

        assert!(result.is_err());
        match result {
            Err(PipelineError::TypeMismatch {
                expected, actual, ..
            }) => {
                assert_eq!(expected, Type::Dec32);
                assert_eq!(actual, Type::Vec3);
            }
            _ => panic!("Expected TypeMismatch error, got {:?}", result),
        }
    }

    #[test]
    fn test_rgb_program_requires_rgb_buffer() {
        // This test would have caught the original crash bug!
        // Attempting to use Vec3 return with ImageGrey should fail with clear error
        let program = parse_expr("vec3(xNorm, yNorm, 0.5)");
        let mut output = vec![0i32; 16 * 16];

        let result = execute_expr_step(
            &program,
            &mut output,
            BufferFormat::ImageGrey, // Wrong format!
            16,
            16,
            Dec32::ZERO,
        );

        // Should fail with type mismatch, not crash
        assert!(
            result.is_err(),
            "Should reject Vec3 program with ImageGrey buffer"
        );
        match result {
            Err(PipelineError::TypeMismatch {
                expected,
                actual,
                context,
            }) => {
                assert_eq!(expected, Type::Dec32);
                assert_eq!(actual, Type::Vec3);
                assert!(
                    context.contains("Expression step"),
                    "Error should mention expression step"
                );
            }
            other => panic!("Expected TypeMismatch error, got {:?}", other),
        }
    }

    #[test]
    fn test_execute_rgb_expression() {
        // Test executing an RGB (vec3) expression
        let program = parse_expr("vec3(xNorm, yNorm, 0.5)");
        let mut output = vec![0i32; 4 * 4];

        let result = execute_expr_step(
            &program,
            &mut output,
            BufferFormat::ImageRgb,
            4,
            4,
            Dec32::ZERO,
        );

        assert!(result.is_ok(), "RGB expression should execute successfully");

        // Check that we got non-zero RGB values
        assert!(output[0] != 0, "Should have RGB data in output");

        // First pixel should have:
        // r = (0+0.5)/4 = 0.125
        // g = (0+0.5)/4 = 0.125
        // b = 0.5
        // All scaled to 0-255 range
        let (r, g, b) = super::super::rgb_utils::unpack_rgb(output[0]);
        assert!(
            r > 0 && r < 100,
            "Red channel should be low but not zero, got {}",
            r
        );
        assert!(
            g > 0 && g < 100,
            "Green channel should be low but not zero, got {}",
            g
        );
        assert!(b > 100, "Blue channel should be high, got {}", b);
    }

    #[test]
    fn test_script_with_vec3_return_preserves_type() {
        // This test catches the bug where parse_script loses return type!
        use lp_gfx::lp_script::parse_script;

        let program = parse_script(
            "float r = xNorm; \
             float g = yNorm; \
             float b = 0.5; \
             return vec3(r, g, b);",
        );

        // Verify the program has Vec3 return type
        let main_func = program.main_function().expect("Should have main function");
        assert_eq!(
            main_func.return_type,
            Type::Vec3,
            "Script with vec3 return should have Vec3 type, but got {:?}",
            main_func.return_type
        );
    }

    #[test]
    fn test_execute_rgb_script_with_function() {
        // This test simulates the actual demo program structure
        use lp_gfx::lp_script::parse_script;

        let program = parse_script(
            "float hue = xNorm; \
             float r = hue; \
             float g = 1.0 - hue; \
             float b = 0.5; \
             return vec3(r, g, b);",
        );

        let mut output = vec![0i32; 4 * 4];

        let result = execute_expr_step(
            &program,
            &mut output,
            BufferFormat::ImageRgb,
            4,
            4,
            Dec32::ZERO,
        );

        assert!(
            result.is_ok(),
            "RGB script should execute successfully, got {:?}",
            result
        );
        assert!(output[0] != 0, "Should have RGB data in output");
    }
}
