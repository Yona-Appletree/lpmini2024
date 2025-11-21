use crate::dec32::Dec32;
/// Array access opcodes (stub implementations)
use crate::vm::error::LpsVmError;
use crate::vm::value_stack::ValueStack;

/// Execute GetElemInt32ArrayDec32: pop array_ref, index; push Dec32
/// Stub implementation - returns 0.0
#[inline(always)]
pub fn exec_get_elem_int32_array_dec32(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    // Pop index and array_ref
    let (_array_ref, _index) = stack.pop2()?;

    // TODO: Implement actual array access
    // For now, return stub value (0.0)
    stack.push_dec32(Dec32::ZERO)?;

    Ok(())
}

/// Execute GetElemInt32ArrayU8: pop array_ref, index; push 4 Dec32 (RGBA as bytes)
/// Stub implementation - returns (0, 0, 0, 0)
#[inline(always)]
pub fn exec_get_elem_int32_array_u8(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    // Pop index and array_ref
    let (_array_ref, _index) = stack.pop2()?;

    // TODO: Implement actual array access
    // For now, return stub values (0, 0, 0, 0)
    stack.push4(Dec32::ZERO.0, Dec32::ZERO.0, Dec32::ZERO.0, Dec32::ZERO.0)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_elem_int32_array_dec32_stub() {
        let mut stack = ValueStack::new(64);

        // Push array_ref and index
        stack.push_int32(123).unwrap(); // array_ref (stub)
        stack.push_int32(5).unwrap(); // index

        exec_get_elem_int32_array_dec32(&mut stack).unwrap();

        assert_eq!(stack.sp(), 1);
        assert_eq!(Dec32(stack.raw_slice()[0]).to_f32(), 0.0);
    }

    #[test]
    fn test_get_elem_int32_array_u8_stub() {
        let mut stack = ValueStack::new(64);

        // Push array_ref and index
        stack.push_int32(123).unwrap(); // array_ref (stub)
        stack.push_int32(5).unwrap(); // index

        exec_get_elem_int32_array_u8(&mut stack).unwrap();

        assert_eq!(stack.sp(), 4);
        assert_eq!(Dec32(stack.raw_slice()[0]).to_f32(), 0.0);
        assert_eq!(Dec32(stack.raw_slice()[1]).to_f32(), 0.0);
        assert_eq!(Dec32(stack.raw_slice()[2]).to_f32(), 0.0);
        assert_eq!(Dec32(stack.raw_slice()[3]).to_f32(), 0.0);
    }

    #[test]
    fn test_array_access_underflow() {
        let mut stack = ValueStack::new(64);
        stack.push_int32(1).unwrap(); // Only 1 item, need 2

        let result = exec_get_elem_int32_array_dec32(&mut stack);
        assert!(matches!(
            result,
            Err(LpsVmError::StackUnderflow {
                required: 2,
                actual: 1
            })
        ));
    }
}
