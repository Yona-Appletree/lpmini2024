/// AST-level optimizations
extern crate alloc;

use crate::lpscript::compiler::ast::{AstPool, ExprId, Program, StmtId};
use crate::lpscript::compiler::optimize::OptimizeOptions;

pub mod algebraic;
pub mod constant_fold;
// pub mod dead_code; // TODO: Update to pool-based API

#[cfg(test)]
mod algebraic_tests;
#[cfg(test)]
mod constant_fold_tests;

/// Optimize an expression using pool-based API
pub fn optimize_expr_id(
    mut expr_id: ExprId,
    mut pool: AstPool,
    options: &OptimizeOptions,
) -> (ExprId, AstPool) {
    for _ in 0..options.max_ast_passes {
        let initial_expr_count = pool.exprs.len();

        // Apply constant folding if enabled
        if options.constant_folding {
            (expr_id, pool) = constant_fold::fold_constants(expr_id, pool);
        }

        // Apply algebraic simplification if enabled
        if options.algebraic_simplification {
            (expr_id, pool) = algebraic::simplify_expr(expr_id, pool);
        }

        // Stop if no changes (fixed point reached)
        if pool.exprs.len() == initial_expr_count {
            break;
        }
    }

    (expr_id, pool)
}

/// Optimize a program using pool-based API
pub fn optimize_program_id(
    mut program: Program,
    mut pool: AstPool,
    options: &OptimizeOptions,
) -> (Program, AstPool) {
    if options.max_ast_passes == 0 {
        return (program, pool);
    }

    // Optimize each statement in the program
    for _ in 0..options.max_ast_passes {
        let initial_count = pool.exprs.len() + pool.stmts.len();

        // Optimize all statements
        for &stmt_id in &program.stmts {
            (pool) = optimize_stmt_id(stmt_id, pool, options);
        }

        // Stop if no changes
        if pool.exprs.len() + pool.stmts.len() == initial_count {
            break;
        }
    }

    (program, pool)
}

/// Optimize a statement (recursive, mutates pool)
fn optimize_stmt_id(stmt_id: StmtId, mut pool: AstPool, options: &OptimizeOptions) -> AstPool {
    use crate::lpscript::compiler::ast::StmtKind;

    let stmt = pool.stmt(stmt_id);
    let kind = stmt.kind.clone();

    match kind {
        StmtKind::VarDecl { ty, name, init } => {
            if let Some(init_id) = init {
                let (new_init, new_pool) = optimize_expr_id(init_id, pool, options);
                pool = new_pool;
                pool.stmt_mut(stmt_id).kind = StmtKind::VarDecl {
                    ty,
                    name,
                    init: Some(new_init),
                };
            }
        }
        StmtKind::Return(expr_id) => {
            let (new_expr, new_pool) = optimize_expr_id(expr_id, pool, options);
            pool = new_pool;
            pool.stmt_mut(stmt_id).kind = StmtKind::Return(new_expr);
        }
        StmtKind::Expr(expr_id) => {
            let (new_expr, new_pool) = optimize_expr_id(expr_id, pool, options);
            pool = new_pool;
            pool.stmt_mut(stmt_id).kind = StmtKind::Expr(new_expr);
        }
        StmtKind::Block(stmts) => {
            for &s_id in &stmts {
                pool = optimize_stmt_id(s_id, pool, options);
            }
        }
        StmtKind::If {
            condition,
            then_stmt,
            else_stmt,
        } => {
            let (new_cond, new_pool) = optimize_expr_id(condition, pool, options);
            pool = new_pool;
            pool = optimize_stmt_id(then_stmt, pool, options);
            if let Some(else_id) = else_stmt {
                pool = optimize_stmt_id(else_id, pool, options);
            }
            pool.stmt_mut(stmt_id).kind = StmtKind::If {
                condition: new_cond,
                then_stmt,
                else_stmt,
            };
        }
        StmtKind::While { condition, body } => {
            let (new_cond, new_pool) = optimize_expr_id(condition, pool, options);
            pool = new_pool;
            pool = optimize_stmt_id(body, pool, options);
            pool.stmt_mut(stmt_id).kind = StmtKind::While {
                condition: new_cond,
                body,
            };
        }
        StmtKind::For {
            init,
            condition,
            increment,
            body,
        } => {
            if let Some(init_id) = init {
                pool = optimize_stmt_id(init_id, pool, options);
            }
            let new_cond = if let Some(cond_id) = condition {
                let (new_cond, new_pool) = optimize_expr_id(cond_id, pool, options);
                pool = new_pool;
                Some(new_cond)
            } else {
                None
            };
            let new_inc = if let Some(inc_id) = increment {
                let (new_inc, new_pool) = optimize_expr_id(inc_id, pool, options);
                pool = new_pool;
                Some(new_inc)
            } else {
                None
            };
            pool = optimize_stmt_id(body, pool, options);
            pool.stmt_mut(stmt_id).kind = StmtKind::For {
                init,
                condition: new_cond,
                increment: new_inc,
                body,
            };
        }
    }

    pool
}
