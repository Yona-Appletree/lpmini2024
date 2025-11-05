/// Fixed-point logical operations (boolean logic on Fixed values)
use crate::lpscript::vm::error::RuntimeError;
use crate::lpscript::vm::vm_stack::Stack;
use crate::math::Fixed;

/// Execute AndFixed: pop b, a; push (a && b) as Fixed
#[inline(always)]
pub fn exec_and_fixed(stack: &mut Stack) -> Result<(), RuntimeError> {
    let (a, b) = stack.pop2()?;
    let result = if a != 0 && b != 0 {
        Fixed::ONE
    } else {
        Fixed::ZERO
    };
    stack.push_fixed(result)?;
    Ok(())
}

/// Execute OrFixed: pop b, a; push (a || b) as Fixed
#[inline(always)]
pub fn exec_or_fixed(stack: &mut Stack) -> Result<(), RuntimeError> {
    let (a, b) = stack.pop2()?;
    let result = if a != 0 || b != 0 {
        Fixed::ONE
    } else {
        Fixed::ZERO
    };
    stack.push_fixed(result)?;
    Ok(())
}

/// Execute NotFixed: pop a; push (!a) as Fixed
#[inline(always)]
pub fn exec_not_fixed(stack: &mut Stack) -> Result<(), RuntimeError> {
    let a = stack.pop_int32()?;
    let result = if a == 0 { Fixed::ONE } else { Fixed::ZERO };
    stack.push_fixed(result)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::ToFixed;

    #[test]
    fn test_and_true() {
        let mut stack = Stack::new(64);
        stack.push_fixed(1.0.to_fixed()).unwrap();
        stack.push_fixed(1.0.to_fixed()).unwrap();
        exec_and_fixed(&mut stack).unwrap();
        assert_eq!(stack.pop_fixed().unwrap(), Fixed::ONE);
    }

    #[test]
    fn test_and_false() {
        let mut stack = Stack::new(64);
        stack.push_fixed(1.0.to_fixed()).unwrap();
        stack.push_fixed(0.0.to_fixed()).unwrap();
        exec_and_fixed(&mut stack).unwrap();
        assert_eq!(stack.pop_fixed().unwrap(), Fixed::ZERO);
    }

    #[test]
    fn test_or_true() {
        let mut stack = Stack::new(64);
        stack.push_fixed(1.0.to_fixed()).unwrap();
        stack.push_fixed(0.0.to_fixed()).unwrap();
        exec_or_fixed(&mut stack).unwrap();
        assert_eq!(stack.pop_fixed().unwrap(), Fixed::ONE);
    }

    #[test]
    fn test_or_false() {
        let mut stack = Stack::new(64);
        stack.push_fixed(0.0.to_fixed()).unwrap();
        stack.push_fixed(0.0.to_fixed()).unwrap();
        exec_or_fixed(&mut stack).unwrap();
        assert_eq!(stack.pop_fixed().unwrap(), Fixed::ZERO);
    }

    #[test]
    fn test_not_true() {
        let mut stack = Stack::new(64);
        stack.push_fixed(0.0.to_fixed()).unwrap();
        exec_not_fixed(&mut stack).unwrap();
        assert_eq!(stack.pop_fixed().unwrap(), Fixed::ONE);
    }

    #[test]
    fn test_not_false() {
        let mut stack = Stack::new(64);
        stack.push_fixed(1.0.to_fixed()).unwrap();
        exec_not_fixed(&mut stack).unwrap();
        assert_eq!(stack.pop_fixed().unwrap(), Fixed::ZERO);
    }
}
