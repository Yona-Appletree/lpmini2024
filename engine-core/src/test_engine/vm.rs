/// Stack-based VM for pixel operations using fixed-point arithmetic
use crate::math::{self, Fixed, FIXED_ONE, FIXED_SHIFT};

// Re-export for backward compatibility
pub use crate::math::fixed_from_f32;
pub use crate::math::fixed_from_int;
pub use crate::math::fixed_to_f32;

/// Compute angle from center (0..1 for 0..2π)
fn compute_center_angle(x: Fixed, y: Fixed, width: usize, height: usize) -> Fixed {
    let center_x = math::fixed::div(
        math::fixed::from_int(width as i32),
        math::fixed::from_int(2),
    );
    let center_y = math::fixed::div(
        math::fixed::from_int(height as i32),
        math::fixed::from_int(2),
    );
    let dx = x - center_x;
    let dy = y - center_y;

    if dx == 0 && dy == 0 {
        return 0; // Center has no angle
    }

    // Approximate atan2 using octants
    let abs_dx = dx.abs();
    let abs_dy = dy.abs();

    let angle = if abs_dx > abs_dy {
        // Closer to horizontal
        let ratio = math::fixed::div(abs_dy, abs_dx);
        math::fixed::div(ratio, math::fixed::from_int(8)) // Scale to ~0..0.125
    } else if abs_dy > 0 {
        // Closer to vertical
        let ratio = math::fixed::div(abs_dx, abs_dy);
        math::fixed::div(FIXED_ONE, math::fixed::from_int(4))
            - math::fixed::div(ratio, math::fixed::from_int(8))
    } else {
        0
    };

    // Adjust based on quadrant
    if dx >= 0 && dy >= 0 {
        angle // Q1: 0 to 0.25
    } else if dx < 0 && dy >= 0 {
        math::fixed::div(FIXED_ONE, math::fixed::from_int(2)) - angle // Q2: 0.25 to 0.5
    } else if dx < 0 && dy < 0 {
        math::fixed::div(FIXED_ONE, math::fixed::from_int(2)) + angle // Q3: 0.5 to 0.75
    } else {
        FIXED_ONE - angle // Q4: 0.75 to 1.0
    }
}

