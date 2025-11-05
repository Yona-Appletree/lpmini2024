/// Algebraic simplification optimization
///
/// Applies algebraic identities to simplify expressions:
/// - x + 0 = x, x - 0 = x
/// - x * 1 = x, x / 1 = x
/// - x * 0 = 0, 0 * x = 0
/// - x && true = x, x && false = false
/// - x || true = true, x || false = x
/// - !(!(x)) = x
extern crate alloc;
use alloc::boxed::Box;

use crate::lpscript::compiler::ast::{Expr, ExprKind};

/// Simplify an expression using algebraic identities
pub fn simplify_expr(expr: Expr) -> Expr {
    let span = expr.span;
    let ty = expr.ty.clone();

    let kind = match expr.kind {
        // Addition: x + 0 = x, 0 + x = x
        ExprKind::Add(left, right) => {
            let left = simplify_expr(*left);
            let right = simplify_expr(*right);

            if is_zero(&right) {
                left.kind
            } else if is_zero(&left) {
                right.kind
            } else {
                ExprKind::Add(Box::new(left), Box::new(right))
            }
        }

        // Subtraction: x - 0 = x
        ExprKind::Sub(left, right) => {
            let left = simplify_expr(*left);
            let right = simplify_expr(*right);

            if is_zero(&right) {
                left.kind
            } else {
                ExprKind::Sub(Box::new(left), Box::new(right))
            }
        }

        // Multiplication: x * 1 = x, 1 * x = x, x * 0 = 0, 0 * x = 0
        ExprKind::Mul(left, right) => {
            let left = simplify_expr(*left);
            let right = simplify_expr(*right);

            if is_zero(&left) || is_zero(&right) {
                ExprKind::Number(0.0)
            } else if is_one(&right) {
                left.kind
            } else if is_one(&left) {
                right.kind
            } else {
                ExprKind::Mul(Box::new(left), Box::new(right))
            }
        }

        // Division: x / 1 = x
        ExprKind::Div(left, right) => {
            let left = simplify_expr(*left);
            let right = simplify_expr(*right);

            if is_one(&right) {
                left.kind
            } else {
                ExprKind::Div(Box::new(left), Box::new(right))
            }
        }

        // Modulo: x % 1 = 0 (any number mod 1 is 0)
        ExprKind::Mod(left, right) => {
            let left = simplify_expr(*left);
            let right = simplify_expr(*right);

            if is_one(&right) {
                ExprKind::Number(0.0)
            } else {
                ExprKind::Mod(Box::new(left), Box::new(right))
            }
        }

        // Logical AND: x && true = x, x && false = false, true && x = x, false && x = false
        ExprKind::And(left, right) => {
            let left = simplify_expr(*left);
            let right = simplify_expr(*right);

            if is_true(&right) {
                left.kind
            } else if is_false(&right) {
                ExprKind::Number(0.0)
            } else if is_true(&left) {
                right.kind
            } else if is_false(&left) {
                ExprKind::Number(0.0)
            } else {
                ExprKind::And(Box::new(left), Box::new(right))
            }
        }

        // Logical OR: x || true = true, x || false = x, true || x = true, false || x = x
        ExprKind::Or(left, right) => {
            let left = simplify_expr(*left);
            let right = simplify_expr(*right);

            if is_true(&right) {
                ExprKind::Number(1.0)
            } else if is_false(&right) {
                left.kind
            } else if is_true(&left) {
                ExprKind::Number(1.0)
            } else if is_false(&left) {
                right.kind
            } else {
                ExprKind::Or(Box::new(left), Box::new(right))
            }
        }

        // Double negation: !(!x) = x
        ExprKind::Not(operand) => {
            let operand = simplify_expr(*operand);

            if let ExprKind::Not(inner) = operand.kind {
                inner.kind
            } else {
                ExprKind::Not(Box::new(operand))
            }
        }

        // Negation: -(-x) = x
        ExprKind::Neg(operand) => {
            let operand = simplify_expr(*operand);

            if let ExprKind::Neg(inner) = operand.kind {
                inner.kind
            } else {
                ExprKind::Neg(Box::new(operand))
            }
        }

        // Ternary: simplify subexpressions
        ExprKind::Ternary {
            condition,
            true_expr,
            false_expr,
        } => {
            let condition = Box::new(simplify_expr(*condition));
            let true_expr = Box::new(simplify_expr(*true_expr));
            let false_expr = Box::new(simplify_expr(*false_expr));
            ExprKind::Ternary {
                condition,
                true_expr,
                false_expr,
            }
        }

        // Function calls: simplify arguments
        ExprKind::Call { name, args } => {
            let args: alloc::vec::Vec<_> = args.into_iter().map(simplify_expr).collect();

            // Special case: normalize(normalize(x)) = normalize(x)
            if name == "normalize" && args.len() == 1 {
                if let ExprKind::Call {
                    name: ref inner_name,
                    ..
                } = args[0].kind
                {
                    if inner_name == "normalize" {
                        return args.into_iter().next().unwrap();
                    }
                }
            }

            ExprKind::Call { name, args }
        }

        // Vector constructors: simplify arguments
        ExprKind::Vec2Constructor(args) => {
            let args = args.into_iter().map(simplify_expr).collect();
            ExprKind::Vec2Constructor(args)
        }
        ExprKind::Vec3Constructor(args) => {
            let args = args.into_iter().map(simplify_expr).collect();
            ExprKind::Vec3Constructor(args)
        }
        ExprKind::Vec4Constructor(args) => {
            let args = args.into_iter().map(simplify_expr).collect();
            ExprKind::Vec4Constructor(args)
        }

        // Swizzle: simplify inner expression
        ExprKind::Swizzle { expr, components } => {
            let expr = Box::new(simplify_expr(*expr));
            ExprKind::Swizzle { expr, components }
        }

        // Assignment: simplify value
        ExprKind::Assign { target, value } => {
            let value = Box::new(simplify_expr(*value));
            ExprKind::Assign { target, value }
        }

        // Comparisons: simplify subexpressions
        ExprKind::Less(left, right) => {
            let left = Box::new(simplify_expr(*left));
            let right = Box::new(simplify_expr(*right));
            ExprKind::Less(left, right)
        }
        ExprKind::Greater(left, right) => {
            let left = Box::new(simplify_expr(*left));
            let right = Box::new(simplify_expr(*right));
            ExprKind::Greater(left, right)
        }
        ExprKind::LessEq(left, right) => {
            let left = Box::new(simplify_expr(*left));
            let right = Box::new(simplify_expr(*right));
            ExprKind::LessEq(left, right)
        }
        ExprKind::GreaterEq(left, right) => {
            let left = Box::new(simplify_expr(*left));
            let right = Box::new(simplify_expr(*right));
            ExprKind::GreaterEq(left, right)
        }
        ExprKind::Eq(left, right) => {
            let left = Box::new(simplify_expr(*left));
            let right = Box::new(simplify_expr(*right));
            ExprKind::Eq(left, right)
        }
        ExprKind::NotEq(left, right) => {
            let left = Box::new(simplify_expr(*left));
            let right = Box::new(simplify_expr(*right));
            ExprKind::NotEq(left, right)
        }

        // Bitwise operations: simplify subexpressions
        ExprKind::BitwiseAnd(left, right) => {
            let left = Box::new(simplify_expr(*left));
            let right = Box::new(simplify_expr(*right));
            ExprKind::BitwiseAnd(left, right)
        }
        ExprKind::BitwiseOr(left, right) => {
            let left = Box::new(simplify_expr(*left));
            let right = Box::new(simplify_expr(*right));
            ExprKind::BitwiseOr(left, right)
        }
        ExprKind::BitwiseXor(left, right) => {
            let left = Box::new(simplify_expr(*left));
            let right = Box::new(simplify_expr(*right));
            ExprKind::BitwiseXor(left, right)
        }
        ExprKind::BitwiseNot(operand) => {
            let operand = Box::new(simplify_expr(*operand));
            ExprKind::BitwiseNot(operand)
        }
        ExprKind::LeftShift(left, right) => {
            let left = Box::new(simplify_expr(*left));
            let right = Box::new(simplify_expr(*right));
            ExprKind::LeftShift(left, right)
        }
        ExprKind::RightShift(left, right) => {
            let left = Box::new(simplify_expr(*left));
            let right = Box::new(simplify_expr(*right));
            ExprKind::RightShift(left, right)
        }

        // Increment/Decrement - no simplification (they modify state)
        ExprKind::PreIncrement(_)
        | ExprKind::PreDecrement(_)
        | ExprKind::PostIncrement(_)
        | ExprKind::PostDecrement(_) => expr.kind,

        // Literals and variables: no simplification
        ExprKind::Number(_) | ExprKind::IntNumber(_) | ExprKind::Variable(_) => expr.kind,
    };

    Expr { kind, span, ty }
}

/// Check if expression is zero (0.0 or 0)
fn is_zero(expr: &Expr) -> bool {
    match &expr.kind {
        ExprKind::Number(x) => *x == 0.0,
        ExprKind::IntNumber(x) => *x == 0,
        _ => false,
    }
}

/// Check if expression is one (1.0 or 1)
fn is_one(expr: &Expr) -> bool {
    match &expr.kind {
        ExprKind::Number(x) => *x == 1.0,
        ExprKind::IntNumber(x) => *x == 1,
        _ => false,
    }
}

/// Check if expression is true (non-zero)
fn is_true(expr: &Expr) -> bool {
    match &expr.kind {
        ExprKind::Number(x) => *x == 1.0, // In our system, 1.0 is true
        _ => false,
    }
}

/// Check if expression is false (zero)
fn is_false(expr: &Expr) -> bool {
    match &expr.kind {
        ExprKind::Number(x) => *x == 0.0,
        ExprKind::IntNumber(x) => *x == 0,
        _ => false,
    }
}
