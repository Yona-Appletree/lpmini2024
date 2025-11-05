/// Comparison opcodes for Fixed-point values
/// 
/// These return FIXED_ONE (1.0) for true, 0 for false to match GLSL semantics
use crate::math::{Fixed, FIXED_ONE};
use crate::lpscript::vm::error::RuntimeError;

/// Execute GreaterFixed: pop b, a; push (a > b ? 1.0 : 0.0)
#[inline(always)]
pub fn exec_greater_fixed(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 2 {
        return Err(RuntimeError::StackUnderflow {
            required: 2,
            actual: *sp,
        });
    }

    *sp -= 1;
    let b = Fixed(stack[*sp]);
    *sp -= 1;
    let a = Fixed(stack[*sp]);
    stack[*sp] = if a > b { FIXED_ONE } else { 0 };
    *sp += 1;

    Ok(())
}

/// Execute LessFixed: pop b, a; push (a < b ? 1.0 : 0.0)
#[inline(always)]
pub fn exec_less_fixed(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 2 {
        return Err(RuntimeError::StackUnderflow {
            required: 2,
            actual: *sp,
        });
    }

    *sp -= 1;
    let b = Fixed(stack[*sp]);
    *sp -= 1;
    let a = Fixed(stack[*sp]);
    stack[*sp] = if a < b { FIXED_ONE } else { 0 };
    *sp += 1;

    Ok(())
}

/// Execute GreaterEqFixed: pop b, a; push (a >= b ? 1.0 : 0.0)
#[inline(always)]
pub fn exec_greater_eq_fixed(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 2 {
        return Err(RuntimeError::StackUnderflow {
            required: 2,
            actual: *sp,
        });
    }

    *sp -= 1;
    let b = Fixed(stack[*sp]);
    *sp -= 1;
    let a = Fixed(stack[*sp]);
    stack[*sp] = if a >= b { FIXED_ONE } else { 0 };
    *sp += 1;

    Ok(())
}

/// Execute LessEqFixed: pop b, a; push (a <= b ? 1.0 : 0.0)
#[inline(always)]
pub fn exec_less_eq_fixed(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 2 {
        return Err(RuntimeError::StackUnderflow {
            required: 2,
            actual: *sp,
        });
    }

    *sp -= 1;
    let b = Fixed(stack[*sp]);
    *sp -= 1;
    let a = Fixed(stack[*sp]);
    stack[*sp] = if a <= b { FIXED_ONE } else { 0 };
    *sp += 1;

    Ok(())
}

/// Execute EqFixed: pop b, a; push (a == b ? 1.0 : 0.0)
#[inline(always)]
pub fn exec_eq_fixed(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 2 {
        return Err(RuntimeError::StackUnderflow {
            required: 2,
            actual: *sp,
        });
    }

    *sp -= 1;
    let b = Fixed(stack[*sp]);
    *sp -= 1;
    let a = Fixed(stack[*sp]);
    stack[*sp] = if a == b { FIXED_ONE } else { 0 };
    *sp += 1;

    Ok(())
}

/// Execute NotEqFixed: pop b, a; push (a != b ? 1.0 : 0.0)
#[inline(always)]
pub fn exec_not_eq_fixed(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 2 {
        return Err(RuntimeError::StackUnderflow {
            required: 2,
            actual: *sp,
        });
    }

    *sp -= 1;
    let b = Fixed(stack[*sp]);
    *sp -= 1;
    let a = Fixed(stack[*sp]);
    stack[*sp] = if a != b { FIXED_ONE } else { 0 };
    *sp += 1;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::ToFixed;

    #[test]
    fn test_greater_fixed() {
        let mut stack = [0i32; 64];
        let mut sp = 0;

        stack[sp] = 5.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 3.0f32.to_fixed().0;
        sp += 1;

        exec_greater_fixed(&mut stack, &mut sp).unwrap();

        assert_eq!(sp, 1);
        assert_eq!(Fixed(stack[0]).to_f32(), 1.0); // true
    }

    #[test]
    fn test_greater_fixed_false() {
        let mut stack = [0i32; 64];
        let mut sp = 0;

        stack[sp] = 3.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 5.0f32.to_fixed().0;
        sp += 1;

        exec_greater_fixed(&mut stack, &mut sp).unwrap();

        assert_eq!(sp, 1);
        assert_eq!(Fixed(stack[0]).to_f32(), 0.0); // false
    }

    #[test]
    fn test_eq_fixed() {
        let mut stack = [0i32; 64];
        let mut sp = 0;

        stack[sp] = 5.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 5.0f32.to_fixed().0;
        sp += 1;

        exec_eq_fixed(&mut stack, &mut sp).unwrap();

        assert_eq!(sp, 1);
        assert_eq!(Fixed(stack[0]).to_f32(), 1.0); // true
    }

    #[test]
    fn test_not_eq_fixed() {
        let mut stack = [0i32; 64];
        let mut sp = 0;

        stack[sp] = 5.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 3.0f32.to_fixed().0;
        sp += 1;

        exec_not_eq_fixed(&mut stack, &mut sp).unwrap();

        assert_eq!(sp, 1);
        assert_eq!(Fixed(stack[0]).to_f32(), 1.0); // true
    }

    #[test]
    fn test_less_eq_fixed() {
        let mut stack = [0i32; 64];
        let mut sp = 0;

        stack[sp] = 3.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 3.0f32.to_fixed().0;
        sp += 1;

        exec_less_eq_fixed(&mut stack, &mut sp).unwrap();

        assert_eq!(sp, 1);
        assert_eq!(Fixed(stack[0]).to_f32(), 1.0); // true (equal)
    }
}

