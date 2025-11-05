/// AST-level optimizations
extern crate alloc;
use alloc::boxed::Box;
use alloc::format;

use crate::lpscript::compiler::ast::{Expr, Program, Stmt};
use crate::lpscript::compiler::optimize::OptimizeOptions;

pub mod algebraic;
pub mod constant_fold;
pub mod dead_code;

#[cfg(test)]
mod algebraic_tests;
#[cfg(test)]
mod constant_fold_tests;

/// Optimize an expression, running multiple passes until fixed point
pub fn optimize_expr(mut expr: Expr, options: &OptimizeOptions) -> Expr {
    for _ in 0..options.max_ast_passes {
        let before = format!("{:?}", expr);

        // Apply optimization passes in order
        if options.constant_folding {
            expr = constant_fold::fold_expr(expr);
        }

        if options.algebraic_simplification {
            expr = algebraic::simplify_expr(expr);
        }

        let after = format!("{:?}", expr);

        // Stop if no changes (fixed point reached)
        if before == after {
            break;
        }
    }

    expr
}

/// Optimize a program (statements + expressions)
pub fn optimize_program(mut program: Program, options: &OptimizeOptions) -> Program {
    for _ in 0..options.max_ast_passes {
        let before = format!("{:?}", program);

        // Optimize statements
        if options.constant_folding
            || options.algebraic_simplification
            || options.dead_code_elimination
        {
            program.stmts = program
                .stmts
                .into_iter()
                .map(|stmt| optimize_stmt(stmt, options))
                .collect();
        }

        // Apply dead code elimination at statement level
        if options.dead_code_elimination {
            program.stmts = dead_code::eliminate_dead_stmts(program.stmts);
        }

        let after = format!("{:?}", program);

        // Stop if no changes
        if before == after {
            break;
        }
    }

    program
}

/// Optimize a single statement (recursive)
fn optimize_stmt(stmt: Stmt, options: &OptimizeOptions) -> Stmt {
    let kind = match stmt.kind {
        crate::lpscript::compiler::ast::StmtKind::VarDecl { ty, name, init } => {
            let init = init.map(|e| optimize_expr(e, options));
            crate::lpscript::compiler::ast::StmtKind::VarDecl { ty, name, init }
        }
        crate::lpscript::compiler::ast::StmtKind::Return(expr) => {
            let expr = optimize_expr(expr, options);
            crate::lpscript::compiler::ast::StmtKind::Return(expr)
        }
        crate::lpscript::compiler::ast::StmtKind::Expr(expr) => {
            let expr = optimize_expr(expr, options);
            crate::lpscript::compiler::ast::StmtKind::Expr(expr)
        }
        crate::lpscript::compiler::ast::StmtKind::Block(stmts) => {
            let stmts = stmts
                .into_iter()
                .map(|s| optimize_stmt(s, options))
                .collect();
            crate::lpscript::compiler::ast::StmtKind::Block(stmts)
        }
        crate::lpscript::compiler::ast::StmtKind::If {
            condition,
            then_stmt,
            else_stmt,
        } => {
            let condition = optimize_expr(condition, options);
            let then_stmt = Box::new(optimize_stmt(*then_stmt, options));
            let else_stmt = else_stmt.map(|s| Box::new(optimize_stmt(*s, options)));

            // If condition is constant, we can simplify
            if options.dead_code_elimination {
                return dead_code::simplify_if(condition, then_stmt, else_stmt, stmt.span);
            }

            crate::lpscript::compiler::ast::StmtKind::If {
                condition,
                then_stmt,
                else_stmt,
            }
        }
        crate::lpscript::compiler::ast::StmtKind::While { condition, body } => {
            let condition = optimize_expr(condition, options);
            let body = Box::new(optimize_stmt(*body, options));
            crate::lpscript::compiler::ast::StmtKind::While { condition, body }
        }
        crate::lpscript::compiler::ast::StmtKind::For {
            init,
            condition,
            increment,
            body,
        } => {
            let init = init.map(|s| Box::new(optimize_stmt(*s, options)));
            let condition = condition.map(|e| optimize_expr(e, options));
            let increment = increment.map(|e| optimize_expr(e, options));
            let body = Box::new(optimize_stmt(*body, options));
            crate::lpscript::compiler::ast::StmtKind::For {
                init,
                condition,
                increment,
                body,
            }
        }
    };

    Stmt {
        kind,
        span: stmt.span,
    }
}
