/// Constant folding optimization (LpBox AST)
///
/// TODO: Re-implement full constant folding for the new recursive AST. For now
/// this pass simply traverses the expression tree and returns whether a change
/// was made. No actual folding is performed yet.
use crate::compiler::ast::{Expr, ExprKind};

/// Fold constants in an expression tree. Returns `true` if any change was made.
pub fn fold_constants(expr: &mut Expr) -> bool {
    use ExprKind::*;

    let mut changed = false;

    match &mut expr.kind {
        Add(left, right)
        | Sub(left, right)
        | Mul(left, right)
        | Div(left, right)
        | Mod(left, right)
        | BitwiseAnd(left, right)
        | BitwiseOr(left, right)
        | BitwiseXor(left, right)
        | LeftShift(left, right)
        | RightShift(left, right)
        | Less(left, right)
        | Greater(left, right)
        | LessEq(left, right)
        | GreaterEq(left, right)
        | Eq(left, right)
        | NotEq(left, right)
        | And(left, right)
        | Or(left, right) => {
            changed |= fold_constants(left.as_mut());
            changed |= fold_constants(right.as_mut());
        }

        Neg(operand) | BitwiseNot(operand) | Not(operand) => {
            changed |= fold_constants(operand.as_mut());
        }

        Ternary {
            condition,
            true_expr,
            false_expr,
        } => {
            changed |= fold_constants(condition.as_mut());
            changed |= fold_constants(true_expr.as_mut());
            changed |= fold_constants(false_expr.as_mut());
        }

        Assign { value, .. } => {
            changed |= fold_constants(value.as_mut());
        }

        Call { args, .. }
        | Vec2Constructor(args)
        | Vec3Constructor(args)
        | Vec4Constructor(args) => {
            for arg in args {
                changed |= fold_constants(arg);
            }
        }

        Swizzle { expr: inner, .. } => {
            changed |= fold_constants(inner.as_mut());
        }

        Number(_) | IntNumber(_) | Variable(_) | PreIncrement(_) | PreDecrement(_)
        | PostIncrement(_) | PostDecrement(_) => {}
    }

    changed
}
