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

/// Execute Swizzle3to2: pop 3 values, push 2 based on indices
#[inline(always)]
pub fn exec_swizzle3to2(stack: &mut [i32], sp: &mut usize, idx0: u8, idx1: u8) -> Result<(), RuntimeError> {
    if *sp < 3 {
        return Err(RuntimeError::StackUnderflow { required: 3, actual: *sp });
    }
    
    let c2 = stack[*sp - 1];
    let c1 = stack[*sp - 2];
    let c0 = stack[*sp - 3];
    
    let components = [c0, c1, c2];
    let result0 = components[idx0 as usize];
    let result1 = components[idx1 as usize];
    
    *sp -= 3; // Pop all 3
    stack[*sp] = result0;
    stack[*sp + 1] = result1;
    *sp += 2; // Push 2
    
    Ok(())
}

/// Execute Swizzle3to3: pop 3 values, push 3 based on indices
#[inline(always)]
pub fn exec_swizzle3to3(stack: &mut [i32], sp: &mut usize, idx0: u8, idx1: u8, idx2: u8) -> Result<(), RuntimeError> {
    if *sp < 3 {
        return Err(RuntimeError::StackUnderflow { required: 3, actual: *sp });
    }
    
    let c2 = stack[*sp - 1];
    let c1 = stack[*sp - 2];
    let c0 = stack[*sp - 3];
    
    let components = [c0, c1, c2];
    let result0 = components[idx0 as usize];
    let result1 = components[idx1 as usize];
    let result2 = components[idx2 as usize];
    
    *sp -= 3;
    stack[*sp] = result0;
    stack[*sp + 1] = result1;
    stack[*sp + 2] = result2;
    *sp += 3;
    
    Ok(())
}

/// Execute Swizzle4to2: pop 4 values, push 2 based on indices
#[inline(always)]
pub fn exec_swizzle4to2(stack: &mut [i32], sp: &mut usize, idx0: u8, idx1: u8) -> Result<(), RuntimeError> {
    if *sp < 4 {
        return Err(RuntimeError::StackUnderflow { required: 4, actual: *sp });
    }
    
    let c3 = stack[*sp - 1];
    let c2 = stack[*sp - 2];
    let c1 = stack[*sp - 3];
    let c0 = stack[*sp - 4];
    
    let components = [c0, c1, c2, c3];
    let result0 = components[idx0 as usize];
    let result1 = components[idx1 as usize];
    
    *sp -= 4; // Pop all 4
    stack[*sp] = result0;
    stack[*sp + 1] = result1;
    *sp += 2; // Push 2
    
    Ok(())
}

/// Execute Swizzle4to3: pop 4 values, push 3 based on indices
#[inline(always)]
pub fn exec_swizzle4to3(stack: &mut [i32], sp: &mut usize, idx0: u8, idx1: u8, idx2: u8) -> Result<(), RuntimeError> {
    if *sp < 4 {
        return Err(RuntimeError::StackUnderflow { required: 4, actual: *sp });
    }
    
    let c3 = stack[*sp - 1];
    let c2 = stack[*sp - 2];
    let c1 = stack[*sp - 3];
    let c0 = stack[*sp - 4];
    
    let components = [c0, c1, c2, c3];
    let result0 = components[idx0 as usize];
    let result1 = components[idx1 as usize];
    let result2 = components[idx2 as usize];
    
    *sp -= 4;
    stack[*sp] = result0;
    stack[*sp + 1] = result1;
    stack[*sp + 2] = result2;
    *sp += 3;
    
    Ok(())
}

