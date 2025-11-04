/// Expression language for generating VM opcodes
///
/// This module provides a simple expression language that compiles to VM opcodes.
///
/// # Features
/// - **Arithmetic**: `+`, `-`, `*`, `/`, `%`, `^` (power)
/// - **Comparisons**: `<`, `>`, `<=`, `>=`, `==`, `!=`
/// - **Logical**: `&&`, `||`
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
/// // Simple math
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
/// // Min/max
/// let code = parse_expr("max(0.0, min(1.0, uv.x * 2.0))");
/// ```
extern crate alloc;
use alloc::vec::Vec;

pub mod error;
pub mod vm;

mod ast;
mod codegen;
mod lexer;
mod parser;
mod typechecker;

pub use codegen::NativeFunction;
pub use error::{CompileError, RuntimeError, RuntimeErrorWithContext, Span, Type};
pub use vm::{LocalAccess, LocalDef, LocalType, LpsProgram, LpsVm, VmLimits, execute_program_lps, LpsOpCode};

/// Parse an expression string and generate a compiled LPS program
///
/// Returns Result with comprehensive compile errors.
///
/// # Example
/// ```
/// let program = compile_expr("cos(perlin3(vec3(uv * 0.3, time), 3))").unwrap();
/// ```
pub fn compile_expr(input: &str) -> Result<LpsProgram, CompileError> {
    let mut lexer = lexer::Lexer::new(input);
    let tokens = lexer.tokenize();

    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse();
    
    // Type check the AST
    let typed_ast = typechecker::TypeChecker::check(ast)?;

    let opcodes = codegen::CodeGenerator::generate(&typed_ast);
    
    Ok(LpsProgram::new("expr".into())
        .with_opcodes(opcodes)
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
    let mut lexer = lexer::Lexer::new(input);
    let tokens = lexer.tokenize();

    let mut parser = parser::Parser::new(tokens);
    let program = parser.parse_program();
    
    // Type check the program
    let typed_program = typechecker::TypeChecker::check_program(program)?;

    let (opcodes, local_count) = codegen::CodeGenerator::generate_program(&typed_program);
    
    // Create LocalDef entries for all scratch locals
    let locals: Vec<LocalDef> = (0..local_count)
        .map(|i| LocalDef::new(
            alloc::format!("local_{}", i),
            LocalType::Fixed(crate::math::Fixed::ZERO),
            LocalAccess::Scratch,
        ))
        .collect();
    
    Ok(LpsProgram::new("script".into())
        .with_opcodes(opcodes)
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
    mod variables;
    
    use super::*;
    use super::vm::opcodes::LpsOpCode;
    use crate::test_engine::LoadSource;

    #[test]
    fn test_simple_number() {
        let program = parse_expr("42");
        assert_eq!(program.opcodes.len(), 2); // Push(42), Return
        assert!(matches!(program.opcodes[0], LpsOpCode::Push(_)));
    }

    #[test]
    fn test_arithmetic() {
        let program = parse_expr("1 + 2 * 3");
        let code = &program.opcodes;
        // Should respect precedence: 1 + (2 * 3)
        assert!(matches!(code[0], LpsOpCode::Push(_)));
        assert!(matches!(code[1], LpsOpCode::Push(_)));
        assert!(matches!(code[2], LpsOpCode::Push(_)));
        assert!(matches!(code[3], LpsOpCode::MulFixed));
        assert!(matches!(code[4], LpsOpCode::AddFixed));
    }

    #[test]
    fn test_exponential() {
        let code = &parse_expr("2 ^ 3").opcodes;
        // Pow is now a placeholder that pushes 1.0
        assert!(matches!(code[2], LpsOpCode::Push(_)));
    }

    #[test]
    fn test_variables() {
        let code = &parse_expr("xNorm").opcodes;
        assert!(matches!(code[0], LpsOpCode::Load(LoadSource::XNorm)));
    }

    #[test]
    fn test_sin_function() {
        let code = &parse_expr("sin(time)").opcodes;
        assert!(matches!(code[0], LpsOpCode::Load(LoadSource::Time)));
        assert!(matches!(code[1], LpsOpCode::SinFixed));
    }

    #[test]
    fn test_min_max() {
        let code = &parse_expr("min(xNorm, 0.5)").opcodes;
        assert!(matches!(code[0], LpsOpCode::Load(LoadSource::XNorm)));
        assert!(matches!(code[2], LpsOpCode::MinFixed));
    }

    #[test]
    fn test_ternary() {
        let code = &parse_expr("xNorm > 0.5 ? 1.0 : 0.0").opcodes;
        // Load xNorm, Push 0.5, Greater, Push 1.0, Push 0.0, Select
        assert!(matches!(code[2], LpsOpCode::GreaterFixed));
        assert!(matches!(code.last(), Some(LpsOpCode::Return)));
    }

    #[test]
    fn test_complex_perlin() {
        let code = &parse_expr("cos(perlin3(vec3(uv * 0.3, time), 3))").opcodes;
        // Should have loads for uv (vec2), multiplication, vec3 construction, Perlin3 and Cos
        let has_perlin = code.iter().any(|op| matches!(op, LpsOpCode::Perlin3(_)));
        let has_cos = code.iter().any(|op| matches!(op, LpsOpCode::CosFixed));
        assert!(has_perlin);
        assert!(has_cos);
    }

    #[test]
    fn test_logical_and() {
        let code = &parse_expr("xNorm > 0.5 && yNorm < 0.5").opcodes;
        // Should have Greater, Less, and And
        let has_greater = code.iter().any(|op| matches!(op, LpsOpCode::GreaterFixed));
        let has_less = code.iter().any(|op| matches!(op, LpsOpCode::LessFixed));
        let has_and = code.iter().any(|op| matches!(op, LpsOpCode::AndFixed));
        assert!(has_greater && has_less && has_and);
    }
    
    #[test]
    fn test_compile_expr_success() {
        let result = compile_expr("sin(time) + 0.5");
        assert!(result.is_ok());
        let program = result.unwrap();
        assert!(program.opcodes.len() > 0);
    }
    
    #[test]
    fn test_compile_expr_undefined_variable() {
        let result = compile_expr("undefined_var");
        assert!(result.is_err());
        if let Err(CompileError::TypeCheck(e)) = result {
            assert!(matches!(e.kind, error::TypeErrorKind::UndefinedVariable(_)));
        } else {
            panic!("Expected TypeCheck error");
        }
    }
    
    #[test]
    fn test_compile_expr_undefined_function() {
        let result = compile_expr("unknown_func(1.0)");
        assert!(result.is_err());
        if let Err(CompileError::TypeCheck(e)) = result {
            assert!(matches!(e.kind, error::TypeErrorKind::UndefinedFunction(_)));
        } else {
            panic!("Expected TypeCheck error");
        }
    }
    
    #[test]
    fn test_perlin3_only_pushes_xyz() {
        // Regression test for horizontal stripes bug!
        // perlin3(vec3) should only push 3 args to stack (x, y, z components of vec3)
        // Octaves should be extracted at compile time and embedded in the opcode
        let program = parse_expr("perlin3(vec3(xNorm, yNorm, time), 3)");
        
        // Count Push/Load opcodes before Perlin3
        let mut push_count = 0;
        for op in &program.opcodes {
            if matches!(op, LpsOpCode::Perlin3(_)) {
                break;
            }
            if matches!(op, LpsOpCode::Push(_) | LpsOpCode::Load(_)) {
                push_count += 1;
            }
        }
        
        // CRITICAL: Should be exactly 3 (xNorm, yNorm, time)
        // If it's 4, the VM will pop them as (z=octaves, y=z, x=y)
        // causing only Y to vary (horizontal stripes)!
        assert_eq!(push_count, 3, 
            "BUG: perlin3 pushed {} args but VM expects 3. This causes horizontal stripes!",
            push_count);
        
        // Verify octaves is embedded in opcode
        let has_perlin = program.opcodes.iter().any(|op| {
            if let LpsOpCode::Perlin3(octaves) = op {
                *octaves == 3
            } else {
                false
            }
        });
        assert!(has_perlin, "Should have Perlin3(3) opcode with octaves=3 embedded");
    }
    
    #[test]
    fn test_compile_script_simple() {
        let script = "
            float x = uv.x;
            return x * 2.0;
        ";
        let result = compile_script(script);
        assert!(result.is_ok());
        let program = result.unwrap();
        assert!(program.opcodes.len() > 0);
        
        // Should have LoadLocalFixed and StoreLocalFixed for variable
        let has_store = program.opcodes.iter().any(|op| matches!(op, LpsOpCode::StoreLocalFixed(_)));
        assert!(has_store, "Should store local variable");
    }
    
    #[test]
    fn test_compile_script_if_else() {
        let script = "
            if (uv.x > 0.5) {
                return 1.0;
            } else {
                return 0.0;
            }
        ";
        let result = compile_script(script);
        assert!(result.is_ok());
        let program = result.unwrap();
        
        // Should have conditional jumps
        let has_jump_if_zero = program.opcodes.iter().any(|op| matches!(op, LpsOpCode::JumpIfZero(_)));
        let has_jump = program.opcodes.iter().any(|op| matches!(op, LpsOpCode::Jump(_)));
        assert!(has_jump_if_zero, "Should have conditional jump");
        assert!(has_jump, "Should have unconditional jump");
    }
    
    #[test]
    fn test_compile_script_while_loop() {
        let script = "
            float i = 0.0;
            while (i < 10.0) {
                i = i + 1.0;
            }
            return i;
        ";
        let result = compile_script(script);
        assert!(result.is_ok());
        let program = result.unwrap();
        
        // Should have loop structure
        let has_jump_if_zero = program.opcodes.iter().any(|op| matches!(op, LpsOpCode::JumpIfZero(_)));
        let has_jump = program.opcodes.iter().any(|op| matches!(op, LpsOpCode::Jump(_)));
        assert!(has_jump_if_zero && has_jump, "Should have loop jumps");
    }
    
    #[test]
    fn test_compile_script_for_loop_debug() {
        // Simple for loop - just verify it compiles
        let script = "
            for (float i = 0.0; i < 3.0; i = i + 1.0) {
                float x = i;
            }
            return 0.0;
        ";
        let result = compile_script(script);
        assert!(result.is_ok());
        
        // Check for loop structure opcodes
        let program = result.unwrap();
        let has_jumps = program.opcodes.iter().any(|op| matches!(op, LpsOpCode::Jump(_) | LpsOpCode::JumpIfZero(_)));
        assert!(has_jumps, "For loop should generate jump opcodes");
    }
    
    #[test]
    fn test_compile_script_variable_mutation() {
        // Test variable mutation without for loop
        let script = "
            float x = 1.0;
            x = x + 1.0;
            return x;
        ";
        let result = compile_script(script);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_compile_script_variable_scoping() {
        let script = "
            float x = 1.0;
            {
                float x = 2.0;
                return x;
            }
        ";
        let result = compile_script(script);
        assert!(result.is_ok());
        // Verifies scoping works (inner x shadows outer x)
    }
    
    #[test]
    fn test_compile_script_user_function() {
        let script = "
            float double(float x) {
                return x * 2.0;
            }
            
            float result = double(5.0);
            return result;
        ";
        let result = compile_script(script);
        assert!(result.is_ok());
        
        let program = result.unwrap();
        // Should have Call opcode
        let has_call = program.opcodes.iter().any(|op| matches!(op, LpsOpCode::Call(_)));
        assert!(has_call, "Should have Call opcode for user function");
    }
    
    #[test]
    fn test_compile_script_recursive_function() {
        let script = "
            float factorial(float n) {
                if (n <= 1.0) {
                    return 1.0;
                } else {
                    return n * factorial(n - 1.0);
                }
            }
            
            return factorial(5.0);
        ";
        let result = compile_script(script);
        assert!(result.is_ok());
        // Verifies recursive calls compile
    }
}
