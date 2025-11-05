/// LPS VM executor - runs compiled programs
extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

use super::locals::LocalType;
use super::program::LpsProgram;
use crate::lpscript::error::{RuntimeError, RuntimeErrorWithContext};
use crate::lpscript::vm::opcodes::LpsOpCode;
use crate::math::{Fixed, Vec2, Vec3, Vec4};

// Import all opcode handler modules
use super::opcodes::{
    arrays, comparisons, control, fixed_advanced, fixed_basic, fixed_logic, int32, int32_compare,
    load, locals, stack, textures, vec2, vec3, vec4,
};

/// Configuration limits for the VM
#[derive(Debug, Clone, Copy)]
pub struct VmLimits {
    pub max_call_stack_depth: usize,
    pub max_stack_size: usize,
    pub max_instructions: usize,
}

impl Default for VmLimits {
    fn default() -> Self {
        VmLimits {
            max_call_stack_depth: 64,
            max_stack_size: 256,
            max_instructions: 10_000,
        }
    }
}

/// Call frame for function calls
#[derive(Debug, Clone, Copy)]
struct CallFrame {
    return_pc: usize,
    // TODO: Could add frame pointer for local variables if needed
}

/// LightPlayer Script Virtual Machine
///
/// Executes compiled LPS programs. Designed to be reusable - create once,
/// then call run() for each pixel.
pub struct LpsVm<'a> {
    pub program: &'a LpsProgram,
    stack: Vec<i32>,
    sp: usize,
    #[allow(dead_code)]
    pc: usize,
    #[allow(dead_code)]
    locals: Vec<LocalType>,
    call_stack: Vec<CallFrame>,
    call_stack_depth: usize,
    limits: VmLimits,
}

impl<'a> LpsVm<'a> {
    /// Create a new VM from a program with input locals and custom limits
    pub fn new(
        program: &'a LpsProgram,
        inputs: Vec<(usize, LocalType)>,
        limits: VmLimits,
    ) -> Result<Self, RuntimeError> {
        // Pre-allocate locals - use either program.locals.len() or a reasonable default (64)
        // This prevents runtime allocations during run()
        let local_count = if program.locals.len() > 0 {
            program.locals.len()
        } else {
            64 // Default: allocate 64 locals for scripts without explicit local definitions
        };
        let mut locals = Vec::new();
        locals.resize(local_count, LocalType::Fixed(Fixed::ZERO));

        // Set input locals
        for (idx, local) in inputs {
            if idx >= locals.len() {
                return Err(RuntimeError::LocalOutOfBounds {
                    local_idx: idx,
                    max: locals.len(),
                });
            }
            locals[idx] = local;
        }

        Ok(LpsVm {
            program,
            stack: vec![0; limits.max_stack_size],
            sp: 0,
            pc: 0,
            locals,
            call_stack: vec![CallFrame { return_pc: 0 }; limits.max_call_stack_depth],
            call_stack_depth: 0,
            limits,
        })
    }

    /// Create a new VM with default limits
    pub fn new_with_defaults(
        program: &'a LpsProgram,
        inputs: Vec<(usize, LocalType)>,
    ) -> Result<Self, RuntimeError> {
        Self::new(program, inputs, VmLimits::default())
    }

