/// Vec4 opcodes - vectors are represented as consecutive stack slots
use crate::math::{Fixed, Vec4};
use crate::lpscript::vm::error::RuntimeError;

/// Execute AddVec4: pop 8, push 4
/// Stack: [..., a.x, a.y, a.z, a.w, b.x, b.y, b.z, b.w] -> [..., result.x, result.y, result.z, result.w]
#[inline(always)]
pub fn exec_add_vec4(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 8 {
        return Err(RuntimeError::StackUnderflow { required: 8, actual: *sp });
    }
    
    *sp -= 1;
    let bw = Fixed(stack[*sp]);
    *sp -= 1;
    let bz = Fixed(stack[*sp]);
    *sp -= 1;
    let by = Fixed(stack[*sp]);
    *sp -= 1;
    let bx = Fixed(stack[*sp]);
    *sp -= 1;
    let aw = Fixed(stack[*sp]);
    *sp -= 1;
    let az = Fixed(stack[*sp]);
    *sp -= 1;
    let ay = Fixed(stack[*sp]);
    *sp -= 1;
    let ax = Fixed(stack[*sp]);
    
    let a = Vec4::new(ax, ay, az, aw);
    let b = Vec4::new(bx, by, bz, bw);
    let result = a + b;
    
    stack[*sp] = result.x.0;
    *sp += 1;
    stack[*sp] = result.y.0;
    *sp += 1;
    stack[*sp] = result.z.0;
    *sp += 1;
    stack[*sp] = result.w.0;
    *sp += 1;
    
    Ok(())
}

/// Execute SubVec4: pop 8, push 4
#[inline(always)]
pub fn exec_sub_vec4(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 8 {
        return Err(RuntimeError::StackUnderflow { required: 8, actual: *sp });
    }
    
    *sp -= 1;
    let bw = Fixed(stack[*sp]);
    *sp -= 1;
    let bz = Fixed(stack[*sp]);
    *sp -= 1;
    let by = Fixed(stack[*sp]);
    *sp -= 1;
    let bx = Fixed(stack[*sp]);
    *sp -= 1;
    let aw = Fixed(stack[*sp]);
    *sp -= 1;
    let az = Fixed(stack[*sp]);
    *sp -= 1;
    let ay = Fixed(stack[*sp]);
    *sp -= 1;
    let ax = Fixed(stack[*sp]);
    
    let a = Vec4::new(ax, ay, az, aw);
    let b = Vec4::new(bx, by, bz, bw);
    let result = a - b;
    
    stack[*sp] = result.x.0;
    *sp += 1;
    stack[*sp] = result.y.0;
    *sp += 1;
    stack[*sp] = result.z.0;
    *sp += 1;
    stack[*sp] = result.w.0;
    *sp += 1;
    
    Ok(())
}

/// Execute MulVec4: component-wise multiplication, pop 8, push 4
#[inline(always)]
pub fn exec_mul_vec4(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 8 {
        return Err(RuntimeError::StackUnderflow { required: 8, actual: *sp });
    }
    
    *sp -= 1;
    let bw = Fixed(stack[*sp]);
    *sp -= 1;
    let bz = Fixed(stack[*sp]);
    *sp -= 1;
    let by = Fixed(stack[*sp]);
    *sp -= 1;
    let bx = Fixed(stack[*sp]);
    *sp -= 1;
    let aw = Fixed(stack[*sp]);
    *sp -= 1;
    let az = Fixed(stack[*sp]);
    *sp -= 1;
    let ay = Fixed(stack[*sp]);
    *sp -= 1;
    let ax = Fixed(stack[*sp]);
    
    let a = Vec4::new(ax, ay, az, aw);
    let b = Vec4::new(bx, by, bz, bw);
    let result = a.mul_comp(b);
    
    stack[*sp] = result.x.0;
    *sp += 1;
    stack[*sp] = result.y.0;
    *sp += 1;
    stack[*sp] = result.z.0;
    *sp += 1;
    stack[*sp] = result.w.0;
    *sp += 1;
    
    Ok(())
}

