/// Increment/Decrement operator type checking
extern crate alloc;

use crate::lpscript::compiler::typechecker::{FunctionTable, SymbolTable, TypeChecker};
use crate::lpscript::compiler::error::{TypeError, TypeErrorKind};
use crate::lpscript::shared::{Span, Type};
use alloc::string::String;

impl TypeChecker {
    /// Type check increment/decrement operators (++, --)
    ///
    /// These work on Fixed and Int32 types and return the same type.
    pub(crate) fn check_incdec(
        var_name: &String,
        symbols: &mut SymbolTable,
        span: Span,
    ) -> Result<Type, TypeError> {
        // Look up the variable
        let var_ty = symbols.get(var_name).ok_or_else(|| {
            TypeError::new(
                TypeErrorKind::UndefinedVariable(var_name.clone()),
                span,
            )
        })?;

        // Increment/decrement only work on Fixed and Int32
        match var_ty {
            Type::Fixed | Type::Int32 => Ok(var_ty.clone()),
            _ => Err(TypeError::new(
                TypeErrorKind::TypeMismatch {
                    expected: "int or float".into(),
                    found: var_ty.to_string().into(),
                },
                span,
            )),
        }
    }
}

