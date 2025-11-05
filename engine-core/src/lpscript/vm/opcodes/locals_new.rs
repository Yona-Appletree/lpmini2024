/// Local variable operations
extern crate alloc;
use alloc::vec::Vec;

use crate::lpscript::vm::error::RuntimeError;
use crate::lpscript::vm::locals::LocalType;
use crate::lpscript::vm::vm_stack::Stack;
use crate::math::Fixed;

/// Execute LoadLocalFixed: pop nothing; push local[idx]
#[inline(always)]
pub fn exec_load_local_fixed(
    stack: &mut Stack,
    locals: &[LocalType],
    idx: u32,
) -> Result<(), RuntimeError> {
    if (idx as usize) >= locals.len() {
        return Err(RuntimeError::LocalOutOfBounds {
            local_idx: idx as usize,
            max: locals.len(),
        });
    }

    match locals[idx as usize] {
        LocalType::Fixed(val) => {
            stack.push_fixed(val)?;
            Ok(())
        }
        _ => Err(RuntimeError::TypeMismatch),
    }
}

/// Execute StoreLocalFixed: pop value; store to local[idx]
#[inline(always)]
pub fn exec_store_local_fixed(
    stack: &mut Stack,
    locals: &mut Vec<LocalType>,
    idx: u32,
) -> Result<(), RuntimeError> {
    let val = stack.pop_fixed()?;

    if (idx as usize) >= locals.len() {
        return Err(RuntimeError::LocalOutOfBounds {
            local_idx: idx as usize,
            max: locals.len(),
        });
    }

    locals[idx as usize] = LocalType::Fixed(val);

    Ok(())
}

/// Execute LoadLocalInt32: pop nothing; push local[idx]
#[inline(always)]
pub fn exec_load_local_int32(
    stack: &mut Stack,
    locals: &[LocalType],
    idx: u32,
) -> Result<(), RuntimeError> {
    if (idx as usize) >= locals.len() {
        return Err(RuntimeError::LocalOutOfBounds {
            local_idx: idx as usize,
            max: locals.len(),
        });
    }

    match locals[idx as usize] {
        LocalType::Int32(val) => {
            stack.push_int32(val)?;
            Ok(())
        }
        _ => Err(RuntimeError::TypeMismatch),
    }
}

/// Execute StoreLocalInt32: pop value; store to local[idx]
#[inline(always)]
pub fn exec_store_local_int32(
    stack: &mut Stack,
    locals: &mut Vec<LocalType>,
    idx: u32,
) -> Result<(), RuntimeError> {
    let val = stack.pop_int32()?;

    if (idx as usize) >= locals.len() {
        return Err(RuntimeError::LocalOutOfBounds {
            local_idx: idx as usize,
            max: locals.len(),
        });
    }

    locals[idx as usize] = LocalType::Int32(val);

    Ok(())
}

/// Execute LoadLocalVec2: pop nothing; push local[idx] as 2 Fixed
#[inline(always)]
pub fn exec_load_local_vec2(
    stack: &mut Stack,
    locals: &[LocalType],
    idx: u32,
) -> Result<(), RuntimeError> {
    if (idx as usize) >= locals.len() {
        return Err(RuntimeError::LocalOutOfBounds {
            local_idx: idx as usize,
            max: locals.len(),
        });
    }

    match locals[idx as usize] {
        LocalType::Vec2(val) => {
            stack.push_vec2(val)?;
            Ok(())
        }
        _ => Err(RuntimeError::TypeMismatch),
    }
}

/// Execute StoreLocalVec2: pop 2 Fixed; store to local[idx]
#[inline(always)]
pub fn exec_store_local_vec2(
    stack: &mut Stack,
    locals: &mut Vec<LocalType>,
    idx: u32,
) -> Result<(), RuntimeError> {
    let val = stack.pop_vec2()?;

    if (idx as usize) >= locals.len() {
        return Err(RuntimeError::LocalOutOfBounds {
            local_idx: idx as usize,
            max: locals.len(),
        });
    }

    locals[idx as usize] = LocalType::Vec2(val);

    Ok(())
}

/// Execute LoadLocalVec3: pop nothing; push local[idx] as 3 Fixed
#[inline(always)]
pub fn exec_load_local_vec3(
    stack: &mut Stack,
    locals: &[LocalType],
    idx: u32,
) -> Result<(), RuntimeError> {
    if (idx as usize) >= locals.len() {
        return Err(RuntimeError::LocalOutOfBounds {
            local_idx: idx as usize,
            max: locals.len(),
        });
    }

    match locals[idx as usize] {
        LocalType::Vec3(val) => {
            stack.push_vec3(val)?;
            Ok(())
        }
        _ => Err(RuntimeError::TypeMismatch),
    }
}

/// Execute StoreLocalVec3: pop 3 Fixed; store to local[idx]
#[inline(always)]
pub fn exec_store_local_vec3(
    stack: &mut Stack,
    locals: &mut Vec<LocalType>,
    idx: u32,
) -> Result<(), RuntimeError> {
    let val = stack.pop_vec3()?;

    if (idx as usize) >= locals.len() {
        return Err(RuntimeError::LocalOutOfBounds {
            local_idx: idx as usize,
            max: locals.len(),
        });
    }

    locals[idx as usize] = LocalType::Vec3(val);

    Ok(())
}

