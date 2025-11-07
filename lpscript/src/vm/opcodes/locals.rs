/// Local variable operations (updated for optimized storage)
use crate::vm::error::LpsVmError;
use crate::vm::local_stack::LocalStack;
use crate::vm::value_stack::ValueStack;
use crate::math::Fixed;

/// Execute LoadLocalFixed: pop nothing; push local[idx]
#[inline(always)]
pub fn exec_load_local_fixed(
    stack: &mut ValueStack,
    locals: &LocalStack,
    idx: usize,
) -> Result<(), LpsVmError> {
    let val = locals.get_fixed(idx)?;
    stack.push_fixed(val)?;
    Ok(())
}

/// Execute StoreLocalFixed: pop value; store to local[idx]
#[inline(always)]
pub fn exec_store_local_fixed(
    stack: &mut ValueStack,
    locals: &mut LocalStack,
    idx: usize,
) -> Result<(), LpsVmError> {
    let val = stack.pop_fixed()?;
    locals.set_fixed(idx, val)?;
    Ok(())
}

/// Execute LoadLocalInt32: pop nothing; push local[idx]
#[inline(always)]
pub fn exec_load_local_int32(
    stack: &mut ValueStack,
    locals: &LocalStack,
    idx: usize,
) -> Result<(), LpsVmError> {
    let val = locals.get_int32(idx)?;
    stack.push_int32(val)?;
    Ok(())
}

/// Execute StoreLocalInt32: pop value; store to local[idx]
#[inline(always)]
pub fn exec_store_local_int32(
    stack: &mut ValueStack,
    locals: &mut LocalStack,
    idx: usize,
) -> Result<(), LpsVmError> {
    let val = stack.pop_int32()?;
    locals.set_int32(idx, val)?;
    Ok(())
}

/// Execute LoadLocalVec2: pop nothing; push local[idx] as 2 Fixed
#[inline(always)]
pub fn exec_load_local_vec2(
    stack: &mut ValueStack,
    locals: &LocalStack,
    idx: usize,
) -> Result<(), LpsVmError> {
    let (x, y) = locals.get_vec2(idx)?;
    stack.push2(x.0, y.0)?;
    Ok(())
}

/// Execute StoreLocalVec2: pop 2 Fixed; store to local[idx]
#[inline(always)]
pub fn exec_store_local_vec2(
    stack: &mut ValueStack,
    locals: &mut LocalStack,
    idx: usize,
) -> Result<(), LpsVmError> {
    let (x, y) = stack.pop2()?;
    locals.set_vec2(idx, Fixed(x), Fixed(y))?;
    Ok(())
}

/// Execute LoadLocalVec3: pop nothing; push local[idx] as 3 Fixed
#[inline(always)]
pub fn exec_load_local_vec3(
    stack: &mut ValueStack,
    locals: &LocalStack,
    idx: usize,
) -> Result<(), LpsVmError> {
    let (x, y, z) = locals.get_vec3(idx)?;
    stack.push3(x.0, y.0, z.0)?;
    Ok(())
}

/// Execute StoreLocalVec3: pop 3 Fixed; store to local[idx]
#[inline(always)]
pub fn exec_store_local_vec3(
    stack: &mut ValueStack,
    locals: &mut LocalStack,
    idx: usize,
) -> Result<(), LpsVmError> {
    let (x, y, z) = stack.pop3()?;
    locals.set_vec3(idx, Fixed(x), Fixed(y), Fixed(z))?;
    Ok(())
}

/// Execute LoadLocalVec4: pop nothing; push local[idx] as 4 Fixed
#[inline(always)]
pub fn exec_load_local_vec4(
    stack: &mut ValueStack,
    locals: &LocalStack,
    idx: usize,
) -> Result<(), LpsVmError> {
    let (x, y, z, w) = locals.get_vec4(idx)?;
    stack.push4(x.0, y.0, z.0, w.0)?;
    Ok(())
}

