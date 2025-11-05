/// Variable expression type checking
extern crate alloc;
use alloc::string::String;

use crate::lpscript::compiler::typechecker::{SymbolTable, TypeChecker};
use crate::lpscript::compiler::error::{TypeError, TypeErrorKind};
use crate::lpscript::shared::Type;

impl TypeChecker {
    /// Type check variable reference
    ///
    /// Returns the type of the variable.
    pub(crate) fn check_variable(
        name: &str,
        symbols: &SymbolTable,
        span: crate::lpscript::shared::Span,
    ) -> Result<Type, TypeError> {
        // Check built-ins first, then symbol table
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

            // Not a built-in, check symbol table
            _ => symbols.lookup(name).ok_or_else(|| TypeError {
                kind: TypeErrorKind::UndefinedVariable(String::from(name)),
                span,
            })?,
        };

        Ok(var_type)
    }
}
