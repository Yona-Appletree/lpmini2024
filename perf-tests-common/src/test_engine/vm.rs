/// Stack-based VM for pixel operations using fixed-point arithmetic
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
/// Input: 0..1 represents 0..2π
/// Output: 0..1 represents -1..1 mapped to 0..1
#[inline(always)]
fn sin_fixed(x: Fixed) -> Fixed {
    // Map 0..1 input to 0..255 table index
    let index = ((x as i64 * 256) >> FIXED_SHIFT) as usize & 0xFF;
    let sin_val = SIN_TABLE[index];
    
    // Map -1..1 to 0..1
    // result = (sin_val + 1) / 2
    (sin_val + FIXED_ONE) >> 1
}

/// Fast cosine using lookup table
/// Input: 0..1 represents 0..2π
/// Output: 0..1 represents -1..1 mapped to 0..1
#[inline(always)]
fn cos_fixed(x: Fixed) -> Fixed {
    // cos(x) = sin(x + 0.25) where 0.25 represents π/2 in 0..1 range
    const QUARTER: Fixed = FIXED_ONE / 4;
    sin_fixed(x + QUARTER)
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

/// Load source specifier
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoadSource {
    XInt,        // Integer X coordinate (0..width-1)
    YInt,        // Integer Y coordinate (0..height-1)
    XNorm,       // Normalized X (0..1)
    YNorm,       // Normalized Y (0..1)
    Time,        // Time value
    TimeNorm,    // Time normalized to 0..1 range (wraps at 1.0)
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
    Load(LoadSource),

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

// Convenience constructors for backward compatibility
impl OpCode {
    pub const fn LoadX() -> Self {
        OpCode::Load(LoadSource::XNorm)
    }
    pub const fn LoadY() -> Self {
        OpCode::Load(LoadSource::YNorm)
    }
    pub const fn LoadTime() -> Self {
        OpCode::Load(LoadSource::Time)
    }
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
            OpCode::Load(source) => {
                let value = match source {
                    LoadSource::XInt => self.x,
                    LoadSource::YInt => self.y,
                    LoadSource::XNorm => {
                        // Normalize to 0..1 range
                        if self.width > 0 {
                            fixed_div(self.x, ((self.width - 1) as i32) << FIXED_SHIFT)
                        } else {
                            0
                        }
                    }
                    LoadSource::YNorm => {
                        // Normalize to 0..1 range
                        if self.height > 0 {
                            fixed_div(self.y, ((self.height - 1) as i32) << FIXED_SHIFT)
                        } else {
                            0
                        }
                    }
                    LoadSource::Time => self.time,
                    LoadSource::TimeNorm => {
                        // Wrap time to 0..1 range
                        (self.time as i64).rem_euclid(FIXED_ONE as i64) as Fixed
                    }
                };
                self.push(value);
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

