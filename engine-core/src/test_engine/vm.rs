use crate::math::fixed_from_int;
/// Stack-based VM for pixel operations using fixed-point arithmetic
use crate::sin_table::SIN_TABLE_I32 as SIN_TABLE;

// Permutation table for perlin noise (standard 256-entry table)
const PERM: [u8; 256] = [
    151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69,
    142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219,
    203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175,
    74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230,
    220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76,
    132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173,
    186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206,
    59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163,
    70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232,
    178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
    241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204,
    176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141,
    128, 195, 78, 66, 215, 61, 156, 180,
];

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
pub fn sin_fixed(x: Fixed) -> Fixed {
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
pub fn cos_fixed(x: Fixed) -> Fixed {
    // cos(x) = sin(x + 0.25) where 0.25 represents π/2 in 0..1 range
    const QUARTER: Fixed = FIXED_ONE / 4;
    sin_fixed(x + QUARTER)
}

// Fade function for perlin noise: 6t^5 - 15t^4 + 10t^3
#[inline(always)]
fn fade(t: Fixed) -> Fixed {
    // Convert to 0..1 range for calculation
    let t_norm = t as i64;
    let t2 = (t_norm * t_norm) >> FIXED_SHIFT;
    let t3 = (t2 * t_norm) >> FIXED_SHIFT;
    let t4 = (t3 * t_norm) >> FIXED_SHIFT;
    let t5 = (t4 * t_norm) >> FIXED_SHIFT;

    let result = (6 * t5 - 15 * t4 + 10 * t3) as i32;
    result
}

// Linear interpolation
#[inline(always)]
fn lerp(t: Fixed, a: Fixed, b: Fixed) -> Fixed {
    a + fixed_mul(t, b - a)
}

// Gradient function - uses permutation table to get pseudo-random gradient
#[inline(always)]
fn grad(hash: u8, x: Fixed, y: Fixed, z: Fixed) -> Fixed {
    let h = hash & 15;
    // Convert hash to one of 12 gradient directions
    let u = if h < 8 { x } else { y };
    let v = if h < 4 {
        y
    } else if h == 12 || h == 14 {
        x
    } else {
        z
    };
    (if h & 1 == 0 { u } else { -u }) + (if h & 2 == 0 { v } else { -v })
}

/// Single octave of Perlin3 noise
#[inline(always)]
fn perlin3_octave(x: Fixed, y: Fixed, z: Fixed) -> Fixed {
    // Get integer parts
    let xi = ((x >> FIXED_SHIFT) & 0xFF) as usize;
    let yi = ((y >> FIXED_SHIFT) & 0xFF) as usize;
    let zi = ((z >> FIXED_SHIFT) & 0xFF) as usize;

    // Get fractional parts (0..1 in fixed point)
    let xf = x & (FIXED_ONE - 1);
    let yf = y & (FIXED_ONE - 1);
    let zf = z & (FIXED_ONE - 1);

    // Fade curves
    let u = fade(xf);
    let v = fade(yf);
    let w = fade(zf);

    // Hash coordinates of 8 cube corners
    let p = &PERM;
    let a = p[xi] as usize;
    let aa = p[(a + yi) & 0xFF] as usize;
    let ab = p[(a + yi + 1) & 0xFF] as usize;
    let b = p[(xi + 1) & 0xFF] as usize;
    let ba = p[(b + yi) & 0xFF] as usize;
    let bb = p[(b + yi + 1) & 0xFF] as usize;

    // Blend results from 8 corners
    let x1 = lerp(
        u,
        grad(p[(aa + zi) & 0xFF], xf, yf, zf),
        grad(p[(ba + zi) & 0xFF], xf - FIXED_ONE, yf, zf),
    );
    let x2 = lerp(
        u,
        grad(p[(ab + zi) & 0xFF], xf, yf - FIXED_ONE, zf),
        grad(p[(bb + zi) & 0xFF], xf - FIXED_ONE, yf - FIXED_ONE, zf),
    );
    let y1 = lerp(v, x1, x2);

    let x3 = lerp(
        u,
        grad(p[(aa + zi + 1) & 0xFF], xf, yf, zf - FIXED_ONE),
        grad(p[(ba + zi + 1) & 0xFF], xf - FIXED_ONE, yf, zf - FIXED_ONE),
    );
    let x4 = lerp(
        u,
        grad(p[(ab + zi + 1) & 0xFF], xf, yf - FIXED_ONE, zf - FIXED_ONE),
        grad(
            p[(bb + zi + 1) & 0xFF],
            xf - FIXED_ONE,
            yf - FIXED_ONE,
            zf - FIXED_ONE,
        ),
    );
    let y2 = lerp(v, x3, x4);

    let result = lerp(w, y1, y2);

    // Output is roughly -0.7..0.7, leave as-is for octave mixing
    result
}

/// Fractal Perlin3 noise with multiple octaves
/// More octaves = more detail but slower
/// Returns value in 0..1 range
#[inline(always)]
fn perlin3_fixed(x: Fixed, y: Fixed, z: Fixed, octaves: u8) -> Fixed {
    let octaves = octaves.min(8).max(1);
    let mut total = 0i64;
    let mut amplitude = FIXED_ONE as i64;
    let mut max_value = 0i64;
    let mut frequency = FIXED_ONE;

    for _ in 0..octaves {
        let sample = perlin3_octave(
            fixed_mul(x, frequency),
            fixed_mul(y, frequency),
            fixed_mul(z, frequency),
        );
        total += (sample as i64 * amplitude) >> FIXED_SHIFT;
        max_value += amplitude;
        amplitude >>= 1; // Halve amplitude each octave
        frequency <<= 1; // Double frequency each octave
    }

    // Normalize to 0..1
    let normalized = ((total << FIXED_SHIFT) / max_value) as i32;
    let shifted = normalized + (FIXED_ONE >> 1);
    shifted.max(0).min(FIXED_ONE)
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
    CenterDist,  // Distance from center (0 at center, 1 at farthest corner)
    CenterAngle, // Angle from center (0-1 for 0-2π, 0 = east/right)
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
    Frac,        // Get fractional part of a number
    Perlin3(u8), // Perlin noise with N octaves (1-8)

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
            OpCode::Frac => {
                let a = self.pop();
                // Get fractional part: keep only fractional bits
                self.push(a & (FIXED_ONE - 1));
            }
            OpCode::Perlin3(octaves) => {
                let z = self.pop();
                let y = self.pop();
                let x = self.pop();
                self.push(perlin3_fixed(x, y, z, *octaves));
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
                    LoadSource::CenterDist => {
                        // Distance from center (0 at center, 1 at farthest corner)
                        let center_x = fixed_from_int(self.width as i32) >> 1;
                        let center_y = fixed_from_int(self.height as i32) >> 1;
                        let dx = self.x - center_x;
                        let dy = self.y - center_y;

                        // Use Manhattan distance normalized by half-diagonal
                        let manhattan =
                            (if dx < 0 { -dx } else { dx }) + (if dy < 0 { -dy } else { dy });
                        let max_manhattan = center_x + center_y;
                        if max_manhattan == 0 {
                            0
                        } else {
                            ((manhattan as i64 * FIXED_ONE as i64) / max_manhattan as i64) as Fixed
                        }
                    }
                    LoadSource::CenterAngle => {
                        // Angle from center (0-1 for 0-2π, 0 = east/right)
                        let center_x = fixed_from_int(self.width as i32) >> 1;
                        let center_y = fixed_from_int(self.height as i32) >> 1;
                        let dx = self.x - center_x;
                        let dy = self.y - center_y;

                        // atan2(dy, dx) normalized to 0..1
                        if dx == 0 && dy == 0 {
                            0 // Center has no angle
                        } else {
                            // Approximate atan2 using octants
                            let abs_dx = if dx < 0 { -dx } else { dx };
                            let abs_dy = if dy < 0 { -dy } else { dy };

                            let angle = if abs_dx > abs_dy {
                                // Closer to horizontal
                                let ratio = ((abs_dy as i64) << FIXED_SHIFT) / (abs_dx as i64);
                                (ratio as Fixed) >> 3 // Scale to ~0..0.125
                            } else if abs_dy > 0 {
                                // Closer to vertical
                                let ratio = ((abs_dx as i64) << FIXED_SHIFT) / (abs_dy as i64);
                                (FIXED_ONE >> 2) - ((ratio as Fixed) >> 3) // 0.25 - scaled ratio
                            } else {
                                0
                            };

                            // Adjust based on quadrant
                            if dx >= 0 && dy >= 0 {
                                // Q1: 0 to 0.25
                                angle
                            } else if dx < 0 && dy >= 0 {
                                // Q2: 0.25 to 0.5
                                (FIXED_ONE >> 1) - angle
                            } else if dx < 0 && dy < 0 {
                                // Q3: 0.5 to 0.75
                                (FIXED_ONE >> 1) + angle
                            } else {
                                // Q4: 0.75 to 1.0
                                FIXED_ONE - angle
                            }
                        }
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
