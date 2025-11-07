/// Int32 comparison opcodes
use crate::vm::error::RuntimeError;
use crate::vm::vm_stack::Stack;
use crate::math::{Fixed, FIXED_ONE};

/// Execute GreaterEqInt32: pop b, a; push (a >= b ? 1.0 : 0.0)
#[inline(always)]
pub fn exec_greater_eq_int32(stack: &mut Stack) -> Result<(), RuntimeError> {
    let (a, b) = stack.pop2()?;
    let result = if a >= b { FIXED_ONE } else { 0 };
    stack.push_int32(result)?;
    Ok(())
}

/// Execute LessEqInt32: pop b, a; push (a <= b ? 1.0 : 0.0)
#[inline(always)]
pub fn exec_less_eq_int32(stack: &mut Stack) -> Result<(), RuntimeError> {
    let (a, b) = stack.pop2()?;
    let result = if a <= b { FIXED_ONE } else { 0 };
    stack.push_int32(result)?;
    Ok(())
}

/// Execute EqInt32: pop b, a; push (a == b ? 1.0 : 0.0)
#[inline(always)]
pub fn exec_eq_int32(stack: &mut Stack) -> Result<(), RuntimeError> {
    let (a, b) = stack.pop2()?;
    let result = if a == b { FIXED_ONE } else { 0 };
    stack.push_int32(result)?;
    Ok(())
}

/// Execute NotEqInt32: pop b, a; push (a != b ? 1.0 : 0.0)
#[inline(always)]
pub fn exec_not_eq_int32(stack: &mut Stack) -> Result<(), RuntimeError> {
    let (a, b) = stack.pop2()?;
    let result = if a != b { FIXED_ONE } else { 0 };
    stack.push_int32(result)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greater_eq() {
        let mut stack = Stack::new(64);
        stack.push_int32(5).unwrap();
        stack.push_int32(5).unwrap();
        exec_greater_eq_int32(&mut stack).unwrap();
        assert_eq!(stack.pop_int32().unwrap(), FIXED_ONE);
    }

    #[test]
    fn test_less_eq() {
        let mut stack = Stack::new(64);
        stack.push_int32(3).unwrap();
        stack.push_int32(5).unwrap();
        exec_less_eq_int32(&mut stack).unwrap();
        assert_eq!(stack.pop_int32().unwrap(), FIXED_ONE);
    }

    #[test]
    fn test_eq() {
        let mut stack = Stack::new(64);
        stack.push_int32(5).unwrap();
        stack.push_int32(5).unwrap();
        exec_eq_int32(&mut stack).unwrap();
        assert_eq!(stack.pop_int32().unwrap(), FIXED_ONE);
    }

    #[test]
    fn test_not_eq() {
        let mut stack = Stack::new(64);
        stack.push_int32(5).unwrap();
        stack.push_int32(3).unwrap();
        exec_not_eq_int32(&mut stack).unwrap();
        assert_eq!(stack.pop_int32().unwrap(), FIXED_ONE);
    }
}


