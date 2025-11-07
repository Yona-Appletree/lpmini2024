/// Array access opcodes (stub implementations)
use crate::vm::error::LpsVmError;
use crate::vm::value_stack::ValueStack;
use crate::math::Fixed;

/// Execute GetElemInt32ArrayFixed: pop array_ref, index; push Fixed
/// Stub implementation - returns 0.0
#[inline(always)]
pub fn exec_get_elem_int32_array_fixed(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    // Pop index and array_ref
    let (_array_ref, _index) = stack.pop2()?;

    // TODO: Implement actual array access
    // For now, return stub value (0.0)
    stack.push_fixed(Fixed::ZERO)?;

    Ok(())
}

/// Execute GetElemInt32ArrayU8: pop array_ref, index; push 4 Fixed (RGBA as bytes)
/// Stub implementation - returns (0, 0, 0, 0)
#[inline(always)]
pub fn exec_get_elem_int32_array_u8(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    // Pop index and array_ref
    let (_array_ref, _index) = stack.pop2()?;

    // TODO: Implement actual array access
    // For now, return stub values (0, 0, 0, 0)
    stack.push4(Fixed::ZERO.0, Fixed::ZERO.0, Fixed::ZERO.0, Fixed::ZERO.0)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_elem_int32_array_fixed_stub() {
        let mut stack = ValueStack::new(64);

        // Push array_ref and index
        stack.push_int32(123).unwrap(); // array_ref (stub)
        stack.push_int32(5).unwrap(); // index

        exec_get_elem_int32_array_fixed(&mut stack).unwrap();

        assert_eq!(stack.sp(), 1);
        assert_eq!(Fixed(stack.raw_slice()[0]).to_f32(), 0.0);
    }

    #[test]
    fn test_get_elem_int32_array_u8_stub() {
        let mut stack = ValueStack::new(64);

        // Push array_ref and index
        stack.push_int32(123).unwrap(); // array_ref (stub)
        stack.push_int32(5).unwrap(); // index

        exec_get_elem_int32_array_u8(&mut stack).unwrap();

        assert_eq!(stack.sp(), 4);
        assert_eq!(Fixed(stack.raw_slice()[0]).to_f32(), 0.0);
        assert_eq!(Fixed(stack.raw_slice()[1]).to_f32(), 0.0);
        assert_eq!(Fixed(stack.raw_slice()[2]).to_f32(), 0.0);
        assert_eq!(Fixed(stack.raw_slice()[3]).to_f32(), 0.0);
    }

    #[test]
    fn test_array_access_underflow() {
        let mut stack = ValueStack::new(64);
        stack.push_int32(1).unwrap(); // Only 1 item, need 2

        let result = exec_get_elem_int32_array_fixed(&mut stack);
        assert!(matches!(
            result,
            Err(LpsVmError::StackUnderflow {
                required: 2,
                actual: 1
            })
        ));
    }
}

