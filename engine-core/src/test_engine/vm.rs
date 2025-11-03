use crate::math::noise::perlin3;
use crate::math::trig::{cos, sin};
/// Stack-based VM for pixel operations using fixed-point arithmetic
use crate::math::{Fixed, ToFixed, FIXED_ONE, FIXED_SHIFT};

// Re-export for backward compatibility
#[deprecated(note = "Use Fixed::from_f32 instead")]
pub fn fixed_from_f32(f: f32) -> i32 {
    Fixed::from_f32(f).0
}

#[deprecated(note = "Use Fixed::from_i32 instead")]
pub fn fixed_from_int(i: i32) -> i32 {
    Fixed::from_i32(i).0
}

#[deprecated(note = "Use Fixed::to_f32 instead")]
pub fn fixed_to_f32(f: i32) -> f32 {
    Fixed(f).to_f32()
}

/// Compute angle from center (0..1 for 0..2π)
fn compute_center_angle(x: i32, y: i32, width: usize, height: usize) -> i32 {
    let center_x = Fixed::from_i32(width as i32) / 2i32.to_fixed();
    let center_y = Fixed::from_i32(height as i32) / 2i32.to_fixed();
    let dx = Fixed(x) - center_x;
    let dy = Fixed(y) - center_y;

    if dx.0 == 0 && dy.0 == 0 {
        return 0; // Center has no angle
    }

    // Approximate atan2 using octants
    let abs_dx = Fixed(dx.0.abs());
    let abs_dy = Fixed(dy.0.abs());

    let angle = if abs_dx.0 > abs_dy.0 {
        // Closer to horizontal
        let ratio = abs_dy / abs_dx;
        ratio / 8i32.to_fixed() // Scale to ~0..0.125
    } else if abs_dy.0 > 0 {
        // Closer to vertical
        let ratio = abs_dx / abs_dy;
        Fixed::ONE / 4i32.to_fixed() - ratio / 8i32.to_fixed()
    } else {
        Fixed::ZERO
    };

    // Adjust based on quadrant
    let result = if dx.0 >= 0 && dy.0 >= 0 {
        angle // Q1: 0 to 0.25
    } else if dx.0 < 0 && dy.0 >= 0 {
        Fixed::ONE / 2i32.to_fixed() - angle // Q2: 0.25 to 0.5
    } else if dx.0 < 0 && dy.0 < 0 {
        Fixed::ONE / 2i32.to_fixed() + angle // Q3: 0.5 to 0.75
    } else {
        Fixed::ONE - angle // Q4: 0.75 to 1.0
    };
    result.0
}

