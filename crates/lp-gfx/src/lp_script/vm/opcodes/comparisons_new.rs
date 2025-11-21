/// Comparison opcodes for Dec32-point values
///
/// These return Dec32::ONE.0 (1.0) for true, 0 for false to match GLSL semantics
use crate::lp_script::vm::error::RuntimeError;
use crate::lp_script::vm::vm_stack::Stack;
use crate::math::Dec32;

/// Execute GreaterDec32: pop b, a; push (a > b ? 1.0 : 0.0)
#[inline(always)]
pub fn exec_greater_dec32(stack: &mut Stack) -> Result<(), RuntimeError> {
    let (a, b) = stack.pop2()?;
    let result = if Dec32(a) > Dec32(b) { Dec32::ONE.0 } else { 0 };
    stack.push_int32(result)?;
    Ok(())
}

/// Execute LessDec32: pop b, a; push (a < b ? 1.0 : 0.0)
#[inline(always)]
pub fn exec_less_dec32(stack: &mut Stack) -> Result<(), RuntimeError> {
    let (a, b) = stack.pop2()?;
    let result = if Dec32(a) < Dec32(b) { Dec32::ONE.0 } else { 0 };
    stack.push_int32(result)?;
    Ok(())
}

/// Execute GreaterEqDec32: pop b, a; push (a >= b ? 1.0 : 0.0)
#[inline(always)]
pub fn exec_greater_eq_dec32(stack: &mut Stack) -> Result<(), RuntimeError> {
    let (a, b) = stack.pop2()?;
    let result = if Dec32(a) >= Dec32(b) {
        Dec32::ONE.0
    } else {
        0
    };
    stack.push_int32(result)?;
    Ok(())
}

/// Execute LessEqDec32: pop b, a; push (a <= b ? 1.0 : 0.0)
#[inline(always)]
pub fn exec_less_eq_dec32(stack: &mut Stack) -> Result<(), RuntimeError> {
    let (a, b) = stack.pop2()?;
    let result = if Dec32(a) <= Dec32(b) {
        Dec32::ONE.0
    } else {
        0
    };
    stack.push_int32(result)?;
    Ok(())
}

/// Execute EqDec32: pop b, a; push (a == b ? 1.0 : 0.0)
#[inline(always)]
pub fn exec_eq_dec32(stack: &mut Stack) -> Result<(), RuntimeError> {
    let (a, b) = stack.pop2()?;
    let result = if a == b { Dec32::ONE.0 } else { 0 };
    stack.push_int32(result)?;
    Ok(())
}

/// Execute NotEqDec32: pop b, a; push (a != b ? 1.0 : 0.0)
#[inline(always)]
pub fn exec_not_eq_dec32(stack: &mut Stack) -> Result<(), RuntimeError> {
    let (a, b) = stack.pop2()?;
    let result = if a != b { Dec32::ONE.0 } else { 0 };
    stack.push_int32(result)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::ToDec32;

    #[test]
    fn test_greater() {
        let mut stack = Stack::new(64);
        stack.push_dec32(5.0.to_dec32()).unwrap();
        stack.push_dec32(3.0.to_dec32()).unwrap();
        exec_greater_dec32(&mut stack).unwrap();
        assert_eq!(stack.pop_int32().unwrap(), Dec32::ONE.0);
    }

    #[test]
    fn test_less() {
        let mut stack = Stack::new(64);
        stack.push_dec32(3.0.to_dec32()).unwrap();
        stack.push_dec32(5.0.to_dec32()).unwrap();
        exec_less_dec32(&mut stack).unwrap();
        assert_eq!(stack.pop_int32().unwrap(), Dec32::ONE.0);
    }

    #[test]
    fn test_eq() {
        let mut stack = Stack::new(64);
        stack.push_dec32(5.0.to_dec32()).unwrap();
        stack.push_dec32(5.0.to_dec32()).unwrap();
        exec_eq_dec32(&mut stack).unwrap();
        assert_eq!(stack.pop_int32().unwrap(), Dec32::ONE.0);
    }

    #[test]
    fn test_not_eq() {
        let mut stack = Stack::new(64);
        stack.push_dec32(5.0.to_dec32()).unwrap();
        stack.push_dec32(3.0.to_dec32()).unwrap();
        exec_not_eq_dec32(&mut stack).unwrap();
        assert_eq!(stack.pop_int32().unwrap(), Dec32::ONE.0);
    }
}
