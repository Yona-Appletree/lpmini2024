/// Algebraic simplification (pool-based API)
///
/// Applies algebraic identities to simplify expressions.
extern crate alloc;

use crate::lpscript::compiler::ast::{AstPool, ExprId, ExprKind};

/// Simplify an expression tree using algebraic identities
pub fn simplify_expr(expr_id: ExprId, pool: AstPool) -> (ExprId, AstPool) {
    let kind = pool.expr(expr_id).kind.clone();
    let span = pool.expr(expr_id).span;

    match kind {
        // x + 0 = x, 0 + x = x
        ExprKind::Add(left_id, right_id) => {
            let (new_left, mut pool2) = simplify_expr(left_id, pool);
            let (new_right, mut pool3) = simplify_expr(right_id, pool2);

            let left = &pool3.expr(new_left).kind;
            let right = &pool3.expr(new_right).kind;

            if matches!(left, &ExprKind::Number(n) if n == 0.0) {
                return (new_right, pool3);
            }
            if matches!(right, &ExprKind::Number(n) if n == 0.0) {
                return (new_left, pool3);
            }

            pool3.expr_mut(expr_id).kind = ExprKind::Add(new_left, new_right);
            (expr_id, pool3)
        }

        // x * 1 = x, 1 * x = x
        // x * 0 = 0, 0 * x = 0
        ExprKind::Mul(left_id, right_id) => {
            let (new_left, mut pool2) = simplify_expr(left_id, pool);
            let (new_right, mut pool3) = simplify_expr(right_id, pool2);

            let left = &pool3.expr(new_left).kind;
            let right = &pool3.expr(new_right).kind;

            if matches!(left, &ExprKind::Number(n) if n == 0.0) {
                return (new_left, pool3); // 0 * x = 0
            }
            if matches!(right, &ExprKind::Number(n) if n == 0.0) {
                return (new_right, pool3); // x * 0 = 0
            }
            if matches!(left, &ExprKind::Number(n) if n == 1.0) {
                return (new_right, pool3); // 1 * x = x
            }
            if matches!(right, &ExprKind::Number(n) if n == 1.0) {
                return (new_left, pool3); // x * 1 = x
            }

            pool3.expr_mut(expr_id).kind = ExprKind::Mul(new_left, new_right);
            (expr_id, pool3)
        }

        // x - 0 = x
        ExprKind::Sub(left_id, right_id) => {
            let (new_left, mut pool2) = simplify_expr(left_id, pool);
            let (new_right, mut pool3) = simplify_expr(right_id, pool2);

            let right = &pool3.expr(new_right).kind;
            if matches!(right, &ExprKind::Number(n) if n == 0.0) {
                return (new_left, pool3);
            }

            pool3.expr_mut(expr_id).kind = ExprKind::Sub(new_left, new_right);
            (expr_id, pool3)
        }

        // x / 1 = x
        ExprKind::Div(left_id, right_id) => {
            let (new_left, mut pool2) = simplify_expr(left_id, pool);
            let (new_right, mut pool3) = simplify_expr(right_id, pool2);

            let right = &pool3.expr(new_right).kind;
            if matches!(right, &ExprKind::Number(n) if n == 1.0) {
                return (new_left, pool3);
            }

            pool3.expr_mut(expr_id).kind = ExprKind::Div(new_left, new_right);
            (expr_id, pool3)
        }

        // Recursively simplify other expressions
        // --x = x (double negation)
        ExprKind::Neg(operand_id) => {
            let (new_operand, mut pool2) = simplify_expr(operand_id, pool);

            // Check if operand is also a negation
            if let ExprKind::Neg(inner_id) = pool2.expr(new_operand).kind {
                return (inner_id, pool2); // --x = x
            }

            pool2.expr_mut(expr_id).kind = ExprKind::Neg(new_operand);
            (expr_id, pool2)
        }

        // x % 1 = frac(x) for fractional numbers
        // Note: We don't optimize this because:
        // 1. For integers: x % 1 = 0 (simplified)
        // 2. For floats: x % 1 = frac(x) (not a simple constant)
        // The simplification would need to generate a frac() call, which isn't simpler
        ExprKind::Mod(left_id, right_id) => {
            let (new_left, mut pool2) = simplify_expr(left_id, pool);
            let (new_right, mut pool3) = simplify_expr(right_id, pool2);

            pool3.expr_mut(expr_id).kind = ExprKind::Mod(new_left, new_right);
            (expr_id, pool3)
        }

        // For all other expressions, return as-is
        _ => (expr_id, pool),
    }
}
