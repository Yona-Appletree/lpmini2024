/// AST-level optimizations
extern crate alloc;

use crate::lp_script::compiler::ast::{Expr, Program, Stmt};
use crate::lp_script::compiler::optimize::OptimizeOptions;

pub mod algebraic;
pub mod constant_fold;
// pub mod dead_code; // TODO: Update to new API

#[cfg(test)]
mod algebraic_tests;
#[cfg(test)]
mod constant_fold_tests;

/// Optimize an expression
pub fn optimize_expr(expr: &mut Expr, options: &OptimizeOptions) {
    for _ in 0..options.max_ast_passes {
        let mut changed = false;

        // Apply constant folding if enabled
        if options.constant_folding {
            changed |= constant_fold::fold_constants(expr);
        }

        // Apply algebraic simplification if enabled
        if options.algebraic_simplification {
            changed |= algebraic::simplify_expr(expr);
        }

        // Stop if no changes (dec32 point reached)
        if !changed {
            break;
        }
    }
}

/// Optimize a program
pub fn optimize_program(program: &mut Program, options: &OptimizeOptions) {
    if options.max_ast_passes == 0 {
        return;
    }

    // Optimize each statement in the program
    for _ in 0..options.max_ast_passes {
        let mut changed = false;

        // Optimize all statements
        for stmt in &mut program.stmts {
            changed |= optimize_stmt(stmt, options);
        }

        // Stop if no changes
        if !changed {
            break;
        }
    }
}

/// Optimize a statement (recursive)
fn optimize_stmt(stmt: &mut Stmt, options: &OptimizeOptions) -> bool {
    use crate::lp_script::compiler::ast::StmtKind;

    let mut changed = false;

    match &mut stmt.kind {
        StmtKind::VarDecl { init, .. } => {
            if let Some(init_expr) = init {
                optimize_expr(init_expr, options);
                changed = true;
            }
        }
        StmtKind::Return(expr) => {
            optimize_expr(expr, options);
            changed = true;
        }
        StmtKind::Expr(expr) => {
            optimize_expr(expr, options);
            changed = true;
        }
        StmtKind::Block(stmts) => {
            for s in stmts {
                changed |= optimize_stmt(s, options);
            }
        }
        StmtKind::If {
            condition,
            then_stmt,
            else_stmt,
        } => {
            optimize_expr(condition, options);
            changed |= optimize_stmt(then_stmt.as_mut(), options);
            if let Some(else_s) = else_stmt {
                changed |= optimize_stmt(else_s.as_mut(), options);
            }
        }
        StmtKind::While { condition, body } => {
            optimize_expr(condition, options);
            changed |= optimize_stmt(body.as_mut(), options);
        }
        StmtKind::For {
            init,
            condition,
            increment,
            body,
        } => {
            if let Some(init_stmt) = init {
                changed |= optimize_stmt(init_stmt.as_mut(), options);
            }
            if let Some(cond) = condition {
                optimize_expr(cond, options);
            }
            if let Some(inc) = increment {
                optimize_expr(inc, options);
            }
            changed |= optimize_stmt(body.as_mut(), options);
        }
    }

    changed
}
