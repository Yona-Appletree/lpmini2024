/// Stack-based VM for pixel operations using fixed-point arithmetic
/// This VM executes simple programs to transform pixel buffers efficiently on ESP32
use crate::sin_table::SIN_TABLE_I32 as SIN_TABLE;

// Fixed-point format: 16.16 (16 bits integer, 16 bits fractional)
pub type Fixed = i32;

pub const FIXED_SHIFT: i32 = 16;
pub const FIXED_ONE: Fixed = 1 << FIXED_SHIFT;

/// Convert f32 to fixed-point
#[inline(always)]
pub fn fixed_from_f32(f: f32) -> Fixed {
    (f * FIXED_ONE as f32) as Fixed
}

/// Convert fixed-point to f32 (for testing/debugging)
#[inline(always)]
pub fn fixed_to_f32(f: Fixed) -> f32 {
    f as f32 / FIXED_ONE as f32
}

/// Fixed-point multiplication
#[inline(always)]
fn fixed_mul(a: Fixed, b: Fixed) -> Fixed {
    ((a as i64 * b as i64) >> FIXED_SHIFT) as Fixed
}

/// Fixed-point division
#[inline(always)]
fn fixed_div(a: Fixed, b: Fixed) -> Fixed {
    ((a as i64 * FIXED_ONE as i64) / b as i64) as Fixed
}

/// Fast sine using lookup table
#[inline(always)]
fn sin_fixed(x: Fixed) -> Fixed {
    const TWO_PI_FIXED: i64 = 411775;
    let normalized = ((x as i64).rem_euclid(TWO_PI_FIXED)) as Fixed;
    let index = ((normalized as i64 * 256) / TWO_PI_FIXED) as usize & 0xFF;
    SIN_TABLE[index]
}

/// Fast cosine using lookup table
#[inline(always)]
fn cos_fixed(x: Fixed) -> Fixed {
    const PI_DIV_2_FIXED: Fixed = 102944;
    sin_fixed(x + PI_DIV_2_FIXED)
}

/// Perlin3 noise using fixed-point
#[inline(always)]
fn perlin3_fixed(x: Fixed, y: Fixed, z: Fixed) -> Fixed {
    let freq1 = FIXED_ONE;
    let freq2 = FIXED_ONE * 2;
    let freq3 = FIXED_ONE * 4;

    let n1 = fixed_mul(
        sin_fixed(fixed_mul(x, freq1) + z),
        cos_fixed(fixed_mul(y, freq1)),
    );
    let n2 = fixed_mul(
        sin_fixed(fixed_mul(x, freq2) - z),
        cos_fixed(fixed_mul(y, freq2)),
    ) / 2;
    let n3 = sin_fixed(fixed_mul(x, freq3) + y + z) / 4;

    let sum = n1 + n2 + n3;
    // Divide by 1.75 = multiply by 1/1.75 ≈ 0.5714
    // In fixed point: 0.5714 * 65536 ≈ 37450
    fixed_mul(sum, 37450)
}

/// OpCode instructions for the VM
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OpCode {
    // Stack operations
    Push(Fixed),
    Dup,
    Drop,
    Swap,

    // Arithmetic operations
    Add,
    Sub,
    Mul,
    Div,

    // Math functions
    Sin,
    Cos,
    Perlin3,

    // Load coordinates
    LoadX,
    LoadY,
    LoadTime,

    // Buffer operations
    LoadInput,   // Load current input pixel
    LoadInputAt, // Load input[x, y] where x, y are popped from stack

    // Control flow
    Jump(i32),    // Unconditional jump by offset
    JumpEq(i32),  // Pop b, a; jump by offset if a == b
    JumpNe(i32),  // Pop b, a; jump by offset if a != b
    JumpLt(i32),  // Pop b, a; jump by offset if a < b
    JumpLte(i32), // Pop b, a; jump by offset if a <= b
    JumpGt(i32),  // Pop b, a; jump by offset if a > b
    JumpGte(i32), // Pop b, a; jump by offset if a >= b

    // Output
    Return, // Pop value and return (ends execution for this pixel)
}

