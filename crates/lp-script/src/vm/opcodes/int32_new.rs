use crate::math::Dec32;
/// Int32 arithmetic and bitwise operations
use crate::vm::error::RuntimeError;
use crate::vm::vm_stack::Stack;

// === Arithmetic Operations ===

/// Execute AddInt32: pop b, a; push a + b
#[inline(always)]
pub fn exec_add_int32(stack: &mut Stack) -> Result<(), RuntimeError> {
    let (a, b) = stack.pop2()?;
    stack.push_int32(a.wrapping_add(b))?;
    Ok(())
}

/// Execute SubInt32: pop b, a; push a - b
#[inline(always)]
pub fn exec_sub_int32(stack: &mut Stack) -> Result<(), RuntimeError> {
    let (a, b) = stack.pop2()?;
    stack.push_int32(a.wrapping_sub(b))?;
    Ok(())
}

/// Execute MulInt32: pop b, a; push a * b
#[inline(always)]
pub fn exec_mul_int32(stack: &mut Stack) -> Result<(), RuntimeError> {
    let (a, b) = stack.pop2()?;
    stack.push_int32(a.wrapping_mul(b))?;
    Ok(())
}

/// Execute DivInt32: pop b, a; push a / b
#[inline(always)]
pub fn exec_div_int32(stack: &mut Stack) -> Result<(), RuntimeError> {
    let (a, b) = stack.pop2()?;

    if b == 0 {
        return Err(RuntimeError::DivisionByZero);
    }

    stack.push_int32(a / b)?;
    Ok(())
}

/// Execute ModInt32: pop b, a; push a % b
#[inline(always)]
pub fn exec_mod_int32(stack: &mut Stack) -> Result<(), RuntimeError> {
    let (a, b) = stack.pop2()?;

    if b == 0 {
        return Err(RuntimeError::DivisionByZero);
    }

    stack.push_int32(a % b)?;
    Ok(())
}

/// Execute NegInt32: pop a; push -a
#[inline(always)]
pub fn exec_neg_int32(stack: &mut Stack) -> Result<(), RuntimeError> {
    let a = stack.pop_int32()?;
    stack.push_int32(-a)?;
    Ok(())
}

/// Execute AbsInt32: pop a; push abs(a)
#[inline(always)]
pub fn exec_abs_int32(stack: &mut Stack) -> Result<(), RuntimeError> {
    let a = stack.pop_int32()?;
    stack.push_int32(a.abs())?;
    Ok(())
}

/// Execute MinInt32: pop b, a; push min(a, b)
#[inline(always)]
pub fn exec_min_int32(stack: &mut Stack) -> Result<(), RuntimeError> {
    let (a, b) = stack.pop2()?;
    stack.push_int32(a.min(b))?;
    Ok(())
}

/// Execute MaxInt32: pop b, a; push max(a, b)
#[inline(always)]
pub fn exec_max_int32(stack: &mut Stack) -> Result<(), RuntimeError> {
    let (a, b) = stack.pop2()?;
    stack.push_int32(a.max(b))?;
    Ok(())
}

// === Comparison Operations ===

/// Execute GreaterInt32: pop b, a; push (a > b ? 1.0 : 0.0)
#[inline(always)]
pub fn exec_greater_int32(stack: &mut Stack) -> Result<(), RuntimeError> {
    let (a, b) = stack.pop2()?;
    let result = if a > b { Dec32::ONE.0 } else { 0 };
    stack.push_int32(result)?;
    Ok(())
}

/// Execute LessInt32: pop b, a; push (a < b ? 1.0 : 0.0)
#[inline(always)]
pub fn exec_less_int32(stack: &mut Stack) -> Result<(), RuntimeError> {
    let (a, b) = stack.pop2()?;
    let result = if a < b { Dec32::ONE.0 } else { 0 };
    stack.push_int32(result)?;
    Ok(())
}

// === Bitwise Operations ===

/// Execute BitwiseAndInt32: pop b, a; push a & b
#[inline(always)]
pub fn exec_bitwise_and_int32(stack: &mut Stack) -> Result<(), RuntimeError> {
    let (a, b) = stack.pop2()?;
    stack.push_int32(a & b)?;
    Ok(())
}

/// Execute BitwiseOrInt32: pop b, a; push a | b
#[inline(always)]
pub fn exec_bitwise_or_int32(stack: &mut Stack) -> Result<(), RuntimeError> {
    let (a, b) = stack.pop2()?;
    stack.push_int32(a | b)?;
    Ok(())
}

/// Execute BitwiseXorInt32: pop b, a; push a ^ b
#[inline(always)]
pub fn exec_bitwise_xor_int32(stack: &mut Stack) -> Result<(), RuntimeError> {
    let (a, b) = stack.pop2()?;
    stack.push_int32(a ^ b)?;
    Ok(())
}

/// Execute BitwiseNotInt32: pop a; push !a
#[inline(always)]
pub fn exec_bitwise_not_int32(stack: &mut Stack) -> Result<(), RuntimeError> {
    let a = stack.pop_int32()?;
    stack.push_int32(!a)?;
    Ok(())
}

/// Execute LeftShiftInt32: pop b, a; push a << b
#[inline(always)]
pub fn exec_left_shift_int32(stack: &mut Stack) -> Result<(), RuntimeError> {
    let (a, b) = stack.pop2()?;
    // Clamp shift amount to prevent overflow
    let shift = (b as u32) & 0x1F; // Limit to 0-31
    stack.push_int32(a << shift)?;
    Ok(())
}

