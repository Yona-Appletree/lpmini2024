/// Int32 arithmetic opcodes
use crate::lpscript::vm::error::RuntimeError;

/// Execute AddInt32: pop b, a; push a + b
#[inline(always)]
pub fn exec_add_int32(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
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
    stack[*sp] = a.wrapping_add(b);
    *sp += 1;

    Ok(())
}

/// Execute SubInt32: pop b, a; push a - b
#[inline(always)]
pub fn exec_sub_int32(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
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
    stack[*sp] = a.wrapping_sub(b);
    *sp += 1;

    Ok(())
}

/// Execute MulInt32: pop b, a; push a * b
#[inline(always)]
pub fn exec_mul_int32(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
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
    stack[*sp] = a.wrapping_mul(b);
    *sp += 1;

    Ok(())
}

/// Execute DivInt32: pop b, a; push a / b
#[inline(always)]
pub fn exec_div_int32(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
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

    if b == 0 {
        return Err(RuntimeError::DivisionByZero);
    }

    stack[*sp] = a / b;
    *sp += 1;

    Ok(())
}

/// Execute ModInt32: pop b, a; push a % b
#[inline(always)]
pub fn exec_mod_int32(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
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

    if b == 0 {
        return Err(RuntimeError::DivisionByZero);
    }

    stack[*sp] = a % b;
    *sp += 1;

    Ok(())
}

/// Execute NegInt32: pop a; push -a
#[inline(always)]
pub fn exec_neg_int32(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 1 {
        return Err(RuntimeError::StackUnderflow {
            required: 1,
            actual: *sp,
        });
    }

    *sp -= 1;
    let a = stack[*sp];
    stack[*sp] = a.wrapping_neg();
    *sp += 1;

    Ok(())
}

/// Execute AbsInt32: pop a; push |a|
#[inline(always)]
pub fn exec_abs_int32(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 1 {
        return Err(RuntimeError::StackUnderflow {
            required: 1,
            actual: *sp,
        });
    }

    *sp -= 1;
    let a = stack[*sp];
    stack[*sp] = a.abs();
    *sp += 1;

    Ok(())
}

/// Execute MinInt32: pop b, a; push min(a, b)
#[inline(always)]
pub fn exec_min_int32(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
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
    stack[*sp] = if a < b { a } else { b };
    *sp += 1;

    Ok(())
}

/// Execute MaxInt32: pop b, a; push max(a, b)
#[inline(always)]
pub fn exec_max_int32(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
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
    stack[*sp] = if a > b { a } else { b };
    *sp += 1;

    Ok(())
}

/// Execute GreaterInt32: pop b, a; push (a > b ? 1 : 0)
#[inline(always)]
pub fn exec_greater_int32(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
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
    stack[*sp] = if a > b { 1 } else { 0 };
    *sp += 1;

    Ok(())
}

/// Execute LessInt32: pop b, a; push (a < b ? 1 : 0)
#[inline(always)]
pub fn exec_less_int32(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
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
    stack[*sp] = if a < b { 1 } else { 0 };
    *sp += 1;

    Ok(())
}

/// Execute BitwiseAndInt32: pop b, a; push a & b
#[inline(always)]
pub fn exec_bitwise_and_int32(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
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
    stack[*sp] = a & b;
    *sp += 1;

    Ok(())
}

/// Execute BitwiseOrInt32: pop b, a; push a | b
#[inline(always)]
pub fn exec_bitwise_or_int32(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
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
    stack[*sp] = a | b;
    *sp += 1;

    Ok(())
}

/// Execute BitwiseXorInt32: pop b, a; push a ^ b
#[inline(always)]
pub fn exec_bitwise_xor_int32(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
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
    stack[*sp] = a ^ b;
    *sp += 1;

    Ok(())
}

/// Execute BitwiseNotInt32: pop a; push ~a
#[inline(always)]
pub fn exec_bitwise_not_int32(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 1 {
        return Err(RuntimeError::StackUnderflow {
            required: 1,
            actual: *sp,
        });
    }

    *sp -= 1;
    let a = stack[*sp];
    stack[*sp] = !a;
    *sp += 1;

    Ok(())
}

/// Execute LeftShiftInt32: pop b, a; push a << b
#[inline(always)]
pub fn exec_left_shift_int32(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
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
    // Clamp shift amount to avoid undefined behavior
    let shift = (b as u32) & 31;
    stack[*sp] = a << shift;
    *sp += 1;

    Ok(())
}

/// Execute RightShiftInt32: pop b, a; push a >> b
#[inline(always)]
pub fn exec_right_shift_int32(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
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
    // Clamp shift amount to avoid undefined behavior
    let shift = (b as u32) & 31;
    stack[*sp] = a >> shift;
    *sp += 1;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_int32() {
        let mut stack = [0i32; 64];
        let mut sp = 0;

        stack[sp] = 10;
        sp += 1;
        stack[sp] = 20;
        sp += 1;

        exec_add_int32(&mut stack, &mut sp).unwrap();

        assert_eq!(sp, 1);
        assert_eq!(stack[0], 30);
    }

    #[test]
    fn test_div_int32() {
        let mut stack = [0i32; 64];
        let mut sp = 0;

        stack[sp] = 20;
        sp += 1;
        stack[sp] = 4;
        sp += 1;

        exec_div_int32(&mut stack, &mut sp).unwrap();

        assert_eq!(sp, 1);
        assert_eq!(stack[0], 5);
    }

    #[test]
    fn test_div_int32_by_zero() {
        let mut stack = [0i32; 64];
        let mut sp = 0;

        stack[sp] = 20;
        sp += 1;
        stack[sp] = 0;
        sp += 1;

        let result = exec_div_int32(&mut stack, &mut sp);
        assert!(matches!(result, Err(RuntimeError::DivisionByZero)));
    }

    #[test]
    fn test_mod_int32() {
        let mut stack = [0i32; 64];
        let mut sp = 0;

        stack[sp] = 17;
        sp += 1;
        stack[sp] = 5;
        sp += 1;

        exec_mod_int32(&mut stack, &mut sp).unwrap();

        assert_eq!(sp, 1);
        assert_eq!(stack[0], 2);
    }

    #[test]
    fn test_greater_int32() {
        let mut stack = [0i32; 64];
        let mut sp = 0;

        stack[sp] = 10;
        sp += 1;
        stack[sp] = 5;
        sp += 1;

        exec_greater_int32(&mut stack, &mut sp).unwrap();

        assert_eq!(sp, 1);
        assert_eq!(stack[0], 1); // true
    }
}

