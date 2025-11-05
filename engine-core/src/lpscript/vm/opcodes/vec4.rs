/// Vec4 operations
use crate::lpscript::vm::error::LpsVmError;
use crate::lpscript::vm::value_stack::ValueStack;
use crate::math::{modulo, Vec4};

#[inline(always)]
pub fn exec_add_vec4(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let b = stack.pop_vec4()?;
    let a = stack.pop_vec4()?;
    stack.push_vec4(a + b)?;
    Ok(())
}

#[inline(always)]
pub fn exec_sub_vec4(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let b = stack.pop_vec4()?;
    let a = stack.pop_vec4()?;
    stack.push_vec4(a - b)?;
    Ok(())
}

#[inline(always)]
pub fn exec_neg_vec4(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let a = stack.pop_vec4()?;
    stack.push_vec4(-a)?;
    Ok(())
}

#[inline(always)]
pub fn exec_mul_vec4(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let b = stack.pop_vec4()?;
    let a = stack.pop_vec4()?;
    stack.push_vec4(Vec4::new(a.x * b.x, a.y * b.y, a.z * b.z, a.w * b.w))?;
    Ok(())
}

#[inline(always)]
pub fn exec_div_vec4(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let b = stack.pop_vec4()?;
    let a = stack.pop_vec4()?;
    stack.push_vec4(Vec4::new(a.x / b.x, a.y / b.y, a.z / b.z, a.w / b.w))?;
    Ok(())
}

#[inline(always)]
pub fn exec_mod_vec4(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let b = stack.pop_vec4()?;
    let a = stack.pop_vec4()?;
    stack.push_vec4(Vec4::new(
        modulo(a.x, b.x),
        modulo(a.y, b.y),
        modulo(a.z, b.z),
        modulo(a.w, b.w),
    ))?;
    Ok(())
}

#[inline(always)]
pub fn exec_mul_vec4_scalar(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let scalar = stack.pop_fixed()?;
    let vec = stack.pop_vec4()?;
    stack.push_vec4(vec * scalar)?;
    Ok(())
}

#[inline(always)]
pub fn exec_div_vec4_scalar(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let scalar = stack.pop_fixed()?;
    let vec = stack.pop_vec4()?;
    stack.push_vec4(vec / scalar)?;
    Ok(())
}

#[inline(always)]
pub fn exec_dot4(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let b = stack.pop_vec4()?;
    let a = stack.pop_vec4()?;
    stack.push_fixed(a.dot(b))?;
    Ok(())
}

#[inline(always)]
pub fn exec_length4(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let a = stack.pop_vec4()?;
    stack.push_fixed(a.length())?;
    Ok(())
}

#[inline(always)]
pub fn exec_normalize4(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let a = stack.pop_vec4()?;
    stack.push_vec4(a.normalize())?;
    Ok(())
}

#[inline(always)]
pub fn exec_distance4(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let b = stack.pop_vec4()?;
    let a = stack.pop_vec4()?;
    stack.push_fixed(a.distance(b))?;
    Ok(())
}