/// Execute DivVec4: component-wise division, pop 8, push 4
#[inline(always)]
pub fn exec_div_vec4(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 8 {
        return Err(RuntimeError::StackUnderflow { required: 8, actual: *sp });
    }
    
    *sp -= 1;
    let bw = Fixed(stack[*sp]);
    *sp -= 1;
    let bz = Fixed(stack[*sp]);
    *sp -= 1;
    let by = Fixed(stack[*sp]);
    *sp -= 1;
    let bx = Fixed(stack[*sp]);
    *sp -= 1;
    let aw = Fixed(stack[*sp]);
    *sp -= 1;
    let az = Fixed(stack[*sp]);
    *sp -= 1;
    let ay = Fixed(stack[*sp]);
    *sp -= 1;
    let ax = Fixed(stack[*sp]);
    
    let a = Vec4::new(ax, ay, az, aw);
    let b = Vec4::new(bx, by, bz, bw);
    let result = a.div_comp(b);
    
    stack[*sp] = result.x.0;
    *sp += 1;
    stack[*sp] = result.y.0;
    *sp += 1;
    stack[*sp] = result.z.0;
    *sp += 1;
    stack[*sp] = result.w.0;
    *sp += 1;
    
    Ok(())
}

/// Execute MulVec4Scalar: pop 5 (scalar, vec.w, vec.z, vec.y, vec.x), push 4
#[inline(always)]
pub fn exec_mul_vec4_scalar(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 5 {
        return Err(RuntimeError::StackUnderflow { required: 5, actual: *sp });
    }
    
    *sp -= 1;
    let scalar = Fixed(stack[*sp]);
    *sp -= 1;
    let vw = Fixed(stack[*sp]);
    *sp -= 1;
    let vz = Fixed(stack[*sp]);
    *sp -= 1;
    let vy = Fixed(stack[*sp]);
    *sp -= 1;
    let vx = Fixed(stack[*sp]);
    
    let v = Vec4::new(vx, vy, vz, vw);
    let result = v * scalar;
    
    stack[*sp] = result.x.0;
    *sp += 1;
    stack[*sp] = result.y.0;
    *sp += 1;
    stack[*sp] = result.z.0;
    *sp += 1;
    stack[*sp] = result.w.0;
    *sp += 1;
    
    Ok(())
}

/// Execute DivVec4Scalar: pop 5, push 4
#[inline(always)]
pub fn exec_div_vec4_scalar(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 5 {
        return Err(RuntimeError::StackUnderflow { required: 5, actual: *sp });
    }
    
    *sp -= 1;
    let scalar = Fixed(stack[*sp]);
    *sp -= 1;
    let vw = Fixed(stack[*sp]);
    *sp -= 1;
    let vz = Fixed(stack[*sp]);
    *sp -= 1;
    let vy = Fixed(stack[*sp]);
    *sp -= 1;
    let vx = Fixed(stack[*sp]);
    
    let v = Vec4::new(vx, vy, vz, vw);
    let result = v / scalar;
    
    stack[*sp] = result.x.0;
    *sp += 1;
    stack[*sp] = result.y.0;
    *sp += 1;
    stack[*sp] = result.z.0;
    *sp += 1;
    stack[*sp] = result.w.0;
    *sp += 1;
    
    Ok(())
}

/// Execute Dot4: pop 8 (two vec4s), push 1 (dot product)
#[inline(always)]
pub fn exec_dot4(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 8 {
        return Err(RuntimeError::StackUnderflow { required: 8, actual: *sp });
    }
    
    *sp -= 1;
    let bw = Fixed(stack[*sp]);
    *sp -= 1;
    let bz = Fixed(stack[*sp]);
    *sp -= 1;
    let by = Fixed(stack[*sp]);
    *sp -= 1;
    let bx = Fixed(stack[*sp]);
    *sp -= 1;
    let aw = Fixed(stack[*sp]);
    *sp -= 1;
    let az = Fixed(stack[*sp]);
    *sp -= 1;
    let ay = Fixed(stack[*sp]);
    *sp -= 1;
    let ax = Fixed(stack[*sp]);
    
    let a = Vec4::new(ax, ay, az, aw);
    let b = Vec4::new(bx, by, bz, bw);
    let dot = a.dot(b);
    
    stack[*sp] = dot.0;
    *sp += 1;
    
    Ok(())
}

