/// Vec3 operations
use crate::vm::error::LpsVmError;
use crate::vm::value_stack::ValueStack;
use crate::fixed::{modulo, Vec3};

#[inline(always)]
pub fn exec_add_vec3(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let b = stack.pop_vec3()?;
    let a = stack.pop_vec3()?;
    stack.push_vec3(a + b)?;
    Ok(())
}

#[inline(always)]
pub fn exec_sub_vec3(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let b = stack.pop_vec3()?;
    let a = stack.pop_vec3()?;
    stack.push_vec3(a - b)?;
    Ok(())
}

#[inline(always)]
pub fn exec_neg_vec3(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let a = stack.pop_vec3()?;
    stack.push_vec3(-a)?;
    Ok(())
}

#[inline(always)]
pub fn exec_mul_vec3(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let b = stack.pop_vec3()?;
    let a = stack.pop_vec3()?;
    stack.push_vec3(Vec3::new(a.x * b.x, a.y * b.y, a.z * b.z))?;
    Ok(())
}

#[inline(always)]
pub fn exec_div_vec3(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let b = stack.pop_vec3()?;
    let a = stack.pop_vec3()?;
    stack.push_vec3(Vec3::new(a.x / b.x, a.y / b.y, a.z / b.z))?;
    Ok(())
}

#[inline(always)]
pub fn exec_mod_vec3(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let b = stack.pop_vec3()?;
    let a = stack.pop_vec3()?;
    stack.push_vec3(Vec3::new(
        modulo(a.x, b.x),
        modulo(a.y, b.y),
        modulo(a.z, b.z),
    ))?;
    Ok(())
}

#[inline(always)]
pub fn exec_mul_vec3_scalar(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let scalar = stack.pop_fixed()?;
    let vec = stack.pop_vec3()?;
    stack.push_vec3(vec * scalar)?;
    Ok(())
}

#[inline(always)]
pub fn exec_div_vec3_scalar(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let scalar = stack.pop_fixed()?;
    let vec = stack.pop_vec3()?;
    stack.push_vec3(vec / scalar)?;
    Ok(())
}

#[inline(always)]
pub fn exec_dot3(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let b = stack.pop_vec3()?;
    let a = stack.pop_vec3()?;
    stack.push_fixed(a.dot(b))?;
    Ok(())
}

#[inline(always)]
pub fn exec_cross3(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let b = stack.pop_vec3()?;
    let a = stack.pop_vec3()?;
    stack.push_vec3(a.cross(b))?;
    Ok(())
}

#[inline(always)]
pub fn exec_length3(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let a = stack.pop_vec3()?;
    stack.push_fixed(a.length())?;
    Ok(())
}

#[inline(always)]
pub fn exec_normalize3(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let a = stack.pop_vec3()?;
    stack.push_vec3(a.normalize())?;
    Ok(())
}

#[inline(always)]
pub fn exec_distance3(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let b = stack.pop_vec3()?;
    let a = stack.pop_vec3()?;
    stack.push_fixed(a.distance(b))?;
    Ok(())
}