/// VM execution state
struct VM<'a> {
    stack: [Fixed; 64],
    sp: usize,          // Stack pointer
    pc: usize,          // Program counter
    x: Fixed,           // Current pixel x coordinate
    y: Fixed,           // Current pixel y coordinate
    time: Fixed,        // Time value
    input: &'a [Fixed], // Input buffer
    width: usize,
    height: usize,
}

impl<'a> VM<'a> {
    fn new(
        input: &'a [Fixed],
        width: usize,
        height: usize,
        time: Fixed,
        x: Fixed,
        y: Fixed,
    ) -> Self {
        VM {
            stack: [0; 64],
            sp: 0,
            pc: 0,
            x,
            y,
            time,
            input,
            width,
            height,
        }
    }

    #[inline(always)]
    fn push(&mut self, value: Fixed) {
        if self.sp < 64 {
            self.stack[self.sp] = value;
            self.sp += 1;
        }
    }

    #[inline(always)]
    fn pop(&mut self) -> Fixed {
        if self.sp > 0 {
            self.sp -= 1;
            self.stack[self.sp]
        } else {
            0
        }
    }

    #[inline(always)]
    fn peek(&self) -> Fixed {
        if self.sp > 0 {
            self.stack[self.sp - 1]
        } else {
            0
        }
    }

    /// Execute a single instruction
    #[inline(always)]
    fn execute_op(&mut self, op: &OpCode) -> Option<Fixed> {
        match op {
            OpCode::Push(val) => {
                self.push(*val);
            }
            OpCode::Dup => {
                let val = self.peek();
                self.push(val);
            }
            OpCode::Drop => {
                self.pop();
            }
            OpCode::Swap => {
                if self.sp >= 2 {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(a);
                    self.push(b);
                }
            }
            OpCode::Add => {
                let b = self.pop();
                let a = self.pop();
                self.push(a + b);
            }
            OpCode::Sub => {
                let b = self.pop();
                let a = self.pop();
                self.push(a - b);
            }
            OpCode::Mul => {
                let b = self.pop();
                let a = self.pop();
                self.push(fixed_mul(a, b));
            }
            OpCode::Div => {
                let b = self.pop();
                let a = self.pop();
                if b != 0 {
                    self.push(fixed_div(a, b));
                } else {
                    self.push(0);
                }
            }
            OpCode::Sin => {
                let a = self.pop();
                self.push(sin_fixed(a));
            }
            OpCode::Cos => {
                let a = self.pop();
                self.push(cos_fixed(a));
            }
            OpCode::Perlin3 => {
                let z = self.pop();
                let y = self.pop();
                let x = self.pop();
                self.push(perlin3_fixed(x, y, z));
            }
            OpCode::LoadX => {
                self.push(self.x);
            }
            OpCode::LoadY => {
                self.push(self.y);
            }
            OpCode::LoadTime => {
                self.push(self.time);
            }
            OpCode::LoadInput => {
                let x_int = (self.x >> FIXED_SHIFT) as usize;
                let y_int = (self.y >> FIXED_SHIFT) as usize;
                let idx = y_int * self.width + x_int;
                if idx < self.input.len() {
                    self.push(self.input[idx]);
                } else {
                    self.push(0);
                }
            }
            OpCode::LoadInputAt => {
                let y = self.pop();
                let x = self.pop();
                let x_int = (x >> FIXED_SHIFT) as usize;
                let y_int = (y >> FIXED_SHIFT) as usize;
                if x_int < self.width && y_int < self.height {
                    let idx = y_int * self.width + x_int;
                    if idx < self.input.len() {
                        self.push(self.input[idx]);
                    } else {
                        self.push(0);
                    }
                } else {
                    self.push(0);
                }
            }
            OpCode::Jump(offset) => {
                // Subtract 1 because the main loop will increment pc after this
                self.pc = ((self.pc as i32 + offset) - 1) as usize;
                return None;
            }
            OpCode::JumpEq(offset) => {
                let b = self.pop();
                let a = self.pop();
                if a == b {
                    // Subtract 1 because the main loop will increment pc after this
                    self.pc = ((self.pc as i32 + offset) - 1) as usize;
                    return None;
                }
            }
            OpCode::JumpNe(offset) => {
                let b = self.pop();
                let a = self.pop();
                if a != b {
                    self.pc = ((self.pc as i32 + offset) - 1) as usize;
                    return None;
                }
            }
            OpCode::JumpLt(offset) => {
                let b = self.pop();
                let a = self.pop();
                if a < b {
                    self.pc = ((self.pc as i32 + offset) - 1) as usize;
                    return None;
                }
            }
            OpCode::JumpLte(offset) => {
                let b = self.pop();
                let a = self.pop();
                if a <= b {
                    self.pc = ((self.pc as i32 + offset) - 1) as usize;
                    return None;
                }
            }
            OpCode::JumpGt(offset) => {
                let b = self.pop();
                let a = self.pop();
                if a > b {
                    self.pc = ((self.pc as i32 + offset) - 1) as usize;
                    return None;
                }
            }
            OpCode::JumpGte(offset) => {
                let b = self.pop();
                let a = self.pop();
                if a >= b {
                    self.pc = ((self.pc as i32 + offset) - 1) as usize;
                    return None;
                }
            }
            OpCode::Return => {
                return Some(self.pop());
            }
        }
        None
    }

