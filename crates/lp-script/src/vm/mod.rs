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

pub use call_stack::{CallFrame, CallStack};
pub use error::{LpsVmError, RuntimeErrorWithContext};
pub use local_stack::LocalStack;
pub use lps_program::{FunctionDef, LocalVarDef, LpsProgram, ParamDef};
pub use lps_vm::LpsVm;
pub use opcodes::LpsOpCode;
pub use value_stack::ValueStack;
pub use vm_limits::VmLimits;

use crate::fixed::Fixed;

#[cfg(test)]
pub(crate) mod test_pool {
    use core::ptr::NonNull;
    use lp_pool::LpMemoryPool;
    use std::cell::RefCell;
    use std::thread_local;

    const POOL_SIZE: usize = 256 * 1024;
    thread_local! {
        static THREAD_POOL_MEMORY: RefCell<Option<Box<[u8; POOL_SIZE]>>> =
            const { RefCell::new(None) };
    }

    pub(crate) fn ensure_initialized() {
        THREAD_POOL_MEMORY.with(|cell| {
            if cell.borrow().is_none() {
                let mut memory = Box::new([0u8; POOL_SIZE]);
                let ptr = NonNull::new(memory.as_mut_ptr()).expect("test pool memory pointer");
                unsafe {
                    LpMemoryPool::new(ptr, POOL_SIZE).expect("failed to create test thread pool");
                }
                cell.replace(Some(memory));
            }
        });
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
    let mut vm = LpsVm::new(program, VmLimits::default()).expect("Failed to create VM");

    for y in 0..height {
        for x in 0..width {
            // Calculate normalized coordinates (0..1 range)
            // Add 0.5 to center pixels (x + 0.5, y + 0.5)
            // Use fixed-point arithmetic throughout to avoid float fixed
            let x_plus_half = Fixed::from_i32(x as i32) + Fixed::HALF;
            let x_norm = x_plus_half / Fixed::from_i32(width as i32);
            let y_plus_half = Fixed::from_i32(y as i32) + Fixed::HALF;
            let y_norm = y_plus_half / Fixed::from_i32(height as i32);

            // Pass both normalized AND pixel coordinates
            let result = vm
                .run_scalar_with_coords(
                    x_norm,
                    y_norm,
                    x_plus_half,
                    y_plus_half,
                    time,
                    width,
                    height,
                )
                .unwrap_or_else(|e| {
                    panic!("Runtime error at pixel ({}, {}): {}", x, y, e);
                });

            let idx = y * width + x;
            if idx < output.len() {
                output[idx] = result;
            }
        }
    }
}

/// Execute a program that returns Vec3 (RGB) for each pixel
/// Output buffer should be sized width * height * 3 (r, g, b values)
pub fn execute_program_lps_vec3(
    program: &LpsProgram,
    output: &mut [Fixed],
    width: usize,
    height: usize,
    time: Fixed,
) {
    // Create VM once and reuse it for all pixels
    let mut vm = LpsVm::new(program, VmLimits::default()).expect("Failed to create VM");

    for y in 0..height {
        for x in 0..width {
            // Calculate normalized coordinates
            let x_plus_half = Fixed::from_i32(x as i32) + Fixed::HALF;
            let x_norm = x_plus_half / Fixed::from_i32(width as i32);
            let y_plus_half = Fixed::from_i32(y as i32) + Fixed::HALF;
            let y_norm = y_plus_half / Fixed::from_i32(height as i32);

            // Run program - it should return 3 values on stack for Vec3
            vm.run_with_coords(
                x_norm,
                y_norm,
                x_plus_half,
                y_plus_half,
                time,
                width,
                height,
            )
            .unwrap_or_else(|e| {
                panic!("Runtime error at pixel ({}, {}): {}", x, y, e);
            });

            // Pop 3 values from stack (b, g, r in reverse order)
            let b = vm
                .stack
                .pop_fixed()
                .expect("Vec3 should have blue component");
            let g = vm
                .stack
                .pop_fixed()
                .expect("Vec3 should have green component");
            let r = vm
                .stack
                .pop_fixed()
                .expect("Vec3 should have red component");

            let idx = (y * width + x) * 3;
            if idx + 2 < output.len() {
                output[idx] = r;
                output[idx + 1] = g;
                output[idx + 2] = b;
            }
        }
    }
}
