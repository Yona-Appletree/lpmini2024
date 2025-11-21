use crate::dec32::Dec32;
/// Dec32-point logical operations (boolean logic on Dec32 values)
use crate::vm::error::LpsVmError;
use crate::vm::value_stack::ValueStack;

/// Execute AndDec32: pop b, a; push (a && b) as Dec32
#[inline(always)]
pub fn exec_and_dec32(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let (a, b) = stack.pop2()?;
    let result = if a != 0 && b != 0 {
        Dec32::ONE
    } else {
        Dec32::ZERO
    };
    stack.push_dec32(result)?;
    Ok(())
}

/// Execute OrDec32: pop b, a; push (a || b) as Dec32
#[inline(always)]
pub fn exec_or_dec32(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let (a, b) = stack.pop2()?;
    let result = if a != 0 || b != 0 {
        Dec32::ONE
    } else {
        Dec32::ZERO
    };
    stack.push_dec32(result)?;
    Ok(())
}

/// Execute NotDec32: pop a; push (!a) as Dec32
#[inline(always)]
pub fn exec_not_dec32(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let a = stack.pop_int32()?;
    let result = if a == 0 { Dec32::ONE } else { Dec32::ZERO };
    stack.push_dec32(result)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dec32::ToDec32;

    #[test]
    fn test_and_true() {
        let mut stack = ValueStack::new(64);
        stack.push_dec32(1.0.to_dec32()).unwrap();
        stack.push_dec32(1.0.to_dec32()).unwrap();
        exec_and_dec32(&mut stack).unwrap();
        assert_eq!(stack.pop_dec32().unwrap(), Dec32::ONE);
    }

    #[test]
    fn test_and_false() {
        let mut stack = ValueStack::new(64);
        stack.push_dec32(1.0.to_dec32()).unwrap();
        stack.push_dec32(0.0.to_dec32()).unwrap();
        exec_and_dec32(&mut stack).unwrap();
        assert_eq!(stack.pop_dec32().unwrap(), Dec32::ZERO);
    }

    #[test]
    fn test_or_true() {
        let mut stack = ValueStack::new(64);
        stack.push_dec32(1.0.to_dec32()).unwrap();
        stack.push_dec32(0.0.to_dec32()).unwrap();
        exec_or_dec32(&mut stack).unwrap();
        assert_eq!(stack.pop_dec32().unwrap(), Dec32::ONE);
    }

    #[test]
    fn test_or_false() {
        let mut stack = ValueStack::new(64);
        stack.push_dec32(0.0.to_dec32()).unwrap();
        stack.push_dec32(0.0.to_dec32()).unwrap();
        exec_or_dec32(&mut stack).unwrap();
        assert_eq!(stack.pop_dec32().unwrap(), Dec32::ZERO);
    }

    #[test]
    fn test_not_true() {
        let mut stack = ValueStack::new(64);
        stack.push_dec32(0.0.to_dec32()).unwrap();
        exec_not_dec32(&mut stack).unwrap();
        assert_eq!(stack.pop_dec32().unwrap(), Dec32::ONE);
    }

    #[test]
    fn test_not_false() {
        let mut stack = ValueStack::new(64);
        stack.push_dec32(1.0.to_dec32()).unwrap();
        exec_not_dec32(&mut stack).unwrap();
        assert_eq!(stack.pop_dec32().unwrap(), Dec32::ZERO);
    }
}