/// Execute StoreLocalVec4: pop 4 Fixed; store to local[idx]
#[inline(always)]
pub fn exec_store_local_vec4(
    stack: &mut ValueStack,
    locals: &mut LocalStack,
    idx: usize,
) -> Result<(), LpsVmError> {
    let (x, y, z, w) = stack.pop4()?;
    locals.set_vec4(idx, Fixed(x), Fixed(y), Fixed(z), Fixed(w))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::Type;
    use crate::vm::lps_program::LocalVarDef;
    use crate::math::ToFixed;

    #[test]
    fn test_load_store_fixed() {
        let mut stack = ValueStack::new(64);
        let mut locals = LocalStack::new(64);
        
        // Allocate a Fixed local
        let defs = vec![LocalVarDef::new("x".into(), Type::Fixed)];
        locals.allocate_locals(&defs).unwrap();

        // Store
        stack.push_fixed(5.5.to_fixed()).unwrap();
        exec_store_local_fixed(&mut stack, &mut locals, 0).unwrap();

        // Load
        exec_load_local_fixed(&mut stack, &locals, 0).unwrap();
        assert_eq!(stack.pop_fixed().unwrap().to_f32(), 5.5);
    }

    #[test]
    fn test_load_store_int32() {
        let mut stack = ValueStack::new(64);
        let mut locals = LocalStack::new(64);

        // Allocate an Int32 local
        let defs = vec![LocalVarDef::new("count".into(), Type::Int32)];
        locals.allocate_locals(&defs).unwrap();

        // Store
        stack.push_int32(42).unwrap();
        exec_store_local_int32(&mut stack, &mut locals, 0).unwrap();

        // Load
        exec_load_local_int32(&mut stack, &locals, 0).unwrap();
        assert_eq!(stack.pop_int32().unwrap(), 42);
    }

    #[test]
    fn test_load_store_vec2() {
        let mut stack = ValueStack::new(64);
        let mut locals = LocalStack::new(64);

        // Allocate a Vec2 local
        let defs = vec![LocalVarDef::new("pos".into(), Type::Vec2)];
        locals.allocate_locals(&defs).unwrap();

        // Store
        stack.push2(1.0.to_fixed().0, 2.0.to_fixed().0).unwrap();
        exec_store_local_vec2(&mut stack, &mut locals, 0).unwrap();

        // Load
        exec_load_local_vec2(&mut stack, &locals, 0).unwrap();
        let (x, y) = stack.pop2().unwrap();
        assert_eq!(Fixed(x).to_f32(), 1.0);
        assert_eq!(Fixed(y).to_f32(), 2.0);
    }

    #[test]
    fn test_load_store_vec3() {
        let mut stack = ValueStack::new(64);
        let mut locals = LocalStack::new(64);

        // Allocate a Vec3 local
        let defs = vec![LocalVarDef::new("pos".into(), Type::Vec3)];
        locals.allocate_locals(&defs).unwrap();

        // Store
        stack
            .push3(1.0.to_fixed().0, 2.0.to_fixed().0, 3.0.to_fixed().0)
            .unwrap();
        exec_store_local_vec3(&mut stack, &mut locals, 0).unwrap();

        // Load
        exec_load_local_vec3(&mut stack, &locals, 0).unwrap();
        let (x, y, z) = stack.pop3().unwrap();
        assert_eq!(Fixed(x).to_f32(), 1.0);
        assert_eq!(Fixed(y).to_f32(), 2.0);
        assert_eq!(Fixed(z).to_f32(), 3.0);
    }

    #[test]
    fn test_load_store_vec4() {
        let mut stack = ValueStack::new(64);
        let mut locals = LocalStack::new(64);

        // Allocate a Vec4 local
        let defs = vec![LocalVarDef::new("color".into(), Type::Vec4)];
        locals.allocate_locals(&defs).unwrap();

        // Store
        stack
            .push4(
                1.0.to_fixed().0,
                2.0.to_fixed().0,
                3.0.to_fixed().0,
                4.0.to_fixed().0,
            )
            .unwrap();
        exec_store_local_vec4(&mut stack, &mut locals, 0).unwrap();

        // Load
        exec_load_local_vec4(&mut stack, &locals, 0).unwrap();
        let (x, y, z, w) = stack.pop4().unwrap();
        assert_eq!(Fixed(x).to_f32(), 1.0);
        assert_eq!(Fixed(y).to_f32(), 2.0);
        assert_eq!(Fixed(z).to_f32(), 3.0);
        assert_eq!(Fixed(w).to_f32(), 4.0);
    }

    #[test]
    fn test_local_out_of_bounds() {
        let mut stack = ValueStack::new(64);
        let locals = LocalStack::new(64);

        // No locals allocated
        let result = exec_load_local_fixed(&mut stack, &locals, 0);
        assert!(matches!(
            result,
            Err(LpsVmError::LocalOutOfBounds {
                local_idx: 0,
                max: 0
            })
        ));
    }

    #[test]
    fn test_type_mismatch() {
        let mut stack = ValueStack::new(64);
        let mut locals = LocalStack::new(64);

        // Allocate a Fixed local
        let defs = vec![LocalVarDef::new("x".into(), Type::Fixed)];
        locals.allocate_locals(&defs).unwrap();

        // Try to load as Int32 when it's Fixed
        let result = exec_load_local_int32(&mut stack, &locals, 0);
        assert!(matches!(result, Err(LpsVmError::TypeMismatch)));
    }

    #[test]
    fn test_multiple_locals() {
        let mut stack = ValueStack::new(64);
        let mut locals = LocalStack::new(64);

        // Allocate multiple locals of different types
        let defs = vec![
            LocalVarDef::new("x".into(), Type::Fixed),
            LocalVarDef::new("pos".into(), Type::Vec2),
            LocalVarDef::new("count".into(), Type::Int32),
        ];
        locals.allocate_locals(&defs).unwrap();

        // Store values
        stack.push_fixed(10.5.to_fixed()).unwrap();
        exec_store_local_fixed(&mut stack, &mut locals, 0).unwrap();

        stack.push2(1.0.to_fixed().0, 2.0.to_fixed().0).unwrap();
        exec_store_local_vec2(&mut stack, &mut locals, 1).unwrap();

        stack.push_int32(99).unwrap();
        exec_store_local_int32(&mut stack, &mut locals, 2).unwrap();

        // Load and verify
        exec_load_local_fixed(&mut stack, &locals, 0).unwrap();
        assert_eq!(stack.pop_fixed().unwrap().to_f32(), 10.5);

        exec_load_local_vec2(&mut stack, &locals, 1).unwrap();
        let (x, y) = stack.pop2().unwrap();
        assert_eq!(Fixed(x).to_f32(), 1.0);
        assert_eq!(Fixed(y).to_f32(), 2.0);

        exec_load_local_int32(&mut stack, &locals, 2).unwrap();
        assert_eq!(stack.pop_int32().unwrap(), 99);
    }
}
