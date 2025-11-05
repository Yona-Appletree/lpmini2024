/// Vec3 opcodes - vectors are represented as consecutive stack slots
use crate::math::{Fixed, Vec3};
use crate::lpscript::vm::error::RuntimeError;

/// Execute AddVec3: pop 6 (b.z, b.y, b.x, a.z, a.y, a.x), push 3 ((a+b).z, (a+b).y, (a+b).x)
/// Stack grows upward: [..., a.x, a.y, a.z, b.x, b.y, b.z] -> [..., result.x, result.y, result.z]
#[inline(always)]
pub fn exec_add_vec3(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 6 {
        return Err(RuntimeError::StackUnderflow { required: 6, actual: *sp });
    }
    
    *sp -= 1;
    let bz = Fixed(stack[*sp]);
    *sp -= 1;
    let by = Fixed(stack[*sp]);
    *sp -= 1;
    let bx = Fixed(stack[*sp]);
    *sp -= 1;
    let az = Fixed(stack[*sp]);
    *sp -= 1;
    let ay = Fixed(stack[*sp]);
    *sp -= 1;
    let ax = Fixed(stack[*sp]);
    
    let a = Vec3::new(ax, ay, az);
    let b = Vec3::new(bx, by, bz);
    let result = a + b;
    
    stack[*sp] = result.x.0;
    *sp += 1;
    stack[*sp] = result.y.0;
    *sp += 1;
    stack[*sp] = result.z.0;
    *sp += 1;
    
    Ok(())
}

/// Execute SubVec3: pop 6, push 3
#[inline(always)]
pub fn exec_sub_vec3(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 6 {
        return Err(RuntimeError::StackUnderflow { required: 6, actual: *sp });
    }
    
    *sp -= 1;
    let bz = Fixed(stack[*sp]);
    *sp -= 1;
    let by = Fixed(stack[*sp]);
    *sp -= 1;
    let bx = Fixed(stack[*sp]);
    *sp -= 1;
    let az = Fixed(stack[*sp]);
    *sp -= 1;
    let ay = Fixed(stack[*sp]);
    *sp -= 1;
    let ax = Fixed(stack[*sp]);
    
    let a = Vec3::new(ax, ay, az);
    let b = Vec3::new(bx, by, bz);
    let result = a - b;
    
    stack[*sp] = result.x.0;
    *sp += 1;
    stack[*sp] = result.y.0;
    *sp += 1;
    stack[*sp] = result.z.0;
    *sp += 1;
    
    Ok(())
}

/// Execute NegVec3: negate all components, pop 3, push 3
#[inline(always)]
pub fn exec_neg_vec3(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 3 {
        return Err(RuntimeError::StackUnderflow { required: 3, actual: *sp });
    }
    
    *sp -= 1;
    let vz = Fixed(stack[*sp]);
    *sp -= 1;
    let vy = Fixed(stack[*sp]);
    *sp -= 1;
    let vx = Fixed(stack[*sp]);
    
    let v = Vec3::new(vx, vy, vz);
    let result = Vec3::new(-v.x, -v.y, -v.z);
    
    stack[*sp] = result.x.0;
    *sp += 1;
    stack[*sp] = result.y.0;
    *sp += 1;
    stack[*sp] = result.z.0;
    *sp += 1;
    
    Ok(())
}

/// Execute MulVec3: component-wise multiplication, pop 6, push 3
#[inline(always)]
pub fn exec_mul_vec3(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 6 {
        return Err(RuntimeError::StackUnderflow { required: 6, actual: *sp });
    }
    
    *sp -= 1;
    let bz = Fixed(stack[*sp]);
    *sp -= 1;
    let by = Fixed(stack[*sp]);
    *sp -= 1;
    let bx = Fixed(stack[*sp]);
    *sp -= 1;
    let az = Fixed(stack[*sp]);
    *sp -= 1;
    let ay = Fixed(stack[*sp]);
    *sp -= 1;
    let ax = Fixed(stack[*sp]);
    
    let a = Vec3::new(ax, ay, az);
    let b = Vec3::new(bx, by, bz);
    let result = a.mul_comp(b);
    
    stack[*sp] = result.x.0;
    *sp += 1;
    stack[*sp] = result.y.0;
    *sp += 1;
    stack[*sp] = result.z.0;
    *sp += 1;
    
    Ok(())
}

