/// Local variable operations
use crate::lpscript::error::RuntimeError;
use crate::lpscript::vm::locals::LocalType;
use crate::math::Fixed;
extern crate alloc;
use alloc::vec::Vec;

/// Execute LoadLocalFixed: pop nothing; push local[idx]
#[inline(always)]
pub fn exec_load_local_fixed(
    stack: &mut [i32],
    sp: &mut usize,
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
            if *sp >= 64 {
                return Err(RuntimeError::StackOverflow { sp: *sp });
            }
            stack[*sp] = val.0;
            *sp += 1;
            Ok(())
        }
        _ => Err(RuntimeError::TypeMismatch),
    }
}

/// Execute StoreLocalFixed: pop value; store to local[idx]
#[inline(always)]
pub fn exec_store_local_fixed(
    stack: &mut [i32],
    sp: &mut usize,
    locals: &mut Vec<LocalType>,
    idx: u32,
) -> Result<(), RuntimeError> {
    if *sp < 1 {
        return Err(RuntimeError::StackUnderflow {
            required: 1,
            actual: *sp,
        });
    }

    *sp -= 1;
    let val = Fixed(stack[*sp]);

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
    stack: &mut [i32],
    sp: &mut usize,
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
            if *sp >= 64 {
                return Err(RuntimeError::StackOverflow { sp: *sp });
            }
            stack[*sp] = val;
            *sp += 1;
            Ok(())
        }
        _ => Err(RuntimeError::TypeMismatch),
    }
}

/// Execute StoreLocalInt32: pop value; store to local[idx]
#[inline(always)]
pub fn exec_store_local_int32(
    stack: &mut [i32],
    sp: &mut usize,
    locals: &mut Vec<LocalType>,
    idx: u32,
) -> Result<(), RuntimeError> {
    if *sp < 1 {
        return Err(RuntimeError::StackUnderflow {
            required: 1,
            actual: *sp,
        });
    }

    *sp -= 1;
    let val = stack[*sp];

    if (idx as usize) >= locals.len() {
        return Err(RuntimeError::LocalOutOfBounds {
            local_idx: idx as usize,
            max: locals.len(),
        });
    }
    locals[idx as usize] = LocalType::Int32(val);

    Ok(())
}

/// Execute LoadLocalVec2: pop nothing; push local[idx] (x, y)
#[inline(always)]
pub fn exec_load_local_vec2(
    stack: &mut [i32],
    sp: &mut usize,
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
        LocalType::Vec2(x, y) => {
            if *sp >= 62 {
                return Err(RuntimeError::StackOverflow { sp: *sp });
            }
            stack[*sp] = x.0;
            *sp += 1;
            stack[*sp] = y.0;
            *sp += 1;
            Ok(())
        }
        _ => Err(RuntimeError::TypeMismatch),
    }
}

/// Execute StoreLocalVec2: pop y, x; store to local[idx]
#[inline(always)]
pub fn exec_store_local_vec2(
    stack: &mut [i32],
    sp: &mut usize,
    locals: &mut Vec<LocalType>,
    idx: u32,
) -> Result<(), RuntimeError> {
    if *sp < 2 {
        return Err(RuntimeError::StackUnderflow {
            required: 2,
            actual: *sp,
        });
    }

    *sp -= 1;
    let y = Fixed(stack[*sp]);
    *sp -= 1;
    let x = Fixed(stack[*sp]);

    if (idx as usize) >= locals.len() {
        return Err(RuntimeError::LocalOutOfBounds {
            local_idx: idx as usize,
            max: locals.len(),
        });
    }
    locals[idx as usize] = LocalType::Vec2(x, y);

    Ok(())
}

/// Execute LoadLocalVec3: pop nothing; push local[idx] (x, y, z)
#[inline(always)]
pub fn exec_load_local_vec3(
    stack: &mut [i32],
    sp: &mut usize,
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
        LocalType::Vec3(x, y, z) => {
            if *sp >= 61 {
                return Err(RuntimeError::StackOverflow { sp: *sp });
            }
            stack[*sp] = x.0;
            *sp += 1;
            stack[*sp] = y.0;
            *sp += 1;
            stack[*sp] = z.0;
            *sp += 1;
            Ok(())
        }
        _ => Err(RuntimeError::TypeMismatch),
    }
}

/// Execute StoreLocalVec3: pop z, y, x; store to local[idx]
#[inline(always)]
pub fn exec_store_local_vec3(
    stack: &mut [i32],
    sp: &mut usize,
    locals: &mut Vec<LocalType>,
    idx: u32,
) -> Result<(), RuntimeError> {
    if *sp < 3 {
        return Err(RuntimeError::StackUnderflow {
            required: 3,
            actual: *sp,
        });
    }

    *sp -= 1;
    let z = Fixed(stack[*sp]);
    *sp -= 1;
    let y = Fixed(stack[*sp]);
    *sp -= 1;
    let x = Fixed(stack[*sp]);

    if (idx as usize) >= locals.len() {
        return Err(RuntimeError::LocalOutOfBounds {
            local_idx: idx as usize,
            max: locals.len(),
        });
    }
    locals[idx as usize] = LocalType::Vec3(x, y, z);

    Ok(())
}

