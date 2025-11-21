pub mod compiler;
pub mod shared;
/// Shared sine lookup table to avoid duplication
mod sin_table;
pub mod vm;

extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

// Re-exports
pub use compiler::codegen::NativeFunction;
pub use compiler::error::CompileError;
pub use compiler::optimize::OptimizeOptions;
use compiler::{codegen, lexer, optimize, parser, typechecker};
// Re-export dec32 from lp-math crate
pub use lp_math::dec32;
pub use shared::{Span, Type};
pub use vm::lps_vm::LpsVm;
pub use vm::vm_limits::VmLimits;
pub use vm::{
    execute_program_lps, execute_program_lps_vec3, LocalStack, LocalVarDef, LpsOpCode, LpsProgram,
    LpsVmError, ParamDef, RuntimeErrorWithContext,
};

/// Parse an expression string and generate a compiled LPS program
///
/// Returns Result with comprehensive compile errors.
///
/// # Example
/// ```
/// use lp_gfx::lp_script::compile_expr;
/// let program = compile_expr("cos(perlin3(vec3(uv * 0.3, time), 3))").unwrap();
/// ```
pub fn compile_expr(input: &str) -> Result<LpsProgram, CompileError> {
    compile_expr_with_options(input, &OptimizeOptions::default())
}

/// Compile an expression with custom optimization options
///
/// # Example
/// ```
/// use lp_gfx::lp_script::{compile_expr_with_options, OptimizeOptions};
/// let program = compile_expr_with_options("2.0 + 3.0", &OptimizeOptions::all()).unwrap();
/// ```
pub fn compile_expr_with_options(
    input: &str,
    options: &OptimizeOptions,
) -> Result<LpsProgram, CompileError> {
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
    let opcodes = codegen::CodeGenerator::generate(&expr)?;
    let optimized_opcodes = optimize::optimize_opcodes(opcodes, options);

    // Create main function with the expression's actual return type
    let main_function =
        vm::FunctionDef::new("main".into(), expr_type).with_opcodes(optimized_opcodes);

    Ok(LpsProgram::new("expr".into())
        .with_functions(vec![main_function])
        .with_source(input.into()))
}

/// Compile a full script (with statements, variables, control flow)
///
/// Returns Result with comprehensive compile errors.
///
/// # Example
/// ```
/// use lp_gfx::lp_script::compile_script;
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
/// use lp_gfx::lp_script::{compile_script_with_options, OptimizeOptions};
/// let script = "float x = 2.0 + 3.0; return x;";
/// let program = compile_script_with_options(script, &OptimizeOptions::all()).unwrap();
/// ```
pub fn compile_script_with_options(
    input: &str,
    options: &OptimizeOptions,
) -> Result<LpsProgram, CompileError> {
    let mut lexer = lexer::Lexer::new(input);
    let tokens = lexer.tokenize();

    let parser = parser::Parser::new(tokens);
    let mut program = parser.parse_program()?;

    // Analyze program to build function types table
    let func_table = compiler::analyzer::FunctionAnalyzer::analyze_program(&program)?;

    // Type check the program with the analyzed function table
    typechecker::TypeChecker::check_program(&mut program, &func_table)?;

    // Optimize program AST in place
    optimize::optimize_ast_program(&mut program, options);

    // Generate functions using new API with function table
    let functions = codegen::CodeGenerator::generate_program_with_functions(&program, &func_table)?;

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
}

/// Parse an expression string and generate a compiled LPS program
///
/// Panics on compile errors. Use `compile_expr()` for error handling.
///
/// # Example
/// ```
/// use lp_gfx::lp_script::parse_expr;
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
/// use lp_gfx::lp_script::parse_script;
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
    use alloc::vec::Vec;

    #[test]
    fn auto_pool_supports_lp_vec_allocations() {
        let vec = Vec::from([42]);
        assert_eq!(vec.len(), 1);
    }
}
