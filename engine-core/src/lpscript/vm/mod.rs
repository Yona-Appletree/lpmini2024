pub mod call_stack;
pub mod error;
pub mod local_stack;
pub mod lps_program;
pub mod lps_vm;
/// Virtual Machine for LightPlayer Script
pub mod opcodes;
pub mod value_stack;
pub mod vm_dispatch;
pub mod vm_limits;

use crate::math::Fixed;
pub use call_stack::{CallFrame, CallStack};
pub use error::{LpsVmError, RuntimeErrorWithContext};
pub use local_stack::LocalStack;
pub use lps_program::{FunctionDef, LocalVarDef, LpsProgram, ParamDef};
pub use lps_vm::LpsVm;
pub use opcodes::LpsOpCode;
pub use value_stack::ValueStack;
pub use vm_limits::VmLimits;

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
