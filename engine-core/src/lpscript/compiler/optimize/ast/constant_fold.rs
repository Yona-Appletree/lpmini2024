/// Constant folding optimization
///
/// Evaluates expressions with constant operands at compile time.
extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::lpscript::compiler::ast::{Expr, ExprKind};
use crate::lpscript::shared::Span;

// Import libm for math operations in no_std
use libm::{ceil, cos, floor, powf, sin, sqrt, tan};

/// Fold constants in an expression tree
pub fn fold_expr(expr: Expr) -> Expr {
    let span = expr.span;
    let ty = expr.ty.clone();

    let kind = match expr.kind {
        // Recursively fold subexpressions first
        ExprKind::Add(left, right) => {
            fold_binary_op(*left, *right, span, |a, b| a + b, ExprKind::Add)
        }
        ExprKind::Sub(left, right) => {
            fold_binary_op(*left, *right, span, |a, b| a - b, ExprKind::Sub)
        }
        ExprKind::Mul(left, right) => {
            fold_binary_op(*left, *right, span, |a, b| a * b, ExprKind::Mul)
        }
        ExprKind::Div(left, right) => fold_binary_op(
            *left,
            *right,
            span,
            |a, b| {
                if b == 0.0 {
                    f32::INFINITY
                } else {
                    a / b
                }
            },
            ExprKind::Div,
        ),
        ExprKind::Mod(left, right) => fold_binary_op(
            *left,
            *right,
            span,
            |a, b| {
                if b == 0.0 {
                    0.0
                } else {
                    a % b
                }
            },
            ExprKind::Mod,
        ),

        // Comparisons (return 1.0 for true, 0.0 for false)
        ExprKind::Less(left, right) => {
            fold_comparison(*left, *right, span, |a, b| a < b, ExprKind::Less)
        }
        ExprKind::Greater(left, right) => {
            fold_comparison(*left, *right, span, |a, b| a > b, ExprKind::Greater)
        }
        ExprKind::LessEq(left, right) => {
            fold_comparison(*left, *right, span, |a, b| a <= b, ExprKind::LessEq)
        }
        ExprKind::GreaterEq(left, right) => {
            fold_comparison(*left, *right, span, |a, b| a >= b, ExprKind::GreaterEq)
        }
        ExprKind::Eq(left, right) => {
            fold_comparison(*left, *right, span, |a, b| a == b, ExprKind::Eq)
        }
        ExprKind::NotEq(left, right) => {
            fold_comparison(*left, *right, span, |a, b| a != b, ExprKind::NotEq)
        }

        // Logical operations
        ExprKind::And(left, right) => fold_logical_and(*left, *right, span),
        ExprKind::Or(left, right) => fold_logical_or(*left, *right, span),
        ExprKind::Not(operand) => fold_not(*operand, span),

        // Unary negation
        ExprKind::Neg(operand) => fold_neg(*operand, span),

        // Ternary operator
        ExprKind::Ternary {
            condition,
            true_expr,
            false_expr,
        } => fold_ternary(*condition, *true_expr, *false_expr, span),

        // Function calls (fold arguments, and if all are constants, evaluate)
        ExprKind::Call { name, args } => fold_call(name, args, span),

        // Vector constructors (fold arguments)
        ExprKind::Vec2Constructor(args) => {
            let args = args.into_iter().map(fold_expr).collect();
            ExprKind::Vec2Constructor(args)
        }
        ExprKind::Vec3Constructor(args) => {
            let args = args.into_iter().map(fold_expr).collect();
            ExprKind::Vec3Constructor(args)
        }
        ExprKind::Vec4Constructor(args) => {
            let args = args.into_iter().map(fold_expr).collect();
            ExprKind::Vec4Constructor(args)
        }

        // Swizzle (fold inner expression)
        ExprKind::Swizzle { expr, components } => {
            let expr = Box::new(fold_expr(*expr));
            ExprKind::Swizzle { expr, components }
        }

        // Assignment (fold value)
        ExprKind::Assign { target, value } => {
            let value = Box::new(fold_expr(*value));
            ExprKind::Assign { target, value }
        }

        // Bitwise operations (fold operands, no constant folding for now)
        ExprKind::BitwiseAnd(left, right) => {
            let left = Box::new(fold_expr(*left));
            let right = Box::new(fold_expr(*right));
            ExprKind::BitwiseAnd(left, right)
        }
        ExprKind::BitwiseOr(left, right) => {
            let left = Box::new(fold_expr(*left));
            let right = Box::new(fold_expr(*right));
            ExprKind::BitwiseOr(left, right)
        }
        ExprKind::BitwiseXor(left, right) => {
            let left = Box::new(fold_expr(*left));
            let right = Box::new(fold_expr(*right));
            ExprKind::BitwiseXor(left, right)
        }
        ExprKind::BitwiseNot(operand) => {
            let operand = Box::new(fold_expr(*operand));
            ExprKind::BitwiseNot(operand)
        }
        ExprKind::LeftShift(left, right) => {
            let left = Box::new(fold_expr(*left));
            let right = Box::new(fold_expr(*right));
            ExprKind::LeftShift(left, right)
        }
        ExprKind::RightShift(left, right) => {
            let left = Box::new(fold_expr(*left));
            let right = Box::new(fold_expr(*right));
            ExprKind::RightShift(left, right)
        }

        // Increment/Decrement - no folding (they modify state)
        ExprKind::PreIncrement(_)
        | ExprKind::PreDecrement(_)
        | ExprKind::PostIncrement(_)
        | ExprKind::PostDecrement(_) => expr.kind,

        // Literals and variables - no folding needed
        ExprKind::Number(_) | ExprKind::IntNumber(_) | ExprKind::Variable(_) => expr.kind,
    };

    Expr { kind, span, ty }
}