/// Execute Length4: pop 4 (v.w, v.z, v.y, v.x), push 1 (length)
#[inline(always)]
pub fn exec_length4(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 4 {
        return Err(RuntimeError::StackUnderflow { required: 4, actual: *sp });
    }
    
    *sp -= 1;
    let vw = Fixed(stack[*sp]);
    *sp -= 1;
    let vz = Fixed(stack[*sp]);
    *sp -= 1;
    let vy = Fixed(stack[*sp]);
    *sp -= 1;
    let vx = Fixed(stack[*sp]);
    
    let v = Vec4::new(vx, vy, vz, vw);
    let len = v.length();
    
    stack[*sp] = len.0;
    *sp += 1;
    
    Ok(())
}

/// Execute Normalize4: pop 4, push 4 (normalized vector)
#[inline(always)]
pub fn exec_normalize4(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 4 {
        return Err(RuntimeError::StackUnderflow { required: 4, actual: *sp });
    }
    
    *sp -= 1;
    let vw = Fixed(stack[*sp]);
    *sp -= 1;
    let vz = Fixed(stack[*sp]);
    *sp -= 1;
    let vy = Fixed(stack[*sp]);
    *sp -= 1;
    let vx = Fixed(stack[*sp]);
    
    let v = Vec4::new(vx, vy, vz, vw);
    let result = v.normalize();
    
    stack[*sp] = result.x.0;
    *sp += 1;
    stack[*sp] = result.y.0;
    *sp += 1;
    stack[*sp] = result.z.0;
    *sp += 1;
    stack[*sp] = result.w.0;
    *sp += 1;
    
    Ok(())
}

