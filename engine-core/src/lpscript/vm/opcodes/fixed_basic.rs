use crate::lpscript::vm::error::RuntimeError;
use crate::math::trig::{cos, sin};
/// Basic fixed-point arithmetic opcodes with error handling
use crate::math::Fixed;
use crate::math::{ceil, floor, sqrt};

/// Execute AddFixed: pop b, a; push a + b
#[inline(always)]
pub fn exec_add_fixed(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
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
    stack[*sp] = (a + b).0;
    *sp += 1;

    Ok(())
}

/// Execute SubFixed: pop b, a; push a - b
#[inline(always)]
pub fn exec_sub_fixed(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
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
    stack[*sp] = (a - b).0;
    *sp += 1;

    Ok(())
}

/// Execute MulFixed: pop b, a; push a * b
#[inline(always)]
pub fn exec_mul_fixed(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
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
    stack[*sp] = (a * b).0;
    *sp += 1;

    Ok(())
}

/// Execute DivFixed: pop b, a; push a / b
#[inline(always)]
pub fn exec_div_fixed(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
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

    if b.0 == 0 {
        return Err(RuntimeError::DivisionByZero);
    }

    stack[*sp] = (a / b).0;
    *sp += 1;

    Ok(())
}

/// Execute NegFixed: pop a; push -a
#[inline(always)]
pub fn exec_neg_fixed(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 1 {
        return Err(RuntimeError::StackUnderflow {
            required: 1,
            actual: *sp,
        });
    }

    *sp -= 1;
    let a = Fixed(stack[*sp]);
    stack[*sp] = (-a).0;
    *sp += 1;

    Ok(())
}

/// Execute AbsFixed: pop a; push |a|
#[inline(always)]
pub fn exec_abs_fixed(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 1 {
        return Err(RuntimeError::StackUnderflow {
            required: 1,
            actual: *sp,
        });
    }

    *sp -= 1;
    let a = Fixed(stack[*sp]);
    stack[*sp] = a.0.abs();
    *sp += 1;

    Ok(())
}

/// Execute MinFixed: pop b, a; push min(a, b)
#[inline(always)]
pub fn exec_min_fixed(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
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
    stack[*sp] = if a.0 < b.0 { a.0 } else { b.0 };
    *sp += 1;

    Ok(())
}

/// Execute MaxFixed: pop b, a; push max(a, b)
#[inline(always)]
pub fn exec_max_fixed(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
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
    stack[*sp] = if a.0 > b.0 { a.0 } else { b.0 };
    *sp += 1;

    Ok(())
}

/// Execute SinFixed: pop a; push sin(a)
#[inline(always)]
pub fn exec_sin_fixed(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 1 {
        return Err(RuntimeError::StackUnderflow {
            required: 1,
            actual: *sp,
        });
    }

    *sp -= 1;
    let a = Fixed(stack[*sp]);
    stack[*sp] = sin(a).0;
    *sp += 1;

    Ok(())
}

/// Execute CosFixed: pop a; push cos(a)
#[inline(always)]
pub fn exec_cos_fixed(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 1 {
        return Err(RuntimeError::StackUnderflow {
            required: 1,
            actual: *sp,
        });
    }

    *sp -= 1;
    let a = Fixed(stack[*sp]);
    stack[*sp] = cos(a).0;
    *sp += 1;

    Ok(())
}

/// Execute SqrtFixed: pop a; push sqrt(a)
#[inline(always)]
pub fn exec_sqrt_fixed(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 1 {
        return Err(RuntimeError::StackUnderflow {
            required: 1,
            actual: *sp,
        });
    }

    *sp -= 1;
    let a = Fixed(stack[*sp]);
    stack[*sp] = sqrt(a).0;
    *sp += 1;

    Ok(())
}

/// Execute FloorFixed: pop a; push floor(a)
#[inline(always)]
pub fn exec_floor_fixed(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 1 {
        return Err(RuntimeError::StackUnderflow {
            required: 1,
            actual: *sp,
        });
    }

    *sp -= 1;
    let a = Fixed(stack[*sp]);
    stack[*sp] = floor(a).0;
    *sp += 1;

    Ok(())
}

