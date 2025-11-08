/// Variable type checking
extern crate alloc;

use crate::compiler::error::{TypeError, TypeErrorKind};
use crate::compiler::typechecker::SymbolTable;
use crate::shared::Type;
use alloc::string::String;

/// Check variable type (including built-in variables)
pub(in crate::compiler) fn check_variable(
    name: &str,
    symbols: &mut SymbolTable,
    span: crate::shared::Span,
) -> Result<Type, TypeError> {
    // Check symbol table first to allow shadowing of built-ins
    if let Some(ty) = symbols.lookup(name) {
        return Ok(ty);
    }

    // Then check built-ins
    let var_type = match name {
        // Vec2 built-ins (GLSL-style)
        "uv" => Type::Vec2,    // normalized coordinates (0..1)
        "coord" => Type::Vec2, // pixel coordinates

        // Scalar built-ins
        "time" | "t" => Type::Fixed,
        "timeNorm" => Type::Fixed,
        "centerAngle" | "angle" => Type::Fixed,
        "centerDist" | "dist" => Type::Fixed,

        // Legacy scalar built-ins (deprecated, kept for compatibility)
        "x" | "xNorm" | "y" | "yNorm" => Type::Fixed,

        // Not found anywhere
        _ => {
            return Err(TypeError {
                kind: TypeErrorKind::UndefinedVariable(String::from(name)),
                span,
            });
        }
    };

    Ok(var_type)
}

/// Check increment/decrement operations
pub(in crate::compiler) fn check_incdec(
    name: &str,
    symbols: &mut SymbolTable,
    span: crate::shared::Span,
) -> Result<Type, TypeError> {
    check_variable(name, symbols, span)
}
