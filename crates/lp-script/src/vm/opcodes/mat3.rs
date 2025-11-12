use crate::fixed::Mat3;
/// Mat3 operations
use crate::vm::error::LpsVmError;
use crate::vm::value_stack::ValueStack;

#[inline(always)]
pub fn exec_add_mat3(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let b = stack.pop_mat3()?;
    let a = stack.pop_mat3()?;
    stack.push_mat3(a + b)?;
    Ok(())
}

#[inline(always)]
pub fn exec_sub_mat3(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let b = stack.pop_mat3()?;
    let a = stack.pop_mat3()?;
    stack.push_mat3(a - b)?;
    Ok(())
}

#[inline(always)]
pub fn exec_neg_mat3(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let a = stack.pop_mat3()?;
    stack.push_mat3(-a)?;
    Ok(())
}

#[inline(always)]
pub fn exec_mul_mat3(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let b = stack.pop_mat3()?;
    let a = stack.pop_mat3()?;
    stack.push_mat3(a * b)?;
    Ok(())
}

#[inline(always)]
pub fn exec_mul_mat3_scalar(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let scalar = stack.pop_fixed()?;
    let mat = stack.pop_mat3()?;
    stack.push_mat3(mat * scalar)?;
    Ok(())
}

#[inline(always)]
pub fn exec_div_mat3_scalar(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let scalar = stack.pop_fixed()?;
    let mat = stack.pop_mat3()?;
    stack.push_mat3(mat / scalar)?;
    Ok(())
}

#[inline(always)]
pub fn exec_mul_mat3_vec3(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let vec = stack.pop_vec3()?;
    let mat = stack.pop_mat3()?;
    stack.push_vec3(mat * vec)?;
    Ok(())
}

#[inline(always)]
pub fn exec_transpose_mat3(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let a = stack.pop_mat3()?;
    stack.push_mat3(a.transpose())?;
    Ok(())
}

#[inline(always)]
pub fn exec_determinant_mat3(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let a = stack.pop_mat3()?;
    stack.push_fixed(a.determinant())?;
    Ok(())
}

#[inline(always)]
pub fn exec_inverse_mat3(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let a = stack.pop_mat3()?;
    // Return identity if singular (GLSL behavior)
    let result = a.inverse().unwrap_or(Mat3::identity());
    stack.push_mat3(result)?;
    Ok(())
}
