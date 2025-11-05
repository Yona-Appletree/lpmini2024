/// Basic fixed-point arithmetic opcodes with error handling
use crate::lpscript::vm::error::RuntimeError;
use crate::lpscript::vm::vm_stack::Stack;
use crate::math::trig::{cos, sin};
use crate::math::Fixed;
use crate::math::{ceil, floor, sqrt};

/// Execute AddFixed: pop b, a; push a + b
#[inline(always)]
pub fn exec_add_fixed(stack: &mut Stack) -> Result<(), RuntimeError> {
    let (a, b) = stack.pop2()?;
    let result = Fixed(a) + Fixed(b);
    stack.push_fixed(result)?;
    Ok(())
}

/// Execute SubFixed: pop b, a; push a - b
#[inline(always)]
pub fn exec_sub_fixed(stack: &mut Stack) -> Result<(), RuntimeError> {
    let (a, b) = stack.pop2()?;
    let result = Fixed(a) - Fixed(b);
    stack.push_fixed(result)?;
    Ok(())
}

/// Execute MulFixed: pop b, a; push a * b
#[inline(always)]
pub fn exec_mul_fixed(stack: &mut Stack) -> Result<(), RuntimeError> {
    let (a, b) = stack.pop2()?;
    let result = Fixed(a) * Fixed(b);
    stack.push_fixed(result)?;
    Ok(())
}

/// Execute DivFixed: pop b, a; push a / b
#[inline(always)]
pub fn exec_div_fixed(stack: &mut Stack) -> Result<(), RuntimeError> {
    let (a, b) = stack.pop2()?;

    if b == 0 {
        return Err(RuntimeError::DivisionByZero);
    }

    let result = Fixed(a) / Fixed(b);
    stack.push_fixed(result)?;
    Ok(())
}

/// Execute NegFixed: pop a; push -a
#[inline(always)]
pub fn exec_neg_fixed(stack: &mut Stack) -> Result<(), RuntimeError> {
    let a = stack.pop_fixed()?;
    stack.push_fixed(-a)?;
    Ok(())
}

/// Execute AbsFixed: pop a; push abs(a)
#[inline(always)]
pub fn exec_abs_fixed(stack: &mut Stack) -> Result<(), RuntimeError> {
    let a = stack.pop_fixed()?;
    stack.push_fixed(a.abs())?;
    Ok(())
}

/// Execute MinFixed: pop b, a; push min(a, b)
#[inline(always)]
pub fn exec_min_fixed(stack: &mut Stack) -> Result<(), RuntimeError> {
    let (a, b) = stack.pop2()?;
    let result = Fixed(a).min(Fixed(b));
    stack.push_fixed(result)?;
    Ok(())
}

/// Execute MaxFixed: pop b, a; push max(a, b)
#[inline(always)]
pub fn exec_max_fixed(stack: &mut Stack) -> Result<(), RuntimeError> {
    let (a, b) = stack.pop2()?;
    let result = Fixed(a).max(Fixed(b));
    stack.push_fixed(result)?;
    Ok(())
}

/// Execute SinFixed: pop a; push sin(a)
#[inline(always)]
pub fn exec_sin_fixed(stack: &mut Stack) -> Result<(), RuntimeError> {
    let a = stack.pop_fixed()?;
    stack.push_fixed(sin(a))?;
    Ok(())
}

/// Execute CosFixed: pop a; push cos(a)
#[inline(always)]
pub fn exec_cos_fixed(stack: &mut Stack) -> Result<(), RuntimeError> {
    let a = stack.pop_fixed()?;
    stack.push_fixed(cos(a))?;
    Ok(())
}

/// Execute SqrtFixed: pop a; push sqrt(a)
#[inline(always)]
pub fn exec_sqrt_fixed(stack: &mut Stack) -> Result<(), RuntimeError> {
    let a = stack.pop_fixed()?;
    stack.push_fixed(sqrt(a))?;
    Ok(())
}

