#![cfg_attr(not(test), no_std)]

extern crate alloc;

/// Expression language for generating VM opcodes
///
/// This module provides a simple expression language that compiles to VM opcodes.
///
/// # Features
/// - **Arithmetic**: `+`, `-`, `*`, `/`, `%`
/// - **Bitwise** (int only): `&`, `|`, `^`, `~`, `<<`, `>>`
/// - **Comparisons**: `<`, `>`, `<=`, `>=`, `==`, `!=`
/// - **Logical**: `&&`, `||`, `!`
/// - **Increment/Decrement**: `++`, `--` (prefix and postfix)
/// - **Compound Assignment**: `+=`, `-=`, `*=`, `/=`, `%=`, `&=`, `|=`, `^=`, `<<=`, `>>=`
/// - **Ternary**: `condition ? true_val : false_val`
/// - **Vector Swizzling**: `.x`, `.xy`, `.yx`, `.rgba`, `.stpq`, etc.
///
/// # Built-in Variables
/// - **`uv`**: vec2, normalized coordinates (0..1)
/// - **`coord`**: vec2, pixel coordinates
/// - **`time`**: float, time value
/// - **Legacy**: `xNorm`, `yNorm`, `centerAngle`, `centerDist`
///
/// # GLSL/HLSL Shader Functions
/// - **Math**: `sin`, `cos`, `abs`, `floor`, `ceil`, `sqrt`, `sign`, `pow`, `min`, `max`
/// - **Clamping**: `clamp(value, min, max)`, `saturate(x)` (clamp to 0..1), `step(edge, x)`
/// - **Interpolation**: `lerp(a, b, t)` or `mix(a, b, t)`, `smoothstep(edge0, edge1, x)`
/// - **Perlin noise**: `perlin3(vec3)` or `perlin3(vec3, octaves)`
///
/// # Examples
/// ```
/// use lp_gfx::lp_script::parse_expr;
///
/// // Simple dec32 (constant expressions are folded at compile time)
/// let code = parse_expr("2.0 + 3.0"); // Compiles to Push(5.0)
/// let code = parse_expr("sin(time) * 0.5 + 0.5");
///
/// // Vector swizzling
/// let code = parse_expr("uv.x * 2.0");
/// let code = parse_expr("uv.yx");
///
/// // Perlin noise with GLSL-style vec3 constructor
/// let code = parse_expr("cos(perlin3(vec3(uv * 0.3, time), 3))");
///
/// // Ternary operator
/// let code = parse_expr("centerDist < 0.5 ? 1.0 : 0.0");
///
/// // Min/max (folded if all arguments are constant)
/// let code = parse_expr("max(2.0, 3.0)"); // Compiles to Push(3.0)
/// let code = parse_expr("max(0.0, min(1.0, uv.x * 2.0))");
/// ```
///
/// # Optimization
/// The compiler includes automatic optimizations (enabled by default):
/// - **Constant folding**: `sin(0.0)` → `0.0`, `2.0 + 3.0` → `5.0`
/// - **Algebraic simplification**: `x * 1.0` → `x`, `x + 0.0` → `x`
/// - **Dead code elimination**: Remove unreachable statements
/// - **Peephole optimization**: Eliminate redundant opcode sequences
///
/// Control optimization with `OptimizeOptions`:
/// ```
/// use lp_gfx::lp_script::{compile_expr_with_options, OptimizeOptions};
///
/// // Disable all optimizations (for debugging)
/// let program = compile_expr_with_options("2.0 + 3.0", &OptimizeOptions::none()).unwrap();
///
/// // Custom optimization settings
/// let mut options = OptimizeOptions::default();
/// options.constant_folding = true;
/// options.algebraic_simplification = false;
/// let program = compile_expr_with_options("x * 1.0", &options).unwrap();
/// ```
pub mod lp_script;

// Graphics module
pub mod gfx;

// Re-exports
pub use gfx::gfx_context::GfxContext;
pub use gfx::gfx_error::GfxError;
pub use gfx::shader_ref::ShaderRef;
pub use gfx::texture_format::TextureFormat;
pub use gfx::texture_ref::TexRef;
// Re-export contexts
#[cfg(feature = "cpu")]
pub use gfx::CpuContext;
#[cfg(feature = "gpu")]
pub use gfx::GpuContext;
