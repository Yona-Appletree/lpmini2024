use lp_math::dec32::noise::perlin3;
use lp_math::dec32::{
    atan, atan2, fract, lerp, modulo, pow, saturate, sign, smoothstep, step, tan, Dec32,
};

/// Advanced dec32-point dec32 opcodes
use crate::lp_script::vm::error::LpsVmError;
use crate::lp_script::vm::value_stack::ValueStack;

#[inline(always)]
pub fn exec_tan_dec32(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let a = stack.pop_dec32()?;
    stack.push_dec32(tan(a))?;
    Ok(())
}

#[inline(always)]
pub fn exec_atan_dec32(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let a = stack.pop_dec32()?;
    stack.push_dec32(atan(a))?;
    Ok(())
}

#[inline(always)]
pub fn exec_atan2_dec32(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let (y, x) = stack.pop2()?;
    let result = atan2(Dec32(y), Dec32(x));
    stack.push_dec32(result)?;
    Ok(())
}

#[inline(always)]
pub fn exec_fract_dec32(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let a = stack.pop_dec32()?;
    stack.push_dec32(fract(a))?;
    Ok(())
}

#[inline(always)]
pub fn exec_mod_dec32(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let (x, y) = stack.pop2()?;

    if y == 0 {
        return Err(LpsVmError::DivisionByZero);
    }

    let result = modulo(Dec32(x), Dec32(y));
    stack.push_dec32(result)?;
    Ok(())
}

#[inline(always)]
pub fn exec_pow_dec32(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let (base, exponent) = stack.pop2()?;
    // Convert exponent from Dec32 format to raw integer
    // The exponent is stored as Dec32 on stack, but pow() expects raw i32
    let exp_int = Dec32(exponent).to_i32();
    let result = pow(Dec32(base), exp_int);
    stack.push_dec32(result)?;
    Ok(())
}

#[inline(always)]
pub fn exec_sign_dec32(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let a = stack.pop_dec32()?;
    stack.push_dec32(sign(a))?;
    Ok(())
}

#[inline(always)]
pub fn exec_saturate_dec32(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let a = stack.pop_dec32()?;
    stack.push_dec32(saturate(a))?;
    Ok(())
}

#[inline(always)]
pub fn exec_clamp_dec32(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let (x, min_val, max_val) = stack.pop3()?;
    let result = Dec32(x).clamp(Dec32(min_val), Dec32(max_val));
    stack.push_dec32(result)?;
    Ok(())
}

#[inline(always)]
pub fn exec_step_dec32(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let (edge, x) = stack.pop2()?;
    let result = step(Dec32(edge), Dec32(x));
    stack.push_dec32(result)?;
    Ok(())
}

#[inline(always)]
pub fn exec_lerp_dec32(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let (a, b, t) = stack.pop3()?;
    let result = lerp(Dec32(a), Dec32(b), Dec32(t));
    stack.push_dec32(result)?;
    Ok(())
}

#[inline(always)]
pub fn exec_smoothstep_dec32(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let (edge0, edge1, x) = stack.pop3()?;
    let result = smoothstep(Dec32(edge0), Dec32(edge1), Dec32(x));
    stack.push_dec32(result)?;
    Ok(())
}

#[inline(always)]
pub fn exec_perlin3(stack: &mut ValueStack, octaves: u8) -> Result<(), LpsVmError> {
    let (x, y, z) = stack.pop3()?;
    let result = perlin3(Dec32(x), Dec32(y), Dec32(z), octaves);
    stack.push_dec32(result)?;
    Ok(())
}