/// Execute a native function call
fn execute_native_function(func_id: u8, vm: &mut VM) {
    use crate::lpscript::NativeFunction;

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
            let exp_int = exp.to_i32();
            let mut result = Fixed::ONE;
            for _ in 0..exp_int.max(0) {
                result = result * base;
            }
            vm.push(result);
        }
        id if id == NativeFunction::Abs as u8 => {
            let a = vm.pop();
            vm.push(a.abs());
        }
        id if id == NativeFunction::Floor as u8 => {
            let a = vm.pop();
            vm.push(Fixed::from_i32(a.to_i32()));
        }
        id if id == NativeFunction::Ceil as u8 => {
            let a = vm.pop();
            let frac = a.frac();
            vm.push(Fixed(if frac.0 > 0 {
                Fixed::from_i32(a.to_i32()).0 + FIXED_ONE
            } else {
                a.0
            }));
        }
        id if id == NativeFunction::Sqrt as u8 => {
            let a = vm.pop();
            // Simple integer sqrt approximation for fixed-point
            let mut result = 0i32;
            let mut bit = 1i32 << 30;
            while bit > a.0 {
                bit >>= 2;
            }
            while bit != 0 {
                if a.0 >= result + bit {
                    result = (result >> 1) + bit;
                } else {
                    result >>= 1;
                }
                bit >>= 2;
            }
            vm.push(Fixed(result << (FIXED_SHIFT / 2)));
        }
        id if id == NativeFunction::Sign as u8 => {
            let a = vm.pop();
            vm.push(Fixed(if a.0 > 0 {
                FIXED_ONE
            } else if a.0 < 0 {
                -FIXED_ONE
            } else {
                0
            }));
        }
        id if id == NativeFunction::Saturate as u8 => {
            let a = vm.pop();
            vm.push(a.max(Fixed::ZERO).min(Fixed::ONE));
        }
        id if id == NativeFunction::Step as u8 => {
            let x = vm.pop();
            let edge = vm.pop();
            vm.push(Fixed(if x.0 < edge.0 { 0 } else { FIXED_ONE }));
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
            let result = a + (b - a) * t;
            vm.push(result);
        }
        id if id == NativeFunction::Smoothstep as u8 => {
            let x = vm.pop();
            let edge1 = vm.pop();
            let edge0 = vm.pop();
            let t = Fixed(((x - edge0) / (edge1 - edge0)).0.max(0).min(FIXED_ONE));
            let t_sq = t * t;
            let result = t_sq * (Fixed::from_i32(3) - Fixed(t.0 << 1));
            vm.push(result);
        }
        id if id == NativeFunction::Less as u8 => {
            let b = vm.pop();
            let a = vm.pop();
            vm.push(Fixed(if a.0 < b.0 { FIXED_ONE } else { 0 }));
        }
        id if id == NativeFunction::Greater as u8 => {
            let b = vm.pop();
            let a = vm.pop();
            vm.push(Fixed(if a.0 > b.0 { FIXED_ONE } else { 0 }));
        }
        id if id == NativeFunction::LessEq as u8 => {
            let b = vm.pop();
            let a = vm.pop();
            vm.push(Fixed(if a.0 <= b.0 { FIXED_ONE } else { 0 }));
        }
        id if id == NativeFunction::GreaterEq as u8 => {
            let b = vm.pop();
            let a = vm.pop();
            vm.push(Fixed(if a.0 >= b.0 { FIXED_ONE } else { 0 }));
        }
        id if id == NativeFunction::Eq as u8 => {
            let b = vm.pop();
            let a = vm.pop();
            vm.push(Fixed(if a.0 == b.0 { FIXED_ONE } else { 0 }));
        }
        id if id == NativeFunction::NotEq as u8 => {
            let b = vm.pop();
            let a = vm.pop();
            vm.push(Fixed(if a.0 != b.0 { FIXED_ONE } else { 0 }));
        }
        id if id == NativeFunction::And as u8 => {
            let b = vm.pop();
            let a = vm.pop();
            vm.push(Fixed(if a.0 != 0 && b.0 != 0 { FIXED_ONE } else { 0 }));
        }
        id if id == NativeFunction::Or as u8 => {
            let b = vm.pop();
            let a = vm.pop();
            vm.push(Fixed(if a.0 != 0 || b.0 != 0 { FIXED_ONE } else { 0 }));
        }
        id if id == NativeFunction::Select as u8 => {
            let f = vm.pop();
            let t = vm.pop();
            let c = vm.pop();
            vm.push(if c.0 != 0 { t } else { f });
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
            stack: [Fixed::ZERO; 64],
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
            Fixed::ZERO
        }
    }

    #[inline(always)]
    fn peek(&self) -> Fixed {
        if self.sp > 0 {
            self.stack[self.sp - 1]
        } else {
            Fixed::ZERO
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
                self.push(Fixed(a.0 + b.0));
            }
            OpCode::Sub => {
                let b = self.pop();
                let a = self.pop();
                self.push(Fixed(a.0 - b.0));
            }
            OpCode::Mul => {
                let b = self.pop();
                let a = self.pop();
                self.push(a * b);
            }
            OpCode::Div => {
                let b = self.pop();
                let a = self.pop();
                if b.0 != 0 {
                    self.push(a / b);
                } else {
                    self.push(Fixed::ZERO);
                }
            }
            OpCode::Sin => {
                let a = self.pop();
                let sin_val = sin(a).0;
                // Map -1..1 to 0..1 for consistency with rest of system
                self.push(Fixed((sin_val + FIXED_ONE) >> 1));
            }
            OpCode::Cos => {
                let a = self.pop();
                let cos_val = cos(a).0;
                // Map -1..1 to 0..1 for consistency with rest of system
                self.push(Fixed((cos_val + FIXED_ONE) >> 1));
            }
            OpCode::Frac => {
                let a = self.pop();
                // Get fractional part: keep only fractional bits
                self.push(a.frac());
            }
            OpCode::Perlin3(octaves) => {
                let z = self.pop();
                let y = self.pop();
                let x = self.pop();
                self.push(perlin3(x, y, z, *octaves));
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
                            self.x / Fixed(((self.width - 1) as i32) << FIXED_SHIFT)
                        } else {
                            Fixed::ZERO
                        }
                    }
                    LoadSource::YNorm => {
                        // Normalize to 0..1 range
                        if self.height > 0 {
                            self.y / Fixed(((self.height - 1) as i32) << FIXED_SHIFT)
                        } else {
                            Fixed::ZERO
                        }
                    }
                    LoadSource::Time => self.time,
                    LoadSource::TimeNorm => {
                        // Wrap time to 0..1 range
                        Fixed((self.time.0 as i64).rem_euclid(FIXED_ONE as i64) as i32)
                    }
                    LoadSource::CenterDist => {
                        // Distance from center (0 at center, 1 at farthest corner)
                        let center_x = Fixed::from_i32(self.width as i32 / 2).0;
                        let center_y = Fixed::from_i32(self.height as i32 / 2).0;
                        let dx = self.x.0 - center_x;
                        let dy = self.y.0 - center_y;

                        // Use Manhattan distance normalized by half-diagonal
                        let manhattan =
                            (if dx < 0 { -dx } else { dx }) + (if dy < 0 { -dy } else { dy });
                        let max_manhattan = center_x + center_y;
                        if max_manhattan == 0 {
                            Fixed::ZERO
                        } else {
                            Fixed(
                                ((manhattan as i64 * FIXED_ONE as i64) / max_manhattan as i64)
                                    as i32,
                            )
                        }
                    }
                    LoadSource::CenterAngle => {
                        // Angle from center (0-1 for 0-2π, 0 = east/right)
                        let center_x = Fixed::from_i32(self.width as i32 / 2).0;
                        let center_y = Fixed::from_i32(self.height as i32 / 2).0;
                        let dx = self.x.0 - center_x;
                        let dy = self.y.0 - center_y;

                        // atan2(dy, dx) normalized to 0..1
                        if dx == 0 && dy == 0 {
                            Fixed::ZERO // Center has no angle
                        } else {
                            // Approximate atan2 using octants
                            let abs_dx = if dx < 0 { -dx } else { dx };
                            let abs_dy = if dy < 0 { -dy } else { dy };

                            let angle = if abs_dx > abs_dy {
                                // Closer to horizontal
                                let ratio = ((abs_dy as i64) << FIXED_SHIFT) / (abs_dx as i64);
                                (ratio as i32) >> 3 // Scale to ~0..0.125
                            } else if abs_dy > 0 {
                                // Closer to vertical
                                let ratio = ((abs_dx as i64) << FIXED_SHIFT) / (abs_dy as i64);
                                (FIXED_ONE >> 2) - ((ratio as i32) >> 3) // 0.25 - scaled ratio
                            } else {
                                0
                            };

                            // Adjust based on quadrant
                            Fixed(if dx >= 0 && dy >= 0 {
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
                            })
                        }
                    }
                };
                self.push(value);
            }
            OpCode::LoadInput => {
                let x_int = self.x.to_i32() as usize;
                let y_int = self.y.to_i32() as usize;
                let idx = y_int * self.width + x_int;
                if idx < self.input.len() {
                    self.push(self.input[idx]);
                } else {
                    self.push(Fixed::ZERO);
                }
            }
            OpCode::LoadInputAt => {
                let y = self.pop();
                let x = self.pop();
                let x_int = x.to_i32() as usize;
                let y_int = y.to_i32() as usize;
                if x_int < self.width && y_int < self.height {
                    let idx = y_int * self.width + x_int;
                    if idx < self.input.len() {
                        self.push(self.input[idx]);
                    } else {
                        self.push(Fixed::ZERO);
                    }
                } else {
                    self.push(Fixed::ZERO);
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
                if a.0 == b.0 {
                    self.pc = ((self.pc as i32 + offset) - 1) as usize;
                    return None;
                }
            }
            OpCode::JumpNe(offset) => {
                let b = self.pop();
                let a = self.pop();
                if a.0 != b.0 {
                    self.pc = ((self.pc as i32 + offset) - 1) as usize;
                    return None;
                }
            }
            OpCode::JumpLt(offset) => {
                let b = self.pop();
                let a = self.pop();
                if a.0 < b.0 {
                    self.pc = ((self.pc as i32 + offset) - 1) as usize;
                    return None;
                }
            }
            OpCode::JumpLte(offset) => {
                let b = self.pop();
                let a = self.pop();
                if a.0 <= b.0 {
                    self.pc = ((self.pc as i32 + offset) - 1) as usize;
                    return None;
                }
            }
            OpCode::JumpGt(offset) => {
                let b = self.pop();
                let a = self.pop();
                if a.0 > b.0 {
                    self.pc = ((self.pc as i32 + offset) - 1) as usize;
                    return None;
                }
            }
            OpCode::JumpGte(offset) => {
                let b = self.pop();
                let a = self.pop();
                if a.0 >= b.0 {
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
            // Add 0.5 to center pixels (x + 0.5, y + 0.5)
            let x_fixed = Fixed(((x as i32) << FIXED_SHIFT) + (Fixed::HALF.0));
            let y_fixed = Fixed(((y as i32) << FIXED_SHIFT) + (Fixed::HALF.0));

            let mut vm = VM::new(input, width, height, time, x_fixed, y_fixed);
            let result = vm.run(program);

            let idx = y * width + x;
            if idx < output.len() {
                output[idx] = result;
            }
        }
    }
}