/// Execute a native function call
fn execute_native_function(func_id: u8, vm: &mut VM) {
    use crate::expr::NativeFunction;
    use crate::math::fixed::{div as fixed_div, mul as fixed_mul};

    match func_id {
        id if id == NativeFunction::Min as u8 => {
            let b = vm.pop();
            let a = vm.pop();
            vm.push(a.min(b));
        }
        id if id == NativeFunction::Max as u8 => {
            let b = vm.pop();
            let a = vm.pop();
            vm.push(a.max(b));
        }
        id if id == NativeFunction::Pow as u8 => {
            let exp = vm.pop();
            let base = vm.pop();
            let exp_int = exp >> FIXED_SHIFT;
            let mut result = FIXED_ONE;
            for _ in 0..exp_int.max(0) {
                result = fixed_mul(result, base);
            }
            vm.push(result);
        }
        id if id == NativeFunction::Abs as u8 => {
            let a = vm.pop();
            vm.push(a.abs());
        }
        id if id == NativeFunction::Floor as u8 => {
            let a = vm.pop();
            vm.push(a & !(FIXED_ONE - 1));
        }
        id if id == NativeFunction::Ceil as u8 => {
            let a = vm.pop();
            let frac = a & (FIXED_ONE - 1);
            vm.push(if frac > 0 {
                (a & !(FIXED_ONE - 1)) + FIXED_ONE
            } else {
                a
            });
        }
        id if id == NativeFunction::Sqrt as u8 => {
            let a = vm.pop();
            // Simple integer sqrt approximation for fixed-point
            let mut result = 0i32;
            let mut bit = 1i32 << 30;
            while bit > a {
                bit >>= 2;
            }
            while bit != 0 {
                if a >= result + bit {
                    vm.push(result + bit);
                    result = (result >> 1) + bit;
                } else {
                    result >>= 1;
                }
                bit >>= 2;
            }
            vm.push(result << (FIXED_SHIFT / 2));
        }
        id if id == NativeFunction::Sign as u8 => {
            let a = vm.pop();
            vm.push(if a > 0 {
                FIXED_ONE
            } else if a < 0 {
                -FIXED_ONE
            } else {
                0
            });
        }
        id if id == NativeFunction::Saturate as u8 => {
            let a = vm.pop();
            vm.push(a.max(0).min(FIXED_ONE));
        }
        id if id == NativeFunction::Step as u8 => {
            let x = vm.pop();
            let edge = vm.pop();
            vm.push(if x < edge { 0 } else { FIXED_ONE });
        }
        id if id == NativeFunction::Clamp as u8 => {
            let max = vm.pop();
            let min = vm.pop();
            let val = vm.pop();
            vm.push(val.max(min).min(max));
        }
        id if id == NativeFunction::Lerp as u8 => {
            let t = vm.pop();
            let b = vm.pop();
            let a = vm.pop();
            vm.push(a + fixed_mul(b - a, t));
        }
        id if id == NativeFunction::Smoothstep as u8 => {
            let x = vm.pop();
            let edge1 = vm.pop();
            let edge0 = vm.pop();
            let t = fixed_div(x - edge0, edge1 - edge0).max(0).min(FIXED_ONE);
            let t_sq = fixed_mul(t, t);
            vm.push(fixed_mul(t_sq, (FIXED_ONE * 3) - (t << 1)));
        }
        id if id == NativeFunction::Less as u8 => {
            let b = vm.pop();
            let a = vm.pop();
            vm.push(if a < b { FIXED_ONE } else { 0 });
        }
        id if id == NativeFunction::Greater as u8 => {
            let b = vm.pop();
            let a = vm.pop();
            vm.push(if a > b { FIXED_ONE } else { 0 });
        }
        id if id == NativeFunction::LessEq as u8 => {
            let b = vm.pop();
            let a = vm.pop();
            vm.push(if a <= b { FIXED_ONE } else { 0 });
        }
        id if id == NativeFunction::GreaterEq as u8 => {
            let b = vm.pop();
            let a = vm.pop();
            vm.push(if a >= b { FIXED_ONE } else { 0 });
        }
        id if id == NativeFunction::Eq as u8 => {
            let b = vm.pop();
            let a = vm.pop();
            vm.push(if a == b { FIXED_ONE } else { 0 });
        }
        id if id == NativeFunction::NotEq as u8 => {
            let b = vm.pop();
            let a = vm.pop();
            vm.push(if a != b { FIXED_ONE } else { 0 });
        }
        id if id == NativeFunction::And as u8 => {
            let b = vm.pop();
            let a = vm.pop();
            vm.push(if a != 0 && b != 0 { FIXED_ONE } else { 0 });
        }
        id if id == NativeFunction::Or as u8 => {
            let b = vm.pop();
            let a = vm.pop();
            vm.push(if a != 0 || b != 0 { FIXED_ONE } else { 0 });
        }
        id if id == NativeFunction::Select as u8 => {
            let f = vm.pop();
            let t = vm.pop();
            let c = vm.pop();
            vm.push(if c != 0 { t } else { f });
        }
        _ => {} // Unknown function
    }
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

    // Native function call (ID determines which function)
    CallNative(u8),

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
                self.push(math::fixed::mul(a, b));
            }
            OpCode::Div => {
                let b = self.pop();
                let a = self.pop();
                if b != 0 {
                    self.push(math::fixed::div(a, b));
                } else {
                    self.push(0);
                }
            }
            OpCode::Sin => {
                let a = self.pop();
                let sin_val = math::trig::sin(a);
                // Map -1..1 to 0..1 for consistency with rest of system
                self.push((sin_val + FIXED_ONE) >> 1);
            }
            OpCode::Cos => {
                let a = self.pop();
                let cos_val = math::trig::cos(a);
                // Map -1..1 to 0..1 for consistency with rest of system
                self.push((cos_val + FIXED_ONE) >> 1);
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
                self.push(math::noise::perlin3(x, y, z, *octaves));
            }

            OpCode::CallNative(func_id) => {
                execute_native_function(*func_id, self);
            }

            OpCode::Load(source) => {
                let value = match source {
                    LoadSource::XInt => self.x,
                    LoadSource::YInt => self.y,
                    LoadSource::XNorm => {
                        // Normalize to 0..1 range
                        if self.width > 0 {
                            math::fixed::div(self.x, ((self.width - 1) as i32) << FIXED_SHIFT)
                        } else {
                            0
                        }
                    }
                    LoadSource::YNorm => {
                        // Normalize to 0..1 range
                        if self.height > 0 {
                            math::fixed::div(self.y, ((self.height - 1) as i32) << FIXED_SHIFT)
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
