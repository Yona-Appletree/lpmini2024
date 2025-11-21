/// Increment/Decrement operator type checking
extern crate alloc;

use alloc::string::String;

use crate::compiler::error::{TypeError, TypeErrorKind};
use crate::compiler::typechecker::{SymbolTable, TypeChecker};
use crate::shared::{Span, Type};

impl TypeChecker {
    /// Type check increment/decrement operators (++, --)
    ///
    /// These work on Dec32 and Int32 types and return the same type.
    pub(crate) fn check_incdec(
        var_name: &String,
        symbols: &mut SymbolTable,
        span: Span,
    ) -> Result<Type, TypeError> {
        // Look up the variable
        let var_ty = symbols.lookup(var_name).ok_or_else(|| TypeError {
            kind: TypeErrorKind::UndefinedVariable(var_name.clone()),
            span,
        })?;

        // Increment/decrement only work on Dec32 and Int32
        match var_ty {
            Type::Dec32 | Type::Int32 => Ok(var_ty.clone()),
            _ => Err(TypeError {
                kind: TypeErrorKind::Mismatch {
                    expected: Type::Dec32, // Or Type::Int32
                    found: var_ty.clone(),
                },
                span,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::compiler::error::{CompileError, TypeErrorKind};
    use crate::{compile_script_with_options, OptimizeOptions};

    // ========================================================================
    // Type Error Tests - Increment/Decrement on Invalid Types
    // ========================================================================

    #[test]
    fn test_increment_vec2() {
        let program_text = "
            vec2 v = vec2(1.0, 2.0);
            ++v;
            return 1.0;
        ";

        let result = compile_script_with_options(program_text, &OptimizeOptions::none());
        assert!(result.is_err(), "++vec2 should be a type error");

        if let Err(CompileError::TypeCheck(err)) = result {
            assert!(matches!(err.kind, TypeErrorKind::Mismatch { .. }));
        } else {
            panic!("Expected TypeCheck error");
        }
    }

    #[test]
    fn test_post_increment_vec3() {
        let program_text = "
            vec3 v = vec3(1.0, 2.0, 3.0);
            v++;
            return 1.0;
        ";

        let result = compile_script_with_options(program_text, &OptimizeOptions::none());
        assert!(result.is_err(), "vec3++ should be a type error");

        if let Err(CompileError::TypeCheck(err)) = result {
            assert!(matches!(err.kind, TypeErrorKind::Mismatch { .. }));
        } else {
            panic!("Expected TypeCheck error");
        }
    }

    #[test]
    fn test_decrement_vec4() {
        let program_text = "
            vec4 v = vec4(1.0, 2.0, 3.0, 4.0);
            --v;
            return 1.0;
        ";

        let result = compile_script_with_options(program_text, &OptimizeOptions::none());
        assert!(result.is_err(), "--vec4 should be a type error");

        if let Err(CompileError::TypeCheck(err)) = result {
            assert!(matches!(err.kind, TypeErrorKind::Mismatch { .. }));
        } else {
            panic!("Expected TypeCheck error");
        }
    }
}
