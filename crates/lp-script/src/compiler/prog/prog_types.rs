/// Program type checking
extern crate alloc;
use alloc::string::ToString;

use crate::compiler::ast::{Expr, Program, Stmt, StmtKind};
use crate::compiler::error::{TypeError, TypeErrorKind};
use crate::compiler::typechecker::{FunctionTable, SymbolTable, TypeChecker};
use crate::shared::Type;

impl TypeChecker {
    /// Type check a program (script mode) with pre-built function table
    pub fn check_program(
        program: &mut Program,
        func_table: &FunctionTable,
    ) -> Result<(), TypeError> {
        // Type check each function body
        for func in &mut program.functions {
            Self::check_function_body(
                &mut func.body,
                &func.return_type,
                &func.params,
                func.span,
                &func.name,
                func_table,
            )?;
        }

        // Type check top-level statements
        let mut symbols = SymbolTable::new();
        for stmt in &mut program.stmts {
            Self::check_stmt(stmt, &mut symbols, func_table)?;
        }

        Ok(())
    }

    /// Type check a function body
    fn check_function_body(
        body: &mut [Stmt],
        expected_return_type: &Type,
        params: &[crate::compiler::ast::Parameter],
        func_span: crate::shared::Span,
        func_name: &str,
        func_table: &FunctionTable,
    ) -> Result<(), TypeError> {
        let mut symbols = SymbolTable::new();

        // Add parameters to symbol table
        for param in params {
            symbols
                .declare(param.name.clone(), param.ty.clone())
                .map_err(|msg| TypeError {
                    kind: TypeErrorKind::UndefinedVariable(msg),
                    span: func_span,
                })?;
        }

        // Type check each statement in the function body
        for stmt in body.iter_mut() {
            Self::check_stmt(stmt, &mut symbols, func_table)?;

            // Check if this is a return statement and validate its type
            if let StmtKind::Return(expr) = &stmt.kind {
                Self::check_return_type(expr, expected_return_type)?;
            }
        }

        // Verify all code paths return a value (if return_type != Void)
        if *expected_return_type != Type::Void && !Self::all_paths_return(body) {
            return Err(TypeError {
                kind: TypeErrorKind::MissingReturn(func_name.to_string()),
                span: func_span,
            });
        }

        Ok(())
    }

    /// Check if a return expression matches the expected return type
    fn check_return_type(expr: &Expr, expected: &Type) -> Result<(), TypeError> {
        // Get the actual type of the return expression
        let actual_type = if let Some(ty) = &expr.ty {
            ty.clone()
        } else {
            // Type should already be inferred, but if not, this is an error
            return Err(TypeError {
                kind: TypeErrorKind::UndefinedVariable("return expression has no type".into()),
                span: expr.span,
            });
        };

        // Check if types match
        if actual_type != *expected {
            return Err(TypeError {
                kind: TypeErrorKind::Mismatch {
                    expected: expected.clone(),
                    found: actual_type,
                },
                span: expr.span,
            });
        }

        Ok(())
    }

    /// Check if all code paths in a statement list return
    fn all_paths_return(stmts: &[Stmt]) -> bool {
        stmts.iter().any(Self::stmt_always_returns)
    }

    /// Check if a statement always returns
    fn stmt_always_returns(stmt: &Stmt) -> bool {
        match &stmt.kind {
            StmtKind::Return(_) => true,

            StmtKind::Block(stmts) => Self::all_paths_return(stmts),

            StmtKind::If {
                then_stmt,
                else_stmt,
                ..
            } => {
                if let Some(else_s) = else_stmt {
                    Self::stmt_always_returns(then_stmt.as_ref())
                        && Self::stmt_always_returns(else_s.as_ref())
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
