/// Variable declaration type checking
extern crate alloc;
use alloc::string::String;

use crate::lpscript::compiler::ast::Expr;
use crate::lpscript::compiler::typechecker::{FunctionTable, SymbolTable, TypeChecker};
use crate::lpscript::error::{Type, TypeError, TypeErrorKind};

impl TypeChecker {
    /// Type check variable declaration
    pub(crate) fn check_var_decl(
        ty: &Type,
        name: &str,
        init: &mut Option<Expr>,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
        span: crate::lpscript::error::Span,
    ) -> Result<(), TypeError> {
        // If there's an initializer, type check it
        if let Some(init_expr) = init {
            Self::infer_type(init_expr, symbols, func_table)?;
            let init_ty = init_expr.ty.as_ref().unwrap();

            // Check type matches
            if ty != init_ty {
                return Err(TypeError {
                    kind: TypeErrorKind::Mismatch {
                        expected: ty.clone(),
                        found: init_ty.clone(),
                    },
                    span: init_expr.span,
                });
            }
        }

        // Add variable to symbol table
        symbols.declare(String::from(name), ty.clone());

        Ok(())
    }
}
