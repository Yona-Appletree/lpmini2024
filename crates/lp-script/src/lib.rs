#![cfg_attr(not(test), no_std)]

/// Shared sine lookup table to avoid duplication
mod sin_table;

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
/// use lp_script::parse_expr;
///
/// // Simple fixed (constant expressions are folded at compile time)
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
/// use lp_script::{compile_expr_with_options, OptimizeOptions};
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
use alloc::vec;
use alloc::vec::Vec;
use core::ptr::NonNull;

pub mod shared;
pub mod vm;

// Re-export fixed from lp-fixed crate
pub use lp_math::fixed;

mod compiler;

pub use compiler::codegen::NativeFunction;
pub use compiler::error::CompileError;
pub use compiler::optimize::OptimizeOptions;
use compiler::{codegen, lexer, optimize, parser, typechecker};
use lp_pool::LpMemoryPool;
pub use shared::{Span, Type};
pub use vm::lps_vm::LpsVm;
pub use vm::vm_limits::VmLimits;
pub use vm::{
    execute_program_lps, LocalStack, LocalVarDef, LpsOpCode, LpsProgram, LpsVmError, ParamDef,
    RuntimeErrorWithContext,
};

/// Parse an expression string and generate a compiled LPS program
///
/// Returns Result with comprehensive compile errors.
///
/// # Example
/// ```
/// use lp_script::compile_expr;
/// let program = compile_expr("cos(perlin3(vec3(uv * 0.3, time), 3))").unwrap();
/// ```
pub fn compile_expr(input: &str) -> Result<LpsProgram, CompileError> {
    compile_expr_with_options(input, &OptimizeOptions::default())
}

/// Compile an expression with custom optimization options
///
/// # Example
/// ```
/// use lp_script::{compile_expr_with_options, OptimizeOptions};
/// let program = compile_expr_with_options("2.0 + 3.0", &OptimizeOptions::all()).unwrap();
/// ```
pub fn compile_expr_with_options(
    input: &str,
    options: &OptimizeOptions,
) -> Result<LpsProgram, CompileError> {
    const POOL_SIZE: usize = 128 * 1024;
    let mut memory = vec![0u8; POOL_SIZE];
    let memory_ptr =
        NonNull::new(memory.as_mut_ptr()).expect("Failed to get non-null pointer for pool");
    let pool = unsafe { LpMemoryPool::new(memory_ptr, POOL_SIZE).map_err(CompileError::from)? };

    pool.run(|| -> Result<LpsProgram, CompileError> {
        let mut lexer = lexer::Lexer::new(input);
        let tokens = lexer.tokenize();

        let mut parser = parser::Parser::new(tokens);
        let mut expr = parser.parse()?;

        // Type check the AST (in-place, mutating types on nodes)
        typechecker::TypeChecker::check(&mut expr)?;

        // Optimize AST (mutates in place)
        optimize::optimize_ast_expr(&mut expr, options);

        // Determine the expression's return type after type checking
        let expr_type = expr.ty.clone().ok_or_else(|| {
            CompileError::TypeCheck(compiler::error::TypeError {
                kind: compiler::error::TypeErrorKind::UndefinedVariable(
                    "expression has no inferred type".into(),
                ),
                span: expr.span,
            })
        })?;

        // Generate and optimize opcodes
        let opcodes = codegen::CodeGenerator::generate(&expr);
        let optimized_opcodes = optimize::optimize_opcodes(opcodes, options);

        // Create main function with the expression's actual return type
        let main_function =
            vm::FunctionDef::new("main".into(), expr_type).with_opcodes(optimized_opcodes);

        Ok(LpsProgram::new("expr".into())
            .with_functions(vec![main_function])
            .with_source(input.into()))
    })
}

/// Compile a full script (with statements, variables, control flow)
///
/// Returns Result with comprehensive compile errors.
///
/// # Example
/// ```
/// use lp_script::compile_script;
/// let script = "
///     float radius = length(uv - vec2(0.5));
///     if (radius < 0.3) {
///         return sin(time);
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
/// use lp_script::{compile_script_with_options, OptimizeOptions};
/// let script = "float x = 2.0 + 3.0; return x;";
/// let program = compile_script_with_options(script, &OptimizeOptions::all()).unwrap();
/// ```
pub fn compile_script_with_options(
    input: &str,
    options: &OptimizeOptions,
) -> Result<LpsProgram, CompileError> {
    const POOL_SIZE: usize = 512 * 1024;
    let mut memory = vec![0u8; POOL_SIZE];
    let memory_ptr =
        NonNull::new(memory.as_mut_ptr()).expect("Failed to get non-null pointer for pool");
    let pool = unsafe { LpMemoryPool::new(memory_ptr, POOL_SIZE).map_err(CompileError::from)? };

    pool.run(|| -> Result<LpsProgram, CompileError> {
        let mut lexer = lexer::Lexer::new(input);
        let tokens = lexer.tokenize();

        let parser = parser::Parser::new(tokens);
        let mut program = parser.parse_program()?;

        // Analyze program to build function metadata table
        let func_table = compiler::analyzer::FunctionAnalyzer::analyze_program(&program)?;

        // Type check the program with the analyzed function table
        typechecker::TypeChecker::check_program(&mut program, &func_table)?;

        // Optimize program AST in place
        optimize::optimize_ast_program(&mut program, options);

        // Generate functions using new API with function table
        let functions =
            codegen::CodeGenerator::generate_program_with_functions(&program, &func_table);

        // Optimize opcodes for each function
        let optimized_functions: Vec<vm::FunctionDef> = functions
            .into_iter()
            .map(|func| {
                let optimized_opcodes = optimize::optimize_opcodes(func.opcodes.clone(), options);
                vm::FunctionDef::new(func.name.clone(), func.return_type.clone())
                    .with_params(func.params.clone())
                    .with_locals(func.locals.clone())
                    .with_opcodes(optimized_opcodes)
            })
            .collect();

        Ok(LpsProgram::new("script".into())
            .with_functions(optimized_functions)
            .with_source(input.into()))
    })
}

/// Parse an expression string and generate a compiled LPS program
///
/// Panics on compile errors. Use `compile_expr()` for error handling.
///
/// # Example
/// ```
/// use lp_script::parse_expr;
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
/// use lp_script::parse_script;
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