/// Execute Swizzle4to4: pop 4 values, push 4 based on indices
#[inline(always)]
pub fn exec_swizzle4to4(stack: &mut [i32], sp: &mut usize, idx0: u8, idx1: u8, idx2: u8, idx3: u8) -> Result<(), RuntimeError> {
    if *sp < 4 {
        return Err(RuntimeError::StackUnderflow { required: 4, actual: *sp });
    }
    
    let c3 = stack[*sp - 1];
    let c2 = stack[*sp - 2];
    let c1 = stack[*sp - 3];
    let c0 = stack[*sp - 4];
    
    let components = [c0, c1, c2, c3];
    let result0 = components[idx0 as usize];
    let result1 = components[idx1 as usize];
    let result2 = components[idx2 as usize];
    let result3 = components[idx3 as usize];
    
    *sp -= 4;
    stack[*sp] = result0;
    stack[*sp + 1] = result1;
    stack[*sp + 2] = result2;
    stack[*sp + 3] = result3;
    *sp += 4;
    
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

    #[test]
    fn test_swizzle3to2_xy() {
        let mut stack = [0i32; 64];
        let mut sp = 0;
        
        // Push vec3(10, 20, 30)
        stack[sp] = 10;
        sp += 1;
        stack[sp] = 20;
        sp += 1;
        stack[sp] = 30;
        sp += 1;
        
        // Swizzle to .xy (indices 0, 1)
        exec_swizzle3to2(&mut stack, &mut sp, 0, 1).unwrap();
        
        assert_eq!(sp, 2);
        assert_eq!(stack[0], 10);
        assert_eq!(stack[1], 20);
    }

    #[test]
    fn test_swizzle3to2_yz() {
        let mut stack = [0i32; 64];
        let mut sp = 0;
        
        // Push vec3(10, 20, 30)
        stack[sp] = 10;
        sp += 1;
        stack[sp] = 20;
        sp += 1;
        stack[sp] = 30;
        sp += 1;
        
        // Swizzle to .yz (indices 1, 2)
        exec_swizzle3to2(&mut stack, &mut sp, 1, 2).unwrap();
        
        assert_eq!(sp, 2);
        assert_eq!(stack[0], 20);
        assert_eq!(stack[1], 30);
    }

    #[test]
    fn test_swizzle3to3_zyx() {
        let mut stack = [0i32; 64];
        let mut sp = 0;
        
        // Push vec3(10, 20, 30)
        stack[sp] = 10;
        sp += 1;
        stack[sp] = 20;
        sp += 1;
        stack[sp] = 30;
        sp += 1;
        
        // Swizzle to .zyx (indices 2, 1, 0)
        exec_swizzle3to3(&mut stack, &mut sp, 2, 1, 0).unwrap();
        
        assert_eq!(sp, 3);
        assert_eq!(stack[0], 30);
        assert_eq!(stack[1], 20);
        assert_eq!(stack[2], 10);
    }

    #[test]
    fn test_swizzle4to2_xy() {
        let mut stack = [0i32; 64];
        let mut sp = 0;
        
        // Push vec4(10, 20, 30, 40)
        stack[sp] = 10;
        sp += 1;
        stack[sp] = 20;
        sp += 1;
        stack[sp] = 30;
        sp += 1;
        stack[sp] = 40;
        sp += 1;
        
        // Swizzle to .xy (indices 0, 1)
        exec_swizzle4to2(&mut stack, &mut sp, 0, 1).unwrap();
        
        assert_eq!(sp, 2);
        assert_eq!(stack[0], 10);
        assert_eq!(stack[1], 20);
    }

    #[test]
    fn test_swizzle4to2_zw() {
        let mut stack = [0i32; 64];
        let mut sp = 0;
        
        // Push vec4(10, 20, 30, 40)
        stack[sp] = 10;
        sp += 1;
        stack[sp] = 20;
        sp += 1;
        stack[sp] = 30;
        sp += 1;
        stack[sp] = 40;
        sp += 1;
        
        // Swizzle to .zw (indices 2, 3)
        exec_swizzle4to2(&mut stack, &mut sp, 2, 3).unwrap();
        
        assert_eq!(sp, 2);
        assert_eq!(stack[0], 30);
        assert_eq!(stack[1], 40);
    }

    #[test]
    fn test_swizzle4to3_xyz() {
        let mut stack = [0i32; 64];
        let mut sp = 0;
        
        // Push vec4(10, 20, 30, 40)
        stack[sp] = 10;
        sp += 1;
        stack[sp] = 20;
        sp += 1;
        stack[sp] = 30;
        sp += 1;
        stack[sp] = 40;
        sp += 1;
        
        // Swizzle to .xyz (indices 0, 1, 2)
        exec_swizzle4to3(&mut stack, &mut sp, 0, 1, 2).unwrap();
        
        assert_eq!(sp, 3);
        assert_eq!(stack[0], 10);
        assert_eq!(stack[1], 20);
        assert_eq!(stack[2], 30);
    }

    #[test]
    fn test_swizzle4to4_wzyx() {
        let mut stack = [0i32; 64];
        let mut sp = 0;
        
        // Push vec4(10, 20, 30, 40)
        stack[sp] = 10;
        sp += 1;
        stack[sp] = 20;
        sp += 1;
        stack[sp] = 30;
        sp += 1;
        stack[sp] = 40;
        sp += 1;
        
        // Swizzle to .wzyx (indices 3, 2, 1, 0)
        exec_swizzle4to4(&mut stack, &mut sp, 3, 2, 1, 0).unwrap();
        
        assert_eq!(sp, 4);
        assert_eq!(stack[0], 40);
        assert_eq!(stack[1], 30);
        assert_eq!(stack[2], 20);
        assert_eq!(stack[3], 10);
    }

    #[test]
    fn test_swizzle3to2_underflow() {
        let mut stack = [0i32; 64];
        let mut sp = 2; // Only 2 values on stack
        
        let result = exec_swizzle3to2(&mut stack, &mut sp, 0, 1);
        assert!(matches!(result, Err(RuntimeError::StackUnderflow { required: 3, actual: 2 })));
    }

    #[test]
    fn test_swizzle4to2_underflow() {
        let mut stack = [0i32; 64];
        let mut sp = 3; // Only 3 values on stack
        
        let result = exec_swizzle4to2(&mut stack, &mut sp, 0, 1);
        assert!(matches!(result, Err(RuntimeError::StackUnderflow { required: 4, actual: 3 })));
    }
}
