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
/// use engine_core::expr::parse_expr;
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

mod lexer;
mod ast;
mod parser;
mod codegen;

pub use codegen::NativeFunction;

use crate::test_engine::OpCode;

/// Parse an expression string and generate VM opcodes
/// 
/// # Example
/// ```
/// let code = parse_expr("cos(perlin3(xNorm*0.3, yNorm*0.3, time, 3))");
/// ```
pub fn parse_expr(input: &str) -> Vec<OpCode> {
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
        let code = parse_expr("42");
        assert_eq!(code.len(), 2); // Push(42), Return
        assert!(matches!(code[0], OpCode::Push(_)));
    }
    
    #[test]
    fn test_arithmetic() {
        let code = parse_expr("1 + 2 * 3");
        // Should respect precedence: 1 + (2 * 3)
        assert!(matches!(code[0], OpCode::Push(_)));
        assert!(matches!(code[1], OpCode::Push(_)));
        assert!(matches!(code[2], OpCode::Push(_)));
        assert!(matches!(code[3], OpCode::Mul));
        assert!(matches!(code[4], OpCode::Add));
    }
    
    #[test]
    fn test_exponential() {
        let code = parse_expr("2 ^ 3");
        assert!(matches!(code[2], OpCode::CallNative(_)));
    }
    
    #[test]
    fn test_variables() {
        let code = parse_expr("xNorm");
        assert!(matches!(code[0], OpCode::Load(LoadSource::XNorm)));
    }
    
    #[test]
    fn test_sin_function() {
        let code = parse_expr("sin(time)");
        assert!(matches!(code[0], OpCode::Load(LoadSource::Time)));
        assert!(matches!(code[1], OpCode::Sin));
    }
    
    #[test]
    fn test_min_max() {
        let code = parse_expr("min(xNorm, 0.5)");
        assert!(matches!(code[0], OpCode::Load(LoadSource::XNorm)));
        assert!(matches!(code[2], OpCode::CallNative(_)));
    }
    
    #[test]
    fn test_ternary() {
        let code = parse_expr("xNorm > 0.5 ? 1.0 : 0.0");
        // Load xNorm, Push 0.5, Greater, Push 1.0, Push 0.0, Select
        assert!(matches!(code[2], OpCode::CallNative(_))); // Greater
        assert!(matches!(code.last(), Some(OpCode::Return)));
    }
    
    #[test]
    fn test_complex_perlin() {
        let code = parse_expr("cos(perlin3(xNorm*0.3, yNorm*0.3, time, 3))");
        assert!(matches!(code[0], OpCode::Load(LoadSource::XNorm)));
        // Should have Perlin3 and Cos
        let has_perlin = code.iter().any(|op| matches!(op, OpCode::Perlin3(_)));
        let has_cos = code.iter().any(|op| matches!(op, OpCode::Cos));
        assert!(has_perlin);
        assert!(has_cos);
    }
    
    #[test]
    fn test_logical_and() {
        let code = parse_expr("xNorm > 0.5 && yNorm < 0.5");
        // Should have Greater, Less, and And
        let and_count = code.iter().filter(|op| matches!(op, OpCode::CallNative(_))).count();
        assert!(and_count >= 3); // Greater, Less, And
    }
}

