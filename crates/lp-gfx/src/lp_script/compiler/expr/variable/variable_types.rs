/// Variable type checking
extern crate alloc;

use alloc::string::String;

use crate::lp_script::compiler::error::{TypeError, TypeErrorKind};
use crate::lp_script::compiler::typechecker::SymbolTable;
use crate::lp_script::shared::Type;

/// Check variable type (including built-in variables)
pub(crate) fn check_variable(
    name: &str,
    symbols: &mut SymbolTable,
    span: crate::lp_script::shared::Span,
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
        "time" | "t" => Type::Dec32,
        "timeNorm" => Type::Dec32,
        "centerAngle" | "angle" => Type::Dec32,
        "centerDist" | "dist" => Type::Dec32,

        // Legacy scalar built-ins (deprecated, kept for compatibility)
        "x" | "xNorm" | "y" | "yNorm" => Type::Dec32,

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
pub(crate) fn check_incdec(
    name: &str,
    symbols: &mut SymbolTable,
    span: crate::lp_script::shared::Span,
) -> Result<Type, TypeError> {
    check_variable(name, symbols, span)
}
