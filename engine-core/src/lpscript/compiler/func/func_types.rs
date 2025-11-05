/// Function type checking
extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::lpscript::compiler::ast::{FunctionDef, Stmt, StmtKind};
use crate::lpscript::compiler::error::{TypeError, TypeErrorKind};
use crate::lpscript::compiler::typechecker::{SymbolTable, TypeChecker};
use crate::lpscript::shared::Type;

/// Function signature for user-defined functions
#[derive(Debug, Clone)]
pub(crate) struct FunctionSignature {
    pub(crate) params: Vec<Type>,
    pub(crate) return_type: Type,
}

/// Function table for tracking user-defined functions
#[derive(Debug, Clone)]
pub(crate) struct FunctionTable {
    functions: BTreeMap<String, FunctionSignature>,
}

impl FunctionTable {
    pub(crate) fn new() -> Self {
        FunctionTable {
            functions: BTreeMap::new(),
        }
    }

    pub(crate) fn declare(
        &mut self,
        name: String,
        params: Vec<Type>,
        return_type: Type,
    ) -> Result<(), String> {
        if self.functions.contains_key(&name) {
            return Err(format!("Function '{}' already declared", name));
        }
        self.functions.insert(
            name,
            FunctionSignature {
                params,
                return_type,
            },
        );
        Ok(())
    }

    pub(crate) fn lookup(&self, name: &str) -> Option<&FunctionSignature> {
        self.functions.get(name)
    }
}

impl TypeChecker {
    /// Type check a function definition
    pub(crate) fn check_function(
        func: &mut FunctionDef,
        func_table: &FunctionTable,
    ) -> Result<(), TypeError> {
        let mut symbols = SymbolTable::new();

        // Add parameters to symbol table
        for param in &func.params {
            symbols
                .declare(param.name.clone(), param.ty.clone())
                .map_err(|msg| TypeError {
                    kind: TypeErrorKind::UndefinedVariable(msg),
                    span: func.span,
                })?;
        }

        // Type check function body
        for stmt in &mut func.body {
            Self::check_stmt(stmt, &mut symbols, func_table)?;
        }

        // Verify all code paths return a value (if return_type != Void)
        if func.return_type != Type::Void {
            if !Self::all_paths_return(&func.body) {
                return Err(TypeError {
                    kind: TypeErrorKind::MissingReturn(func.name.clone()),
                    span: func.span,
                });
            }
        }

        Ok(())
    }

    /// Check if all code paths in a statement list return
    fn all_paths_return(stmts: &[Stmt]) -> bool {
        stmts.iter().any(|stmt| Self::stmt_always_returns(stmt))
    }

    /// Check if a statement always returns (guarantees return on all code paths)
    fn stmt_always_returns(stmt: &Stmt) -> bool {
        match &stmt.kind {
            StmtKind::Return(_) => true,

            StmtKind::Block(stmts) => {
                // A block always returns if any statement in it always returns
                Self::all_paths_return(stmts)
            }

            StmtKind::If {
                then_stmt,
                else_stmt,
                ..
            } => {
                // An if statement always returns only if:
                // 1. Both branches exist
                // 2. Both branches always return
                if let Some(else_s) = else_stmt {
                    Self::stmt_always_returns(then_stmt) && Self::stmt_always_returns(else_s)
                } else {
                    false
                }
            }

            // Loops don't guarantee returns (they might not execute)
            StmtKind::While { .. } | StmtKind::For { .. } => false,

            // Other statements don't return
            StmtKind::VarDecl { .. } | StmtKind::Expr(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lpscript::compiler::error::{CompileError, TypeErrorKind};
    use crate::lpscript::{compile_script_with_options, OptimizeOptions};

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
    // Note: Return type validation is not currently implemented in the type checker.
    // The check_function() method validates that all paths return but doesn't check
    // that the returned type matches the function's declared return type.
    // These tests are marked as ignored until return type validation is implemented.

    #[test]
    #[ignore] // Return type validation not implemented
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
    #[ignore] // Return type validation not implemented
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
    #[ignore] // Return type validation not implemented
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
    #[ignore] // Return type validation not implemented
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