/// Execute DivVec3: component-wise division, pop 6, push 3
#[inline(always)]
pub fn exec_div_vec3(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 6 {
        return Err(RuntimeError::StackUnderflow { required: 6, actual: *sp });
    }
    
    *sp -= 1;
    let bz = Fixed(stack[*sp]);
    *sp -= 1;
    let by = Fixed(stack[*sp]);
    *sp -= 1;
    let bx = Fixed(stack[*sp]);
    *sp -= 1;
    let az = Fixed(stack[*sp]);
    *sp -= 1;
    let ay = Fixed(stack[*sp]);
    *sp -= 1;
    let ax = Fixed(stack[*sp]);
    
    let a = Vec3::new(ax, ay, az);
    let b = Vec3::new(bx, by, bz);
    let result = a.div_comp(b);
    
    stack[*sp] = result.x.0;
    *sp += 1;
    stack[*sp] = result.y.0;
    *sp += 1;
    stack[*sp] = result.z.0;
    *sp += 1;
    
    Ok(())
}

/// Execute ModVec3: component-wise modulo, pop 6, push 3
#[inline(always)]
pub fn exec_mod_vec3(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 6 {
        return Err(RuntimeError::StackUnderflow { required: 6, actual: *sp });
    }
    
    *sp -= 1;
    let bz = Fixed(stack[*sp]);
    *sp -= 1;
    let by = Fixed(stack[*sp]);
    *sp -= 1;
    let bx = Fixed(stack[*sp]);
    *sp -= 1;
    let az = Fixed(stack[*sp]);
    *sp -= 1;
    let ay = Fixed(stack[*sp]);
    *sp -= 1;
    let ax = Fixed(stack[*sp]);
    
    // Component-wise modulo: (ax % bx, ay % by, az % bz)
    let result_x = crate::math::modulo(ax, bx);
    let result_y = crate::math::modulo(ay, by);
    let result_z = crate::math::modulo(az, bz);
    
    stack[*sp] = result_x.0;
    *sp += 1;
    stack[*sp] = result_y.0;
    *sp += 1;
    stack[*sp] = result_z.0;
    *sp += 1;
    
    Ok(())
}

/// Execute MulVec3Scalar: pop 4 (scalar, vec.z, vec.y, vec.x), push 3 (result.z, result.y, result.x)
#[inline(always)]
pub fn exec_mul_vec3_scalar(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 4 {
        return Err(RuntimeError::StackUnderflow { required: 4, actual: *sp });
    }
    
    *sp -= 1;
    let scalar = Fixed(stack[*sp]);
    *sp -= 1;
    let vz = Fixed(stack[*sp]);
    *sp -= 1;
    let vy = Fixed(stack[*sp]);
    *sp -= 1;
    let vx = Fixed(stack[*sp]);
    
    let v = Vec3::new(vx, vy, vz);
    let result = v * scalar;
    
    stack[*sp] = result.x.0;
    *sp += 1;
    stack[*sp] = result.y.0;
    *sp += 1;
    stack[*sp] = result.z.0;
    *sp += 1;
    
    Ok(())
}

/// Execute DivVec3Scalar: pop 4 (scalar, vec.z, vec.y, vec.x), push 3 (result.z, result.y, result.x)
#[inline(always)]
pub fn exec_div_vec3_scalar(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 4 {
        return Err(RuntimeError::StackUnderflow { required: 4, actual: *sp });
    }
    
    *sp -= 1;
    let scalar = Fixed(stack[*sp]);
    *sp -= 1;
    let vz = Fixed(stack[*sp]);
    *sp -= 1;
    let vy = Fixed(stack[*sp]);
    *sp -= 1;
    let vx = Fixed(stack[*sp]);
    
    let v = Vec3::new(vx, vy, vz);
    let result = v / scalar;
    
    stack[*sp] = result.x.0;
    *sp += 1;
    stack[*sp] = result.y.0;
    *sp += 1;
    stack[*sp] = result.z.0;
    *sp += 1;
    
    Ok(())
}

/// Execute Dot3: pop 6 (b.z, b.y, b.x, a.z, a.y, a.x), push 1 (a·b)
#[inline(always)]
pub fn exec_dot3(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 6 {
        return Err(RuntimeError::StackUnderflow { required: 6, actual: *sp });
    }
    
    *sp -= 1;
    let bz = Fixed(stack[*sp]);
    *sp -= 1;
    let by = Fixed(stack[*sp]);
    *sp -= 1;
    let bx = Fixed(stack[*sp]);
    *sp -= 1;
    let az = Fixed(stack[*sp]);
    *sp -= 1;
    let ay = Fixed(stack[*sp]);
    *sp -= 1;
    let ax = Fixed(stack[*sp]);
    
    let a = Vec3::new(ax, ay, az);
    let b = Vec3::new(bx, by, bz);
    let dot = a.dot(b);
    
    stack[*sp] = dot.0;
    *sp += 1;
    
    Ok(())
}