/// Execute FloorFixed: pop a; push floor(a)
#[inline(always)]
pub fn exec_floor_fixed(stack: &mut Stack) -> Result<(), RuntimeError> {
    let a = stack.pop_fixed()?;
    stack.push_fixed(floor(a))?;
    Ok(())
}

/// Execute CeilFixed: pop a; push ceil(a)
#[inline(always)]
pub fn exec_ceil_fixed(stack: &mut Stack) -> Result<(), RuntimeError> {
    let a = stack.pop_fixed()?;
    stack.push_fixed(ceil(a))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::ToFixed;

    #[test]
    fn test_add() {
        let mut stack = Stack::new(64);

        stack.push_fixed(2.0.to_fixed()).unwrap();
        stack.push_fixed(3.0.to_fixed()).unwrap();

        exec_add_fixed(&mut stack).unwrap();

        assert_eq!(stack.sp(), 1);
        assert_eq!(stack.pop_fixed().unwrap().to_f32(), 5.0);
    }

    #[test]
    fn test_sub() {
        let mut stack = Stack::new(64);

        stack.push_fixed(5.0.to_fixed()).unwrap();
        stack.push_fixed(3.0.to_fixed()).unwrap();

        exec_sub_fixed(&mut stack).unwrap();

        assert_eq!(stack.sp(), 1);
        assert_eq!(stack.pop_fixed().unwrap().to_f32(), 2.0);
    }

    #[test]
    fn test_mul() {
        let mut stack = Stack::new(64);

        stack.push_fixed(4.0.to_fixed()).unwrap();
        stack.push_fixed(3.0.to_fixed()).unwrap();

        exec_mul_fixed(&mut stack).unwrap();

        assert_eq!(stack.sp(), 1);
        assert_eq!(stack.pop_fixed().unwrap().to_f32(), 12.0);
    }

    #[test]
    fn test_div() {
        let mut stack = Stack::new(64);

        stack.push_fixed(12.0.to_fixed()).unwrap();
        stack.push_fixed(4.0.to_fixed()).unwrap();

        exec_div_fixed(&mut stack).unwrap();

        assert_eq!(stack.sp(), 1);
        assert_eq!(stack.pop_fixed().unwrap().to_f32(), 3.0);
    }

    #[test]
    fn test_div_by_zero() {
        let mut stack = Stack::new(64);

        stack.push_fixed(5.0.to_fixed()).unwrap();
        stack.push_fixed(0.0.to_fixed()).unwrap();

        let result = exec_div_fixed(&mut stack);
        assert!(matches!(result, Err(RuntimeError::DivisionByZero)));
    }

    #[test]
    fn test_neg() {
        let mut stack = Stack::new(64);

        stack.push_fixed(5.0.to_fixed()).unwrap();

        exec_neg_fixed(&mut stack).unwrap();

        assert_eq!(stack.sp(), 1);
        assert_eq!(stack.pop_fixed().unwrap().to_f32(), -5.0);
    }

    #[test]
    fn test_abs() {
        let mut stack = Stack::new(64);

        stack.push_fixed((-5.0).to_fixed()).unwrap();

        exec_abs_fixed(&mut stack).unwrap();

        assert_eq!(stack.sp(), 1);
        assert_eq!(stack.pop_fixed().unwrap().to_f32(), 5.0);
    }

    #[test]
    fn test_min() {
        let mut stack = Stack::new(64);

        stack.push_fixed(5.0.to_fixed()).unwrap();
        stack.push_fixed(3.0.to_fixed()).unwrap();

        exec_min_fixed(&mut stack).unwrap();

        assert_eq!(stack.sp(), 1);
        assert_eq!(stack.pop_fixed().unwrap().to_f32(), 3.0);
    }

    #[test]
    fn test_max() {
        let mut stack = Stack::new(64);

        stack.push_fixed(5.0.to_fixed()).unwrap();
        stack.push_fixed(3.0.to_fixed()).unwrap();

        exec_max_fixed(&mut stack).unwrap();

        assert_eq!(stack.sp(), 1);
        assert_eq!(stack.pop_fixed().unwrap().to_f32(), 5.0);
    }
}