    /// Execute the program for this pixel
    fn run(&mut self, program: &[OpCode]) -> Fixed {
        self.pc = 0;
        while self.pc < program.len() {
            let op = &program[self.pc];
            if let Some(result) = self.execute_op(op) {
                return result;
            }
            self.pc += 1;
        }
        // If no explicit return, pop top of stack
        self.pop()
    }
}

/// Execute a program on all pixels in the buffer
///
/// # Arguments
/// * `input` - Input buffer (16.16 fixed-point grayscale values)
/// * `output` - Output buffer (16.16 fixed-point grayscale values)
/// * `program` - Array of opcodes to execute
/// * `width` - Width of the image
/// * `height` - Height of the image
/// * `time` - Time value in 16.16 fixed-point format
#[inline(never)]
pub fn execute_program(
    input: &[Fixed],
    output: &mut [Fixed],
    program: &[OpCode],
    width: usize,
    height: usize,
    time: Fixed,
) {
    for y in 0..height {
        for x in 0..width {
            let x_fixed = (x as i32) << FIXED_SHIFT;
            let y_fixed = (y as i32) << FIXED_SHIFT;

            let mut vm = VM::new(input, width, height, time, x_fixed, y_fixed);
            let result = vm.run(program);

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

    #[test]
    fn test_stack_operations() {
        let input = vec![0; 1];
        let mut output = vec![0; 1];

        // Test Dup: push 5, dup, add (should be 10)
        let program = vec![
            OpCode::Push(5 << FIXED_SHIFT),
            OpCode::Dup,
            OpCode::Add,
            OpCode::Return,
        ];
        execute_program(&input, &mut output, &program, 1, 1, 0);
        assert_eq!(output[0], 10 << FIXED_SHIFT);

        // Test Swap: push 3, push 7, swap, sub (should be 7-3=4)
        let program = vec![
            OpCode::Push(3 << FIXED_SHIFT),
            OpCode::Push(7 << FIXED_SHIFT),
            OpCode::Swap,
            OpCode::Sub,
            OpCode::Return,
        ];
        execute_program(&input, &mut output, &program, 1, 1, 0);
        assert_eq!(output[0], 4 << FIXED_SHIFT);

        // Test Drop: push 100, push 5, drop (should leave 100)
        let program = vec![
            OpCode::Push(100 << FIXED_SHIFT),
            OpCode::Push(5 << FIXED_SHIFT),
            OpCode::Drop,
            OpCode::Return,
        ];
        execute_program(&input, &mut output, &program, 1, 1, 0);
        assert_eq!(output[0], 100 << FIXED_SHIFT);
    }

    #[test]
    fn test_arithmetic() {
        let input = vec![0; 1];
        let mut output = vec![0; 1];

        // Addition
        let program = vec![
            OpCode::Push(2 << FIXED_SHIFT),
            OpCode::Push(3 << FIXED_SHIFT),
            OpCode::Add,
            OpCode::Return,
        ];
        execute_program(&input, &mut output, &program, 1, 1, 0);
        assert_eq!(output[0], 5 << FIXED_SHIFT);

        // Subtraction
        let program = vec![
            OpCode::Push(10 << FIXED_SHIFT),
            OpCode::Push(3 << FIXED_SHIFT),
            OpCode::Sub,
            OpCode::Return,
        ];
        execute_program(&input, &mut output, &program, 1, 1, 0);
        assert_eq!(output[0], 7 << FIXED_SHIFT);

        // Multiplication
        let program = vec![
            OpCode::Push(4 << FIXED_SHIFT),
            OpCode::Push(5 << FIXED_SHIFT),
            OpCode::Mul,
            OpCode::Return,
        ];
        execute_program(&input, &mut output, &program, 1, 1, 0);
        assert_eq!(output[0], 20 << FIXED_SHIFT);

        // Division
        let program = vec![
            OpCode::Push(20 << FIXED_SHIFT),
            OpCode::Push(4 << FIXED_SHIFT),
            OpCode::Div,
            OpCode::Return,
        ];
        execute_program(&input, &mut output, &program, 1, 1, 0);
        assert_eq!(output[0], 5 << FIXED_SHIFT);
    }

    #[test]
    fn test_jump_eq() {
        let input = vec![0; 1];
        let mut output = vec![0; 1];

        // Test equal: 5 == 5 should jump
        let program = vec![
            OpCode::Push(5 << FIXED_SHIFT),
            OpCode::Push(5 << FIXED_SHIFT),
            OpCode::JumpEq(3), // Jump over next 2 instructions
            OpCode::Push(0),   // Should be skipped
            OpCode::Return,
            OpCode::Push(100), // Should execute
            OpCode::Return,
        ];
        execute_program(&input, &mut output, &program, 1, 1, 0);
        assert_eq!(output[0], 100);

        // Test not equal: 5 != 3 should not jump
        let program = vec![
            OpCode::Push(5 << FIXED_SHIFT),
            OpCode::Push(3 << FIXED_SHIFT),
            OpCode::JumpEq(3), // Should not jump
            OpCode::Push(42),  // Should execute
            OpCode::Return,
            OpCode::Push(100), // Should not execute
            OpCode::Return,
        ];
        execute_program(&input, &mut output, &program, 1, 1, 0);
        assert_eq!(output[0], 42);
    }

    #[test]
    fn test_jump_ne() {
        let input = vec![0; 1];
        let mut output = vec![0; 1];

        // Test not equal: 5 != 3 should jump
        let program = vec![
            OpCode::Push(5 << FIXED_SHIFT),
            OpCode::Push(3 << FIXED_SHIFT),
            OpCode::JumpNe(3), // Should jump
            OpCode::Push(0),   // Should be skipped
            OpCode::Return,
            OpCode::Push(100), // Should execute
            OpCode::Return,
        ];
        execute_program(&input, &mut output, &program, 1, 1, 0);
        assert_eq!(output[0], 100);
    }

    #[test]
    fn test_jump_lt() {
        let input = vec![0; 1];
        let mut output = vec![0; 1];

        // Test less than: 3 < 5 should jump
        let program = vec![
            OpCode::Push(3 << FIXED_SHIFT),
            OpCode::Push(5 << FIXED_SHIFT),
            OpCode::JumpLt(3), // Should jump
            OpCode::Push(0),   // Should be skipped
            OpCode::Return,
            OpCode::Push(100), // Should execute
            OpCode::Return,
        ];
        execute_program(&input, &mut output, &program, 1, 1, 0);
        assert_eq!(output[0], 100);

        // Test not less than: 5 < 3 should not jump
        let program = vec![
            OpCode::Push(5 << FIXED_SHIFT),
            OpCode::Push(3 << FIXED_SHIFT),
            OpCode::JumpLt(3), // Should not jump
            OpCode::Push(42),  // Should execute
            OpCode::Return,
            OpCode::Push(100), // Should not execute
            OpCode::Return,
        ];
        execute_program(&input, &mut output, &program, 1, 1, 0);
        assert_eq!(output[0], 42);
    }

    #[test]
    fn test_jump_lte() {
        let input = vec![0; 1];
        let mut output = vec![0; 1];

        // Test less than or equal: 5 <= 5 should jump
        let program = vec![
            OpCode::Push(5 << FIXED_SHIFT),
            OpCode::Push(5 << FIXED_SHIFT),
            OpCode::JumpLte(3), // Should jump
            OpCode::Push(0),    // Should be skipped
            OpCode::Return,
            OpCode::Push(100), // Should execute
            OpCode::Return,
        ];
        execute_program(&input, &mut output, &program, 1, 1, 0);
        assert_eq!(output[0], 100);
    }

    #[test]
    fn test_jump_gt() {
        let input = vec![0; 1];
        let mut output = vec![0; 1];

        // Test greater than: 7 > 3 should jump
        let program = vec![
            OpCode::Push(7 << FIXED_SHIFT),
            OpCode::Push(3 << FIXED_SHIFT),
            OpCode::JumpGt(3), // Should jump
            OpCode::Push(0),   // Should be skipped
            OpCode::Return,
            OpCode::Push(100), // Should execute
            OpCode::Return,
        ];
        execute_program(&input, &mut output, &program, 1, 1, 0);
        assert_eq!(output[0], 100);
    }

    #[test]
    fn test_jump_gte() {
        let input = vec![0; 1];
        let mut output = vec![0; 1];

        // Test greater than or equal: 5 >= 5 should jump
        let program = vec![
            OpCode::Push(5 << FIXED_SHIFT),
            OpCode::Push(5 << FIXED_SHIFT),
            OpCode::JumpGte(3), // Should jump
            OpCode::Push(0),    // Should be skipped
            OpCode::Return,
            OpCode::Push(100), // Should execute
            OpCode::Return,
        ];
        execute_program(&input, &mut output, &program, 1, 1, 0);
        assert_eq!(output[0], 100);
    }

    #[test]
    fn test_unconditional_jump() {
        let input = vec![0; 1];
        let mut output = vec![0; 1];

        // Jump forward
        let program = vec![
            OpCode::Jump(3),  // Jump forward 3 instructions
            OpCode::Push(0),  // Should be skipped
            OpCode::Push(0),  // Should be skipped
            OpCode::Push(0),  // Should be skipped
            OpCode::Push(42), // Should execute
            OpCode::Return,
        ];
        execute_program(&input, &mut output, &program, 1, 1, 0);
        assert_eq!(output[0], 42);
    }

    #[test]
    fn test_coordinates() {
        let input = vec![0; 4];
        let mut output = vec![0; 4];

        // Program: load x, load y, add, return
        let program = vec![OpCode::LoadX, OpCode::LoadY, OpCode::Add, OpCode::Return];

        execute_program(&input, &mut output, &program, 2, 2, 0);

        // (0,0) -> 0, (1,0) -> 1, (0,1) -> 1, (1,1) -> 2
        assert_eq!(output[0], 0);
        assert_eq!(output[1], 1 << FIXED_SHIFT);
        assert_eq!(output[2], 1 << FIXED_SHIFT);
        assert_eq!(output[3], 2 << FIXED_SHIFT);
    }

    #[test]
    fn test_sin_cos() {
        let input = vec![0; 1];
        let mut output = vec![0; 1];

        // Program: push 0, sin, return (sin(0) should be ~0)
        let program = vec![OpCode::Push(0), OpCode::Sin, OpCode::Return];

        execute_program(&input, &mut output, &program, 1, 1, 0);
        let result = fixed_to_f32(output[0]);
        assert!(result.abs() < 0.01, "sin(0) should be ~0, got {}", result);

        // Test cos(0) should be ~1
        let program = vec![OpCode::Push(0), OpCode::Cos, OpCode::Return];

        execute_program(&input, &mut output, &program, 1, 1, 0);
        let result = fixed_to_f32(output[0]);
        assert!(
            (result - 1.0).abs() < 0.01,
            "cos(0) should be ~1, got {}",
            result
        );
    }

    #[test]
    fn test_perlin3() {
        let input = vec![0; 4];
        let mut output = vec![0; 4];

        // Program: perlin3(x, y, 0)
        let program = vec![
            OpCode::LoadX,
            OpCode::LoadY,
            OpCode::Push(0),
            OpCode::Perlin3,
            OpCode::Return,
        ];

        execute_program(&input, &mut output, &program, 2, 2, 0);

        // Check that we got some values (not all zero)
        let has_nonzero = output.iter().any(|&v| v != 0);
        assert!(has_nonzero, "Perlin3 should produce non-zero values");

        // Check that values are in reasonable range (-1 to 1 in fixed point)
        for (i, &val) in output.iter().enumerate() {
            let f = fixed_to_f32(val);
            assert!(
                f >= -1.5 && f <= 1.5,
                "Perlin value at {} out of range: {}",
                i,
                f
            );
        }
    }

    #[test]
    fn test_conditional_jump() {
        let input = vec![0; 4];
        let mut output = vec![0; 4];

        // Program: if x < 0.5 then 0 else 1
        let program = vec![
            OpCode::LoadX,
            OpCode::Push(1 << (FIXED_SHIFT - 1)), // 0.5
            OpCode::JumpLt(3),                    // if x < 0.5, jump to Push(0)
            OpCode::Push(1 << FIXED_SHIFT),       // push 1
            OpCode::Return,
            OpCode::Push(0), // push 0
            OpCode::Return,
        ];

        execute_program(&input, &mut output, &program, 2, 2, 0);

        // At x=0, y=0: should return 0 (0 < 0.5)
        assert_eq!(output[0], 0, "x=0 should produce 0");
        // At x=1, y=0: should return 1 (1 >= 0.5)
        assert_eq!(output[1], 1 << FIXED_SHIFT, "x=1 should produce 1");
        // At x=0, y=1: should return 0 (0 < 0.5)
        assert_eq!(output[2], 0, "x=0 should produce 0");
        // At x=1, y=1: should return 1 (1 >= 0.5)
        assert_eq!(output[3], 1 << FIXED_SHIFT, "x=1 should produce 1");
    }

    #[test]
    fn test_full_example_program() {
        let input = vec![0; 16];
        let mut output = vec![0; 16];

        // The actual benchmark program:
        // v = perlin3(x, y, time)
        // v = cos(v)
        // v = v < 0.5 ? 0 : 1
        let program = vec![
            OpCode::LoadX,
            OpCode::LoadY,
            OpCode::LoadTime,
            OpCode::Perlin3,
            OpCode::Cos,
            OpCode::Push(1 << (FIXED_SHIFT - 1)), // 0.5
            OpCode::JumpLt(3),                    // if v < 0.5, jump to Push(0)
            OpCode::Push(1 << FIXED_SHIFT),       // push 1
            OpCode::Return,
            OpCode::Push(0), // push 0
            OpCode::Return,
        ];

        execute_program(&input, &mut output, &program, 4, 4, fixed_from_f32(1.5));

        // Verify all outputs are either 0 or 1 (since we have a threshold)
        for (i, &val) in output.iter().enumerate() {
            let f = fixed_to_f32(val);
            assert!(
                (f - 0.0).abs() < 0.01 || (f - 1.0).abs() < 0.01,
                "Pixel {} should be 0 or 1, got {}",
                i,
                f
            );
        }

        println!("Example program output (4x4):");
        for y in 0..4 {
            for x in 0..4 {
                let val = fixed_to_f32(output[y * 4 + x]);
                print!("{:.0} ", val);
            }
            println!();
        }

        // With a non-zero time, we should get some variation
        // Just verify the program runs successfully producing valid binary output
        assert_eq!(output.len(), 16, "Should have 16 outputs");
    }

    #[test]
    fn test_load_input() {
        // Set up input buffer with known values
        let input: Vec<i32> = (0..9).map(|i| i << FIXED_SHIFT).collect();
        let mut output = vec![0; 9];

        // Program: load current input value and return it
        let program = vec![OpCode::LoadInput, OpCode::Return];

        execute_program(&input, &mut output, &program, 3, 3, 0);

        // Output should match input
        for i in 0..9 {
            assert_eq!(output[i], input[i], "Pixel {} mismatch", i);
        }
    }

    #[test]
    fn test_load_input_at() {
        // Set up input buffer with values equal to their index
        let input: Vec<i32> = (0..9).map(|i| i << FIXED_SHIFT).collect();
        let mut output = vec![0; 9];

        // Program: load input[1, 1] (center pixel, index 4)
        let program = vec![
            OpCode::Push(1 << FIXED_SHIFT), // x = 1
            OpCode::Push(1 << FIXED_SHIFT), // y = 1
            OpCode::LoadInputAt,
            OpCode::Return,
        ];

        execute_program(&input, &mut output, &program, 3, 3, 0);

        // All outputs should be 4 (the value at input[1,1])
        for i in 0..9 {
            assert_eq!(output[i], 4 << FIXED_SHIFT, "Pixel {} should be 4", i);
        }
    }
}
