/// Vec2 opcodes - vectors are represented as consecutive stack slots
use crate::math::{Fixed, Vec2};
use crate::lpscript::vm::error::RuntimeError;

/// Execute AddVec2: pop 4 (b.y, b.x, a.y, a.x), push 2 ((a+b).y, (a+b).x)
/// Stack grows upward: [..., a.x, a.y, b.x, b.y] -> [..., result.x, result.y]
#[inline(always)]
pub fn exec_add_vec2(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 4 {
        return Err(RuntimeError::StackUnderflow { required: 4, actual: *sp });
    }
    
    *sp -= 1;
    let by = Fixed(stack[*sp]);
    *sp -= 1;
    let bx = Fixed(stack[*sp]);
    *sp -= 1;
    let ay = Fixed(stack[*sp]);
    *sp -= 1;
    let ax = Fixed(stack[*sp]);
    
    let a = Vec2::new(ax, ay);
    let b = Vec2::new(bx, by);
    let result = a + b;
    
    stack[*sp] = result.x.0;
    *sp += 1;
    stack[*sp] = result.y.0;
    *sp += 1;
    
    Ok(())
}

/// Execute SubVec2: pop 4, push 2
#[inline(always)]
pub fn exec_sub_vec2(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 4 {
        return Err(RuntimeError::StackUnderflow { required: 4, actual: *sp });
    }
    
    *sp -= 1;
    let by = Fixed(stack[*sp]);
    *sp -= 1;
    let bx = Fixed(stack[*sp]);
    *sp -= 1;
    let ay = Fixed(stack[*sp]);
    *sp -= 1;
    let ax = Fixed(stack[*sp]);
    
    let a = Vec2::new(ax, ay);
    let b = Vec2::new(bx, by);
    let result = a - b;
    
    stack[*sp] = result.x.0;
    *sp += 1;
    stack[*sp] = result.y.0;
    *sp += 1;
    
    Ok(())
}

/// Execute MulVec2: component-wise multiplication, pop 4, push 2
#[inline(always)]
pub fn exec_mul_vec2(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 4 {
        return Err(RuntimeError::StackUnderflow { required: 4, actual: *sp });
    }
    
    *sp -= 1;
    let by = Fixed(stack[*sp]);
    *sp -= 1;
    let bx = Fixed(stack[*sp]);
    *sp -= 1;
    let ay = Fixed(stack[*sp]);
    *sp -= 1;
    let ax = Fixed(stack[*sp]);
    
    let a = Vec2::new(ax, ay);
    let b = Vec2::new(bx, by);
    let result = a.mul_comp(b);
    
    stack[*sp] = result.x.0;
    *sp += 1;
    stack[*sp] = result.y.0;
    *sp += 1;
    
    Ok(())
}

/// Execute DivVec2: component-wise division, pop 4, push 2
#[inline(always)]
pub fn exec_div_vec2(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 4 {
        return Err(RuntimeError::StackUnderflow { required: 4, actual: *sp });
    }
    
    *sp -= 1;
    let by = Fixed(stack[*sp]);
    *sp -= 1;
    let bx = Fixed(stack[*sp]);
    *sp -= 1;
    let ay = Fixed(stack[*sp]);
    *sp -= 1;
    let ax = Fixed(stack[*sp]);
    
    let a = Vec2::new(ax, ay);
    let b = Vec2::new(bx, by);
    let result = a.div_comp(b);
    
    stack[*sp] = result.x.0;
    *sp += 1;
    stack[*sp] = result.y.0;
    *sp += 1;
    
    Ok(())
}

/// Execute MulVec2Scalar: pop 3 (scalar, vec.y, vec.x), push 2 (result.y, result.x)
#[inline(always)]
pub fn exec_mul_vec2_scalar(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 3 {
        return Err(RuntimeError::StackUnderflow { required: 3, actual: *sp });
    }
    
    *sp -= 1;
    let scalar = Fixed(stack[*sp]);
    *sp -= 1;
    let vy = Fixed(stack[*sp]);
    *sp -= 1;
    let vx = Fixed(stack[*sp]);
    
    let v = Vec2::new(vx, vy);
    let result = v * scalar;
    
    stack[*sp] = result.x.0;
    *sp += 1;
    stack[*sp] = result.y.0;
    *sp += 1;
    
    Ok(())
}

/// Execute DivVec2Scalar: pop 3 (scalar, vec.y, vec.x), push 2 (result.y, result.x)
#[inline(always)]
pub fn exec_div_vec2_scalar(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 3 {
        return Err(RuntimeError::StackUnderflow { required: 3, actual: *sp });
    }
    
    *sp -= 1;
    let scalar = Fixed(stack[*sp]);
    *sp -= 1;
    let vy = Fixed(stack[*sp]);
    *sp -= 1;
    let vx = Fixed(stack[*sp]);
    
    let v = Vec2::new(vx, vy);
    let result = v / scalar;
    
    stack[*sp] = result.x.0;
    *sp += 1;
    stack[*sp] = result.y.0;
    *sp += 1;
    
    Ok(())
}

/// Execute Dot2: pop 4 (b.y, b.x, a.y, a.x), push 1 (a·b)
#[inline(always)]
pub fn exec_dot2(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 4 {
        return Err(RuntimeError::StackUnderflow { required: 4, actual: *sp });
    }
    
    *sp -= 1;
    let by = Fixed(stack[*sp]);
    *sp -= 1;
    let bx = Fixed(stack[*sp]);
    *sp -= 1;
    let ay = Fixed(stack[*sp]);
    *sp -= 1;
    let ax = Fixed(stack[*sp]);
    
    let a = Vec2::new(ax, ay);
    let b = Vec2::new(bx, by);
    let dot = (a.x * b.x) + (a.y * b.y);
    
    stack[*sp] = dot.0;
    *sp += 1;
    
    Ok(())
}

