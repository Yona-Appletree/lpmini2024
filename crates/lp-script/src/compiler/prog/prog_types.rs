/// Program type checking
extern crate alloc;
use alloc::string::ToString;

use crate::compiler::ast::{AstPool, ExprId, Program, StmtId, StmtKind};
use crate::compiler::error::{TypeError, TypeErrorKind};
use crate::compiler::typechecker::{FunctionTable, SymbolTable, TypeChecker};
use crate::shared::Type;

impl TypeChecker {
    /// Type check a program (script mode) with pre-built function table
    pub fn check_program(
        program: Program,
        mut pool: AstPool,
        func_table: &FunctionTable,
    ) -> Result<(Program, AstPool), TypeError> {
        // Type check each function body
        for func in &program.functions {
            Self::check_function_body(
                &func.body,
                &func.return_type,
                &func.params,
                func.span,
                &func.name,
                &mut pool,
                func_table,
            )?;
        }

        // Type check top-level statements
        let mut symbols = SymbolTable::new();
        for stmt_id in &program.stmts {
            Self::check_stmt_id(&mut pool, *stmt_id, &mut symbols, func_table)?;
        }

        Ok((program, pool))
    }

    /// Type check a function body
    fn check_function_body(
        body: &[StmtId],
        expected_return_type: &Type,
        params: &[crate::compiler::ast::Parameter],
        func_span: crate::shared::Span,
        func_name: &str,
        pool: &mut AstPool,
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
        for &stmt_id in body {
            Self::check_stmt_id(pool, stmt_id, &mut symbols, func_table)?;

            // Check if this is a return statement and validate its type
            let stmt = pool.stmt(stmt_id);
            if let StmtKind::Return(expr_id) = &stmt.kind {
                Self::check_return_type(*expr_id, expected_return_type, pool, func_table)?;
            }
        }

        // Verify all code paths return a value (if return_type != Void)
        if *expected_return_type != Type::Void {
            if !Self::all_paths_return_id(body, pool) {
                return Err(TypeError {
                    kind: TypeErrorKind::MissingReturn(func_name.to_string()),
                    span: func_span,
                });
            }
        }

        Ok(())
    }

    /// Check if a return expression matches the expected return type
    fn check_return_type(
        expr_id: ExprId,
        expected: &Type,
        pool: &AstPool,
        _func_table: &FunctionTable,
    ) -> Result<(), TypeError> {
        let expr = pool.expr(expr_id);

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

    /// Check if all code paths in a statement list return (using StmtId)
    fn all_paths_return_id(stmts: &[StmtId], pool: &AstPool) -> bool {
        stmts
            .iter()
            .any(|&stmt_id| Self::stmt_always_returns_id(stmt_id, pool))
    }

    /// Check if a statement always returns (using StmtId)
    fn stmt_always_returns_id(stmt_id: StmtId, pool: &AstPool) -> bool {
        let stmt = pool.stmt(stmt_id);
        match &stmt.kind {
            StmtKind::Return(_) => true,

            StmtKind::Block(stmts) => Self::all_paths_return_id(stmts, pool),

            StmtKind::If {
                then_stmt,
                else_stmt,
                ..
            } => {
                if let Some(else_id) = else_stmt {
                    Self::stmt_always_returns_id(*then_stmt, pool)
                        && Self::stmt_always_returns_id(*else_id, pool)
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
