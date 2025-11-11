/// Function type checking
extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::shared::Type;

/// Local variable information
#[derive(Debug, Clone)]
pub(crate) struct LocalVarInfo {
    pub(crate) name: String,
    pub(crate) ty: Type,

    // used in tests
    #[allow(dead_code)]
    pub(crate) index: u32,
}

/// Function types including signature and local variables
#[derive(Debug, Clone)]
pub(crate) struct FunctionMetadata {
    pub(crate) params: Vec<Type>,
    pub(crate) return_type: Type,
    pub(crate) locals: Vec<LocalVarInfo>,

    // used in tests
    #[allow(dead_code)]
    pub(crate) local_count: u32,
}

/// Function table for tracking user-defined functions
#[derive(Debug, Clone)]
pub(crate) struct FunctionTable {
    functions: BTreeMap<String, FunctionMetadata>,
}

impl FunctionTable {
    pub(crate) fn new() -> Self {
        FunctionTable {
            functions: BTreeMap::new(),
        }
    }

    /// Declare a function with full types (params, return type, locals)
    pub(crate) fn declare_with_metadata(
        &mut self,
        name: String,
        metadata: FunctionMetadata,
    ) -> Result<(), String> {
        if self.functions.contains_key(&name) {
            return Err(format!("Function '{}' already declared", name));
        }
        self.functions.insert(name, metadata);
        Ok(())
    }

    /// Get function types
    pub(crate) fn lookup(&self, name: &str) -> Option<&FunctionMetadata> {
        self.functions.get(name)
    }
}

// NOTE: The old check_function implementation has been replaced with
// check_function_body in prog/prog_types.rs which uses the StmtId-based API.
// The old Stmt-based implementation is commented out as it's no longer compatible
// with the pool-based AST architecture.

#[cfg(test)]
mod tests {
    use crate::compiler::error::{CompileError, TypeErrorKind};
    use crate::{compile_script_with_options, OptimizeOptions};

    // ========================================================================
    // Type Error Tests - Function Parameter Mismatches
    // ========================================================================

    #[test]
    fn test_call_vec2_function_with_vec3() {
        let program_text = "
            float sumComponents(vec2 v) {
                return v.x + v.y;
            }
            return sumComponents(vec3(1.0, 2.0, 3.0));
        ";

        let result = compile_script_with_options(program_text, &OptimizeOptions::none());
        assert!(
            result.is_err(),
            "vec2 function called with vec3 should be a type error"
        );

        if let Err(CompileError::TypeCheck(err)) = result {
            // Could be InvalidArgumentCount or Mismatch
            assert!(
                matches!(err.kind, TypeErrorKind::Mismatch { .. })
                    || matches!(err.kind, TypeErrorKind::InvalidArgumentCount { .. })
            );
        } else {
            panic!("Expected TypeCheck error");
        }
    }

    #[test]
    fn test_call_float_function_with_vec2() {
        let program_text = "
            float double(float x) {
                return x * 2.0;
            }
            return double(vec2(1.0, 2.0));
        ";

        let result = compile_script_with_options(program_text, &OptimizeOptions::none());
        assert!(
            result.is_err(),
            "float function called with vec2 should be a type error"
        );

        if let Err(CompileError::TypeCheck(err)) = result {
            assert!(
                matches!(err.kind, TypeErrorKind::Mismatch { .. })
                    || matches!(err.kind, TypeErrorKind::InvalidArgumentCount { .. })
            );
        } else {
            panic!("Expected TypeCheck error");
        }
    }

    #[test]
    fn test_call_vec3_function_with_vec2() {
        let program_text = "
            float sumComponents(vec3 v) {
                return v.x + v.y + v.z;
            }
            return sumComponents(vec2(1.0, 2.0));
        ";

        let result = compile_script_with_options(program_text, &OptimizeOptions::none());
        assert!(
            result.is_err(),
            "vec3 function called with vec2 should be a type error"
        );

        if let Err(CompileError::TypeCheck(err)) = result {
            assert!(
                matches!(err.kind, TypeErrorKind::Mismatch { .. })
                    || matches!(err.kind, TypeErrorKind::InvalidArgumentCount { .. })
            );
        } else {
            panic!("Expected TypeCheck error");
        }
    }

    // ========================================================================
    // Type Error Tests - Function Return Type Mismatches
    // ========================================================================
    // Return type validation is now implemented in check_function_body in prog/prog_types.rs.
    // These tests verify that functions cannot return values of the wrong type.

    #[test]
    fn test_float_function_returns_vec2() {
        let program_text = "
            float getVector() {
                return vec2(1.0, 2.0);
            }
        ";

        let result = compile_script_with_options(program_text, &OptimizeOptions::none());
        assert!(
            result.is_err(),
            "float function returning vec2 should be a type error"
        );

        if let Err(CompileError::TypeCheck(err)) = result {
            assert!(matches!(err.kind, TypeErrorKind::Mismatch { .. }));
        } else {
            panic!("Expected TypeCheck error");
        }
    }

    #[test]
    fn test_vec3_function_returns_vec2() {
        let program_text = "
            vec3 makeVector() {
                return vec2(1.0, 2.0);
            }
        ";

        let result = compile_script_with_options(program_text, &OptimizeOptions::none());
        assert!(
            result.is_err(),
            "vec3 function returning vec2 should be a type error"
        );

        if let Err(CompileError::TypeCheck(err)) = result {
            assert!(matches!(err.kind, TypeErrorKind::Mismatch { .. }));
        } else {
            panic!("Expected TypeCheck error");
        }
    }

    #[test]
    fn test_vec2_function_returns_float() {
        let program_text = "
            vec2 getVector() {
                return 5.0;
            }
        ";

        let result = compile_script_with_options(program_text, &OptimizeOptions::none());
        assert!(
            result.is_err(),
            "vec2 function returning float should be a type error"
        );

        if let Err(CompileError::TypeCheck(err)) = result {
            assert!(matches!(err.kind, TypeErrorKind::Mismatch { .. }));
        } else {
            panic!("Expected TypeCheck error");
        }
    }

    #[test]
    fn test_vec4_function_returns_vec3() {
        let program_text = "
            vec4 makeVector() {
                return vec3(1.0, 2.0, 3.0);
            }
        ";

        let result = compile_script_with_options(program_text, &OptimizeOptions::none());
        assert!(
            result.is_err(),
            "vec4 function returning vec3 should be a type error"
        );

        if let Err(CompileError::TypeCheck(err)) = result {
            assert!(matches!(err.kind, TypeErrorKind::Mismatch { .. }));
        } else {
            panic!("Expected TypeCheck error");
        }
    }
}
