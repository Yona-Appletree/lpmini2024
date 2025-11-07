/// Opcode dispatch logic for LPS VM
///
/// This module contains the main opcode dispatch implementation as a separate
/// impl block for LpsVm to keep the executor focused on orchestration.
extern crate alloc;
use alloc::vec::Vec;

use crate::vm::lps_vm::LpsVm;
use crate::vm::error::{LpsVmError, RuntimeErrorWithContext};
use crate::vm::opcodes::{
    arrays, comparisons, control_flow, fixed_advanced, fixed_basic, fixed_logic, int32,
    int32_compare, load, locals, textures, vec2, vec3, vec4, LpsOpCode, ReturnAction,
};
use crate::math::Fixed;

impl<'a> LpsVm<'a> {
    /// Dispatch a single opcode
    ///
    /// Executes the given opcode and updates PC as needed.
    /// Returns Some(result) if the program should exit, None to continue.
    pub(super) fn dispatch_opcode(
        &mut self,
        opcode: &LpsOpCode,
        x_norm: Fixed,
        y_norm: Fixed,
        x_int: Fixed,
        y_int: Fixed,
        time: Fixed,
        width: usize,
        height: usize,
    ) -> Result<Option<Vec<Fixed>>, RuntimeErrorWithContext> {
        match opcode {
            // === Stack Operations ===
            LpsOpCode::Push(val) => {
                self.stack
                    .push_fixed(*val)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::PushInt32(val) => {
                self.stack
                    .push_int32(*val)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::Dup1 => {
                self.stack.dup1().map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::Dup2 => {
                self.stack.dup2().map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::Dup3 => {
                self.stack.dup3().map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::Dup4 => {
                self.stack.dup4().map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::Drop1 => {
                self.stack.drop1().map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::Drop2 => {
                self.stack.drop2().map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::Drop3 => {
                self.stack.drop3().map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::Drop4 => {
                self.stack.drop4().map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::Swap => {
                self.stack.swap().map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            // === Control Flow ===
            LpsOpCode::Jump(offset) => {
                let max_pc = self.current_function_len();
                self.pc = control_flow::exec_jump(self.pc, *offset, max_pc)
                    .map_err(|e| self.runtime_error(e))?;
                Ok(None)
            }

            LpsOpCode::JumpIfZero(offset) => {
                let offset = *offset;
                if let Some(new_pc) =
                    control_flow::exec_jump_if_zero(&mut self.stack, self.pc, offset)
                        .map_err(|e| self.runtime_error(e))?
                {
                    let max_pc = self.current_function_len();
                    if new_pc >= max_pc {
                        return Err(self.runtime_error(LpsVmError::ProgramCounterOutOfBounds {
                            pc: new_pc,
                            max: max_pc,
                        }));
                    }
                    self.pc = new_pc;
                } else {
                    self.pc += 1;
                }
                Ok(None)
            }

            LpsOpCode::JumpIfNonZero(offset) => {
                let offset = *offset;
                if let Some(new_pc) =
                    control_flow::exec_jump_if_nonzero(&mut self.stack, self.pc, offset)
                        .map_err(|e| self.runtime_error(e))?
                {
                    let max_pc = self.current_function_len();
                    if new_pc >= max_pc {
                        return Err(self.runtime_error(LpsVmError::ProgramCounterOutOfBounds {
                            pc: new_pc,
                            max: max_pc,
                        }));
                    }
                    self.pc = new_pc;
                } else {
                    self.pc += 1;
                }
                Ok(None)
            }

            LpsOpCode::Call(fn_idx) => {
                let (new_pc, new_fn_idx) = control_flow::exec_call(
                    self.program,
                    self.pc,
                    self.current_fn_idx,
                    *fn_idx as usize,
                    &mut self.locals,
                    &mut self.call_stack,
                )
                .map_err(|e| self.runtime_error(e))?;

                self.pc = new_pc;
                self.current_fn_idx = new_fn_idx;
                Ok(None)
            }

            LpsOpCode::Return => {
                match control_flow::exec_return(&self.stack, &mut self.call_stack, &mut self.locals)
                    .map_err(|e| self.runtime_error(e))?
                {
                    ReturnAction::Continue(return_pc, return_fn_idx) => {
                        self.pc = return_pc;
                        self.current_fn_idx = return_fn_idx; // Switch back to caller
                        Ok(None)
                    }
                    ReturnAction::Exit(values) => Ok(Some(values)),
                }
            }

            // === Select (Ternary) ===
            LpsOpCode::Select => {
                control_flow::exec_select(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            // === Load Built-in Variables ===
            LpsOpCode::Load(source) => {
                load::exec_load(
                    &mut self.stack,
                    *source,
                    x_norm,
                    y_norm,
                    x_int,
                    y_int,
                    time,
                    width,
                    height,
                )
                .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            // === Local Variables ===
            LpsOpCode::LoadLocalFixed(idx) => {
                let local_idx = (self.call_stack.frame_base() + *idx as usize) as usize;
                locals::exec_load_local_fixed(&mut self.stack, &self.locals, local_idx)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::StoreLocalFixed(idx) => {
                let local_idx = (self.call_stack.frame_base() + *idx as usize) as usize;
                locals::exec_store_local_fixed(&mut self.stack, &mut self.locals, local_idx)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::LoadLocalInt32(idx) => {
                let local_idx = (self.call_stack.frame_base() + *idx as usize) as usize;
                locals::exec_load_local_int32(&mut self.stack, &self.locals, local_idx)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::StoreLocalInt32(idx) => {
                let local_idx = (self.call_stack.frame_base() + *idx as usize) as usize;
                locals::exec_store_local_int32(&mut self.stack, &mut self.locals, local_idx)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::LoadLocalVec2(idx) => {
                let local_idx = (self.call_stack.frame_base() + *idx as usize) as usize;
                locals::exec_load_local_vec2(&mut self.stack, &self.locals, local_idx)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::StoreLocalVec2(idx) => {
                let local_idx = (self.call_stack.frame_base() + *idx as usize) as usize;
                locals::exec_store_local_vec2(&mut self.stack, &mut self.locals, local_idx)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::LoadLocalVec3(idx) => {
                let local_idx = (self.call_stack.frame_base() + *idx as usize) as usize;
                locals::exec_load_local_vec3(&mut self.stack, &self.locals, local_idx)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::StoreLocalVec3(idx) => {
                let local_idx = (self.call_stack.frame_base() + *idx as usize) as usize;
                locals::exec_store_local_vec3(&mut self.stack, &mut self.locals, local_idx)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::LoadLocalVec4(idx) => {
                let local_idx = (self.call_stack.frame_base() + *idx as usize) as usize;
                locals::exec_load_local_vec4(&mut self.stack, &self.locals, local_idx)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::StoreLocalVec4(idx) => {
                let local_idx = (self.call_stack.frame_base() + *idx as usize) as usize;
                locals::exec_store_local_vec4(&mut self.stack, &mut self.locals, local_idx)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            // === Basic Fixed-point Arithmetic ===
            LpsOpCode::AddFixed => {
                fixed_basic::exec_add_fixed(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::SubFixed => {
                fixed_basic::exec_sub_fixed(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::MulFixed => {
                fixed_basic::exec_mul_fixed(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::DivFixed => {
                fixed_basic::exec_div_fixed(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::NegFixed => {
                fixed_basic::exec_neg_fixed(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::AbsFixed => {
                fixed_basic::exec_abs_fixed(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::MinFixed => {
                fixed_basic::exec_min_fixed(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::MaxFixed => {
                fixed_basic::exec_max_fixed(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::SinFixed => {
                fixed_basic::exec_sin_fixed(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::CosFixed => {
                fixed_basic::exec_cos_fixed(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::SqrtFixed => {
                fixed_basic::exec_sqrt_fixed(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::FloorFixed => {
                fixed_basic::exec_floor_fixed(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::CeilFixed => {
                fixed_basic::exec_ceil_fixed(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            // === Advanced Fixed-point Math ===
            LpsOpCode::TanFixed => {
                fixed_advanced::exec_tan_fixed(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::AtanFixed => {
                fixed_advanced::exec_atan_fixed(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::Atan2Fixed => {
                fixed_advanced::exec_atan2_fixed(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::FractFixed => {
                fixed_advanced::exec_fract_fixed(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::ModFixed => {
                fixed_advanced::exec_mod_fixed(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::PowFixed => {
                fixed_advanced::exec_pow_fixed(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::SignFixed => {
                fixed_advanced::exec_sign_fixed(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::SaturateFixed => {
                fixed_advanced::exec_saturate_fixed(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::ClampFixed => {
                fixed_advanced::exec_clamp_fixed(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::StepFixed => {
                fixed_advanced::exec_step_fixed(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::LerpFixed => {
                fixed_advanced::exec_lerp_fixed(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::SmoothstepFixed => {
                fixed_advanced::exec_smoothstep_fixed(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::Perlin3(octaves) => {
                fixed_advanced::exec_perlin3(&mut self.stack, *octaves)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            // === Fixed-point Logic ===
            LpsOpCode::AndFixed => {
                fixed_logic::exec_and_fixed(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::OrFixed => {
                fixed_logic::exec_or_fixed(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::NotFixed => {
                fixed_logic::exec_not_fixed(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            // === Fixed-point Comparisons ===
            LpsOpCode::GreaterFixed => {
                comparisons::exec_greater_fixed(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::LessFixed => {
                comparisons::exec_less_fixed(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::GreaterEqFixed => {
                comparisons::exec_greater_eq_fixed(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::LessEqFixed => {
                comparisons::exec_less_eq_fixed(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::EqFixed => {
                comparisons::exec_eq_fixed(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::NotEqFixed => {
                comparisons::exec_not_eq_fixed(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            // === Int32 Arithmetic ===
            LpsOpCode::AddInt32 => {
                int32::exec_add_int32(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::SubInt32 => {
                int32::exec_sub_int32(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::MulInt32 => {
                int32::exec_mul_int32(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::DivInt32 => {
                int32::exec_div_int32(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::ModInt32 => {
                int32::exec_mod_int32(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::NegInt32 => {
                int32::exec_neg_int32(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::AbsInt32 => {
                int32::exec_abs_int32(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::MinInt32 => {
                int32::exec_min_int32(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::MaxInt32 => {
                int32::exec_max_int32(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::GreaterInt32 => {
                int32::exec_greater_int32(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::LessInt32 => {
                int32::exec_less_int32(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::GreaterEqInt32 => {
                int32_compare::exec_greater_eq_int32(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::LessEqInt32 => {
                int32_compare::exec_less_eq_int32(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::EqInt32 => {
                int32_compare::exec_eq_int32(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::NotEqInt32 => {
                int32_compare::exec_not_eq_int32(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            // === Int32 Bitwise Operations ===
            LpsOpCode::BitwiseAndInt32 => {
                int32::exec_bitwise_and_int32(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::BitwiseOrInt32 => {
                int32::exec_bitwise_or_int32(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::BitwiseXorInt32 => {
                int32::exec_bitwise_xor_int32(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::BitwiseNotInt32 => {
                int32::exec_bitwise_not_int32(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::LeftShiftInt32 => {
                int32::exec_left_shift_int32(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::RightShiftInt32 => {
                int32::exec_right_shift_int32(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            // === Type Conversions ===
            LpsOpCode::Int32ToFixed => {
                int32::exec_int32_to_fixed(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::FixedToInt32 => {
                int32::exec_fixed_to_int32(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            // === Vec2 Operations ===
            LpsOpCode::AddVec2 => {
                vec2::exec_add_vec2(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::SubVec2 => {
                vec2::exec_sub_vec2(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::NegVec2 => {
                vec2::exec_neg_vec2(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::MulVec2 => {
                vec2::exec_mul_vec2(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::DivVec2 => {
                vec2::exec_div_vec2(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::ModVec2 => {
                vec2::exec_mod_vec2(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::MulVec2Scalar => {
                vec2::exec_mul_vec2_scalar(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::DivVec2Scalar => {
                vec2::exec_div_vec2_scalar(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::Dot2 => {
                vec2::exec_dot2(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::Length2 => {
                vec2::exec_length2(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::Normalize2 => {
                vec2::exec_normalize2(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::Distance2 => {
                vec2::exec_distance2(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            // === Vec3 Operations ===
            LpsOpCode::AddVec3 => {
                vec3::exec_add_vec3(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::SubVec3 => {
                vec3::exec_sub_vec3(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::NegVec3 => {
                vec3::exec_neg_vec3(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::MulVec3 => {
                vec3::exec_mul_vec3(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::DivVec3 => {
                vec3::exec_div_vec3(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::ModVec3 => {
                vec3::exec_mod_vec3(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::MulVec3Scalar => {
                vec3::exec_mul_vec3_scalar(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::DivVec3Scalar => {
                vec3::exec_div_vec3_scalar(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::Dot3 => {
                vec3::exec_dot3(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::Cross3 => {
                vec3::exec_cross3(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::Length3 => {
                vec3::exec_length3(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::Normalize3 => {
                vec3::exec_normalize3(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::Distance3 => {
                vec3::exec_distance3(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            // === Vec4 Operations ===
            LpsOpCode::AddVec4 => {
                vec4::exec_add_vec4(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::SubVec4 => {
                vec4::exec_sub_vec4(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::NegVec4 => {
                vec4::exec_neg_vec4(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::MulVec4 => {
                vec4::exec_mul_vec4(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::DivVec4 => {
                vec4::exec_div_vec4(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::ModVec4 => {
                vec4::exec_mod_vec4(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::MulVec4Scalar => {
                vec4::exec_mul_vec4_scalar(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::DivVec4Scalar => {
                vec4::exec_div_vec4_scalar(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::Dot4 => {
                vec4::exec_dot4(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::Length4 => {
                vec4::exec_length4(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::Normalize4 => {
                vec4::exec_normalize4(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::Distance4 => {
                vec4::exec_distance4(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            // === Swizzle Operations ===
            LpsOpCode::Swizzle3to2(idx0, idx1) => {
                self.stack
                    .swizzle3to2(*idx0, *idx1)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::Swizzle3to3(idx0, idx1, idx2) => {
                self.stack
                    .swizzle3to3(*idx0, *idx1, *idx2)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::Swizzle4to2(idx0, idx1) => {
                self.stack
                    .swizzle4to2(*idx0, *idx1)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::Swizzle4to3(idx0, idx1, idx2) => {
                self.stack
                    .swizzle4to3(*idx0, *idx1, *idx2)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::Swizzle4to4(idx0, idx1, idx2, idx3) => {
                self.stack
                    .swizzle4to4(*idx0, *idx1, *idx2, *idx3)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            // === Texture Operations ===
            LpsOpCode::TextureSampleR(idx) => {
                textures::exec_texture_sample_r(&mut self.stack, *idx)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::TextureSampleRGBA(idx) => {
                textures::exec_texture_sample_rgba(&mut self.stack, *idx)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            // === Array Operations ===
            LpsOpCode::GetElemInt32ArrayFixed => {
                arrays::exec_get_elem_int32_array_fixed(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::GetElemInt32ArrayU8 => {
                arrays::exec_get_elem_int32_array_u8(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }
        }
    }
}
