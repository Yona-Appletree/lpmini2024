/// Array access opcodes (stub implementations)
use crate::lpscript::error::RuntimeError;
use crate::math::Fixed;

/// Execute GetElemInt32ArrayFixed: pop array_ref, index; push Fixed
/// Stub implementation - returns 0.0
#[inline(always)]
pub fn exec_get_elem_int32_array_fixed(
    stack: &mut [i32],
    sp: &mut usize,
) -> Result<(), RuntimeError> {
    if *sp < 2 {
        return Err(RuntimeError::StackUnderflow {
            required: 2,
            actual: *sp,
        });
    }

    // Pop index and array_ref
    *sp -= 1;
    let _index = stack[*sp];
    *sp -= 1;
    let _array_ref = stack[*sp];

    // TODO: Implement actual array access
    // For now, return stub value (0.0)
    stack[*sp] = Fixed::ZERO.0;
    *sp += 1;

    Ok(())
}

/// Execute GetElemInt32ArrayU8: pop array_ref, index; push 4 Fixed (RGBA as bytes)
/// Stub implementation - returns (0, 0, 0, 0)
#[inline(always)]
pub fn exec_get_elem_int32_array_u8(
    stack: &mut [i32],
    sp: &mut usize,
) -> Result<(), RuntimeError> {
    if *sp < 2 {
        return Err(RuntimeError::StackUnderflow {
            required: 2,
            actual: *sp,
        });
    }

    // Pop index and array_ref
    *sp -= 1;
    let _index = stack[*sp];
    *sp -= 1;
    let _array_ref = stack[*sp];

    // TODO: Implement actual array access
    // For now, return stub values (0, 0, 0, 0)
    stack[*sp] = Fixed::ZERO.0;
    *sp += 1;
    stack[*sp] = Fixed::ZERO.0;
    *sp += 1;
    stack[*sp] = Fixed::ZERO.0;
    *sp += 1;
    stack[*sp] = Fixed::ZERO.0;
    *sp += 1;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_elem_int32_array_fixed_stub() {
        let mut stack = [0i32; 64];
        let mut sp = 0;

        // Push array_ref and index
        stack[sp] = 123; // array_ref (stub)
        sp += 1;
        stack[sp] = 5; // index
        sp += 1;

        exec_get_elem_int32_array_fixed(&mut stack, &mut sp).unwrap();

        assert_eq!(sp, 1);
        assert_eq!(Fixed(stack[0]).to_f32(), 0.0);
    }

    #[test]
    fn test_get_elem_int32_array_u8_stub() {
        let mut stack = [0i32; 64];
        let mut sp = 0;

        // Push array_ref and index
        stack[sp] = 123; // array_ref (stub)
        sp += 1;
        stack[sp] = 5; // index
        sp += 1;

        exec_get_elem_int32_array_u8(&mut stack, &mut sp).unwrap();

        assert_eq!(sp, 4);
        assert_eq!(Fixed(stack[0]).to_f32(), 0.0);
        assert_eq!(Fixed(stack[1]).to_f32(), 0.0);
        assert_eq!(Fixed(stack[2]).to_f32(), 0.0);
        assert_eq!(Fixed(stack[3]).to_f32(), 0.0);
    }

    #[test]
    fn test_array_access_underflow() {
        let mut stack = [0i32; 64];
        let mut sp = 1; // Only 1 item, need 2

        let result = exec_get_elem_int32_array_fixed(&mut stack, &mut sp);
        assert!(matches!(
            result,
            Err(RuntimeError::StackUnderflow {
                required: 2,
                actual: 1
            })
        ));
    }
}

