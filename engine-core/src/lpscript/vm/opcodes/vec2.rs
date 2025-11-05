/// Vec2 operations
use crate::lpscript::vm::error::LpsVmError;
use crate::lpscript::vm::value_stack::ValueStack;
use crate::math::{modulo, Vec2};

#[inline(always)]
pub fn exec_add_vec2(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let b = stack.pop_vec2()?;
    let a = stack.pop_vec2()?;
    stack.push_vec2(a + b)?;
    Ok(())
}

#[inline(always)]
pub fn exec_sub_vec2(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let b = stack.pop_vec2()?;
    let a = stack.pop_vec2()?;
    stack.push_vec2(a - b)?;
    Ok(())
}

#[inline(always)]
pub fn exec_neg_vec2(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let a = stack.pop_vec2()?;
    stack.push_vec2(-a)?;
    Ok(())
}

#[inline(always)]
pub fn exec_mul_vec2(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let b = stack.pop_vec2()?;
    let a = stack.pop_vec2()?;
    stack.push_vec2(Vec2::new(a.x * b.x, a.y * b.y))?;
    Ok(())
}

#[inline(always)]
pub fn exec_div_vec2(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let b = stack.pop_vec2()?;
    let a = stack.pop_vec2()?;
    stack.push_vec2(Vec2::new(a.x / b.x, a.y / b.y))?;
    Ok(())
}

#[inline(always)]
pub fn exec_mod_vec2(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let b = stack.pop_vec2()?;
    let a = stack.pop_vec2()?;
    stack.push_vec2(Vec2::new(modulo(a.x, b.x), modulo(a.y, b.y)))?;
    Ok(())
}

#[inline(always)]
pub fn exec_mul_vec2_scalar(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let scalar = stack.pop_fixed()?;
    let vec = stack.pop_vec2()?;
    stack.push_vec2(vec * scalar)?;
    Ok(())
}

#[inline(always)]
pub fn exec_div_vec2_scalar(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let scalar = stack.pop_fixed()?;
    let vec = stack.pop_vec2()?;
    stack.push_vec2(vec / scalar)?;
    Ok(())
}

#[inline(always)]
pub fn exec_dot2(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let b = stack.pop_vec2()?;
    let a = stack.pop_vec2()?;
    stack.push_fixed(a.dot(b))?;
    Ok(())
}

#[inline(always)]
pub fn exec_length2(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let a = stack.pop_vec2()?;
    stack.push_fixed(a.length())?;
    Ok(())
}

#[inline(always)]
pub fn exec_normalize2(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let a = stack.pop_vec2()?;
    stack.push_vec2(a.normalize())?;
    Ok(())
}

#[inline(always)]
pub fn exec_distance2(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let b = stack.pop_vec2()?;
    let a = stack.pop_vec2()?;
    stack.push_fixed(a.distance(b))?;
    Ok(())
}