/// Execute Length2: pop 2 (v.y, v.x), push 1 (length)
#[inline(always)]
pub fn exec_length2(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 2 {
        return Err(RuntimeError::StackUnderflow { required: 2, actual: *sp });
    }
    
    *sp -= 1;
    let vy = Fixed(stack[*sp]);
    *sp -= 1;
    let vx = Fixed(stack[*sp]);
    
    let v = Vec2::new(vx, vy);
    let len_sq = (v.x * v.x) + (v.y * v.y);
    let len = crate::math::sqrt(len_sq);
    
    stack[*sp] = len.0;
    *sp += 1;
    
    Ok(())
}

/// Execute Normalize2: pop 2 (v.y, v.x), push 2 (normalized.y, normalized.x)
#[inline(always)]
pub fn exec_normalize2(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 2 {
        return Err(RuntimeError::StackUnderflow { required: 2, actual: *sp });
    }
    
    *sp -= 1;
    let vy = Fixed(stack[*sp]);
    *sp -= 1;
    let vx = Fixed(stack[*sp]);
    
    let v = Vec2::new(vx, vy);
    let len_sq = (v.x * v.x) + (v.y * v.y);
    let len = crate::math::sqrt(len_sq);
    
    if len.0 == 0 {
        // Return zero vector
        stack[*sp] = 0;
        *sp += 1;
        stack[*sp] = 0;
        *sp += 1;
    } else {
        let result = Vec2::new(v.x / len, v.y / len);
        stack[*sp] = result.x.0;
        *sp += 1;
        stack[*sp] = result.y.0;
        *sp += 1;
    }
    
    Ok(())
}

/// Execute Distance2: pop 4 (two vec2s), push 1 (distance)
#[inline(always)]
pub fn exec_distance2(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 4 {
        return Err(RuntimeError::StackUnderflow { required: 4, actual: *sp });
    }
    
    *sp -= 1;
    let by = Fixed(stack[*sp]);
    *sp -= 1;
    let bx = Fixed(stack[*sp]);
    *sp -= 1;
    let ay = Fixed(stack[*sp]);
    *sp -= 1;
    let ax = Fixed(stack[*sp]);
    
    let a = Vec2::new(ax, ay);
    let b = Vec2::new(bx, by);
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    let dist_sq = (dx * dx) + (dy * dy);
    let dist = crate::math::sqrt(dist_sq);
    
    stack[*sp] = dist.0;
    *sp += 1;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::ToFixed;
    
    #[test]
    fn test_add_vec2() {
        let mut stack = [0i32; 64];
        let mut sp = 0;
        
        // Push vec2(1, 2)
        stack[sp] = 1.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 2.0f32.to_fixed().0;
        sp += 1;
        
        // Push vec2(3, 4)
        stack[sp] = 3.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 4.0f32.to_fixed().0;
        sp += 1;
        
        exec_add_vec2(&mut stack, &mut sp).unwrap();
        
        assert_eq!(sp, 2);
        assert_eq!(Fixed(stack[0]).to_f32(), 4.0);
        assert_eq!(Fixed(stack[1]).to_f32(), 6.0);
    }
    
    #[test]
    fn test_add_vec2_underflow() {
        let mut stack = [0i32; 64];
        let mut sp = 2; // Only 2 items, need 4
        
        let result = exec_add_vec2(&mut stack, &mut sp);
        assert!(matches!(result, Err(RuntimeError::StackUnderflow { required: 4, actual: 2 })));
    }
    
    #[test]
    fn test_dot2() {
        let mut stack = [0i32; 64];
        let mut sp = 0;
        
        // Push vec2(3, 4)
        stack[sp] = 3.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 4.0f32.to_fixed().0;
        sp += 1;
        
        // Push vec2(2, 1)
        stack[sp] = 2.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 1.0f32.to_fixed().0;
        sp += 1;
        
        exec_dot2(&mut stack, &mut sp).unwrap();
        
        assert_eq!(sp, 1);
        // 3*2 + 4*1 = 6 + 4 = 10
        assert_eq!(Fixed(stack[0]).to_f32(), 10.0);
    }
    
    #[test]
    fn test_length2() {
        let mut stack = [0i32; 64];
        let mut sp = 0;
        
        // Push vec2(3, 4) - length should be 5
        stack[sp] = 3.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 4.0f32.to_fixed().0;
        sp += 1;
        
        exec_length2(&mut stack, &mut sp).unwrap();
        
        assert_eq!(sp, 1);
        assert_eq!(Fixed(stack[0]).to_f32(), 5.0);
    }
    
    #[test]
    fn test_mul_vec2_scalar() {
        let mut stack = [0i32; 64];
        let mut sp = 0;
        
        // Push vec2(2, 3)
        stack[sp] = 2.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 3.0f32.to_fixed().0;
        sp += 1;
        
        // Push scalar 5
        stack[sp] = 5.0f32.to_fixed().0;
        sp += 1;
        
        exec_mul_vec2_scalar(&mut stack, &mut sp).unwrap();
        
        assert_eq!(sp, 2);
        assert_eq!(Fixed(stack[0]).to_f32(), 10.0);
        assert_eq!(Fixed(stack[1]).to_f32(), 15.0);
    }
}
