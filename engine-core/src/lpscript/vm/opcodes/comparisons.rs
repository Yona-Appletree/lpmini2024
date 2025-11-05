/// Comparison opcodes for Fixed-point values
///
/// These return FIXED_ONE (1.0) for true, 0 for false to match GLSL semantics
use crate::lpscript::vm::error::LpsVmError;
use crate::lpscript::vm::value_stack::ValueStack;
use crate::math::{Fixed, FIXED_ONE};

/// Execute GreaterFixed: pop b, a; push (a > b ? 1.0 : 0.0)
#[inline(always)]
pub fn exec_greater_fixed(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let (a, b) = stack.pop2()?;
    let result = if Fixed(a) > Fixed(b) { FIXED_ONE } else { 0 };
    stack.push_int32(result)?;
    Ok(())
}

/// Execute LessFixed: pop b, a; push (a < b ? 1.0 : 0.0)
#[inline(always)]
pub fn exec_less_fixed(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let (a, b) = stack.pop2()?;
    let result = if Fixed(a) < Fixed(b) { FIXED_ONE } else { 0 };
    stack.push_int32(result)?;
    Ok(())
}

/// Execute GreaterEqFixed: pop b, a; push (a >= b ? 1.0 : 0.0)
#[inline(always)]
pub fn exec_greater_eq_fixed(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let (a, b) = stack.pop2()?;
    let result = if Fixed(a) >= Fixed(b) { FIXED_ONE } else { 0 };
    stack.push_int32(result)?;
    Ok(())
}

/// Execute LessEqFixed: pop b, a; push (a <= b ? 1.0 : 0.0)
#[inline(always)]
pub fn exec_less_eq_fixed(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let (a, b) = stack.pop2()?;
    let result = if Fixed(a) <= Fixed(b) { FIXED_ONE } else { 0 };
    stack.push_int32(result)?;
    Ok(())
}

/// Execute EqFixed: pop b, a; push (a == b ? 1.0 : 0.0)
#[inline(always)]
pub fn exec_eq_fixed(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let (a, b) = stack.pop2()?;
    let result = if a == b { FIXED_ONE } else { 0 };
    stack.push_int32(result)?;
    Ok(())
}

/// Execute NotEqFixed: pop b, a; push (a != b ? 1.0 : 0.0)
#[inline(always)]
pub fn exec_not_eq_fixed(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let (a, b) = stack.pop2()?;
    let result = if a != b { FIXED_ONE } else { 0 };
    stack.push_int32(result)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::ToFixed;

    #[test]
    fn test_greater() {
        let mut stack = ValueStack::new(64);
        stack.push_fixed(5.0.to_fixed()).unwrap();
        stack.push_fixed(3.0.to_fixed()).unwrap();
        exec_greater_fixed(&mut stack).unwrap();
        assert_eq!(stack.pop_int32().unwrap(), FIXED_ONE);
    }

    #[test]
    fn test_less() {
        let mut stack = ValueStack::new(64);
        stack.push_fixed(3.0.to_fixed()).unwrap();
        stack.push_fixed(5.0.to_fixed()).unwrap();
        exec_less_fixed(&mut stack).unwrap();
        assert_eq!(stack.pop_int32().unwrap(), FIXED_ONE);
    }

    #[test]
    fn test_eq() {
        let mut stack = ValueStack::new(64);
        stack.push_fixed(5.0.to_fixed()).unwrap();
        stack.push_fixed(5.0.to_fixed()).unwrap();
        exec_eq_fixed(&mut stack).unwrap();
        assert_eq!(stack.pop_int32().unwrap(), FIXED_ONE);
    }

    #[test]
    fn test_not_eq() {
        let mut stack = ValueStack::new(64);
        stack.push_fixed(5.0.to_fixed()).unwrap();
        stack.push_fixed(3.0.to_fixed()).unwrap();
        exec_not_eq_fixed(&mut stack).unwrap();
        assert_eq!(stack.pop_int32().unwrap(), FIXED_ONE);
    }
}

