/// Control flow opcodes with error handling
use crate::lpscript::vm::error::RuntimeError;
use crate::lpscript::vm::vm_stack::Stack;

/// Execute Select (ternary): pop false_val, true_val, condition; push selected
#[inline(always)]
pub fn exec_select(stack: &mut Stack) -> Result<(), RuntimeError> {
    let (condition, true_val, false_val) = stack.pop3()?;

    stack.push_int32(if condition != 0 { true_val } else { false_val })?;

    Ok(())
}

/// Execute JumpIfZero: pop value, jump if zero
/// Returns Some(new_pc) if jump taken, None otherwise
#[inline(always)]
pub fn exec_jump_if_zero(
    stack: &mut Stack,
    pc: usize,
    offset: i32,
) -> Result<Option<usize>, RuntimeError> {
    let value = stack.pop_int32()?;

    if value == 0 {
        Ok(Some(((pc as i32) + offset + 1) as usize))
    } else {
        Ok(None)
    }
}

/// Execute JumpIfNonZero: pop value, jump if non-zero  
#[inline(always)]
pub fn exec_jump_if_nonzero(
    stack: &mut Stack,
    pc: usize,
    offset: i32,
) -> Result<Option<usize>, RuntimeError> {
    let value = stack.pop_int32()?;

    if value != 0 {
        Ok(Some(((pc as i32) + offset + 1) as usize))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::{Fixed, ToFixed};

    #[test]
    fn test_select_true() {
        let mut stack = Stack::new(64);

        // condition = 1 (true)
        stack.push_fixed(1.0f32.to_fixed()).unwrap();
        // true_val = 10
        stack.push_fixed(10.0f32.to_fixed()).unwrap();
        // false_val = 20
        stack.push_fixed(20.0f32.to_fixed()).unwrap();

        exec_select(&mut stack).unwrap();

        assert_eq!(stack.sp(), 1);
        assert_eq!(Fixed(stack.raw_slice()[0]).to_f32(), 10.0);
    }

    #[test]
    fn test_select_false() {
        let mut stack = Stack::new(64);

        // condition = 0 (false)
        stack.push_int32(0).unwrap();
        // true_val = 10
        stack.push_fixed(10.0f32.to_fixed()).unwrap();
        // false_val = 20
        stack.push_fixed(20.0f32.to_fixed()).unwrap();

        exec_select(&mut stack).unwrap();

        assert_eq!(stack.sp(), 1);
        assert_eq!(Fixed(stack.raw_slice()[0]).to_f32(), 20.0);
    }

    #[test]
    fn test_select_underflow() {
        let mut stack = Stack::new(64);
        // Only push 2 items, need 3
        stack.push_int32(1).unwrap();
        stack.push_int32(2).unwrap();

        let result = exec_select(&mut stack);
        assert!(matches!(
            result,
            Err(RuntimeError::StackUnderflow {
                required: 3,
                actual: 2
            })
        ));
    }

    #[test]
    fn test_jump_if_zero() {
        let mut stack = Stack::new(64);

        stack.push_int32(0).unwrap();

        let result = exec_jump_if_zero(&mut stack, 10, 5).unwrap();

        assert_eq!(stack.sp(), 0);
        assert_eq!(result, Some(16)); // pc + offset + 1 = 10 + 5 + 1 = 16
    }

    #[test]
    fn test_jump_if_zero_no_jump() {
        let mut stack = Stack::new(64);

        stack.push_fixed(1.0f32.to_fixed()).unwrap();

        let result = exec_jump_if_zero(&mut stack, 10, 5).unwrap();

        assert_eq!(stack.sp(), 0);
        assert_eq!(result, None);
    }
}
