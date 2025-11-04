use crate::math::noise::perlin3;
use crate::math::trig::{cos, sin};
/// Stack-based VM for pixel operations using fixed-point arithmetic
use crate::math::{Fixed, FIXED_ONE, FIXED_SHIFT};

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
        id if id == NativeFunction::Tan as u8 => {
            use crate::math::tan;
            let x = vm.pop();
            vm.push(tan(x));
        }
        id if id == NativeFunction::Atan as u8 => {
            use crate::math::{atan, atan2};
            // Check stack size to determine if it's atan(y) or atan(y, x)
            // Actually, we need to handle this based on args count from codegen
            // For simplicity, check if there are 2 args (this is a hack, will fix properly later)
            // For now, just implement single-arg atan
            let y = vm.pop();
            vm.push(atan(y));
        }
        id if id == NativeFunction::Mod as u8 => {
            use crate::math::modulo;
            let y = vm.pop();
            let x = vm.pop();
            vm.push(modulo(x, y));
        }
        id if id == NativeFunction::Length as u8 => {
            // Pop vector components and calculate length
            // Type checker ensures we know what type this is
            // For now, we need a way to know the vector size
            // This is a design issue - we need type info in opcodes for polymorphic functions
            // TODO: Add typed Length opcodes (Length2, Length3, Length4)
            // For now, stub it out
            let a = vm.pop();
            vm.push(a); // Placeholder
        }
        id if id == NativeFunction::Normalize as u8 => {
            // Same issue as length - need typed opcodes
            // TODO: Add Normalize2, Normalize3, Normalize4
        }
        id if id == NativeFunction::Dot as u8 => {
            // Same issue - need typed opcodes
            // TODO: Add Dot2, Dot3, Dot4
            let b = vm.pop();
            let a = vm.pop();
            vm.push(a * b); // Placeholder
        }
        id if id == NativeFunction::Distance as u8 => {
            // Same issue - need typed opcodes
            // TODO: Add Distance2, Distance3, Distance4
            let b = vm.pop();
            let a = vm.pop();
            vm.push((a - b).abs()); // Placeholder
        }
        id if id == NativeFunction::Cross as u8 => {
            // vec3 cross product
            // Pop 6 values (2 vec3s), push 3 values (result vec3)
            // TODO: Implement properly
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

    // Vec2 arithmetic
    AddVec2,        // pop 4, push 2
    SubVec2,        // pop 4, push 2
    MulVec2,        // pop 4, push 2 (component-wise)
    DivVec2,        // pop 4, push 2 (component-wise)
    MulVec2Scalar,  // pop 3 (vec2 + scalar), push 2
    DivVec2Scalar,  // pop 3 (vec2 + scalar), push 2
    
    // Vec3 arithmetic
    AddVec3,        // pop 6, push 3
    SubVec3,        // pop 6, push 3
    MulVec3,        // pop 6, push 3 (component-wise)
    DivVec3,        // pop 6, push 3 (component-wise)
    MulVec3Scalar,  // pop 4 (vec3 + scalar), push 3
    DivVec3Scalar,  // pop 4 (vec3 + scalar), push 3
    
    // Vec4 arithmetic
    AddVec4,        // pop 8, push 4
    SubVec4,        // pop 8, push 4
    MulVec4,        // pop 8, push 4 (component-wise)
    DivVec4,        // pop 8, push 4 (component-wise)
    MulVec4Scalar,  // pop 5 (vec4 + scalar), push 4
    DivVec4Scalar,  // pop 5 (vec4 + scalar), push 4

    // Vector functions (typed)
    Dot2,       // pop 4 (two vec2s), push 1 (dot product)
    Dot3,       // pop 6 (two vec3s), push 1
    Dot4,       // pop 8 (two vec4s), push 1
    Length2,    // pop 2 (vec2), push 1 (length)
    Length3,    // pop 3 (vec3), push 1
    Length4,    // pop 4 (vec4), push 1
    Normalize2, // pop 2 (vec2), push 2 (normalized vec2)
    Normalize3, // pop 3 (vec3), push 3
    Normalize4, // pop 4 (vec4), push 4
    Distance2,  // pop 4 (two vec2s), push 1 (distance)
    Distance3,  // pop 6 (two vec3s), push 1
    Distance4,  // pop 8 (two vec4s), push 1
    Cross3,     // pop 6 (two vec3s), push 3 (cross product vec3)

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
    pub const fn load_x() -> Self {
        OpCode::Load(LoadSource::XNorm)
    }
    pub const fn load_y() -> Self {
        OpCode::Load(LoadSource::YNorm)
    }
    pub const fn load_time() -> Self {
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
            
            // Vec2 arithmetic
            OpCode::AddVec2 => {
                use crate::math::Vec2;
                let y2 = self.pop();
                let x2 = self.pop();
                let y1 = self.pop();
                let x1 = self.pop();
                let v1 = Vec2::new(x1, y1);
                let v2 = Vec2::new(x2, y2);
                let result = v1 + v2;
                self.push(result.x);
                self.push(result.y);
            }
            OpCode::SubVec2 => {
                use crate::math::Vec2;
                let y2 = self.pop();
                let x2 = self.pop();
                let y1 = self.pop();
                let x1 = self.pop();
                let v1 = Vec2::new(x1, y1);
                let v2 = Vec2::new(x2, y2);
                let result = v1 - v2;
                self.push(result.x);
                self.push(result.y);
            }
            OpCode::MulVec2 => {
                use crate::math::Vec2;
                let y2 = self.pop();
                let x2 = self.pop();
                let y1 = self.pop();
                let x1 = self.pop();
                let v1 = Vec2::new(x1, y1);
                let v2 = Vec2::new(x2, y2);
                let result = v1.mul_comp(v2);
                self.push(result.x);
                self.push(result.y);
            }
            OpCode::DivVec2 => {
                use crate::math::Vec2;
                let y2 = self.pop();
                let x2 = self.pop();
                let y1 = self.pop();
                let x1 = self.pop();
                let v1 = Vec2::new(x1, y1);
                let v2 = Vec2::new(x2, y2);
                let result = v1.div_comp(v2);
                self.push(result.x);
                self.push(result.y);
            }
            OpCode::MulVec2Scalar => {
                use crate::math::Vec2;
                let scalar = self.pop();
                let y = self.pop();
                let x = self.pop();
                let v = Vec2::new(x, y);
                let result = v * scalar;
                self.push(result.x);
                self.push(result.y);
            }
            OpCode::DivVec2Scalar => {
                use crate::math::Vec2;
                let scalar = self.pop();
                let y = self.pop();
                let x = self.pop();
                let v = Vec2::new(x, y);
                let result = v / scalar;
                self.push(result.x);
                self.push(result.y);
            }
            
            // Vec3 arithmetic (similar to Vec2)
            OpCode::AddVec3 => {
                use crate::math::Vec3;
                let z2 = self.pop();
                let y2 = self.pop();
                let x2 = self.pop();
                let z1 = self.pop();
                let y1 = self.pop();
                let x1 = self.pop();
                let v1 = Vec3::new(x1, y1, z1);
                let v2 = Vec3::new(x2, y2, z2);
                let result = v1 + v2;
                self.push(result.x);
                self.push(result.y);
                self.push(result.z);
            }
            OpCode::SubVec3 => {
                use crate::math::Vec3;
                let z2 = self.pop();
                let y2 = self.pop();
                let x2 = self.pop();
                let z1 = self.pop();
                let y1 = self.pop();
                let x1 = self.pop();
                let v1 = Vec3::new(x1, y1, z1);
                let v2 = Vec3::new(x2, y2, z2);
                let result = v1 - v2;
                self.push(result.x);
                self.push(result.y);
                self.push(result.z);
            }
            OpCode::MulVec3 => {
                use crate::math::Vec3;
                let z2 = self.pop();
                let y2 = self.pop();
                let x2 = self.pop();
                let z1 = self.pop();
                let y1 = self.pop();
                let x1 = self.pop();
                let v1 = Vec3::new(x1, y1, z1);
                let v2 = Vec3::new(x2, y2, z2);
                let result = v1.mul_comp(v2);
                self.push(result.x);
                self.push(result.y);
                self.push(result.z);
            }
            OpCode::DivVec3 => {
                use crate::math::Vec3;
                let z2 = self.pop();
                let y2 = self.pop();
                let x2 = self.pop();
                let z1 = self.pop();
                let y1 = self.pop();
                let x1 = self.pop();
                let v1 = Vec3::new(x1, y1, z1);
                let v2 = Vec3::new(x2, y2, z2);
                let result = v1.div_comp(v2);
                self.push(result.x);
                self.push(result.y);
                self.push(result.z);
            }
            OpCode::MulVec3Scalar => {
                use crate::math::Vec3;
                let scalar = self.pop();
                let z = self.pop();
                let y = self.pop();
                let x = self.pop();
                let v = Vec3::new(x, y, z);
                let result = v * scalar;
                self.push(result.x);
                self.push(result.y);
                self.push(result.z);
            }
            OpCode::DivVec3Scalar => {
                use crate::math::Vec3;
                let scalar = self.pop();
                let z = self.pop();
                let y = self.pop();
                let x = self.pop();
                let v = Vec3::new(x, y, z);
                let result = v / scalar;
                self.push(result.x);
                self.push(result.y);
                self.push(result.z);
            }
            
            // Vec4 arithmetic (similar to Vec2/Vec3)
            OpCode::AddVec4 => {
                use crate::math::Vec4;
                let w2 = self.pop();
                let z2 = self.pop();
                let y2 = self.pop();
                let x2 = self.pop();
                let w1 = self.pop();
                let z1 = self.pop();
                let y1 = self.pop();
                let x1 = self.pop();
                let v1 = Vec4::new(x1, y1, z1, w1);
                let v2 = Vec4::new(x2, y2, z2, w2);
                let result = v1 + v2;
                self.push(result.x);
                self.push(result.y);
                self.push(result.z);
                self.push(result.w);
            }
            OpCode::SubVec4 => {
                use crate::math::Vec4;
                let w2 = self.pop();
                let z2 = self.pop();
                let y2 = self.pop();
                let x2 = self.pop();
                let w1 = self.pop();
                let z1 = self.pop();
                let y1 = self.pop();
                let x1 = self.pop();
                let v1 = Vec4::new(x1, y1, z1, w1);
                let v2 = Vec4::new(x2, y2, z2, w2);
                let result = v1 - v2;
                self.push(result.x);
                self.push(result.y);
                self.push(result.z);
                self.push(result.w);
            }
            OpCode::MulVec4 => {
                use crate::math::Vec4;
                let w2 = self.pop();
                let z2 = self.pop();
                let y2 = self.pop();
                let x2 = self.pop();
                let w1 = self.pop();
                let z1 = self.pop();
                let y1 = self.pop();
                let x1 = self.pop();
                let v1 = Vec4::new(x1, y1, z1, w1);
                let v2 = Vec4::new(x2, y2, z2, w2);
                let result = v1.mul_comp(v2);
                self.push(result.x);
                self.push(result.y);
                self.push(result.z);
                self.push(result.w);
            }
            OpCode::DivVec4 => {
                use crate::math::Vec4;
                let w2 = self.pop();
                let z2 = self.pop();
                let y2 = self.pop();
                let x2 = self.pop();
                let w1 = self.pop();
                let z1 = self.pop();
                let y1 = self.pop();
                let x1 = self.pop();
                let v1 = Vec4::new(x1, y1, z1, w1);
                let v2 = Vec4::new(x2, y2, z2, w2);
                let result = v1.div_comp(v2);
                self.push(result.x);
                self.push(result.y);
                self.push(result.z);
                self.push(result.w);
            }
            OpCode::MulVec4Scalar => {
                use crate::math::Vec4;
                let scalar = self.pop();
                let w = self.pop();
                let z = self.pop();
                let y = self.pop();
                let x = self.pop();
                let v = Vec4::new(x, y, z, w);
                let result = v * scalar;
                self.push(result.x);
                self.push(result.y);
                self.push(result.z);
                self.push(result.w);
            }
            OpCode::DivVec4Scalar => {
                use crate::math::Vec4;
                let scalar = self.pop();
                let w = self.pop();
                let z = self.pop();
                let y = self.pop();
                let x = self.pop();
                let v = Vec4::new(x, y, z, w);
                let result = v / scalar;
                self.push(result.x);
                self.push(result.y);
                self.push(result.z);
                self.push(result.w);
            }
            
            // Vector functions (typed)
            OpCode::Dot2 => {
                use crate::math::Vec2;
                let y2 = self.pop();
                let x2 = self.pop();
                let y1 = self.pop();
                let x1 = self.pop();
                let v1 = Vec2::new(x1, y1);
                let v2 = Vec2::new(x2, y2);
                self.push(v1.dot(v2));
            }
            OpCode::Dot3 => {
                use crate::math::Vec3;
                let z2 = self.pop();
                let y2 = self.pop();
                let x2 = self.pop();
                let z1 = self.pop();
                let y1 = self.pop();
                let x1 = self.pop();
                let v1 = Vec3::new(x1, y1, z1);
                let v2 = Vec3::new(x2, y2, z2);
                self.push(v1.dot(v2));
            }
            OpCode::Dot4 => {
                use crate::math::Vec4;
                let w2 = self.pop();
                let z2 = self.pop();
                let y2 = self.pop();
                let x2 = self.pop();
                let w1 = self.pop();
                let z1 = self.pop();
                let y1 = self.pop();
                let x1 = self.pop();
                let v1 = Vec4::new(x1, y1, z1, w1);
                let v2 = Vec4::new(x2, y2, z2, w2);
                self.push(v1.dot(v2));
            }
            OpCode::Length2 => {
                use crate::math::Vec2;
                let y = self.pop();
                let x = self.pop();
                let v = Vec2::new(x, y);
                self.push(v.length());
            }
            OpCode::Length3 => {
                use crate::math::Vec3;
                let z = self.pop();
                let y = self.pop();
                let x = self.pop();
                let v = Vec3::new(x, y, z);
                self.push(v.length());
            }
            OpCode::Length4 => {
                use crate::math::Vec4;
                let w = self.pop();
                let z = self.pop();
                let y = self.pop();
                let x = self.pop();
                let v = Vec4::new(x, y, z, w);
                self.push(v.length());
            }
            OpCode::Normalize2 => {
                use crate::math::Vec2;
                let y = self.pop();
                let x = self.pop();
                let v = Vec2::new(x, y).normalize();
                self.push(v.x);
                self.push(v.y);
            }
            OpCode::Normalize3 => {
                use crate::math::Vec3;
                let z = self.pop();
                let y = self.pop();
                let x = self.pop();
                let v = Vec3::new(x, y, z).normalize();
                self.push(v.x);
                self.push(v.y);
                self.push(v.z);
            }
            OpCode::Normalize4 => {
                use crate::math::Vec4;
                let w = self.pop();
                let z = self.pop();
                let y = self.pop();
                let x = self.pop();
                let v = Vec4::new(x, y, z, w).normalize();
                self.push(v.x);
                self.push(v.y);
                self.push(v.z);
                self.push(v.w);
            }
            OpCode::Distance2 => {
                use crate::math::Vec2;
                let y2 = self.pop();
                let x2 = self.pop();
                let y1 = self.pop();
                let x1 = self.pop();
                let v1 = Vec2::new(x1, y1);
                let v2 = Vec2::new(x2, y2);
                self.push(v1.distance(v2));
            }
            OpCode::Distance3 => {
                use crate::math::Vec3;
                let z2 = self.pop();
                let y2 = self.pop();
                let x2 = self.pop();
                let z1 = self.pop();
                let y1 = self.pop();
                let x1 = self.pop();
                let v1 = Vec3::new(x1, y1, z1);
                let v2 = Vec3::new(x2, y2, z2);
                self.push(v1.distance(v2));
            }
            OpCode::Distance4 => {
                use crate::math::Vec4;
                let w2 = self.pop();
                let z2 = self.pop();
                let y2 = self.pop();
                let x2 = self.pop();
                let w1 = self.pop();
                let z1 = self.pop();
                let y1 = self.pop();
                let x1 = self.pop();
                let v1 = Vec4::new(x1, y1, z1, w1);
                let v2 = Vec4::new(x2, y2, z2, w2);
                self.push(v1.distance(v2));
            }
            OpCode::Cross3 => {
                use crate::math::Vec3;
                let z2 = self.pop();
                let y2 = self.pop();
                let x2 = self.pop();
                let z1 = self.pop();
                let y1 = self.pop();
                let x1 = self.pop();
                let v1 = Vec3::new(x1, y1, z1);
                let v2 = Vec3::new(x2, y2, z2);
                let result = v1.cross(v2);
                self.push(result.x);
                self.push(result.y);
                self.push(result.z);
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
