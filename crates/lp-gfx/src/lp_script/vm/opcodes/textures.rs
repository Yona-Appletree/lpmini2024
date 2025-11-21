use lp_math::dec32::Dec32;

/// Texture sampling opcodes (stub implementations)
use crate::lp_script::vm::error::LpsVmError;
use crate::lp_script::vm::value_stack::ValueStack;

/// Execute TextureSampleR: pop 2 Dec32 (UV), push 1 Dec32 (R)
/// Stub implementation - returns 0.5
#[inline(always)]
pub fn exec_texture_sample_r(stack: &mut ValueStack, _texture_idx: u32) -> Result<(), LpsVmError> {
    // Pop UV coordinates
    let (_u, _v) = stack.pop2()?;

    // TODO: Implement actual texture sampling
    // For now, return a stub value (0.5)
    stack.push_dec32(Dec32::from_f32(0.5))?;

    Ok(())
}

/// Execute TextureSampleRGBA: pop 2 Dec32 (UV), push 4 Dec32 (RGBA)
/// Stub implementation - returns (0.5, 0.5, 0.5, 1.0)
#[inline(always)]
pub fn exec_texture_sample_rgba(
    stack: &mut ValueStack,
    _texture_idx: u32,
) -> Result<(), LpsVmError> {
    // Pop UV coordinates
    let (_u, _v) = stack.pop2()?;

    // TODO: Implement actual texture sampling
    // For now, return stub values (0.5, 0.5, 0.5, 1.0)
    stack.push4(
        Dec32::from_f32(0.5).0,
        Dec32::from_f32(0.5).0,
        Dec32::from_f32(0.5).0,
        Dec32::ONE.0,
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use lp_math::dec32::ToDec32;

    use super::*;

    #[test]
    fn test_texture_sample_r_stub() {
        let mut stack = ValueStack::new(64);

        // Push UV coordinates
        stack.push_dec32(0.5.to_dec32()).unwrap(); // u
        stack.push_dec32(0.5.to_dec32()).unwrap(); // v

        exec_texture_sample_r(&mut stack, 0).unwrap();

        assert_eq!(stack.sp(), 1);
        assert_eq!(Dec32(stack.raw_slice()[0]).to_f32(), 0.5);
    }

    #[test]
    fn test_texture_sample_rgba_stub() {
        let mut stack = ValueStack::new(64);

        // Push UV coordinates
        stack.push_dec32(0.5.to_dec32()).unwrap(); // u
        stack.push_dec32(0.5.to_dec32()).unwrap(); // v

        exec_texture_sample_rgba(&mut stack, 0).unwrap();

        assert_eq!(stack.sp(), 4);
        assert_eq!(Dec32(stack.raw_slice()[0]).to_f32(), 0.5);
        assert_eq!(Dec32(stack.raw_slice()[1]).to_f32(), 0.5);
        assert_eq!(Dec32(stack.raw_slice()[2]).to_f32(), 0.5);
        assert_eq!(Dec32(stack.raw_slice()[3]).to_f32(), 1.0);
    }

    #[test]
    fn test_texture_sample_underflow() {
        let mut stack = ValueStack::new(64);
        stack.push_int32(1).unwrap(); // Only 1 value, need 2 (UV)

        let result = exec_texture_sample_r(&mut stack, 0);
        assert!(matches!(
            result,
            Err(LpsVmError::StackUnderflow {
                required: 2,
                actual: 1
            })
        ));
    }
}