/// Execute Cross3: pop 6 (b.z, b.y, b.x, a.z, a.y, a.x), push 3 (cross product)
#[inline(always)]
pub fn exec_cross3(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 6 {
        return Err(RuntimeError::StackUnderflow { required: 6, actual: *sp });
    }
    
    *sp -= 1;
    let bz = Fixed(stack[*sp]);
    *sp -= 1;
    let by = Fixed(stack[*sp]);
    *sp -= 1;
    let bx = Fixed(stack[*sp]);
    *sp -= 1;
    let az = Fixed(stack[*sp]);
    *sp -= 1;
    let ay = Fixed(stack[*sp]);
    *sp -= 1;
    let ax = Fixed(stack[*sp]);
    
    let a = Vec3::new(ax, ay, az);
    let b = Vec3::new(bx, by, bz);
    let result = a.cross(b);
    
    stack[*sp] = result.x.0;
    *sp += 1;
    stack[*sp] = result.y.0;
    *sp += 1;
    stack[*sp] = result.z.0;
    *sp += 1;
    
    Ok(())
}

/// Execute Length3: pop 3 (v.z, v.y, v.x), push 1 (length)
#[inline(always)]
pub fn exec_length3(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 3 {
        return Err(RuntimeError::StackUnderflow { required: 3, actual: *sp });
    }
    
    *sp -= 1;
    let vz = Fixed(stack[*sp]);
    *sp -= 1;
    let vy = Fixed(stack[*sp]);
    *sp -= 1;
    let vx = Fixed(stack[*sp]);
    
    let v = Vec3::new(vx, vy, vz);
    let len = v.length();
    
    stack[*sp] = len.0;
    *sp += 1;
    
    Ok(())
}

/// Execute Normalize3: pop 3 (v.z, v.y, v.x), push 3 (normalized.z, normalized.y, normalized.x)
#[inline(always)]
pub fn exec_normalize3(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 3 {
        return Err(RuntimeError::StackUnderflow { required: 3, actual: *sp });
    }
    
    *sp -= 1;
    let vz = Fixed(stack[*sp]);
    *sp -= 1;
    let vy = Fixed(stack[*sp]);
    *sp -= 1;
    let vx = Fixed(stack[*sp]);
    
    let v = Vec3::new(vx, vy, vz);
    let result = v.normalize();
    
    stack[*sp] = result.x.0;
    *sp += 1;
    stack[*sp] = result.y.0;
    *sp += 1;
    stack[*sp] = result.z.0;
    *sp += 1;
    
    Ok(())
}