/// Fold binary arithmetic operation
fn fold_binary_op<F, C>(left: Expr, right: Expr, _span: Span, op: F, constructor: C) -> ExprKind
where
    F: FnOnce(f32, f32) -> f32,
    C: FnOnce(Box<Expr>, Box<Expr>) -> ExprKind,
{
    let left = fold_expr(left);
    let right = fold_expr(right);

    match (&left.kind, &right.kind) {
        (ExprKind::Number(a), ExprKind::Number(b)) => ExprKind::Number(op(*a, *b)),
        (ExprKind::IntNumber(a), ExprKind::IntNumber(b)) => {
            // For integer operations, convert to float, compute, and convert back
            let result = op(*a as f32, *b as f32);
            ExprKind::IntNumber(result as i32)
        }
        _ => constructor(Box::new(left), Box::new(right)),
    }
}

/// Fold comparison operation
fn fold_comparison<F, C>(left: Expr, right: Expr, _span: Span, op: F, constructor: C) -> ExprKind
where
    F: FnOnce(f32, f32) -> bool,
    C: FnOnce(Box<Expr>, Box<Expr>) -> ExprKind,
{
    let left = fold_expr(left);
    let right = fold_expr(right);

    match (&left.kind, &right.kind) {
        (ExprKind::Number(a), ExprKind::Number(b)) => {
            ExprKind::Number(if op(*a, *b) { 1.0 } else { 0.0 })
        }
        (ExprKind::IntNumber(a), ExprKind::IntNumber(b)) => {
            ExprKind::Number(if op(*a as f32, *b as f32) { 1.0 } else { 0.0 })
        }
        _ => constructor(Box::new(left), Box::new(right)),
    }
}

/// Fold logical AND
fn fold_logical_and(left: Expr, right: Expr, _span: Span) -> ExprKind {
    let left = fold_expr(left);
    let right = fold_expr(right);

    // Check if left is a constant
    if let Some(left_val) = get_constant_bool(&left) {
        if !left_val {
            // false && x = false
            return ExprKind::Number(0.0);
        } else {
            // true && x = x
            return right.kind;
        }
    }

    // Check if right is a constant
    if let Some(right_val) = get_constant_bool(&right) {
        if !right_val {
            // x && false = false
            return ExprKind::Number(0.0);
        } else {
            // x && true = x
            return left.kind;
        }
    }

    ExprKind::And(Box::new(left), Box::new(right))
}

/// Fold logical OR
fn fold_logical_or(left: Expr, right: Expr, _span: Span) -> ExprKind {
    let left = fold_expr(left);
    let right = fold_expr(right);

    // Check if left is a constant
    if let Some(left_val) = get_constant_bool(&left) {
        if left_val {
            // true || x = true
            return ExprKind::Number(1.0);
        } else {
            // false || x = x
            return right.kind;
        }
    }

    // Check if right is a constant
    if let Some(right_val) = get_constant_bool(&right) {
        if right_val {
            // x || true = true
            return ExprKind::Number(1.0);
        } else {
            // x || false = x
            return left.kind;
        }
    }

    ExprKind::Or(Box::new(left), Box::new(right))
}

/// Fold logical NOT
fn fold_not(operand: Expr, _span: Span) -> ExprKind {
    let operand = fold_expr(operand);

    if let Some(val) = get_constant_bool(&operand) {
        return ExprKind::Number(if val { 0.0 } else { 1.0 });
    }

    ExprKind::Not(Box::new(operand))
}

/// Fold negation
fn fold_neg(operand: Expr, _span: Span) -> ExprKind {
    let operand = fold_expr(operand);

    match operand.kind {
        ExprKind::Number(x) => ExprKind::Number(-x),
        ExprKind::IntNumber(x) => ExprKind::IntNumber(-x),
        _ => ExprKind::Neg(Box::new(operand)),
    }
}

