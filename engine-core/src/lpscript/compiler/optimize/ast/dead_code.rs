/// Dead code elimination
///
/// Removes unreachable code:
/// - Statements after return
/// - Branches of if statements with constant conditions
/// - Empty blocks
extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::lpscript::compiler::ast::{Expr, ExprKind, Stmt, StmtKind};
use crate::lpscript::shared::Span;

/// Eliminate dead statements from a block
///
/// Removes all statements after the first return statement
pub fn eliminate_dead_stmts(stmts: Vec<Stmt>) -> Vec<Stmt> {
    let mut result = Vec::new();

    for stmt in stmts {
        let has_return = contains_return(&stmt);
        result.push(stmt);

        // Stop adding statements after a return
        if has_return {
            break;
        }
    }

    result
}

/// Check if a statement contains a return (or always returns)
fn contains_return(stmt: &Stmt) -> bool {
    match &stmt.kind {
        StmtKind::Return(_) => true,
        StmtKind::Block(stmts) => {
            // Block returns if any statement in it returns
            stmts.iter().any(contains_return)
        }
        StmtKind::If {
            then_stmt,
            else_stmt,
            ..
        } => {
            // If returns if both branches return
            if let Some(else_stmt) = else_stmt {
                contains_return(then_stmt) && contains_return(else_stmt)
            } else {
                false
            }
        }
        _ => false,
    }
}

/// Simplify if statement with constant condition
pub fn simplify_if(
    condition: Expr,
    then_stmt: Box<Stmt>,
    else_stmt: Option<Box<Stmt>>,
    span: Span,
) -> Stmt {
    // Check if condition is a constant
    if let Some(cond_val) = get_constant_bool(&condition) {
        if cond_val {
            // Condition is true, return then branch
            return *then_stmt;
        } else {
            // Condition is false, return else branch or empty block
            return if let Some(else_stmt) = else_stmt {
                *else_stmt
            } else {
                Stmt::new(StmtKind::Block(Vec::new()), span)
            };
        }
    }

    // Condition is not constant, return original if statement
    Stmt::new(
        StmtKind::If {
            condition,
            then_stmt,
            else_stmt,
        },
        span,
    )
}

/// Get boolean value from constant expression
fn get_constant_bool(expr: &Expr) -> Option<bool> {
    match &expr.kind {
        ExprKind::Number(x) => Some(*x != 0.0),
        ExprKind::IntNumber(x) => Some(*x != 0),
        _ => None,
    }
}


