/// Texture sampling opcodes (stub implementations)
use crate::lpscript::vm::error::RuntimeError;
use crate::lpscript::vm::vm_stack::Stack;
use crate::math::Fixed;

/// Execute TextureSampleR: pop 2 Fixed (UV), push 1 Fixed (R)
/// Stub implementation - returns 0.5
#[inline(always)]
pub fn exec_texture_sample_r(stack: &mut Stack, _texture_idx: u32) -> Result<(), RuntimeError> {
    // Pop UV coordinates
    let (_u, _v) = stack.pop2()?;

    // TODO: Implement actual texture sampling
    // For now, return a stub value (0.5)
    stack.push_fixed(Fixed::from_f32(0.5))?;

    Ok(())
}

/// Execute TextureSampleRGBA: pop 2 Fixed (UV), push 4 Fixed (RGBA)
/// Stub implementation - returns (0.5, 0.5, 0.5, 1.0)
#[inline(always)]
pub fn exec_texture_sample_rgba(stack: &mut Stack, _texture_idx: u32) -> Result<(), RuntimeError> {
    // Pop UV coordinates
    let (_u, _v) = stack.pop2()?;

    // TODO: Implement actual texture sampling
    // For now, return stub values (0.5, 0.5, 0.5, 1.0)
    stack.push4(
        Fixed::from_f32(0.5).0,
        Fixed::from_f32(0.5).0,
        Fixed::from_f32(0.5).0,
        Fixed::ONE.0,
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::ToFixed;

    #[test]
    fn test_texture_sample_r_stub() {
        let mut stack = Stack::new(64);

        // Push UV coordinates
        stack.push_fixed(0.5.to_fixed()).unwrap(); // u
        stack.push_fixed(0.5.to_fixed()).unwrap(); // v

        exec_texture_sample_r(&mut stack, 0).unwrap();

        assert_eq!(stack.sp(), 1);
        assert_eq!(Fixed(stack.raw_slice()[0]).to_f32(), 0.5);
    }

    #[test]
    fn test_texture_sample_rgba_stub() {
        let mut stack = Stack::new(64);

        // Push UV coordinates
        stack.push_fixed(0.5.to_fixed()).unwrap(); // u
        stack.push_fixed(0.5.to_fixed()).unwrap(); // v

        exec_texture_sample_rgba(&mut stack, 0).unwrap();

        assert_eq!(stack.sp(), 4);
        assert_eq!(Fixed(stack.raw_slice()[0]).to_f32(), 0.5);
        assert_eq!(Fixed(stack.raw_slice()[1]).to_f32(), 0.5);
        assert_eq!(Fixed(stack.raw_slice()[2]).to_f32(), 0.5);
        assert_eq!(Fixed(stack.raw_slice()[3]).to_f32(), 1.0);
    }

    #[test]
    fn test_texture_sample_underflow() {
        let mut stack = Stack::new(64);
        stack.push_int32(1).unwrap(); // Only 1 value, need 2 (UV)

        let result = exec_texture_sample_r(&mut stack, 0);
        assert!(matches!(
            result,
            Err(RuntimeError::StackUnderflow {
                required: 2,
                actual: 1
            })
        ));
    }
}

