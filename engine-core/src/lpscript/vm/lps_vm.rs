use alloc::vec::Vec;
use crate::lpscript::{LocalStack, LpsProgram, LpsVmError, RuntimeErrorWithContext};
use crate::lpscript::vm::{CallStack, ValueStack};
use crate::lpscript::vm::vm_limits::VmLimits;
use crate::math::{Fixed, Vec2, Vec3, Vec4};

/// LightPlayer Script Virtual Machine
///
/// Executes compiled LPS programs. Designed to be reusable - create once,
/// then call run() for each pixel.
pub struct LpsVm<'a> {
    pub program: &'a LpsProgram,
    pub(in crate::lpscript::vm) stack: ValueStack,
    pub(in crate::lpscript::vm) pc: usize,
    pub(in crate::lpscript::vm) locals: LocalStack,
    pub(in crate::lpscript::vm) call_stack: CallStack,
    pub(in crate::lpscript::vm) limits: VmLimits,
    pub(in crate::lpscript::vm) current_fn_idx: usize,  // Track which function we're executing
}

impl<'a> LpsVm<'a> {
    /// Create a new VM from a program with custom limits
    pub fn new(program: &'a LpsProgram, limits: VmLimits) -> Result<Self, LpsVmError> {
        // Pre-allocate locals storage for frame-based allocation
        // Estimate: 32 i32s per frame * 64 max frames = 2048 i32s
        let local_capacity = 32 * limits.max_call_stack_depth;
        let mut locals = LocalStack::new(local_capacity);

        // Allocate main function's locals (function 0)
        if let Some(main_fn) = program.main_function() {
            locals.allocate_locals(&main_fn.locals)?;
        }

        Ok(LpsVm {
            program,
            stack: ValueStack::new(limits.max_stack_size),
            pc: 0,
            locals,
            call_stack: CallStack::new(limits.max_call_stack_depth),
            limits,
            current_fn_idx: 0,  // Start in main
        })
    }

    /// Create a new VM with default limits
    pub fn new_with_defaults(program: &'a LpsProgram) -> Result<Self, LpsVmError> {
        Self::new(program, VmLimits::default())
    }

    /// Get access to locals storage for debugging/testing
    pub fn locals(&self) -> &LocalStack {
        &self.locals
    }

    /// Get a local value by name (for debugging/testing)
    pub fn get_local_by_name(&self, name: &str) -> Option<Fixed> {
        self.locals.get_fixed_by_name(name)
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
        self.stack.reset();
        self.pc = 0;
        self.call_stack.reset(0);
        self.current_fn_idx = 0;  // Reset to main
        
        // Reset locals to main function's state
        if let Some(main_fn) = self.program.main_function() {
            let main_local_count = main_fn.locals.len();
            self.locals.reset_locals(main_local_count, &main_fn.locals)
                .map_err(|e| self.runtime_error(e))?;
        }

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
                    error: LpsVmError::InstructionLimitExceeded,
                    pc: self.pc,
                    opcode: "LIMIT_EXCEEDED",
                });
            }

            // Get opcode from current function (new system) or legacy flat array
            let opcode = if let Some(func) = self.program.function(self.current_fn_idx) {
                // New function-based system - fetch from current function
                if self.pc >= func.opcodes.len() {
                    return Err(RuntimeErrorWithContext {
                        error: LpsVmError::ProgramCounterOutOfBounds {
                            pc: self.pc,
                            max: func.opcodes.len(),
                        },
                        pc: self.pc,
                        opcode: "EOF",
                    });
                }
                &func.opcodes[self.pc]
            } else {
                // Legacy flat opcodes system (for backward compat)
                #[allow(deprecated)]
                if self.pc >= self.program.opcodes.len() {
                    return Err(RuntimeErrorWithContext {
                        error: LpsVmError::ProgramCounterOutOfBounds {
                            pc: self.pc,
                            max: self.program.opcodes.len(),
                        },
                        pc: self.pc,
                        opcode: "EOF",
                    });
                }
                #[allow(deprecated)]
                &self.program.opcodes[self.pc]
            };

            // Dispatch the opcode - returns Some(result) if program should exit
            if let Some(result) =
                self.dispatch_opcode(opcode, x_norm, y_norm, x_int, y_int, time)?
            {
                return Ok(result);
            }
        }
    }

    pub(in crate::lpscript::vm) fn runtime_error(&self, error: LpsVmError) -> RuntimeErrorWithContext {
        RuntimeErrorWithContext {
            error,
            pc: self.pc,
            opcode: "opcode", // TODO: Get actual opcode name
        }
    }

    /// Get the length of the current function's opcodes (helper for bounds checking)
    pub(in crate::lpscript::vm) fn current_function_len(&self) -> usize {
        if let Some(func) = self.program.function(self.current_fn_idx) {
            func.opcodes.len()
        } else {
            #[allow(deprecated)]
            self.program.opcodes.len()
        }
    }
}

impl<'a> LpsVm<'a> {
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
                error: LpsVmError::TypeMismatch,
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
                error: LpsVmError::TypeMismatch,
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
                error: LpsVmError::TypeMismatch,
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
                error: LpsVmError::TypeMismatch,
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
        output.push_str(&format!("  stack pointer: {}\n", self.stack.sp()));

        // Show top of stack
        let sp = self.stack.sp();
        if sp > 0 {
            output.push_str("  stack (top 5): [");
            let start = if sp > 5 { sp - 5 } else { 0 };
            for i in start..sp {
                if i > start {
                    output.push_str(", ");
                }
                output.push_str(&format!("{}", Fixed(self.stack.raw_slice()[i]).to_f32()));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vm_creation() {
        use crate::lpscript::parse_expr;
        let program = parse_expr("1.0 + 2.0");
        let vm = LpsVm::new(&program, VmLimits::default()).unwrap();

        // Verify VM can be created with correct initialization
        assert!(vm.program.main_function().unwrap().opcodes.len() > 0);
        assert_eq!(vm.stack.sp(), 0);
        assert_eq!(vm.pc, 0);
        assert_eq!(vm.call_stack.depth(), 0);
        assert_eq!(vm.call_stack.frame_base(), 0);
    }
}
