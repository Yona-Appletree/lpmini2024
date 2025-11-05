/// Native function IDs for CallNative opcode
#[repr(u8)]
pub enum NativeFunction {
    // Math basics
    Min = 0,
    Max = 1,
    Pow = 2,
    Abs = 3,
    Floor = 4,
    Ceil = 5,
    Sqrt = 6,
    Sign = 7,
    Saturate = 8,
    Step = 9,
    
    // Utility
    Clamp = 10,
    Lerp = 11,
    Smoothstep = 12,
    
    // Trig (new GLSL functions)
    Tan = 13,
    Atan = 14,
    Mod = 15,
    
    // Comparisons
    Less = 20,
    Greater = 21,
    LessEq = 22,
    GreaterEq = 23,
    Eq = 24,
    NotEq = 25,
    
    // Logical
    And = 30,
    Or = 31,
    
    // Ternary select
    Select = 40,
    
    // Vector functions (polymorphic - work on vec2/vec3/vec4)
    Length = 50,
    Normalize = 51,
    Dot = 52,
    Distance = 53,
    Cross = 54,  // vec3 only
}

