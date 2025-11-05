/// Int32 comparison opcodes
/// 
/// These return 1 for true, 0 for false (not FIXED_ONE, but integer 1)
use crate::lpscript::vm::error::RuntimeError;

/// Execute GreaterEqInt32: pop b, a; push (a >= b ? 1 : 0)
#[inline(always)]
pub fn exec_greater_eq_int32(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 2 {
        return Err(RuntimeError::StackUnderflow {
            required: 2,
            actual: *sp,
        });
    }

    *sp -= 1;
    let b = stack[*sp];
    *sp -= 1;
    let a = stack[*sp];
    stack[*sp] = if a >= b { 1 } else { 0 };
    *sp += 1;

    Ok(())
}

/// Execute LessEqInt32: pop b, a; push (a <= b ? 1 : 0)
#[inline(always)]
pub fn exec_less_eq_int32(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 2 {
        return Err(RuntimeError::StackUnderflow {
            required: 2,
            actual: *sp,
        });
    }

    *sp -= 1;
    let b = stack[*sp];
    *sp -= 1;
    let a = stack[*sp];
    stack[*sp] = if a <= b { 1 } else { 0 };
    *sp += 1;

    Ok(())
}

/// Execute EqInt32: pop b, a; push (a == b ? 1 : 0)
#[inline(always)]
pub fn exec_eq_int32(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 2 {
        return Err(RuntimeError::StackUnderflow {
            required: 2,
            actual: *sp,
        });
    }

    *sp -= 1;
    let b = stack[*sp];
    *sp -= 1;
    let a = stack[*sp];
    stack[*sp] = if a == b { 1 } else { 0 };
    *sp += 1;

    Ok(())
}

/// Execute NotEqInt32: pop b, a; push (a != b ? 1 : 0)
#[inline(always)]
pub fn exec_not_eq_int32(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 2 {
        return Err(RuntimeError::StackUnderflow {
            required: 2,
            actual: *sp,
        });
    }

    *sp -= 1;
    let b = stack[*sp];
    *sp -= 1;
    let a = stack[*sp];
    stack[*sp] = if a != b { 1 } else { 0 };
    *sp += 1;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greater_eq_int32() {
        let mut stack = [0i32; 64];
        let mut sp = 0;

        // 10 >= 5 = true
        stack[sp] = 10;
        sp += 1;
        stack[sp] = 5;
        sp += 1;
        exec_greater_eq_int32(&mut stack, &mut sp).unwrap();
        assert_eq!(stack[0], 1);

        // 5 >= 5 = true
        sp = 0;
        stack[sp] = 5;
        sp += 1;
        stack[sp] = 5;
        sp += 1;
        exec_greater_eq_int32(&mut stack, &mut sp).unwrap();
        assert_eq!(stack[0], 1);

        // 3 >= 5 = false
        sp = 0;
        stack[sp] = 3;
        sp += 1;
        stack[sp] = 5;
        sp += 1;
        exec_greater_eq_int32(&mut stack, &mut sp).unwrap();
        assert_eq!(stack[0], 0);
    }

    #[test]
    fn test_less_eq_int32() {
        let mut stack = [0i32; 64];
        let mut sp = 0;

        // 3 <= 5 = true
        stack[sp] = 3;
        sp += 1;
        stack[sp] = 5;
        sp += 1;
        exec_less_eq_int32(&mut stack, &mut sp).unwrap();
        assert_eq!(stack[0], 1);

        // 5 <= 5 = true
        sp = 0;
        stack[sp] = 5;
        sp += 1;
        stack[sp] = 5;
        sp += 1;
        exec_less_eq_int32(&mut stack, &mut sp).unwrap();
        assert_eq!(stack[0], 1);

        // 10 <= 5 = false
        sp = 0;
        stack[sp] = 10;
        sp += 1;
        stack[sp] = 5;
        sp += 1;
        exec_less_eq_int32(&mut stack, &mut sp).unwrap();
        assert_eq!(stack[0], 0);
    }

    #[test]
    fn test_eq_int32() {
        let mut stack = [0i32; 64];
        let mut sp = 0;

        // 5 == 5 = true
        stack[sp] = 5;
        sp += 1;
        stack[sp] = 5;
        sp += 1;
        exec_eq_int32(&mut stack, &mut sp).unwrap();
        assert_eq!(stack[0], 1);

        // 5 == 3 = false
        sp = 0;
        stack[sp] = 5;
        sp += 1;
        stack[sp] = 3;
        sp += 1;
        exec_eq_int32(&mut stack, &mut sp).unwrap();
        assert_eq!(stack[0], 0);
    }

    #[test]
    fn test_not_eq_int32() {
        let mut stack = [0i32; 64];
        let mut sp = 0;

        // 5 != 3 = true
        stack[sp] = 5;
        sp += 1;
        stack[sp] = 3;
        sp += 1;
        exec_not_eq_int32(&mut stack, &mut sp).unwrap();
        assert_eq!(stack[0], 1);

        // 5 != 5 = false
        sp = 0;
        stack[sp] = 5;
        sp += 1;
        stack[sp] = 5;
        sp += 1;
        exec_not_eq_int32(&mut stack, &mut sp).unwrap();
        assert_eq!(stack[0], 0);
    }

    #[test]
    fn test_negative_numbers() {
        let mut stack = [0i32; 64];
        let mut sp = 0;

        // -5 < -3 = true
        stack[sp] = -5;
        sp += 1;
        stack[sp] = -3;
        sp += 1;
        exec_less_eq_int32(&mut stack, &mut sp).unwrap();
        assert_eq!(stack[0], 1);
    }
}

