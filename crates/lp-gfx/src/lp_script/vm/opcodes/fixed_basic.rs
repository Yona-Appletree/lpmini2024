use lp_math::dec32::trig::{cos, sin};
use lp_math::dec32::{ceil, floor, sqrt, Dec32};

/// Basic dec32-point arithmetic opcodes with error handling
use crate::lp_script::vm::error::LpsVmError;
use crate::lp_script::vm::value_stack::ValueStack;

/// Execute AddDec32: pop b, a; push a + b
#[inline(always)]
pub fn exec_add_dec32(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let (a, b) = stack.pop2()?;
    let result = Dec32(a) + Dec32(b);
    stack.push_dec32(result)?;
    Ok(())
}

/// Execute SubDec32: pop b, a; push a - b
#[inline(always)]
pub fn exec_sub_dec32(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let (a, b) = stack.pop2()?;
    let result = Dec32(a) - Dec32(b);
    stack.push_dec32(result)?;
    Ok(())
}

/// Execute MulDec32: pop b, a; push a * b
#[inline(always)]
pub fn exec_mul_dec32(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let (a, b) = stack.pop2()?;
    let result = Dec32(a) * Dec32(b);
    stack.push_dec32(result)?;
    Ok(())
}

/// Execute DivDec32: pop b, a; push a / b
#[inline(always)]
pub fn exec_div_dec32(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let (a, b) = stack.pop2()?;

    if b == 0 {
        return Err(LpsVmError::DivisionByZero);
    }

    let result = Dec32(a) / Dec32(b);
    stack.push_dec32(result)?;
    Ok(())
}

/// Execute NegDec32: pop a; push -a
#[inline(always)]
pub fn exec_neg_dec32(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let a = stack.pop_dec32()?;
    stack.push_dec32(-a)?;
    Ok(())
}

/// Execute AbsDec32: pop a; push abs(a)
#[inline(always)]
pub fn exec_abs_dec32(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let a = stack.pop_dec32()?;
    stack.push_dec32(a.abs())?;
    Ok(())
}

/// Execute MinDec32: pop b, a; push min(a, b)
#[inline(always)]
pub fn exec_min_dec32(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let (a, b) = stack.pop2()?;
    let result = Dec32(a).min(Dec32(b));
    stack.push_dec32(result)?;
    Ok(())
}

/// Execute MaxDec32: pop b, a; push max(a, b)
#[inline(always)]
pub fn exec_max_dec32(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let (a, b) = stack.pop2()?;
    let result = Dec32(a).max(Dec32(b));
    stack.push_dec32(result)?;
    Ok(())
}

/// Execute SinDec32: pop a; push sin(a)
#[inline(always)]
pub fn exec_sin_dec32(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let a = stack.pop_dec32()?;
    stack.push_dec32(sin(a))?;
    Ok(())
}

/// Execute CosDec32: pop a; push cos(a)
#[inline(always)]
pub fn exec_cos_dec32(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let a = stack.pop_dec32()?;
    stack.push_dec32(cos(a))?;
    Ok(())
}

/// Execute SqrtDec32: pop a; push sqrt(a)
#[inline(always)]
pub fn exec_sqrt_dec32(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let a = stack.pop_dec32()?;
    stack.push_dec32(sqrt(a))?;
    Ok(())
}

/// Execute FloorDec32: pop a; push floor(a)
#[inline(always)]
pub fn exec_floor_dec32(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let a = stack.pop_dec32()?;
    stack.push_dec32(floor(a))?;
    Ok(())
}

/// Execute CeilDec32: pop a; push ceil(a)
#[inline(always)]
pub fn exec_ceil_dec32(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let a = stack.pop_dec32()?;
    stack.push_dec32(ceil(a))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use lp_math::dec32::ToDec32;

    use super::*;

    #[test]
    fn test_add() {
        let mut stack = ValueStack::new(64);

        stack.push_dec32(2.0.to_dec32()).unwrap();
        stack.push_dec32(3.0.to_dec32()).unwrap();

        exec_add_dec32(&mut stack).unwrap();

        assert_eq!(stack.sp(), 1);
        assert_eq!(stack.pop_dec32().unwrap().to_f32(), 5.0);
    }

    #[test]
    fn test_sub() {
        let mut stack = ValueStack::new(64);

        stack.push_dec32(5.0.to_dec32()).unwrap();
        stack.push_dec32(3.0.to_dec32()).unwrap();

        exec_sub_dec32(&mut stack).unwrap();

        assert_eq!(stack.sp(), 1);
        assert_eq!(stack.pop_dec32().unwrap().to_f32(), 2.0);
    }

    #[test]
    fn test_mul() {
        let mut stack = ValueStack::new(64);

        stack.push_dec32(4.0.to_dec32()).unwrap();
        stack.push_dec32(3.0.to_dec32()).unwrap();

        exec_mul_dec32(&mut stack).unwrap();

        assert_eq!(stack.sp(), 1);
        assert_eq!(stack.pop_dec32().unwrap().to_f32(), 12.0);
    }

    #[test]
    fn test_div() {
        let mut stack = ValueStack::new(64);

        stack.push_dec32(12.0.to_dec32()).unwrap();
        stack.push_dec32(4.0.to_dec32()).unwrap();

        exec_div_dec32(&mut stack).unwrap();

        assert_eq!(stack.sp(), 1);
        assert_eq!(stack.pop_dec32().unwrap().to_f32(), 3.0);
    }

    #[test]
    fn test_div_by_zero() {
        let mut stack = ValueStack::new(64);

        stack.push_dec32(5.0.to_dec32()).unwrap();
        stack.push_dec32(0.0.to_dec32()).unwrap();

        let result = exec_div_dec32(&mut stack);
        assert!(matches!(result, Err(LpsVmError::DivisionByZero)));
    }

    #[test]
    fn test_neg() {
        let mut stack = ValueStack::new(64);

        stack.push_dec32(5.0.to_dec32()).unwrap();

        exec_neg_dec32(&mut stack).unwrap();

        assert_eq!(stack.sp(), 1);
        assert_eq!(stack.pop_dec32().unwrap().to_f32(), -5.0);
    }

    #[test]
    fn test_abs() {
        let mut stack = ValueStack::new(64);

        stack.push_dec32((-5.0).to_dec32()).unwrap();

        exec_abs_dec32(&mut stack).unwrap();

        assert_eq!(stack.sp(), 1);
        assert_eq!(stack.pop_dec32().unwrap().to_f32(), 5.0);
    }

    #[test]
    fn test_min() {
        let mut stack = ValueStack::new(64);

        stack.push_dec32(5.0.to_dec32()).unwrap();
        stack.push_dec32(3.0.to_dec32()).unwrap();

        exec_min_dec32(&mut stack).unwrap();

        assert_eq!(stack.sp(), 1);
        assert_eq!(stack.pop_dec32().unwrap().to_f32(), 3.0);
    }

    #[test]
    fn test_max() {
        let mut stack = ValueStack::new(64);

        stack.push_dec32(5.0.to_dec32()).unwrap();
        stack.push_dec32(3.0.to_dec32()).unwrap();

        exec_max_dec32(&mut stack).unwrap();

        assert_eq!(stack.sp(), 1);
        assert_eq!(stack.pop_dec32().unwrap().to_f32(), 5.0);
    }
}