/// Execute RightShiftInt32: pop b, a; push a >> b (arithmetic shift)
#[inline(always)]
pub fn exec_right_shift_int32(stack: &mut Stack) -> Result<(), RuntimeError> {
    let (a, b) = stack.pop2()?;
    // Clamp shift amount to prevent overflow
    let shift = (b as u32) & 0x1F; // Limit to 0-31
    stack.push_int32(a >> shift)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut stack = Stack::new(64);
        stack.push_int32(5).unwrap();
        stack.push_int32(3).unwrap();
        exec_add_int32(&mut stack).unwrap();
        assert_eq!(stack.pop_int32().unwrap(), 8);
    }

    #[test]
    fn test_sub() {
        let mut stack = Stack::new(64);
        stack.push_int32(10).unwrap();
        stack.push_int32(3).unwrap();
        exec_sub_int32(&mut stack).unwrap();
        assert_eq!(stack.pop_int32().unwrap(), 7);
    }

    #[test]
    fn test_mul() {
        let mut stack = Stack::new(64);
        stack.push_int32(4).unwrap();
        stack.push_int32(3).unwrap();
        exec_mul_int32(&mut stack).unwrap();
        assert_eq!(stack.pop_int32().unwrap(), 12);
    }

    #[test]
    fn test_div() {
        let mut stack = Stack::new(64);
        stack.push_int32(15).unwrap();
        stack.push_int32(3).unwrap();
        exec_div_int32(&mut stack).unwrap();
        assert_eq!(stack.pop_int32().unwrap(), 5);
    }

    #[test]
    fn test_div_by_zero() {
        let mut stack = Stack::new(64);
        stack.push_int32(10).unwrap();
        stack.push_int32(0).unwrap();
        let result = exec_div_int32(&mut stack);
        assert!(matches!(result, Err(RuntimeError::DivisionByZero)));
    }

    #[test]
    fn test_mod() {
        let mut stack = Stack::new(64);
        stack.push_int32(10).unwrap();
        stack.push_int32(3).unwrap();
        exec_mod_int32(&mut stack).unwrap();
        assert_eq!(stack.pop_int32().unwrap(), 1);
    }

    #[test]
    fn test_neg() {
        let mut stack = Stack::new(64);
        stack.push_int32(5).unwrap();
        exec_neg_int32(&mut stack).unwrap();
        assert_eq!(stack.pop_int32().unwrap(), -5);
    }

    #[test]
    fn test_abs() {
        let mut stack = Stack::new(64);
        stack.push_int32(-7).unwrap();
        exec_abs_int32(&mut stack).unwrap();
        assert_eq!(stack.pop_int32().unwrap(), 7);
    }

    #[test]
    fn test_min() {
        let mut stack = Stack::new(64);
        stack.push_int32(5).unwrap();
        stack.push_int32(3).unwrap();
        exec_min_int32(&mut stack).unwrap();
        assert_eq!(stack.pop_int32().unwrap(), 3);
    }

    #[test]
    fn test_max() {
        let mut stack = Stack::new(64);
        stack.push_int32(5).unwrap();
        stack.push_int32(3).unwrap();
        exec_max_int32(&mut stack).unwrap();
        assert_eq!(stack.pop_int32().unwrap(), 5);
    }

    #[test]
    fn test_greater() {
        let mut stack = Stack::new(64);
        stack.push_int32(5).unwrap();
        stack.push_int32(3).unwrap();
        exec_greater_int32(&mut stack).unwrap();
        assert_eq!(stack.pop_int32().unwrap(), Dec32::ONE.0);
    }

    #[test]
    fn test_less() {
        let mut stack = Stack::new(64);
        stack.push_int32(3).unwrap();
        stack.push_int32(5).unwrap();
        exec_less_int32(&mut stack).unwrap();
        assert_eq!(stack.pop_int32().unwrap(), Dec32::ONE.0);
    }

    #[test]
    fn test_bitwise_and() {
        let mut stack = Stack::new(64);
        stack.push_int32(0b1100).unwrap();
        stack.push_int32(0b1010).unwrap();
        exec_bitwise_and_int32(&mut stack).unwrap();
        assert_eq!(stack.pop_int32().unwrap(), 0b1000);
    }

    #[test]
    fn test_bitwise_or() {
        let mut stack = Stack::new(64);
        stack.push_int32(0b1100).unwrap();
        stack.push_int32(0b1010).unwrap();
        exec_bitwise_or_int32(&mut stack).unwrap();
        assert_eq!(stack.pop_int32().unwrap(), 0b1110);
    }

    #[test]
    fn test_bitwise_xor() {
        let mut stack = Stack::new(64);
        stack.push_int32(0b1100).unwrap();
        stack.push_int32(0b1010).unwrap();
        exec_bitwise_xor_int32(&mut stack).unwrap();
        assert_eq!(stack.pop_int32().unwrap(), 0b0110);
    }

    #[test]
    fn test_bitwise_not() {
        let mut stack = Stack::new(64);
        stack.push_int32(0).unwrap();
        exec_bitwise_not_int32(&mut stack).unwrap();
        assert_eq!(stack.pop_int32().unwrap(), -1);
    }

    #[test]
    fn test_left_shift() {
        let mut stack = Stack::new(64);
        stack.push_int32(5).unwrap();
        stack.push_int32(2).unwrap();
        exec_left_shift_int32(&mut stack).unwrap();
        assert_eq!(stack.pop_int32().unwrap(), 20); // 5 << 2 = 20
    }

    #[test]
    fn test_right_shift() {
        let mut stack = Stack::new(64);
        stack.push_int32(20).unwrap();
        stack.push_int32(2).unwrap();
        exec_right_shift_int32(&mut stack).unwrap();
        assert_eq!(stack.pop_int32().unwrap(), 5); // 20 >> 2 = 5
    }
}
