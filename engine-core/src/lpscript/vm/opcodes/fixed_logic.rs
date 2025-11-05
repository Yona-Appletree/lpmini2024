/// Logical operations for fixed-point values
/// 
/// These treat non-zero as true, zero as false, and return FIXED_ONE for true, 0 for false
use crate::lpscript::vm::error::RuntimeError;
use crate::math::FIXED_ONE;

/// Execute AndFixed: pop b, a; push (a && b ? 1.0 : 0.0)
#[inline(always)]
pub fn exec_and_fixed(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
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
    stack[*sp] = if a != 0 && b != 0 { FIXED_ONE } else { 0 };
    *sp += 1;

    Ok(())
}

/// Execute OrFixed: pop b, a; push (a || b ? 1.0 : 0.0)
#[inline(always)]
pub fn exec_or_fixed(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
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
    stack[*sp] = if a != 0 || b != 0 { FIXED_ONE } else { 0 };
    *sp += 1;

    Ok(())
}

/// Execute NotFixed: pop a; push (!a ? 1.0 : 0.0)
#[inline(always)]
pub fn exec_not_fixed(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 1 {
        return Err(RuntimeError::StackUnderflow {
            required: 1,
            actual: *sp,
        });
    }

    *sp -= 1;
    let a = stack[*sp];
    stack[*sp] = if a == 0 { FIXED_ONE } else { 0 };
    *sp += 1;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::{Fixed, ToFixed};

    #[test]
    fn test_and_fixed() {
        let mut stack = [0i32; 64];
        let mut sp = 0;

        // true && true = true
        stack[sp] = 1.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 1.0f32.to_fixed().0;
        sp += 1;
        exec_and_fixed(&mut stack, &mut sp).unwrap();
        assert_eq!(Fixed(stack[0]).to_f32(), 1.0);

        // true && false = false
        sp = 0;
        stack[sp] = 1.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 0;
        sp += 1;
        exec_and_fixed(&mut stack, &mut sp).unwrap();
        assert_eq!(Fixed(stack[0]).to_f32(), 0.0);

        // false && false = false
        sp = 0;
        stack[sp] = 0;
        sp += 1;
        stack[sp] = 0;
        sp += 1;
        exec_and_fixed(&mut stack, &mut sp).unwrap();
        assert_eq!(Fixed(stack[0]).to_f32(), 0.0);
    }

    #[test]
    fn test_or_fixed() {
        let mut stack = [0i32; 64];
        let mut sp = 0;

        // true || true = true
        stack[sp] = 1.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 1.0f32.to_fixed().0;
        sp += 1;
        exec_or_fixed(&mut stack, &mut sp).unwrap();
        assert_eq!(Fixed(stack[0]).to_f32(), 1.0);

        // true || false = true
        sp = 0;
        stack[sp] = 1.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 0;
        sp += 1;
        exec_or_fixed(&mut stack, &mut sp).unwrap();
        assert_eq!(Fixed(stack[0]).to_f32(), 1.0);

        // false || false = false
        sp = 0;
        stack[sp] = 0;
        sp += 1;
        stack[sp] = 0;
        sp += 1;
        exec_or_fixed(&mut stack, &mut sp).unwrap();
        assert_eq!(Fixed(stack[0]).to_f32(), 0.0);
    }

    #[test]
    fn test_not_fixed() {
        let mut stack = [0i32; 64];
        let mut sp = 0;

        // !true = false
        stack[sp] = 1.0f32.to_fixed().0;
        sp += 1;
        exec_not_fixed(&mut stack, &mut sp).unwrap();
        assert_eq!(Fixed(stack[0]).to_f32(), 0.0);

        // !false = true
        sp = 0;
        stack[sp] = 0;
        sp += 1;
        exec_not_fixed(&mut stack, &mut sp).unwrap();
        assert_eq!(Fixed(stack[0]).to_f32(), 1.0);
    }

    #[test]
    fn test_logic_with_nonzero() {
        let mut stack = [0i32; 64];
        let mut sp = 0;

        // Any non-zero value is treated as true
        stack[sp] = 0.5f32.to_fixed().0; // Non-zero
        sp += 1;
        stack[sp] = 2.0f32.to_fixed().0; // Non-zero
        sp += 1;
        exec_and_fixed(&mut stack, &mut sp).unwrap();
        assert_eq!(Fixed(stack[0]).to_f32(), 1.0);
    }
}