/// Execute LoadLocalVec4: pop nothing; push local[idx] (x, y, z, w)
#[inline(always)]
pub fn exec_load_local_vec4(
    stack: &mut [i32],
    sp: &mut usize,
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
        LocalType::Vec4(x, y, z, w) => {
            if *sp >= 60 {
                return Err(RuntimeError::StackOverflow { sp: *sp });
            }
            stack[*sp] = x.0;
            *sp += 1;
            stack[*sp] = y.0;
            *sp += 1;
            stack[*sp] = z.0;
            *sp += 1;
            stack[*sp] = w.0;
            *sp += 1;
            Ok(())
        }
        _ => Err(RuntimeError::TypeMismatch),
    }
}

/// Execute StoreLocalVec4: pop w, z, y, x; store to local[idx]
#[inline(always)]
pub fn exec_store_local_vec4(
    stack: &mut [i32],
    sp: &mut usize,
    locals: &mut Vec<LocalType>,
    idx: u32,
) -> Result<(), RuntimeError> {
    if *sp < 4 {
        return Err(RuntimeError::StackUnderflow {
            required: 4,
            actual: *sp,
        });
    }

    *sp -= 1;
    let w = Fixed(stack[*sp]);
    *sp -= 1;
    let z = Fixed(stack[*sp]);
    *sp -= 1;
    let y = Fixed(stack[*sp]);
    *sp -= 1;
    let x = Fixed(stack[*sp]);

    if (idx as usize) >= locals.len() {
        return Err(RuntimeError::LocalOutOfBounds {
            local_idx: idx as usize,
            max: locals.len(),
        });
    }
    locals[idx as usize] = LocalType::Vec4(x, y, z, w);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::ToFixed;

    #[test]
    fn test_load_store_local_fixed() {
        let mut stack = [0i32; 64];
        let mut sp = 0;
        let mut locals = Vec::new();
        locals.push(LocalType::Fixed(5.0f32.to_fixed()));

        // Load
        exec_load_local_fixed(&mut stack, &mut sp, &locals, 0).unwrap();
        assert_eq!(sp, 1);
        assert_eq!(Fixed(stack[0]).to_f32(), 5.0);

        // Store
        sp = 0;
        stack[sp] = 10.0f32.to_fixed().0;
        sp += 1;
        exec_store_local_fixed(&mut stack, &mut sp, &mut locals, 0).unwrap();
        assert_eq!(sp, 0);
        assert!(matches!(locals[0], LocalType::Fixed(val) if val.to_f32() == 10.0));
    }

    #[test]
    fn test_load_store_local_vec2() {
        let mut stack = [0i32; 64];
        let mut sp = 0;
        let mut locals = Vec::new();
        locals.push(LocalType::Vec2(
            1.0f32.to_fixed(),
            2.0f32.to_fixed(),
        ));

        // Load
        exec_load_local_vec2(&mut stack, &mut sp, &locals, 0).unwrap();
        assert_eq!(sp, 2);
        assert_eq!(Fixed(stack[0]).to_f32(), 1.0);
        assert_eq!(Fixed(stack[1]).to_f32(), 2.0);

        // Store
        sp = 0;
        stack[sp] = 3.0f32.to_fixed().0;
        sp += 1;
        stack[sp] = 4.0f32.to_fixed().0;
        sp += 1;
        exec_store_local_vec2(&mut stack, &mut sp, &mut locals, 0).unwrap();
        assert_eq!(sp, 0);
        assert!(matches!(
            locals[0],
            LocalType::Vec2(x, y) if x.to_f32() == 3.0 && y.to_f32() == 4.0
        ));
    }

    #[test]
    fn test_load_local_bounds_check() {
        let mut stack = [0i32; 64];
        let mut sp = 0;
        let locals = Vec::new();

        let result = exec_load_local_fixed(&mut stack, &mut sp, &locals, 0);
        assert!(matches!(
            result,
            Err(RuntimeError::LocalOutOfBounds {
                local_idx: 0,
                max: 0
            })
        ));
    }

    #[test]
    #[ignore] // Auto-grow removed to prevent memory leaks during run()
    fn test_store_local_auto_grow() {
        let mut stack = [0i32; 64];
        let mut sp = 0;
        let mut locals = Vec::new();

        stack[sp] = 5.0f32.to_fixed().0;
        sp += 1;

        exec_store_local_fixed(&mut stack, &mut sp, &mut locals, 2).unwrap();
        assert_eq!(locals.len(), 3);
        assert!(matches!(locals[2], LocalType::Fixed(val) if val.to_f32() == 5.0));
    }
}

