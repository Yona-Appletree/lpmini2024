use crate::fixed::noise::perlin3;
use crate::fixed::Fixed;
use crate::fixed::{atan, atan2, fract, lerp, modulo, pow, saturate, sign, smoothstep, step, tan};
/// Advanced fixed-point fixed opcodes
use crate::vm::error::LpsVmError;
use crate::vm::value_stack::ValueStack;

#[inline(always)]
pub fn exec_tan_fixed(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let a = stack.pop_fixed()?;
    stack.push_fixed(tan(a))?;
    Ok(())
}

#[inline(always)]
pub fn exec_atan_fixed(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let a = stack.pop_fixed()?;
    stack.push_fixed(atan(a))?;
    Ok(())
}

#[inline(always)]
pub fn exec_atan2_fixed(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let (y, x) = stack.pop2()?;
    let result = atan2(Fixed(y), Fixed(x));
    stack.push_fixed(result)?;
    Ok(())
}

#[inline(always)]
pub fn exec_fract_fixed(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let a = stack.pop_fixed()?;
    stack.push_fixed(fract(a))?;
    Ok(())
}

#[inline(always)]
pub fn exec_mod_fixed(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let (x, y) = stack.pop2()?;

    if y == 0 {
        return Err(LpsVmError::DivisionByZero);
    }

    let result = modulo(Fixed(x), Fixed(y));
    stack.push_fixed(result)?;
    Ok(())
}

#[inline(always)]
pub fn exec_pow_fixed(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let (base, exponent) = stack.pop2()?;
    // Convert exponent from Fixed format to raw integer
    // The exponent is stored as Fixed on stack, but pow() expects raw i32
    let exp_int = Fixed(exponent).to_i32();
    let result = pow(Fixed(base), exp_int);
    stack.push_fixed(result)?;
    Ok(())
}

#[inline(always)]
pub fn exec_sign_fixed(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let a = stack.pop_fixed()?;
    stack.push_fixed(sign(a))?;
    Ok(())
}

#[inline(always)]
pub fn exec_saturate_fixed(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let a = stack.pop_fixed()?;
    stack.push_fixed(saturate(a))?;
    Ok(())
}

#[inline(always)]
pub fn exec_clamp_fixed(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let (x, min_val, max_val) = stack.pop3()?;
    let result = Fixed(x).clamp(Fixed(min_val), Fixed(max_val));
    stack.push_fixed(result)?;
    Ok(())
}

#[inline(always)]
pub fn exec_step_fixed(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let (edge, x) = stack.pop2()?;
    let result = step(Fixed(edge), Fixed(x));
    stack.push_fixed(result)?;
    Ok(())
}

#[inline(always)]
pub fn exec_lerp_fixed(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let (a, b, t) = stack.pop3()?;
    let result = lerp(Fixed(a), Fixed(b), Fixed(t));
    stack.push_fixed(result)?;
    Ok(())
}

#[inline(always)]
pub fn exec_smoothstep_fixed(stack: &mut ValueStack) -> Result<(), LpsVmError> {
    let (edge0, edge1, x) = stack.pop3()?;
    let result = smoothstep(Fixed(edge0), Fixed(edge1), Fixed(x));
    stack.push_fixed(result)?;
    Ok(())
}

#[inline(always)]
pub fn exec_perlin3(stack: &mut ValueStack, octaves: u8) -> Result<(), LpsVmError> {
    let (x, y, z) = stack.pop3()?;
    let result = perlin3(Fixed(x), Fixed(y), Fixed(z), octaves);
    stack.push_fixed(result)?;
    Ok(())
}
