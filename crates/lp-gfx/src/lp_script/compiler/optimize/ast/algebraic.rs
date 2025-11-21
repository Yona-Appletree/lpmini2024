/// Algebraic simplification
///
/// Applies algebraic identities to simplify expressions.
extern crate alloc;

use crate::lp_script::compiler::ast::{Expr, ExprKind};

/// Simplify an expression tree using algebraic identities
/// Returns true if the expression was modified
pub fn simplify_expr(expr: &mut Expr) -> bool {
    let mut changed = false;

    // First, recursively simplify children
    match &mut expr.kind {
        ExprKind::Add(left, right)
        | ExprKind::Sub(left, right)
        | ExprKind::Mul(left, right)
        | ExprKind::Div(left, right)
        | ExprKind::Mod(left, right) => {
            changed |= simplify_expr(left.as_mut());
            changed |= simplify_expr(right.as_mut());
        }
        ExprKind::Neg(operand) => {
            changed |= simplify_expr(operand.as_mut());
        }
        _ => {}
    }

    // Now apply algebraic simplifications
    match &expr.kind {
        // x + 0 = x, 0 + x = x
        ExprKind::Add(left, right) => {
            if matches!(&left.kind, ExprKind::Number(n) if *n == 0.0) {
                *expr = (**right).clone();
                return true;
            }
            if matches!(&right.kind, ExprKind::Number(n) if *n == 0.0) {
                *expr = (**left).clone();
                return true;
            }
        }

        // x * 1 = x, 1 * x = x
        // x * 0 = 0, 0 * x = 0
        ExprKind::Mul(left, right) => {
            if matches!(&left.kind, ExprKind::Number(n) if *n == 0.0) {
                *expr = (**left).clone(); // 0 * x = 0
                return true;
            }
            if matches!(&right.kind, ExprKind::Number(n) if *n == 0.0) {
                *expr = (**right).clone(); // x * 0 = 0
                return true;
            }
            if matches!(&left.kind, ExprKind::Number(n) if *n == 1.0) {
                *expr = (**right).clone(); // 1 * x = x
                return true;
            }
            if matches!(&right.kind, ExprKind::Number(n) if *n == 1.0) {
                *expr = (**left).clone(); // x * 1 = x
                return true;
            }
        }

        // x - 0 = x
        ExprKind::Sub(left, right) => {
            if matches!(&right.kind, ExprKind::Number(n) if *n == 0.0) {
                *expr = (**left).clone();
                return true;
            }
        }

        // x / 1 = x
        ExprKind::Div(left, right) => {
            if matches!(&right.kind, ExprKind::Number(n) if *n == 1.0) {
                *expr = (**left).clone();
                return true;
            }
        }

        // --x = x (double negation)
        ExprKind::Neg(operand) => {
            if let ExprKind::Neg(inner) = &operand.kind {
                *expr = (**inner).clone();
                return true;
            }
        }

        _ => {}
    }

    changed
}