/// Execute LoadLocalVec4: pop nothing; push local[idx] as 4 Fixed
#[inline(always)]
pub fn exec_load_local_vec4(
    stack: &mut Stack,
    locals: &[LocalType],
    idx: u32,
) -> Result<(), RuntimeError> {
    if (idx as usize) >= locals.len() {
        return Err(RuntimeError::LocalOutOfBounds {
            local_idx: idx as usize,
            max: locals.len(),
        });
    }

    match locals[idx as usize] {
        LocalType::Vec4(val) => {
            stack.push_vec4(val)?;
            Ok(())
        }
        _ => Err(RuntimeError::TypeMismatch),
    }
}

/// Execute StoreLocalVec4: pop 4 Fixed; store to local[idx]
#[inline(always)]
pub fn exec_store_local_vec4(
    stack: &mut Stack,
    locals: &mut Vec<LocalType>,
    idx: u32,
) -> Result<(), RuntimeError> {
    let val = stack.pop_vec4()?;

    if (idx as usize) >= locals.len() {
        return Err(RuntimeError::LocalOutOfBounds {
            local_idx: idx as usize,
            max: locals.len(),
        });
    }

    locals[idx as usize] = LocalType::Vec4(val);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::{ToFixed, Vec2, Vec3, Vec4};

    #[test]
    fn test_load_store_fixed() {
        let mut stack = Stack::new(64);
        let mut locals = vec![LocalType::Fixed(Fixed::ZERO); 10];

        // Store
        stack.push_fixed(5.5.to_fixed()).unwrap();
        exec_store_local_fixed(&mut stack, &mut locals, 3).unwrap();

        // Load
        exec_load_local_fixed(&mut stack, &locals, 3).unwrap();
        assert_eq!(stack.pop_fixed().unwrap().to_f32(), 5.5);
    }

    #[test]
    fn test_load_store_int32() {
        let mut stack = Stack::new(64);
        let mut locals = vec![LocalType::Int32(0); 10];

        // Store
        stack.push_int32(42).unwrap();
        exec_store_local_int32(&mut stack, &mut locals, 5).unwrap();

        // Load
        exec_load_local_int32(&mut stack, &locals, 5).unwrap();
        assert_eq!(stack.pop_int32().unwrap(), 42);
    }

    #[test]
    fn test_load_store_vec2() {
        let mut stack = Stack::new(64);
        let mut locals = vec![LocalType::Vec2(Vec2::ZERO); 10];

        // Store
        stack
            .push_vec2(Vec2::new(1.0.to_fixed(), 2.0.to_fixed()))
            .unwrap();
        exec_store_local_vec2(&mut stack, &mut locals, 2).unwrap();

        // Load
        exec_load_local_vec2(&mut stack, &locals, 2).unwrap();
        let val = stack.pop_vec2().unwrap();
        assert_eq!(val.x.to_f32(), 1.0);
        assert_eq!(val.y.to_f32(), 2.0);
    }

    #[test]
    fn test_load_store_vec3() {
        let mut stack = Stack::new(64);
        let mut locals = vec![LocalType::Vec3(Vec3::ZERO); 10];

        // Store
        stack
            .push_vec3(Vec3::new(1.0.to_fixed(), 2.0.to_fixed(), 3.0.to_fixed()))
            .unwrap();
        exec_store_local_vec3(&mut stack, &mut locals, 1).unwrap();

        // Load
        exec_load_local_vec3(&mut stack, &locals, 1).unwrap();
        let val = stack.pop_vec3().unwrap();
        assert_eq!(val.x.to_f32(), 1.0);
        assert_eq!(val.y.to_f32(), 2.0);
        assert_eq!(val.z.to_f32(), 3.0);
    }

    #[test]
    fn test_load_store_vec4() {
        let mut stack = Stack::new(64);
        let mut locals = vec![LocalType::Vec4(Vec4::ZERO); 10];

        // Store
        stack
            .push_vec4(Vec4::new(
                1.0.to_fixed(),
                2.0.to_fixed(),
                3.0.to_fixed(),
                4.0.to_fixed(),
            ))
            .unwrap();
        exec_store_local_vec4(&mut stack, &mut locals, 0).unwrap();

        // Load
        exec_load_local_vec4(&mut stack, &locals, 0).unwrap();
        let val = stack.pop_vec4().unwrap();
        assert_eq!(val.x.to_f32(), 1.0);
        assert_eq!(val.y.to_f32(), 2.0);
        assert_eq!(val.z.to_f32(), 3.0);
        assert_eq!(val.w.to_f32(), 4.0);
    }

    #[test]
    fn test_local_out_of_bounds() {
        let mut stack = Stack::new(64);
        let locals = vec![LocalType::Fixed(Fixed::ZERO); 5];

        let result = exec_load_local_fixed(&mut stack, &locals, 10);
        assert!(matches!(
            result,
            Err(RuntimeError::LocalOutOfBounds {
                local_idx: 10,
                max: 5
            })
        ));
    }

    #[test]
    fn test_type_mismatch() {
        let mut stack = Stack::new(64);
        let locals = vec![LocalType::Int32(42)];

        // Try to load as Fixed when it's Int32
        let result = exec_load_local_fixed(&mut stack, &locals, 0);
        assert!(matches!(result, Err(RuntimeError::TypeMismatch)));
    }
}

