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
/// use engine_core::lpscript::parse_expr;
///
/// // Simple math (constant expressions are folded at compile time)
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
/// use engine_core::lpscript::{compile_expr_with_options, OptimizeOptions};
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
extern crate alloc;
use alloc::vec::Vec;

pub mod shared;
pub mod vm;

mod compiler;

pub use compiler::codegen::NativeFunction;
pub use compiler::error::CompileError;
pub use compiler::optimize::OptimizeOptions;
use compiler::{codegen, lexer, optimize, parser, typechecker};
pub use shared::{Span, Type};
pub use vm::{
    execute_program_lps, LocalAccess, LocalDef, LocalType, LpsOpCode, LpsProgram, LpsVm,
    RuntimeError, RuntimeErrorWithContext, VmLimits,
};

/// Parse an expression string and generate a compiled LPS program
///
/// Returns Result with comprehensive compile errors.
///
/// # Example
/// ```
/// let program = compile_expr("cos(perlin3(vec3(uv * 0.3, time), 3))").unwrap();
/// ```
pub fn compile_expr(input: &str) -> Result<LpsProgram, CompileError> {
    compile_expr_with_options(input, &OptimizeOptions::default())
}

/// Compile an expression with custom optimization options
///
/// # Example
/// ```
/// use engine_core::lpscript::OptimizeOptions;
/// let program = compile_expr_with_options("2.0 + 3.0", &OptimizeOptions::all()).unwrap();
/// ```
pub fn compile_expr_with_options(
    input: &str,
    options: &OptimizeOptions,
) -> Result<LpsProgram, CompileError> {
    let mut lexer = lexer::Lexer::new(input);
    let tokens = lexer.tokenize();

    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse();

    // Type check the AST
    let typed_ast = typechecker::TypeChecker::check(ast)?;

    // Optimize AST
    let optimized_ast = optimize::optimize_ast_expr(typed_ast, options);

    // Generate opcodes
    let opcodes = codegen::CodeGenerator::generate(&optimized_ast);

    // Optimize opcodes
    let optimized_opcodes = optimize::optimize_opcodes(opcodes, options);

    Ok(LpsProgram::new("expr".into())
        .with_opcodes(optimized_opcodes)
        .with_source(input.into()))
}

/// Compile a full script (with statements, variables, control flow)
///
/// Returns Result with comprehensive compile errors.
///
/// # Example
/// ```
/// let script = "
///     float radius = length(uv - vec2(0.5));
///     if (radius < 0.3) {
///         return sin(time * Fixed::TAU);
///     } else {
///         return 0.0;
///     }
/// ";
/// let program = compile_script(script).unwrap();
/// ```
pub fn compile_script(input: &str) -> Result<LpsProgram, CompileError> {
    compile_script_with_options(input, &OptimizeOptions::default())
}

/// Compile a script with custom optimization options
///
/// # Example
/// ```
/// use engine_core::lpscript::OptimizeOptions;
/// let script = "float x = 2.0 + 3.0; return x;";
/// let program = compile_script_with_options(script, &OptimizeOptions::all()).unwrap();
/// ```
pub fn compile_script_with_options(
    input: &str,
    options: &OptimizeOptions,
) -> Result<LpsProgram, CompileError> {
    let mut lexer = lexer::Lexer::new(input);
    let tokens = lexer.tokenize();

    let mut parser = parser::Parser::new(tokens);
    let program = parser.parse_program();

    // Type check the program
    let typed_program = typechecker::TypeChecker::check_program(program)?;

    // Optimize program AST
    let optimized_program = optimize::optimize_ast_program(typed_program, options);

    // Generate opcodes
    let (opcodes, local_count, local_types) =
        codegen::CodeGenerator::generate_program(&optimized_program);

    // Optimize opcodes
    let optimized_opcodes = optimize::optimize_opcodes(opcodes, options);

    // Create LocalDef entries for all scratch locals with correct types
    let locals: Vec<LocalDef> = (0..local_count)
        .map(|i| {
            let ty_enum = match local_types.get(&i) {
                Some(shared::Type::Int32) => LocalType::Int32(0),
                Some(shared::Type::Vec2) => {
                    LocalType::Vec2(crate::math::Fixed::ZERO, crate::math::Fixed::ZERO)
                }
                Some(shared::Type::Vec3) => LocalType::Vec3(
                    crate::math::Fixed::ZERO,
                    crate::math::Fixed::ZERO,
                    crate::math::Fixed::ZERO,
                ),
                Some(shared::Type::Vec4) => LocalType::Vec4(
                    crate::math::Fixed::ZERO,
                    crate::math::Fixed::ZERO,
                    crate::math::Fixed::ZERO,
                    crate::math::Fixed::ZERO,
                ),
                _ => LocalType::Fixed(crate::math::Fixed::ZERO), // Default to Fixed for Bool and Fixed
            };
            LocalDef::new(alloc::format!("local_{}", i), ty_enum, LocalAccess::Scratch)
        })
        .collect();

    Ok(LpsProgram::new("script".into())
        .with_opcodes(optimized_opcodes)
        .with_locals(locals)
        .with_source(input.into()))
}

/// Parse an expression string and generate a compiled LPS program
///
/// Panics on compile errors. Use `compile_expr()` for error handling.
///
/// # Example
/// ```
/// let program = parse_expr("cos(perlin3(vec3(uv * 0.3, time), 3))");
/// ```
pub fn parse_expr(input: &str) -> LpsProgram {
    compile_expr(input).unwrap_or_else(|e| {
        panic!("Failed to compile LPS expression: {}", e);
    })
}

/// Parse a full script and generate a compiled LPS program
///
/// Panics on compile errors. Use `compile_script()` for error handling.
///
/// # Example
/// ```
/// let script = "
///     float x = uv.x;
///     if (x > 0.5) {
///         return 1.0;
///     } else {
///         return 0.0;
///     }
/// ";
/// let program = parse_script(script);
/// ```
pub fn parse_script(input: &str) -> LpsProgram {
    compile_script(input).unwrap_or_else(|e| {
        panic!("Failed to compile LPS script: {}", e);
    })
}

#[cfg(test)]
mod tests {
    mod control_flow;
    mod functions;
    mod operators;
    mod variables;
}
