/// Stack manipulation opcodes with error handling
use crate::lpscript::error::RuntimeError;

/// Execute Dup: duplicate top of stack
#[inline(always)]
pub fn exec_dup(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 1 {
        return Err(RuntimeError::StackUnderflow { required: 1, actual: *sp });
    }
    if *sp >= 64 {
        return Err(RuntimeError::StackOverflow { sp: *sp });
    }
    
    let val = stack[*sp - 1];
    stack[*sp] = val;
    *sp += 1;
    
    Ok(())
}

/// Execute Drop: remove top of stack
#[inline(always)]
pub fn exec_drop(_stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 1 {
        return Err(RuntimeError::StackUnderflow { required: 1, actual: *sp });
    }
    
    *sp -= 1;
    
    Ok(())
}

/// Execute Swap: swap top two stack items
#[inline(always)]
pub fn exec_swap(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 2 {
        return Err(RuntimeError::StackUnderflow { required: 2, actual: *sp });
    }
    
    let a = stack[*sp - 2];
    let b = stack[*sp - 1];
    stack[*sp - 2] = b;
    stack[*sp - 1] = a;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dup() {
        let mut stack = [0i32; 64];
        let mut sp = 0;
        
        stack[sp] = 42;
        sp += 1;
        
        exec_dup(&mut stack, &mut sp).unwrap();
        
        assert_eq!(sp, 2);
        assert_eq!(stack[0], 42);
        assert_eq!(stack[1], 42);
    }
    
    #[test]
    fn test_dup_underflow() {
        let mut stack = [0i32; 64];
        let mut sp = 0;
        
        let result = exec_dup(&mut stack, &mut sp);
        assert!(matches!(result, Err(RuntimeError::StackUnderflow { required: 1, actual: 0 })));
    }
    
    #[test]
    fn test_drop() {
        let mut stack = [0i32; 64];
        let mut sp = 0;
        
        stack[sp] = 42;
        sp += 1;
        stack[sp] = 99;
        sp += 1;
        
        exec_drop(&mut stack, &mut sp).unwrap();
        
        assert_eq!(sp, 1);
        assert_eq!(stack[0], 42);
    }
    
    #[test]
    fn test_swap() {
        let mut stack = [0i32; 64];
        let mut sp = 0;
        
        stack[sp] = 42;
        sp += 1;
        stack[sp] = 99;
        sp += 1;
        
        exec_swap(&mut stack, &mut sp).unwrap();
        
        assert_eq!(sp, 2);
        assert_eq!(stack[0], 99);
        assert_eq!(stack[1], 42);
    }
}
