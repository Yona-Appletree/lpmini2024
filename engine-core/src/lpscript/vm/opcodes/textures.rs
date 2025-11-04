/// Texture sampling opcodes (stub implementations)
use crate::lpscript::error::RuntimeError;
use crate::math::Fixed;

/// Execute TextureSampleR: pop 2 Fixed (UV), push 1 Fixed (R)
/// Stub implementation - returns 0.5
#[inline(always)]
pub fn exec_texture_sample_r(
    stack: &mut [i32],
    sp: &mut usize,
    _texture_idx: u32,
) -> Result<(), RuntimeError> {
    if *sp < 2 {
        return Err(RuntimeError::StackUnderflow {
            required: 2,
            actual: *sp,
        });
    }

    // Pop UV coordinates
    *sp -= 1;
    let _v = Fixed(stack[*sp]);
    *sp -= 1;
    let _u = Fixed(stack[*sp]);

    // TODO: Implement actual texture sampling
    // For now, return a stub value (0.5)
    stack[*sp] = Fixed::from_f32(0.5).0;
    *sp += 1;

    Ok(())
}

/// Execute TextureSampleRGBA: pop 2 Fixed (UV), push 4 Fixed (RGBA)
/// Stub implementation - returns (0.5, 0.5, 0.5, 1.0)
#[inline(always)]
pub fn exec_texture_sample_rgba(
    stack: &mut [i32],
    sp: &mut usize,
    _texture_idx: u32,
) -> Result<(), RuntimeError> {
    if *sp < 2 {
        return Err(RuntimeError::StackUnderflow {
            required: 2,
            actual: *sp,
        });
    }

    // Pop UV coordinates
    *sp -= 1;
    let _v = Fixed(stack[*sp]);
    *sp -= 1;
    let _u = Fixed(stack[*sp]);

    // TODO: Implement actual texture sampling
    // For now, return stub values (0.5, 0.5, 0.5, 1.0)
    stack[*sp] = Fixed::from_f32(0.5).0;
    *sp += 1;
    stack[*sp] = Fixed::from_f32(0.5).0;
    *sp += 1;
    stack[*sp] = Fixed::from_f32(0.5).0;
    *sp += 1;
    stack[*sp] = Fixed::ONE.0;
    *sp += 1;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::ToFixed;

    #[test]
    fn test_texture_sample_r_stub() {
        let mut stack = [0i32; 64];
        let mut sp = 0;

        // Push UV coordinates
        stack[sp] = 0.5f32.to_fixed().0; // u
        sp += 1;
        stack[sp] = 0.5f32.to_fixed().0; // v
        sp += 1;

        exec_texture_sample_r(&mut stack, &mut sp, 0).unwrap();

        assert_eq!(sp, 1);
        assert_eq!(Fixed(stack[0]).to_f32(), 0.5);
    }

    #[test]
    fn test_texture_sample_rgba_stub() {
        let mut stack = [0i32; 64];
        let mut sp = 0;

        // Push UV coordinates
        stack[sp] = 0.5f32.to_fixed().0; // u
        sp += 1;
        stack[sp] = 0.5f32.to_fixed().0; // v
        sp += 1;

        exec_texture_sample_rgba(&mut stack, &mut sp, 0).unwrap();

        assert_eq!(sp, 4);
        assert_eq!(Fixed(stack[0]).to_f32(), 0.5); // R
        assert_eq!(Fixed(stack[1]).to_f32(), 0.5); // G
        assert_eq!(Fixed(stack[2]).to_f32(), 0.5); // B
        assert_eq!(Fixed(stack[3]).to_f32(), 1.0); // A
    }

    #[test]
    fn test_texture_sample_r_underflow() {
        let mut stack = [0i32; 64];
        let mut sp = 1; // Only 1 item, need 2

        let result = exec_texture_sample_r(&mut stack, &mut sp, 0);
        assert!(matches!(
            result,
            Err(RuntimeError::StackUnderflow {
                required: 2,
                actual: 1
            })
        ));
    }
}