/// Execute Distance4: pop 8 (two vec4s), push 1 (distance)
#[inline(always)]
pub fn exec_distance4(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 8 {
        return Err(RuntimeError::StackUnderflow { required: 8, actual: *sp });
    }
    
    *sp -= 1;
    let bw = Fixed(stack[*sp]);
    *sp -= 1;
    let bz = Fixed(stack[*sp]);
    *sp -= 1;
    let by = Fixed(stack[*sp]);
    *sp -= 1;
    let bx = Fixed(stack[*sp]);
    *sp -= 1;
    let aw = Fixed(stack[*sp]);
    *sp -= 1;
    let az = Fixed(stack[*sp]);
    *sp -= 1;
    let ay = Fixed(stack[*sp]);
    *sp -= 1;
    let ax = Fixed(stack[*sp]);
    
    let a = Vec4::new(ax, ay, az, aw);
    let b = Vec4::new(bx, by, bz, bw);
    let dist = a.distance(b);
    
    stack[*sp] = dist.0;
    *sp += 1;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::ToFixed;
    
    #[test]
    fn test_add_vec4() {
        let mut stack = [0i32; 64];
        let mut sp = 0;
        
        // Push vec4(1, 2, 3, 4)
        stack[sp] = 1.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 2.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 3.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 4.0f32.to_fixed().0;
        sp += 1;
        
        // Push vec4(5, 6, 7, 8)
        stack[sp] = 5.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 6.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 7.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 8.0f32.to_fixed().0;
        sp += 1;
        
        exec_add_vec4(&mut stack, &mut sp).unwrap();
        
        assert_eq!(sp, 4);
        assert_eq!(Fixed(stack[0]).to_f32(), 6.0);
        assert_eq!(Fixed(stack[1]).to_f32(), 8.0);
        assert_eq!(Fixed(stack[2]).to_f32(), 10.0);
        assert_eq!(Fixed(stack[3]).to_f32(), 12.0);
    }
    
    #[test]
    fn test_sub_vec4() {
        let mut stack = [0i32; 64];
        let mut sp = 0;
        
        // Push vec4(10, 20, 30, 40)
        stack[sp] = 10.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 20.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 30.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 40.0f32.to_fixed().0;
        sp += 1;
        
        // Push vec4(1, 2, 3, 4)
        stack[sp] = 1.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 2.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 3.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 4.0f32.to_fixed().0;
        sp += 1;
        
        exec_sub_vec4(&mut stack, &mut sp).unwrap();
        
        assert_eq!(sp, 4);
        assert_eq!(Fixed(stack[0]).to_f32(), 9.0);
        assert_eq!(Fixed(stack[1]).to_f32(), 18.0);
        assert_eq!(Fixed(stack[2]).to_f32(), 27.0);
        assert_eq!(Fixed(stack[3]).to_f32(), 36.0);
    }
    
    #[test]
    fn test_dot4() {
        let mut stack = [0i32; 64];
        let mut sp = 0;
        
        // Push vec4(1, 2, 3, 4)
        stack[sp] = 1.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 2.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 3.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 4.0f32.to_fixed().0;
        sp += 1;
        
        // Push vec4(5, 6, 7, 8)
        stack[sp] = 5.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 6.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 7.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 8.0f32.to_fixed().0;
        sp += 1;
        
        exec_dot4(&mut stack, &mut sp).unwrap();
        
        assert_eq!(sp, 1);
        // 1*5 + 2*6 + 3*7 + 4*8 = 5 + 12 + 21 + 32 = 70
        assert_eq!(Fixed(stack[0]).to_f32(), 70.0);
    }
    
    #[test]
    fn test_length4() {
        let mut stack = [0i32; 64];
        let mut sp = 0;
        
        // Push vec4(2, 3, 6, 0) - should have specific length
        stack[sp] = 2.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 3.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 6.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 0.0f32.to_fixed().0;
        sp += 1;
        
        exec_length4(&mut stack, &mut sp).unwrap();
        
        assert_eq!(sp, 1);
        // sqrt(4 + 9 + 36 + 0) = sqrt(49) = 7
        assert!((Fixed(stack[0]).to_f32() - 7.0).abs() < 0.01);
    }
    
    #[test]
    fn test_mul_vec4_scalar() {
        let mut stack = [0i32; 64];
        let mut sp = 0;
        
        // Push vec4(1, 2, 3, 4)
        stack[sp] = 1.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 2.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 3.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 4.0f32.to_fixed().0;
        sp += 1;
        
        // Push scalar 2
        stack[sp] = 2.0f32.to_fixed().0;
        sp += 1;
        
        exec_mul_vec4_scalar(&mut stack, &mut sp).unwrap();
        
        assert_eq!(sp, 4);
        assert_eq!(Fixed(stack[0]).to_f32(), 2.0);
        assert_eq!(Fixed(stack[1]).to_f32(), 4.0);
        assert_eq!(Fixed(stack[2]).to_f32(), 6.0);
        assert_eq!(Fixed(stack[3]).to_f32(), 8.0);
    }
    
    #[test]
    fn test_normalize4() {
        let mut stack = [0i32; 64];
        let mut sp = 0;
        
        // Push vec4(2, 3, 6, 0)
        stack[sp] = 2.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 3.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 6.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 0.0f32.to_fixed().0;
        sp += 1;
        
        exec_normalize4(&mut stack, &mut sp).unwrap();
        
        assert_eq!(sp, 4);
        // Length is 7, so normalized should be (2/7, 3/7, 6/7, 0)
        assert!((Fixed(stack[0]).to_f32() - 0.285).abs() < 0.01);
        assert!((Fixed(stack[1]).to_f32() - 0.428).abs() < 0.01);
        assert!((Fixed(stack[2]).to_f32() - 0.857).abs() < 0.01);
        assert!((Fixed(stack[3]).to_f32() - 0.0).abs() < 0.01);
    }
    
    #[test]
    fn test_distance4() {
        let mut stack = [0i32; 64];
        let mut sp = 0;
        
        // Push vec4(0, 0, 0, 0)
        stack[sp] = 0.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 0.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 0.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 0.0f32.to_fixed().0;
        sp += 1;
        
        // Push vec4(2, 3, 6, 0)
        stack[sp] = 2.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 3.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 6.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 0.0f32.to_fixed().0;
        sp += 1;
        
        exec_distance4(&mut stack, &mut sp).unwrap();
        
        assert_eq!(sp, 1);
        assert!((Fixed(stack[0]).to_f32() - 7.0).abs() < 0.01);
    }
}

