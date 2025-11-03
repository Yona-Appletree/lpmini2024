/// Expression language for generating VM opcodes
///
/// This module provides a simple expression language that compiles to VM opcodes.
///
/// # Features
/// - **Arithmetic**: `+`, `-`, `*`, `/`, `%`, `^` (power)
/// - **Comparisons**: `<`, `>`, `<=`, `>=`, `==`, `!=`
/// - **Logical**: `&&`, `||`
/// - **Ternary**: `condition ? true_val : false_val`
///
/// # GLSL/HLSL Shader Functions
/// - **Math**: `sin`, `cos`, `abs`, `floor`, `ceil`, `sqrt`, `sign`, `pow`, `min`, `max`
/// - **Clamping**: `clamp(value, min, max)`, `saturate(x)` (clamp to 0..1), `step(edge, x)`
/// - **Interpolation**: `lerp(a, b, t)` or `mix(a, b, t)`, `smoothstep(edge0, edge1, x)`
/// - **Perlin noise**: `perlin3(x, y, z, octaves)`
///
/// # Examples
/// ```
/// use engine_core::lpscript::parse_expr;
///
/// // Simple math
/// let code = parse_expr("sin(time) * 0.5 + 0.5");
///
/// // Perlin noise
/// let code = parse_expr("cos(perlin3(xNorm*0.3, yNorm*0.3, time, 3))");
///
/// // Ternary operator
/// let code = parse_expr("centerDist < 0.5 ? 1.0 : 0.0");
///
/// // Min/max
/// let code = parse_expr("max(0, min(1, xNorm * 2))");
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
pub use vm::{LocalAccess, LocalDef, LocalType, LpsProgram, LpsVm};

use crate::test_engine::OpCode;

/// Parse an expression string and generate a compiled LPS program
///
/// Returns Result with comprehensive compile errors.
///
/// # Example
/// ```
/// let program = compile_expr("cos(perlin3(xNorm*0.3, yNorm*0.3, time, 3))").unwrap();
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

/// Parse an expression string and generate a compiled LPS program
///
/// Panics on compile errors. Use `compile_expr()` for error handling.
///
/// # Example
/// ```
/// let program = parse_expr("cos(perlin3(xNorm*0.3, yNorm*0.3, time, 3))");
/// ```
pub fn parse_expr(input: &str) -> LpsProgram {
    compile_expr(input).unwrap_or_else(|e| {
        panic!("Failed to compile LPS expression: {}", e);
    })
}

/// Legacy API: just get opcodes (for backward compatibility during migration)
pub fn parse_expr_opcodes(input: &str) -> Vec<OpCode> {
    let mut lexer = lexer::Lexer::new(input);
    let tokens = lexer.tokenize();

    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse();

    codegen::CodeGenerator::generate(&ast)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_engine::LoadSource;

    #[test]
    fn test_simple_number() {
        let program = parse_expr("42");
        assert_eq!(program.opcodes.len(), 2); // Push(42), Return
        assert!(matches!(program.opcodes[0], OpCode::Push(_)));
    }

    #[test]
    fn test_arithmetic() {
        let program = parse_expr("1 + 2 * 3");
        let code = &program.opcodes;
        // Should respect precedence: 1 + (2 * 3)
        assert!(matches!(code[0], OpCode::Push(_)));
        assert!(matches!(code[1], OpCode::Push(_)));
        assert!(matches!(code[2], OpCode::Push(_)));
        assert!(matches!(code[3], OpCode::Mul));
        assert!(matches!(code[4], OpCode::Add));
    }

    #[test]
    fn test_exponential() {
        let code = &parse_expr("2 ^ 3").opcodes;
        assert!(matches!(code[2], OpCode::CallNative(_)));
    }

    #[test]
    fn test_variables() {
        let code = &parse_expr("xNorm").opcodes;
        assert!(matches!(code[0], OpCode::Load(LoadSource::XNorm)));
    }

    #[test]
    fn test_sin_function() {
        let code = &parse_expr("sin(time)").opcodes;
        assert!(matches!(code[0], OpCode::Load(LoadSource::Time)));
        assert!(matches!(code[1], OpCode::Sin));
    }

    #[test]
    fn test_min_max() {
        let code = &parse_expr("min(xNorm, 0.5)").opcodes;
        assert!(matches!(code[0], OpCode::Load(LoadSource::XNorm)));
        assert!(matches!(code[2], OpCode::CallNative(_)));
    }

    #[test]
    fn test_ternary() {
        let code = &parse_expr("xNorm > 0.5 ? 1.0 : 0.0").opcodes;
        // Load xNorm, Push 0.5, Greater, Push 1.0, Push 0.0, Select
        assert!(matches!(code[2], OpCode::CallNative(_))); // Greater
        assert!(matches!(code.last(), Some(OpCode::Return)));
    }

    #[test]
    fn test_complex_perlin() {
        let code = &parse_expr("cos(perlin3(xNorm*0.3, yNorm*0.3, time, 3))").opcodes;
        assert!(matches!(code[0], OpCode::Load(LoadSource::XNorm)));
        // Should have Perlin3 and Cos
        let has_perlin = code.iter().any(|op| matches!(op, OpCode::Perlin3(_)));
        let has_cos = code.iter().any(|op| matches!(op, OpCode::Cos));
        assert!(has_perlin);
        assert!(has_cos);
    }

    #[test]
    fn test_logical_and() {
        let code = &parse_expr("xNorm > 0.5 && yNorm < 0.5").opcodes;
        // Should have Greater, Less, and And
        let and_count = code
            .iter()
            .filter(|op| matches!(op, OpCode::CallNative(_)))
            .count();
        assert!(and_count >= 3); // Greater, Less, And
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
        // perlin3(x, y, z, octaves) should only push 3 args to stack (x, y, z)
        // Octaves should be extracted at compile time and embedded in the opcode
        let program = parse_expr("perlin3(xNorm, yNorm, time, 3)");
        
        // Count Push/Load opcodes before Perlin3
        let mut push_count = 0;
        for op in &program.opcodes {
            if matches!(op, OpCode::Perlin3(_)) {
                break;
            }
            if matches!(op, OpCode::Push(_) | OpCode::Load(_)) {
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
            if let OpCode::Perlin3(octaves) = op {
                *octaves == 3
            } else {
                false
            }
        });
        assert!(has_perlin, "Should have Perlin3(3) opcode with octaves=3 embedded");
    }
}
