/// Opcode dispatch logic for LPS VM
///
/// This module contains the main opcode dispatch implementation as a separate
/// impl block for LpsVm to keep the executor focused on orchestration.
extern crate alloc;
use alloc::vec::Vec;

use crate::dec32::Dec32;
use crate::vm::error::{LpsVmError, RuntimeErrorWithContext};
use crate::vm::lps_vm::LpsVm;
use crate::vm::opcodes::{
    arrays, comparisons, control_flow, fixed_advanced, fixed_basic, fixed_logic, int32,
    int32_compare, load, locals, mat3, textures, vec2, vec3, vec4, LpsOpCode, ReturnAction,
};

impl<'a> LpsVm<'a> {
    /// Dispatch a single opcode
    ///
    /// Executes the given opcode and updates PC as needed.
    /// Returns Some(result) if the program should exit, None to continue.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn dispatch_opcode(
        &mut self,
        opcode: &LpsOpCode,
        x_norm: Dec32,
        y_norm: Dec32,
        x_int: Dec32,
        y_int: Dec32,
        time: Dec32,
        width: usize,
        height: usize,
    ) -> Result<Option<Vec<Dec32>>, RuntimeErrorWithContext> {
        match opcode {
            // === Stack Operations ===
            LpsOpCode::Push(val) => {
                self.stack
                    .push_dec32(*val)
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

            LpsOpCode::Dup9 => {
                self.stack.dup9().map_err(|e| self.runtime_error(e))?;
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

            LpsOpCode::Drop9 => {
                self.stack.drop9().map_err(|e| self.runtime_error(e))?;
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
            LpsOpCode::LoadLocalDec32(idx) => {
                let local_idx = self.call_stack.frame_base() + *idx as usize;
                locals::exec_load_local_dec32(&mut self.stack, &self.locals, local_idx)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::StoreLocalDec32(idx) => {
                let local_idx = self.call_stack.frame_base() + *idx as usize;
                locals::exec_store_local_dec32(&mut self.stack, &mut self.locals, local_idx)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::LoadLocalInt32(idx) => {
                let local_idx = self.call_stack.frame_base() + *idx as usize;
                locals::exec_load_local_int32(&mut self.stack, &self.locals, local_idx)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::StoreLocalInt32(idx) => {
                let local_idx = self.call_stack.frame_base() + *idx as usize;
                locals::exec_store_local_int32(&mut self.stack, &mut self.locals, local_idx)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::LoadLocalVec2(idx) => {
                let local_idx = self.call_stack.frame_base() + *idx as usize;
                locals::exec_load_local_vec2(&mut self.stack, &self.locals, local_idx)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::StoreLocalVec2(idx) => {
                let local_idx = self.call_stack.frame_base() + *idx as usize;
                locals::exec_store_local_vec2(&mut self.stack, &mut self.locals, local_idx)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::LoadLocalVec3(idx) => {
                let local_idx = self.call_stack.frame_base() + *idx as usize;
                locals::exec_load_local_vec3(&mut self.stack, &self.locals, local_idx)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::StoreLocalVec3(idx) => {
                let local_idx = self.call_stack.frame_base() + *idx as usize;
                locals::exec_store_local_vec3(&mut self.stack, &mut self.locals, local_idx)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::LoadLocalVec4(idx) => {
                let local_idx = self.call_stack.frame_base() + *idx as usize;
                locals::exec_load_local_vec4(&mut self.stack, &self.locals, local_idx)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::StoreLocalVec4(idx) => {
                let local_idx = self.call_stack.frame_base() + *idx as usize;
                locals::exec_store_local_vec4(&mut self.stack, &mut self.locals, local_idx)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::LoadLocalMat3(idx) => {
                let local_idx = self.call_stack.frame_base() + *idx as usize;
                locals::exec_load_local_mat3(&mut self.stack, &self.locals, local_idx)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::StoreLocalMat3(idx) => {
                let local_idx = self.call_stack.frame_base() + *idx as usize;
                locals::exec_store_local_mat3(&mut self.stack, &mut self.locals, local_idx)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            // === Basic Dec32-point Arithmetic ===
            LpsOpCode::AddDec32 => {
                fixed_basic::exec_add_dec32(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::SubDec32 => {
                fixed_basic::exec_sub_dec32(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::MulDec32 => {
                fixed_basic::exec_mul_dec32(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::DivDec32 => {
                fixed_basic::exec_div_dec32(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::NegDec32 => {
                fixed_basic::exec_neg_dec32(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::AbsDec32 => {
                fixed_basic::exec_abs_dec32(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::MinDec32 => {
                fixed_basic::exec_min_dec32(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::MaxDec32 => {
                fixed_basic::exec_max_dec32(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::SinDec32 => {
                fixed_basic::exec_sin_dec32(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::CosDec32 => {
                fixed_basic::exec_cos_dec32(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::SqrtDec32 => {
                fixed_basic::exec_sqrt_dec32(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::FloorDec32 => {
                fixed_basic::exec_floor_dec32(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::CeilDec32 => {
                fixed_basic::exec_ceil_dec32(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            // === Advanced Dec32-point Math ===
            LpsOpCode::TanDec32 => {
                fixed_advanced::exec_tan_dec32(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::AtanDec32 => {
                fixed_advanced::exec_atan_dec32(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::Atan2Dec32 => {
                fixed_advanced::exec_atan2_dec32(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::FractDec32 => {
                fixed_advanced::exec_fract_dec32(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::ModDec32 => {
                fixed_advanced::exec_mod_dec32(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::PowDec32 => {
                fixed_advanced::exec_pow_dec32(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::SignDec32 => {
                fixed_advanced::exec_sign_dec32(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::SaturateDec32 => {
                fixed_advanced::exec_saturate_dec32(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::ClampDec32 => {
                fixed_advanced::exec_clamp_dec32(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::StepDec32 => {
                fixed_advanced::exec_step_dec32(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::LerpDec32 => {
                fixed_advanced::exec_lerp_dec32(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::SmoothstepDec32 => {
                fixed_advanced::exec_smoothstep_dec32(&mut self.stack)
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

            // === Dec32-point Logic ===
            LpsOpCode::AndDec32 => {
                fixed_logic::exec_and_dec32(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::OrDec32 => {
                fixed_logic::exec_or_dec32(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::NotDec32 => {
                fixed_logic::exec_not_dec32(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            // === Dec32-point Comparisons ===
            LpsOpCode::GreaterDec32 => {
                comparisons::exec_greater_dec32(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::LessDec32 => {
                comparisons::exec_less_dec32(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::GreaterEqDec32 => {
                comparisons::exec_greater_eq_dec32(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::LessEqDec32 => {
                comparisons::exec_less_eq_dec32(&mut self.stack)
                    .map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::EqDec32 => {
                comparisons::exec_eq_dec32(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::NotEqDec32 => {
                comparisons::exec_not_eq_dec32(&mut self.stack)
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
            LpsOpCode::Int32ToDec32 => {
                int32::exec_int32_to_dec32(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::FixedToInt32 => {
                int32::exec_dec32_to_int32(&mut self.stack).map_err(|e| self.runtime_error(e))?;
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

            // === Mat3 Operations ===
            LpsOpCode::AddMat3 => {
                mat3::exec_add_mat3(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::SubMat3 => {
                mat3::exec_sub_mat3(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::NegMat3 => {
                mat3::exec_neg_mat3(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::MulMat3 => {
                mat3::exec_mul_mat3(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::MulMat3Scalar => {
                mat3::exec_mul_mat3_scalar(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::DivMat3Scalar => {
                mat3::exec_div_mat3_scalar(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::MulMat3Vec3 => {
                mat3::exec_mul_mat3_vec3(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::TransposeMat3 => {
                mat3::exec_transpose_mat3(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::DeterminantMat3 => {
                mat3::exec_determinant_mat3(&mut self.stack).map_err(|e| self.runtime_error(e))?;
                self.pc += 1;
                Ok(None)
            }

            LpsOpCode::InverseMat3 => {
                mat3::exec_inverse_mat3(&mut self.stack).map_err(|e| self.runtime_error(e))?;
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
            LpsOpCode::GetElemInt32ArrayDec32 => {
                arrays::exec_get_elem_int32_array_dec32(&mut self.stack)
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