/// Execute CeilFixed: pop a; push ceil(a)
#[inline(always)]
pub fn exec_ceil_fixed(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 1 {
        return Err(RuntimeError::StackUnderflow {
            required: 1,
            actual: *sp,
        });
    }

    *sp -= 1;
    let a = Fixed(stack[*sp]);
    stack[*sp] = ceil(a).0;
    *sp += 1;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::ToFixed;

    #[test]
    fn test_add_fixed() {
        let mut stack = [0i32; 64];
        let mut sp = 0;

        stack[sp] = 1.5f32.to_fixed().0;
        sp += 1;
        stack[sp] = 2.5f32.to_fixed().0;
        sp += 1;

        exec_add_fixed(&mut stack, &mut sp).unwrap();

        assert_eq!(sp, 1);
        assert_eq!(Fixed(stack[0]).to_f32(), 4.0);
    }

    #[test]
    fn test_add_fixed_underflow() {
        let mut stack = [0i32; 64];
        let mut sp = 1; // Only 1 item

        let result = exec_add_fixed(&mut stack, &mut sp);
        assert!(matches!(
            result,
            Err(RuntimeError::StackUnderflow {
                required: 2,
                actual: 1
            })
        ));
    }

    #[test]
    fn test_sub_fixed() {
        let mut stack = [0i32; 64];
        let mut sp = 0;

        stack[sp] = 5.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 2.0f32.to_fixed().0;
        sp += 1;

        exec_sub_fixed(&mut stack, &mut sp).unwrap();

        assert_eq!(sp, 1);
        assert_eq!(Fixed(stack[0]).to_f32(), 3.0);
    }

    #[test]
    fn test_mul_fixed() {
        let mut stack = [0i32; 64];
        let mut sp = 0;

        stack[sp] = 3.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 4.0f32.to_fixed().0;
        sp += 1;

        exec_mul_fixed(&mut stack, &mut sp).unwrap();

        assert_eq!(sp, 1);
        assert_eq!(Fixed(stack[0]).to_f32(), 12.0);
    }

    #[test]
    fn test_div_fixed() {
        let mut stack = [0i32; 64];
        let mut sp = 0;

        stack[sp] = 10.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 2.0f32.to_fixed().0;
        sp += 1;

        exec_div_fixed(&mut stack, &mut sp).unwrap();

        assert_eq!(sp, 1);
        assert_eq!(Fixed(stack[0]).to_f32(), 5.0);
    }

    #[test]
    fn test_div_fixed_by_zero() {
        let mut stack = [0i32; 64];
        let mut sp = 0;

        stack[sp] = 10.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 0;
        sp += 1;

        let result = exec_div_fixed(&mut stack, &mut sp);
        assert!(matches!(result, Err(RuntimeError::DivisionByZero)));
    }

    #[test]
    fn test_neg_fixed() {
        let mut stack = [0i32; 64];
        let mut sp = 0;

        stack[sp] = 5.0f32.to_fixed().0;
        sp += 1;

        exec_neg_fixed(&mut stack, &mut sp).unwrap();

        assert_eq!(sp, 1);
        assert_eq!(Fixed(stack[0]).to_f32(), -5.0);
    }

    #[test]
    fn test_min_max_fixed() {
        let mut stack = [0i32; 64];
        let mut sp = 0;

        stack[sp] = 3.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 7.0f32.to_fixed().0;
        sp += 1;

        exec_min_fixed(&mut stack, &mut sp).unwrap();
        assert_eq!(Fixed(stack[0]).to_f32(), 3.0);

        sp = 0;
        stack[sp] = 3.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 7.0f32.to_fixed().0;
        sp += 1;

        exec_max_fixed(&mut stack, &mut sp).unwrap();
        assert_eq!(Fixed(stack[0]).to_f32(), 7.0);
    }

    #[test]
    fn test_abs_fixed() {
        let mut stack = [0i32; 64];
        let mut sp = 0;

        stack[sp] = (-5.0f32).to_fixed().0;
        sp += 1;

        exec_abs_fixed(&mut stack, &mut sp).unwrap();

        assert_eq!(sp, 1);
        assert_eq!(Fixed(stack[0]).to_f32(), 5.0);
    }

    #[test]
    fn test_sqrt_fixed() {
        let mut stack = [0i32; 64];
        let mut sp = 0;

        stack[sp] = 4.0f32.to_fixed().0;
        sp += 1;

        exec_sqrt_fixed(&mut stack, &mut sp).unwrap();

        assert_eq!(sp, 1);
        assert!((Fixed(stack[0]).to_f32() - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_floor_ceil_fixed() {
        let mut stack = [0i32; 64];
        let mut sp = 0;

        stack[sp] = 3.7f32.to_fixed().0;
        sp += 1;

        exec_floor_fixed(&mut stack, &mut sp).unwrap();
        assert_eq!(Fixed(stack[0]).to_f32(), 3.0);

        sp = 0;
        stack[sp] = 3.7f32.to_fixed().0;
        sp += 1;

        exec_ceil_fixed(&mut stack, &mut sp).unwrap();
        assert_eq!(Fixed(stack[0]).to_f32(), 4.0);
    }
}