/// Fold ternary operator
fn fold_ternary(condition: Expr, true_expr: Expr, false_expr: Expr, _span: Span) -> ExprKind {
    let condition = fold_expr(condition);
    let true_expr = fold_expr(true_expr);
    let false_expr = fold_expr(false_expr);

    // If condition is constant, select the appropriate branch
    if let Some(cond_val) = get_constant_bool(&condition) {
        if cond_val {
            return true_expr.kind;
        } else {
            return false_expr.kind;
        }
    }

    ExprKind::Ternary {
        condition: Box::new(condition),
        true_expr: Box::new(true_expr),
        false_expr: Box::new(false_expr),
    }
}

/// Fold function calls with constant arguments
fn fold_call(name: alloc::string::String, args: Vec<Expr>, _span: Span) -> ExprKind {
    let args: Vec<_> = args.into_iter().map(fold_expr).collect();

    // Try to evaluate if all arguments are constant
    match name.as_str() {
        "sin" if args.len() == 1 => {
            if let ExprKind::Number(x) = args[0].kind {
                return ExprKind::Number(sin(x as f64) as f32);
            }
        }
        "cos" if args.len() == 1 => {
            if let ExprKind::Number(x) = args[0].kind {
                return ExprKind::Number(cos(x as f64) as f32);
            }
        }
        "tan" if args.len() == 1 => {
            if let ExprKind::Number(x) = args[0].kind {
                return ExprKind::Number(tan(x as f64) as f32);
            }
        }
        "abs" if args.len() == 1 => {
            if let ExprKind::Number(x) = args[0].kind {
                return ExprKind::Number(if x < 0.0 { -x } else { x });
            }
        }
        "floor" if args.len() == 1 => {
            if let ExprKind::Number(x) = args[0].kind {
                return ExprKind::Number(floor(x as f64) as f32);
            }
        }
        "ceil" if args.len() == 1 => {
            if let ExprKind::Number(x) = args[0].kind {
                return ExprKind::Number(ceil(x as f64) as f32);
            }
        }
        "sqrt" if args.len() == 1 => {
            if let ExprKind::Number(x) = args[0].kind {
                return ExprKind::Number(sqrt(x as f64) as f32);
            }
        }
        "sign" if args.len() == 1 => {
            if let ExprKind::Number(x) = args[0].kind {
                let s = if x > 0.0 {
                    1.0
                } else if x < 0.0 {
                    -1.0
                } else {
                    0.0
                };
                return ExprKind::Number(s);
            }
        }
        "saturate" if args.len() == 1 => {
            if let ExprKind::Number(x) = args[0].kind {
                let result = if x < 0.0 {
                    0.0
                } else if x > 1.0 {
                    1.0
                } else {
                    x
                };
                return ExprKind::Number(result);
            }
        }
        "min" if args.len() == 2 => {
            if let (ExprKind::Number(a), ExprKind::Number(b)) = (&args[0].kind, &args[1].kind) {
                return ExprKind::Number(if a < b { *a } else { *b });
            }
        }
        "max" if args.len() == 2 => {
            if let (ExprKind::Number(a), ExprKind::Number(b)) = (&args[0].kind, &args[1].kind) {
                return ExprKind::Number(if a > b { *a } else { *b });
            }
        }
        "pow" if args.len() == 2 => {
            if let (ExprKind::Number(a), ExprKind::Number(b)) = (&args[0].kind, &args[1].kind) {
                return ExprKind::Number(powf(*a, *b));
            }
        }
        "clamp" if args.len() == 3 => {
            if let (ExprKind::Number(x), ExprKind::Number(min_val), ExprKind::Number(max_val)) =
                (&args[0].kind, &args[1].kind, &args[2].kind)
            {
                let clamped = if *x < *min_val {
                    *min_val
                } else if *x > *max_val {
                    *max_val
                } else {
                    *x
                };
                return ExprKind::Number(clamped);
            }
        }
        "lerp" | "mix" if args.len() == 3 => {
            if let (ExprKind::Number(a), ExprKind::Number(b), ExprKind::Number(t)) =
                (&args[0].kind, &args[1].kind, &args[2].kind)
            {
                return ExprKind::Number(a + (b - a) * t);
            }
        }
        "step" if args.len() == 2 => {
            if let (ExprKind::Number(edge), ExprKind::Number(x)) = (&args[0].kind, &args[1].kind) {
                return ExprKind::Number(if x < edge { 0.0 } else { 1.0 });
            }
        }
        _ => {}
    }

    ExprKind::Call { name, args }
}

/// Get boolean value from constant expression (0.0 = false, non-zero = true)
fn get_constant_bool(expr: &Expr) -> Option<bool> {
    match &expr.kind {
        ExprKind::Number(x) => Some(*x != 0.0),
        ExprKind::IntNumber(x) => Some(*x != 0),
        _ => None,
    }
}
