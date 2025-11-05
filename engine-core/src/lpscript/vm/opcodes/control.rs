/// Control flow opcodes with error handling
use crate::lpscript::vm::error::RuntimeError;

/// Execute Select (ternary): pop false_val, true_val, condition; push selected
#[inline(always)]
pub fn exec_select(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 3 {
        return Err(RuntimeError::StackUnderflow { required: 3, actual: *sp });
    }
    
    *sp -= 1;
    let false_val = stack[*sp];
    *sp -= 1;
    let true_val = stack[*sp];
    *sp -= 1;
    let condition = stack[*sp];
    
    stack[*sp] = if condition != 0 { true_val } else { false_val };
    *sp += 1;
    
    Ok(())
}

/// Execute JumpIfZero: pop value, jump if zero
/// Returns Some(new_pc) if jump taken, None otherwise
#[inline(always)]
pub fn exec_jump_if_zero(stack: &mut [i32], sp: &mut usize, pc: usize, offset: i32) -> Result<Option<usize>, RuntimeError> {
    if *sp < 1 {
        return Err(RuntimeError::StackUnderflow { required: 1, actual: *sp });
    }
    
    *sp -= 1;
    let value = stack[*sp];
    
    if value == 0 {
        Ok(Some(((pc as i32) + offset + 1) as usize))
    } else {
        Ok(None)
    }
}

/// Execute JumpIfNonZero: pop value, jump if non-zero  
#[inline(always)]
pub fn exec_jump_if_nonzero(stack: &mut [i32], sp: &mut usize, pc: usize, offset: i32) -> Result<Option<usize>, RuntimeError> {
    if *sp < 1 {
        return Err(RuntimeError::StackUnderflow { required: 1, actual: *sp });
    }
    
    *sp -= 1;
    let value = stack[*sp];
    
    if value != 0 {
        Ok(Some(((pc as i32) + offset + 1) as usize))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::{Fixed, ToFixed};
    
    #[test]
    fn test_select_true() {
        let mut stack = [0i32; 64];
        let mut sp = 0;
        
        // condition = 1 (true)
        stack[sp] = 1.0f32.to_fixed().0;
        sp += 1;
        // true_val = 10
        stack[sp] = 10.0f32.to_fixed().0;
        sp += 1;
        // false_val = 20
        stack[sp] = 20.0f32.to_fixed().0;
        sp += 1;
        
        exec_select(&mut stack, &mut sp).unwrap();
        
        assert_eq!(sp, 1);
        assert_eq!(Fixed(stack[0]).to_f32(), 10.0);
    }
    
    #[test]
    fn test_select_false() {
        let mut stack = [0i32; 64];
        let mut sp = 0;
        
        // condition = 0 (false)
        stack[sp] = 0;
        sp += 1;
        // true_val = 10
        stack[sp] = 10.0f32.to_fixed().0;
        sp += 1;
        // false_val = 20
        stack[sp] = 20.0f32.to_fixed().0;
        sp += 1;
        
        exec_select(&mut stack, &mut sp).unwrap();
        
        assert_eq!(sp, 1);
        assert_eq!(Fixed(stack[0]).to_f32(), 20.0);
    }
    
    #[test]
    fn test_select_underflow() {
        let mut stack = [0i32; 64];
        let mut sp = 2; // Only 2 items, need 3
        
        let result = exec_select(&mut stack, &mut sp);
        assert!(matches!(result, Err(RuntimeError::StackUnderflow { required: 3, actual: 2 })));
    }
    
    #[test]
    fn test_jump_if_zero() {
        let mut stack = [0i32; 64];
        let mut sp = 0;
        
        stack[sp] = 0;
        sp += 1;
        
        let result = exec_jump_if_zero(&mut stack, &mut sp, 10, 5).unwrap();
        
        assert_eq!(sp, 0);
        assert_eq!(result, Some(16)); // pc + offset + 1 = 10 + 5 + 1 = 16
    }
    
    #[test]
    fn test_jump_if_zero_no_jump() {
        let mut stack = [0i32; 64];
        let mut sp = 0;
        
        stack[sp] = 1.0f32.to_fixed().0;
        sp += 1;
        
        let result = exec_jump_if_zero(&mut stack, &mut sp, 10, 5).unwrap();
        
        assert_eq!(sp, 0);
        assert_eq!(result, None);
    }
}
