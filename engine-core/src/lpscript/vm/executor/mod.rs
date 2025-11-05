/// LPS VM executor - runs compiled programs
extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

use super::call_stack::CallStack;
use super::locals_storage::LocalsStorage;
use super::program::LpsProgram;
use super::vm_stack::Stack;
use crate::lpscript::vm::error::{RuntimeError, RuntimeErrorWithContext};
use crate::math::{Fixed, Vec2, Vec3, Vec4};

// Import dispatch logic
mod executor_dispatch;

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

/// LightPlayer Script Virtual Machine
///
/// Executes compiled LPS programs. Designed to be reusable - create once,
/// then call run() for each pixel.
pub struct LpsVm<'a> {
    pub program: &'a LpsProgram,
    pub(super) stack: Stack,
    pub(super) pc: usize,
    pub(super) locals: LocalsStorage,
    pub(super) call_stack: CallStack,
    pub(super) limits: VmLimits,
}

impl<'a> LpsVm<'a> {
    /// Create a new VM from a program with custom limits
    pub fn new(program: &'a LpsProgram, limits: VmLimits) -> Result<Self, RuntimeError> {
        // Pre-allocate locals storage for frame-based allocation
        // Estimate: 32 i32s per frame * 64 max frames = 2048 i32s
        let local_capacity = 32 * limits.max_call_stack_depth;
        let mut locals = LocalsStorage::new(local_capacity);

        // Allocate main function's locals (function 0)
        if let Some(main_fn) = program.main_function() {
            locals.allocate_locals(&main_fn.locals)?;
        }

        Ok(LpsVm {
            program,
            stack: Stack::new(limits.max_stack_size),
            pc: 0,
            locals,
            call_stack: CallStack::new(limits.max_call_stack_depth),
            limits,
        })
    }

    /// Create a new VM with default limits
    pub fn new_with_defaults(program: &'a LpsProgram) -> Result<Self, RuntimeError> {
        Self::new(program, VmLimits::default())
    }

    /// Get access to locals storage for debugging/testing
    pub fn locals(&self) -> &LocalsStorage {
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

            // Get opcode from current function (new system) or legacy flat array
            let opcode = if let Some(main_fn) = self.program.main_function() {
                // New function-based system
                if self.pc >= main_fn.opcodes.len() {
                    return Err(RuntimeErrorWithContext {
                        error: RuntimeError::ProgramCounterOutOfBounds {
                            pc: self.pc,
                            max: main_fn.opcodes.len(),
                        },
                        pc: self.pc,
                        opcode: "EOF",
                    });
                }
                &main_fn.opcodes[self.pc]
            } else {
                // Legacy flat opcodes system (for backward compat)
                #[allow(deprecated)]
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

    pub(super) fn runtime_error(&self, error: RuntimeError) -> RuntimeErrorWithContext {
        RuntimeErrorWithContext {
            error,
            pc: self.pc,
            opcode: "opcode", // TODO: Get actual opcode name
        }
    }

    /// Get the length of the current function's opcodes (helper for bounds checking)
    pub(super) fn current_function_len(&self) -> usize {
        if let Some(main_fn) = self.program.main_function() {
            main_fn.opcodes.len()
        } else {
            #[allow(deprecated)]
            self.program.opcodes.len()
        }
    }
}

// Old match statement removed - now in executor_dispatch.rs
// Keeping this comment for reference in case we need to look up the old structure

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
    let mut vm = LpsVm::new(program, VmLimits::default()).expect("Failed to create VM");

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
    use crate::math::ToFixed;

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

    #[test]
    fn test_vm_with_locals() {
        use crate::lpscript::parse_expr;
        let program = parse_expr("xNorm");

        let vm = LpsVm::new(&program, VmLimits::default()).unwrap();

        // VM now pre-allocates locals with i32 capacity
        assert_eq!(vm.locals.capacity(), 32 * 64); // 32 i32s/frame * 64 max depth
        assert_eq!(vm.call_stack.frame_base(), 0);
    }

    #[test]
    fn test_vm_limits() {
        use crate::lpscript::parse_expr;
        let program = parse_expr("1.0");

        let custom_limits = VmLimits {
            max_call_stack_depth: 32,
            max_stack_size: 128,
            max_instructions: 5000,
        };

        let vm = LpsVm::new(&program, custom_limits).unwrap();
        assert_eq!(vm.limits.max_call_stack_depth, 32);
        assert_eq!(vm.limits.max_stack_size, 128);
        assert_eq!(vm.limits.max_instructions, 5000);
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