    /// Execute the program for a single pixel
    ///
    /// Returns all values on the stack after execution. For scalar results, use `run_scalar()`.
    /// For vector results, use `run_vec2()`, `run_vec3()`, or `run_vec4()`.
    ///
    /// # Zero-Allocation Guarantee
    ///
    /// This method performs NO heap allocations during execution. All data structures
    /// (stack, call_stack, locals) are pre-allocated in `new()` and reused across calls.
    /// This is critical for:
    /// - Preventing memory exhaustion in tight loops
    /// - Ensuring predictable performance
    /// - Safe execution on embedded systems with limited RAM
    pub fn run(
        &mut self,
        x: Fixed,
        y: Fixed,
        time: Fixed,
    ) -> Result<Vec<Fixed>, RuntimeErrorWithContext> {
        self.sp = 0;
        self.pc = 0;
        self.call_stack_depth = 0;

        // Store built-in values for Load operations
        let x_norm = x;
        let y_norm = y;
        let x_int = Fixed::from_i32(x.to_i32()); // Convert normalized to int coords
        let y_int = Fixed::from_i32(y.to_i32());

        // Limit instruction count to prevent infinite loops
        let mut instruction_count = 0;

        loop {
            instruction_count += 1;
            if instruction_count > self.limits.max_instructions {
                return Err(RuntimeErrorWithContext {
                    error: RuntimeError::InstructionLimitExceeded,
                    pc: self.pc,
                    opcode: "LIMIT_EXCEEDED",
                });
            }

            if self.pc >= self.program.opcodes.len() {
                return Err(RuntimeErrorWithContext {
                    error: RuntimeError::ProgramCounterOutOfBounds {
                        pc: self.pc,
                        max: self.program.opcodes.len(),
                    },
                    pc: self.pc,
                    opcode: "EOF",
                });
            }

            let opcode = &self.program.opcodes[self.pc];
            
            match opcode {
                // === Stack Operations ===
                LpsOpCode::Push(val) => {
                    self.push(*val)?;
                    self.pc += 1;
                }

                LpsOpCode::PushInt32(val) => {
                    self.push(Fixed::from_i32(*val))?;
                    self.pc += 1;
                }

                LpsOpCode::Dup => {
                    stack::exec_dup(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::Drop => {
                    stack::exec_drop(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::Swap => {
                    stack::exec_swap(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                // === Control Flow ===
                LpsOpCode::Jump(offset) => {
                    let new_pc = (self.pc as i32) + offset + 1;
                    if new_pc < 0 || new_pc as usize >= self.program.opcodes.len() {
                        return Err(self.runtime_error(RuntimeError::ProgramCounterOutOfBounds {
                            pc: new_pc as usize,
                            max: self.program.opcodes.len(),
                        }));
                    }
                    self.pc = new_pc as usize;
                }

                LpsOpCode::JumpIfZero(offset) => {
                    let offset = *offset;
                    if let Some(new_pc) =
                        control::exec_jump_if_zero(&mut self.stack, &mut self.sp, self.pc, offset)
                            .map_err(|e| self.runtime_error(e))?
                    {
                        if new_pc >= self.program.opcodes.len() {
                            return Err(self.runtime_error(
                                RuntimeError::ProgramCounterOutOfBounds {
                                    pc: new_pc,
                                    max: self.program.opcodes.len(),
                                },
                            ));
                        }
                        self.pc = new_pc;
                    } else {
                        self.pc += 1;
                    }
                }

                LpsOpCode::JumpIfNonZero(offset) => {
                    let offset = *offset;
                    if let Some(new_pc) = control::exec_jump_if_nonzero(
                        &mut self.stack,
                        &mut self.sp,
                        self.pc,
                        offset,
                    )
                    .map_err(|e| self.runtime_error(e))?
                    {
                        if new_pc >= self.program.opcodes.len() {
                            return Err(self.runtime_error(
                                RuntimeError::ProgramCounterOutOfBounds {
                                    pc: new_pc,
                                    max: self.program.opcodes.len(),
                                },
                            ));
                        }
                        self.pc = new_pc;
                    } else {
                        self.pc += 1;
                    }
                }

                LpsOpCode::Call(offset) => {
                    // Check call stack depth
                    if self.call_stack_depth >= self.limits.max_call_stack_depth {
                        return Err(self.runtime_error(RuntimeError::CallStackOverflow {
                            depth: self.call_stack_depth,
                        }));
                    }
                    // Push return address onto call stack
                    self.call_stack[self.call_stack_depth] = CallFrame {
                        return_pc: self.pc + 1,
                    };
                    self.call_stack_depth += 1;
                    // Jump to function
                    self.pc = *offset as usize;
                }

                LpsOpCode::Return => {
                    // Check if we're returning from a function or exiting main
                    if self.call_stack_depth > 0 {
                        // Return from function - jump back to caller
                        self.call_stack_depth -= 1;
                        self.pc = self.call_stack[self.call_stack_depth].return_pc;
                    } else {
                        // Exiting main - return all stack values as result
                        let result: Vec<Fixed> = self.stack[0..self.sp]
                            .iter()
                            .map(|&i| Fixed(i))
                            .collect();
                        return Ok(result);
                    }
                }

                // === Select (Ternary) ===
                LpsOpCode::Select => {
                    control::exec_select(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                // === Load Built-in Variables ===
                LpsOpCode::Load(source) => {
                    load::exec_load(
                        &mut self.stack,
                        &mut self.sp,
                        *source,
                        x_norm,
                        y_norm,
                        x_int,
                        y_int,
                        time,
                        0, // width placeholder (TODO: pass actual width/height)
                        0, // height placeholder
                    )
                    .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                // === Local Variables ===
                LpsOpCode::LoadLocalFixed(idx) => {
                    locals::exec_load_local_fixed(
                        &mut self.stack,
                        &mut self.sp,
                        &self.locals,
                        *idx,
                    )
                    .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::StoreLocalFixed(idx) => {
                    let idx = *idx;
                    locals::exec_store_local_fixed(
                        &mut self.stack,
                        &mut self.sp,
                        &mut self.locals,
                        idx,
                    )
                    .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::LoadLocalInt32(idx) => {
                    locals::exec_load_local_int32(
                        &mut self.stack,
                        &mut self.sp,
                        &self.locals,
                        *idx,
                    )
                    .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::StoreLocalInt32(idx) => {
                    let idx = *idx;
                    locals::exec_store_local_int32(
                        &mut self.stack,
                        &mut self.sp,
                        &mut self.locals,
                        idx,
                    )
                    .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::LoadLocalVec2(idx) => {
                    locals::exec_load_local_vec2(&mut self.stack, &mut self.sp, &self.locals, *idx)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::StoreLocalVec2(idx) => {
                    let idx = *idx;
                    locals::exec_store_local_vec2(
                        &mut self.stack,
                        &mut self.sp,
                        &mut self.locals,
                        idx,
                    )
                    .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::LoadLocalVec3(idx) => {
                    locals::exec_load_local_vec3(&mut self.stack, &mut self.sp, &self.locals, *idx)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::StoreLocalVec3(idx) => {
                    let idx = *idx;
                    locals::exec_store_local_vec3(
                        &mut self.stack,
                        &mut self.sp,
                        &mut self.locals,
                        idx,
                    )
                    .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::LoadLocalVec4(idx) => {
                    locals::exec_load_local_vec4(&mut self.stack, &mut self.sp, &self.locals, *idx)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::StoreLocalVec4(idx) => {
                    let idx = *idx;
                    locals::exec_store_local_vec4(
                        &mut self.stack,
                        &mut self.sp,
                        &mut self.locals,
                        idx,
                    )
                    .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                // === Basic Fixed-point Arithmetic ===
                LpsOpCode::AddFixed => {
                    fixed_basic::exec_add_fixed(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::SubFixed => {
                    fixed_basic::exec_sub_fixed(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::MulFixed => {
                    fixed_basic::exec_mul_fixed(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::DivFixed => {
                    fixed_basic::exec_div_fixed(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::NegFixed => {
                    fixed_basic::exec_neg_fixed(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::AbsFixed => {
                    fixed_basic::exec_abs_fixed(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::MinFixed => {
                    fixed_basic::exec_min_fixed(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::MaxFixed => {
                    fixed_basic::exec_max_fixed(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::SinFixed => {
                    fixed_basic::exec_sin_fixed(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::CosFixed => {
                    fixed_basic::exec_cos_fixed(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::SqrtFixed => {
                    fixed_basic::exec_sqrt_fixed(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::FloorFixed => {
                    fixed_basic::exec_floor_fixed(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::CeilFixed => {
                    fixed_basic::exec_ceil_fixed(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                // === Advanced Fixed-point Math ===
                LpsOpCode::TanFixed => {
                    fixed_advanced::exec_tan_fixed(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::AtanFixed => {
                    fixed_advanced::exec_atan_fixed(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::Atan2Fixed => {
                    fixed_advanced::exec_atan2_fixed(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::FractFixed => {
                    fixed_advanced::exec_fract_fixed(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::ModFixed => {
                    fixed_advanced::exec_mod_fixed(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::PowFixed => {
                    fixed_advanced::exec_pow_fixed(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::SignFixed => {
                    fixed_advanced::exec_sign_fixed(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::SaturateFixed => {
                    fixed_advanced::exec_saturate_fixed(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::ClampFixed => {
                    fixed_advanced::exec_clamp_fixed(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::StepFixed => {
                    fixed_advanced::exec_step_fixed(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::LerpFixed => {
                    fixed_advanced::exec_lerp_fixed(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::SmoothstepFixed => {
                    fixed_advanced::exec_smoothstep_fixed(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::Perlin3(octaves) => {
                    fixed_advanced::exec_perlin3(&mut self.stack, &mut self.sp, *octaves)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                // === Fixed-point Logic ===
                LpsOpCode::AndFixed => {
                    fixed_logic::exec_and_fixed(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::OrFixed => {
                    fixed_logic::exec_or_fixed(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::NotFixed => {
                    fixed_logic::exec_not_fixed(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                // === Fixed-point Comparisons ===
                LpsOpCode::GreaterFixed => {
                    comparisons::exec_greater_fixed(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::LessFixed => {
                    comparisons::exec_less_fixed(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::GreaterEqFixed => {
                    comparisons::exec_greater_eq_fixed(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::LessEqFixed => {
                    comparisons::exec_less_eq_fixed(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::EqFixed => {
                    comparisons::exec_eq_fixed(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::NotEqFixed => {
                    comparisons::exec_not_eq_fixed(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                // === Int32 Arithmetic ===
                LpsOpCode::AddInt32 => {
                    int32::exec_add_int32(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::SubInt32 => {
                    int32::exec_sub_int32(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::MulInt32 => {
                    int32::exec_mul_int32(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::DivInt32 => {
                    int32::exec_div_int32(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::ModInt32 => {
                    int32::exec_mod_int32(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::NegInt32 => {
                    int32::exec_neg_int32(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::AbsInt32 => {
                    int32::exec_abs_int32(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::MinInt32 => {
                    int32::exec_min_int32(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::MaxInt32 => {
                    int32::exec_max_int32(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::GreaterInt32 => {
                    int32::exec_greater_int32(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::LessInt32 => {
                    int32::exec_less_int32(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::GreaterEqInt32 => {
                    int32_compare::exec_greater_eq_int32(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::LessEqInt32 => {
                    int32_compare::exec_less_eq_int32(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::EqInt32 => {
                    int32_compare::exec_eq_int32(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::NotEqInt32 => {
                    int32_compare::exec_not_eq_int32(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                // === Vec2 Operations ===
                LpsOpCode::AddVec2 => {
                    vec2::exec_add_vec2(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::SubVec2 => {
                    vec2::exec_sub_vec2(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::MulVec2 => {
                    vec2::exec_mul_vec2(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::DivVec2 => {
                    vec2::exec_div_vec2(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::MulVec2Scalar => {
                    vec2::exec_mul_vec2_scalar(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::DivVec2Scalar => {
                    vec2::exec_div_vec2_scalar(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::Dot2 => {
                    vec2::exec_dot2(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::Length2 => {
                    vec2::exec_length2(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::Normalize2 => {
                    vec2::exec_normalize2(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::Distance2 => {
                    vec2::exec_distance2(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                // === Vec3 Operations ===
                LpsOpCode::AddVec3 => {
                    vec3::exec_add_vec3(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::SubVec3 => {
                    vec3::exec_sub_vec3(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::MulVec3 => {
                    vec3::exec_mul_vec3(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::DivVec3 => {
                    vec3::exec_div_vec3(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::MulVec3Scalar => {
                    vec3::exec_mul_vec3_scalar(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::DivVec3Scalar => {
                    vec3::exec_div_vec3_scalar(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::Dot3 => {
                    vec3::exec_dot3(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::Cross3 => {
                    vec3::exec_cross3(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::Length3 => {
                    vec3::exec_length3(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::Normalize3 => {
                    vec3::exec_normalize3(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::Distance3 => {
                    vec3::exec_distance3(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                // === Vec4 Operations ===
                LpsOpCode::AddVec4 => {
                    vec4::exec_add_vec4(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::SubVec4 => {
                    vec4::exec_sub_vec4(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::MulVec4 => {
                    vec4::exec_mul_vec4(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::DivVec4 => {
                    vec4::exec_div_vec4(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::MulVec4Scalar => {
                    vec4::exec_mul_vec4_scalar(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::DivVec4Scalar => {
                    vec4::exec_div_vec4_scalar(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::Dot4 => {
                    vec4::exec_dot4(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::Length4 => {
                    vec4::exec_length4(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::Normalize4 => {
                    vec4::exec_normalize4(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::Distance4 => {
                    vec4::exec_distance4(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                // === Texture Operations ===
                LpsOpCode::TextureSampleR(idx) => {
                    textures::exec_texture_sample_r(&mut self.stack, &mut self.sp, *idx)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::TextureSampleRGBA(idx) => {
                    textures::exec_texture_sample_rgba(&mut self.stack, &mut self.sp, *idx)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                // === Array Operations ===
                LpsOpCode::GetElemInt32ArrayFixed => {
                    arrays::exec_get_elem_int32_array_fixed(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }

                LpsOpCode::GetElemInt32ArrayU8 => {
                    arrays::exec_get_elem_int32_array_u8(&mut self.stack, &mut self.sp)
                        .map_err(|e| self.runtime_error(e))?;
                    self.pc += 1;
                }
            }
        }
    }

    // Helper methods for stack management
    fn push(&mut self, val: Fixed) -> Result<(), RuntimeErrorWithContext> {
        if self.sp >= self.limits.max_stack_size {
            return Err(self.runtime_error(RuntimeError::StackOverflow { sp: self.sp }));
        }
        self.stack[self.sp] = val.0;
        self.sp += 1;
        Ok(())
    }

    fn pop(&mut self) -> Result<Fixed, RuntimeErrorWithContext> {
        if self.sp == 0 {
            return Err(self.runtime_error(RuntimeError::StackUnderflow {
                required: 1,
                actual: 0,
            }));
        }
        self.sp -= 1;
        Ok(Fixed(self.stack[self.sp]))
    }

    #[allow(dead_code)]
    fn peek(&self) -> Result<Fixed, RuntimeErrorWithContext> {
        if self.sp == 0 {
            return Err(RuntimeErrorWithContext {
                error: RuntimeError::StackUnderflow {
                    required: 1,
                    actual: 0,
                },
                pc: self.pc,
                opcode: "peek",
            });
        }
        Ok(Fixed(self.stack[self.sp - 1]))
    }

    fn runtime_error(&self, error: RuntimeError) -> RuntimeErrorWithContext {
        RuntimeErrorWithContext {
            error,
            pc: self.pc,
            opcode: "opcode", // TODO: Get actual opcode name
        }
    }

    /// Execute the program and expect a scalar result
    pub fn run_scalar(
        &mut self,
        x: Fixed,
        y: Fixed,
        time: Fixed,
    ) -> Result<Fixed, RuntimeErrorWithContext> {
        let stack = self.run(x, y, time)?;
        if stack.len() != 1 {
            return Err(RuntimeErrorWithContext {
                error: RuntimeError::TypeMismatch,
                pc: self.pc,
                opcode: "run_scalar",
            });
        }
        Ok(stack[0])
    }

    /// Execute the program and expect a vec2 result
    pub fn run_vec2(
        &mut self,
        x: Fixed,
        y: Fixed,
        time: Fixed,
    ) -> Result<Vec2, RuntimeErrorWithContext> {
        let stack = self.run(x, y, time)?;
        if stack.len() != 2 {
            return Err(RuntimeErrorWithContext {
                error: RuntimeError::TypeMismatch,
                pc: self.pc,
                opcode: "run_vec2",
            });
        }
        Ok(Vec2::new(stack[0], stack[1]))
    }

    /// Execute the program and expect a vec3 result
    pub fn run_vec3(
        &mut self,
        x: Fixed,
        y: Fixed,
        time: Fixed,
    ) -> Result<Vec3, RuntimeErrorWithContext> {
        let stack = self.run(x, y, time)?;
        if stack.len() != 3 {
            return Err(RuntimeErrorWithContext {
                error: RuntimeError::TypeMismatch,
                pc: self.pc,
                opcode: "run_vec3",
            });
        }
        Ok(Vec3::new(stack[0], stack[1], stack[2]))
    }

    /// Execute the program and expect a vec4 result
    pub fn run_vec4(
        &mut self,
        x: Fixed,
        y: Fixed,
        time: Fixed,
    ) -> Result<Vec4, RuntimeErrorWithContext> {
        let stack = self.run(x, y, time)?;
        if stack.len() != 4 {
            return Err(RuntimeErrorWithContext {
                error: RuntimeError::TypeMismatch,
                pc: self.pc,
                opcode: "run_vec4",
            });
        }
        Ok(Vec4::new(stack[0], stack[1], stack[2], stack[3]))
    }

    /// Format a runtime error with full context
    pub fn format_error(&self, error: &RuntimeErrorWithContext) -> alloc::string::String {
        use alloc::format;

        let mut output = format!("{}\n", error);
        output.push_str(&format!("  at PC {} ({})\n", error.pc, error.opcode));
        output.push_str(&format!("  stack pointer: {}\n", self.sp));

        // Show top of stack
        if self.sp > 0 {
            output.push_str("  stack (top 5): [");
            let start = if self.sp > 5 { self.sp - 5 } else { 0 };
            for i in start..self.sp {
                if i > start {
                    output.push_str(", ");
                }
                output.push_str(&format!("{}", Fixed(self.stack[i]).to_f32()));
            }
            output.push_str("]\n");
        }

        // Show source if available
        if let Some(ref source) = self.program.source {
            if let Some(ref source_map) = self.program.source_map {
                if error.pc < source_map.len() {
                    let span = source_map[error.pc];
                    output.push_str(&format!(
                        "  source: {}\n",
                        &source[span.start..span.end.min(source.len())]
                    ));
                }
            }
        }

        output
    }
}

/// Execute a program on all pixels in the buffer
///
/// This is the main entry point for executing LPS programs on pixel buffers.
/// It creates a VM instance once and reuses it for all pixels.
///
/// # Arguments
/// * `program` - Compiled LPS program to execute
/// * `output` - Output buffer (16.16 fixed-point grayscale values)
/// * `width` - Width of the image
/// * `height` - Height of the image
/// * `time` - Time value in 16.16 fixed-point format
///
/// # Panics
/// Panics if the program encounters a runtime error. In production, you may want
/// to handle errors more gracefully.
#[inline(never)]
pub fn execute_program_lps(
    program: &LpsProgram,
    output: &mut [Fixed],
    width: usize,
    height: usize,
    time: Fixed,
) {
    // CRITICAL: Create VM once and reuse it for all pixels to avoid cloning the program
    // Cloning the program for each pixel causes catastrophic memory usage!
    let mut vm = LpsVm::new(program, Vec::new(), VmLimits::default()).expect("Failed to create VM");

    for y in 0..height {
        for x in 0..width {
            // Calculate normalized coordinates (0..1 range)
            // Add 0.5 to center pixels (x + 0.5, y + 0.5)
            let x_norm = Fixed::from_f32((x as f32 + 0.5) / width as f32);
            let y_norm = Fixed::from_f32((y as f32 + 0.5) / height as f32);

            let result = vm.run_scalar(x_norm, y_norm, time).unwrap_or_else(|e| {
                panic!("Runtime error at pixel ({}, {}): {}", x, y, e);
            });

            let idx = y * width + x;
            if idx < output.len() {
                output[idx] = result;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lpscript::parse_expr;
    use crate::math::ToFixed;

    #[test]
    fn test_vm_creation() {
        let program = parse_expr("1.0 + 2.0");
        let vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();

        // Verify VM can be created
        assert!(vm.program.opcodes.len() > 0);
    }

    #[test]
    fn test_vm_with_locals() {
        let mut program = parse_expr("xNorm");
        program.locals.push(crate::lpscript::LocalDef::new(
            "test".into(),
            LocalType::Fixed(1.0.to_fixed()),
            crate::lpscript::LocalAccess::Scratch,
        ));

        let vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        assert_eq!(vm.locals.len(), 1);
    }

    #[test]
    fn test_vm_simple_expression() {
        let program = parse_expr("1.0 + 2.0");
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();

        let result = vm.run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO).unwrap();
        assert_eq!(result.to_f32(), 3.0);
    }

    #[test]
    fn test_vm_with_builtin_vars() {
        let program = parse_expr("uv.x + uv.y");
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();

        let result = vm.run_scalar(0.5.to_fixed(), 0.3.to_fixed(), Fixed::ZERO).unwrap();
        assert!((result.to_f32() - 0.8).abs() < 0.01); // Account for fixed-point precision
    }

    #[test]
    fn test_vm_comparisons() {
        let program = parse_expr("5.0 > 3.0");
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();

        let result = vm.run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO).unwrap();
        assert_eq!(result, Fixed::ONE); // TRUE
    }

    #[test]
    fn test_vm_user_function() {
        use crate::lpscript::parse_script;

        let script = "
            float double(float x) {
                return x * 2.0;
            }
            
            return double(5.0);
        ";
        let program = parse_script(script);
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();

        let result = vm.run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO).unwrap();
        assert_eq!(result.to_f32(), 10.0);
    }

    #[test]
    fn test_vm_function_with_multiple_params() {
        use crate::lpscript::parse_script;

        let script = "
            float add(float a, float b) {
                return a + b;
            }
            
            return add(3.0, 7.0);
        ";
        let program = parse_script(script);
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();

        let result = vm.run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO).unwrap();
        assert_eq!(result.to_f32(), 10.0);
    }

    // ======== Comprehensive Integration Tests ========

    #[test]
    fn test_all_basic_fixed_arithmetic() {
        // Test Add
        let program = parse_expr("1.0 + 2.0");
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        assert_eq!(
            vm.run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
                .unwrap()
                .to_f32(),
            3.0
        );

        // Test Sub
        let program = parse_expr("5.0 - 2.0");
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        assert_eq!(
            vm.run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
                .unwrap()
                .to_f32(),
            3.0
        );

        // Test Mul
        let program = parse_expr("3.0 * 4.0");
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        assert_eq!(
            vm.run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
                .unwrap()
                .to_f32(),
            12.0
        );

        // Test Div
        let program = parse_expr("10.0 / 2.0");
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        assert_eq!(
            vm.run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
                .unwrap()
                .to_f32(),
            5.0
        );

        // Test Neg
        let program = parse_expr("-5.0");
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        assert_eq!(
            vm.run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
                .unwrap()
                .to_f32(),
            -5.0
        );
    }

    #[test]
    fn test_all_comparisons() {
        // Test Greater
        let program = parse_expr("5.0 > 3.0");
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        assert_eq!(
            vm.run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO).unwrap(),
            Fixed::ONE
        );

        // Test Less
        let program = parse_expr("3.0 < 5.0");
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        assert_eq!(
            vm.run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO).unwrap(),
            Fixed::ONE
        );

        // Test GreaterEq
        let program = parse_expr("5.0 >= 5.0");
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        assert_eq!(
            vm.run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO).unwrap(),
            Fixed::ONE
        );

        // Test LessEq
        let program = parse_expr("3.0 <= 5.0");
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        assert_eq!(
            vm.run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO).unwrap(),
            Fixed::ONE
        );

        // Test Eq
        let program = parse_expr("5.0 == 5.0");
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        assert_eq!(
            vm.run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO).unwrap(),
            Fixed::ONE
        );

        // Test NotEq
        let program = parse_expr("5.0 != 3.0");
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        assert_eq!(
            vm.run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO).unwrap(),
            Fixed::ONE
        );
    }

    #[test]
    fn test_advanced_math_functions() {
        // Test min
        let program = parse_expr("min(3.0, 7.0)");
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        assert_eq!(
            vm.run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
                .unwrap()
                .to_f32(),
            3.0
        );

        // Test max
        let program = parse_expr("max(3.0, 7.0)");
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        assert_eq!(
            vm.run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
                .unwrap()
                .to_f32(),
            7.0
        );

        // Test abs
        let program = parse_expr("abs(-5.0)");
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        assert_eq!(
            vm.run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
                .unwrap()
                .to_f32(),
            5.0
        );

        // Test floor
        let program = parse_expr("floor(3.7)");
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        assert_eq!(
            vm.run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
                .unwrap()
                .to_f32(),
            3.0
        );

        // Test ceil
        let program = parse_expr("ceil(3.2)");
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        assert_eq!(
            vm.run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
                .unwrap()
                .to_f32(),
            4.0
        );

        // Test sqrt
        let program = parse_expr("sqrt(4.0)");
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        let result = vm
            .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
            .unwrap()
            .to_f32();
        assert!((result - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_trigonometric_functions() {
        // Test sin (approximately)
        let program = parse_expr("sin(0.0)");
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        let result = vm
            .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
            .unwrap()
            .to_f32();
        assert!((result - 0.0).abs() < 0.01);

        // Test cos (approximately)
        let program = parse_expr("cos(0.0)");
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        let result = vm
            .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
            .unwrap()
            .to_f32();
        assert!((result - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_ternary_select() {
        // True case
        let program = parse_expr("5.0 > 3.0 ? 10.0 : 20.0");
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        assert_eq!(
            vm.run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
                .unwrap()
                .to_f32(),
            10.0
        );

        // False case
        let program = parse_expr("3.0 > 5.0 ? 10.0 : 20.0");
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        assert_eq!(
            vm.run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
                .unwrap()
                .to_f32(),
            20.0
        );
    }

    #[test]
    fn test_nested_expressions() {
        let program = parse_expr("(2.0 + 3.0) * (4.0 - 1.0)");
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        assert_eq!(
            vm.run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
                .unwrap()
                .to_f32(),
            15.0
        );
    }

    #[test]
    fn test_complex_expression() {
        let program = parse_expr("min(max(2.0, 5.0), 10.0) + sqrt(4.0)");
        let mut vm = LpsVm::new(&program, vec![], VmLimits::default()).unwrap();
        let result = vm
            .run_scalar(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)
            .unwrap()
            .to_f32();
        // max(2, 5) = 5, min(5, 10) = 5, sqrt(4) = 2, 5 + 2 = 7
        assert!((result - 7.0).abs() < 0.01);
    }
}