/// Execute Distance3: pop 6 (two vec3s), push 1 (distance)
#[inline(always)]
pub fn exec_distance3(stack: &mut [i32], sp: &mut usize) -> Result<(), RuntimeError> {
    if *sp < 6 {
        return Err(RuntimeError::StackUnderflow { required: 6, actual: *sp });
    }
    
    *sp -= 1;
    let bz = Fixed(stack[*sp]);
    *sp -= 1;
    let by = Fixed(stack[*sp]);
    *sp -= 1;
    let bx = Fixed(stack[*sp]);
    *sp -= 1;
    let az = Fixed(stack[*sp]);
    *sp -= 1;
    let ay = Fixed(stack[*sp]);
    *sp -= 1;
    let ax = Fixed(stack[*sp]);
    
    let a = Vec3::new(ax, ay, az);
    let b = Vec3::new(bx, by, bz);
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
    fn test_add_vec3() {
        let mut stack = [0i32; 64];
        let mut sp = 0;
        
        // Push vec3(1, 2, 3)
        stack[sp] = 1.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 2.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 3.0f32.to_fixed().0;
        sp += 1;
        
        // Push vec3(4, 5, 6)
        stack[sp] = 4.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 5.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 6.0f32.to_fixed().0;
        sp += 1;
        
        exec_add_vec3(&mut stack, &mut sp).unwrap();
        
        assert_eq!(sp, 3);
        assert_eq!(Fixed(stack[0]).to_f32(), 5.0);
        assert_eq!(Fixed(stack[1]).to_f32(), 7.0);
        assert_eq!(Fixed(stack[2]).to_f32(), 9.0);
    }
    
    #[test]
    fn test_sub_vec3() {
        let mut stack = [0i32; 64];
        let mut sp = 0;
        
        // Push vec3(10, 20, 30)
        stack[sp] = 10.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 20.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 30.0f32.to_fixed().0;
        sp += 1;
        
        // Push vec3(1, 2, 3)
        stack[sp] = 1.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 2.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 3.0f32.to_fixed().0;
        sp += 1;
        
        exec_sub_vec3(&mut stack, &mut sp).unwrap();
        
        assert_eq!(sp, 3);
        assert_eq!(Fixed(stack[0]).to_f32(), 9.0);
        assert_eq!(Fixed(stack[1]).to_f32(), 18.0);
        assert_eq!(Fixed(stack[2]).to_f32(), 27.0);
    }
    
    #[test]
    fn test_dot3() {
        let mut stack = [0i32; 64];
        let mut sp = 0;
        
        // Push vec3(1, 2, 3)
        stack[sp] = 1.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 2.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 3.0f32.to_fixed().0;
        sp += 1;
        
        // Push vec3(4, 5, 6)
        stack[sp] = 4.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 5.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 6.0f32.to_fixed().0;
        sp += 1;
        
        exec_dot3(&mut stack, &mut sp).unwrap();
        
        assert_eq!(sp, 1);
        // 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32
        assert_eq!(Fixed(stack[0]).to_f32(), 32.0);
    }
    
    #[test]
    fn test_cross3() {
        let mut stack = [0i32; 64];
        let mut sp = 0;
        
        // Push vec3(1, 0, 0)
        stack[sp] = 1.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 0.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 0.0f32.to_fixed().0;
        sp += 1;
        
        // Push vec3(0, 1, 0)
        stack[sp] = 0.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 1.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 0.0f32.to_fixed().0;
        sp += 1;
        
        exec_cross3(&mut stack, &mut sp).unwrap();
        
        assert_eq!(sp, 3);
        // (1,0,0) × (0,1,0) = (0,0,1)
        assert_eq!(Fixed(stack[0]).to_f32(), 0.0);
        assert_eq!(Fixed(stack[1]).to_f32(), 0.0);
        assert_eq!(Fixed(stack[2]).to_f32(), 1.0);
    }
    
    #[test]
    fn test_length3() {
        let mut stack = [0i32; 64];
        let mut sp = 0;
        
        // Push vec3(3, 4, 0) - length should be 5
        stack[sp] = 3.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 4.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 0.0f32.to_fixed().0;
        sp += 1;
        
        exec_length3(&mut stack, &mut sp).unwrap();
        
        assert_eq!(sp, 1);
        assert!((Fixed(stack[0]).to_f32() - 5.0).abs() < 0.01);
    }
    
    #[test]
    fn test_mul_vec3_scalar() {
        let mut stack = [0i32; 64];
        let mut sp = 0;
        
        // Push vec3(1, 2, 3)
        stack[sp] = 1.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 2.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 3.0f32.to_fixed().0;
        sp += 1;
        
        // Push scalar 5
        stack[sp] = 5.0f32.to_fixed().0;
        sp += 1;
        
        exec_mul_vec3_scalar(&mut stack, &mut sp).unwrap();
        
        assert_eq!(sp, 3);
        assert_eq!(Fixed(stack[0]).to_f32(), 5.0);
        assert_eq!(Fixed(stack[1]).to_f32(), 10.0);
        assert_eq!(Fixed(stack[2]).to_f32(), 15.0);
    }
    
    #[test]
    fn test_normalize3() {
        let mut stack = [0i32; 64];
        let mut sp = 0;
        
        // Push vec3(3, 4, 0)
        stack[sp] = 3.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 4.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 0.0f32.to_fixed().0;
        sp += 1;
        
        exec_normalize3(&mut stack, &mut sp).unwrap();
        
        assert_eq!(sp, 3);
        // Normalized should be (0.6, 0.8, 0.0)
        assert!((Fixed(stack[0]).to_f32() - 0.6).abs() < 0.01);
        assert!((Fixed(stack[1]).to_f32() - 0.8).abs() < 0.01);
        assert!((Fixed(stack[2]).to_f32() - 0.0).abs() < 0.01);
    }
    
    #[test]
    fn test_distance3() {
        let mut stack = [0i32; 64];
        let mut sp = 0;
        
        // Push vec3(0, 0, 0)
        stack[sp] = 0.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 0.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 0.0f32.to_fixed().0;
        sp += 1;
        
        // Push vec3(3, 4, 0)
        stack[sp] = 3.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 4.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 0.0f32.to_fixed().0;
        sp += 1;
        
        exec_distance3(&mut stack, &mut sp).unwrap();
        
        assert_eq!(sp, 1);
        assert!((Fixed(stack[0]).to_f32() - 5.0).abs() < 0.01);
    }
}

