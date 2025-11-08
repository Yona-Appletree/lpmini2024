use lp_math::fixed::{
    ceil as fixed_ceil, cos as fixed_cos, floor as fixed_floor, lerp as fixed_lerp,
    pow as fixed_pow, saturate as fixed_saturate, sin as fixed_sin, sqrt as fixed_sqrt, Fixed,
};

/// Constant folding optimization (LpBox AST)
///
/// TODO: Re-implement full constant folding for the new recursive AST. For now
/// this pass simply traverses the expression tree and returns whether a change
/// was made. No actual folding is performed yet.
use crate::compiler::ast::{Expr, ExprKind};
use crate::shared::Type;

#[derive(Clone, Copy, Debug, PartialEq)]
enum ConstValue {
    Float(f32),
    Int(i32),
    Bool(bool),
}

impl ConstValue {
    fn as_float(self) -> f32 {
        match self {
            ConstValue::Float(v) => v,
            ConstValue::Int(v) => v as f32,
            ConstValue::Bool(v) => {
                if v {
                    1.0
                } else {
                    0.0
                }
            }
        }
    }

    fn as_int(self) -> Option<i32> {
        match self {
            ConstValue::Int(v) => Some(v),
            _ => None,
        }
    }

    fn as_fixed(self) -> Fixed {
        match self {
            ConstValue::Float(v) => Fixed::from_f32(v),
            ConstValue::Int(v) => Fixed::from_i32(v),
            ConstValue::Bool(v) => {
                if v {
                    Fixed::ONE
                } else {
                    Fixed::ZERO
                }
            }
        }
    }

    fn truthy(self) -> bool {
        match self {
            ConstValue::Bool(v) => v,
            ConstValue::Int(v) => v != 0,
            ConstValue::Float(v) => v != 0.0,
        }
    }
}

fn const_value(expr: &Expr) -> Option<ConstValue> {
    match &expr.kind {
        ExprKind::Number(n) => match expr.ty {
            Some(Type::Bool) => Some(ConstValue::Bool(*n != 0.0)),
            Some(Type::Int32) => Some(ConstValue::Int(*n as i32)),
            _ => Some(ConstValue::Float(*n)),
        },
        ExprKind::IntNumber(i) => {
            if matches!(expr.ty, Some(Type::Bool)) {
                Some(ConstValue::Bool(*i != 0))
            } else {
                Some(ConstValue::Int(*i))
            }
        }
        _ => None,
    }
}

#[derive(Clone)]
struct FoldReplacement {
    kind: ExprKind,
    ty: Option<Type>,
    keep_existing_ty: bool,
}

impl FoldReplacement {
    fn new(kind: ExprKind, ty: Option<Type>, keep_existing_ty: bool) -> Self {
        FoldReplacement {
            kind,
            ty,
            keep_existing_ty,
        }
    }

    fn apply(self, expr: &mut Expr) {
        expr.kind = self.kind;
        if self.keep_existing_ty && expr.ty.is_some() {
            return;
        }
        expr.ty = self.ty;
    }
}

fn replacement_number(value: f32, keep_existing_ty: bool) -> FoldReplacement {
    FoldReplacement::new(ExprKind::Number(value), Some(Type::Fixed), keep_existing_ty)
}

fn replacement_int(value: i32, keep_existing_ty: bool) -> FoldReplacement {
    FoldReplacement::new(
        ExprKind::IntNumber(value),
        Some(Type::Int32),
        keep_existing_ty,
    )
}

fn replacement_bool(value: bool) -> FoldReplacement {
    FoldReplacement::new(
        ExprKind::Number(if value { 1.0 } else { 0.0 }),
        Some(Type::Bool),
        false,
    )
}

fn fold_call(name: &str, args: &mut [Expr], keep_existing_ty: bool) -> Option<FoldReplacement> {
    match name {
        "sin" if args.len() == 1 => {
            let value = const_value(&args[0])?;
            let result = fixed_sin(value.as_fixed());
            Some(replacement_number(result.to_f32(), keep_existing_ty))
        }
        "cos" if args.len() == 1 => {
            let value = const_value(&args[0])?;
            let result = fixed_cos(value.as_fixed());
            Some(replacement_number(result.to_f32(), keep_existing_ty))
        }
        "sqrt" if args.len() == 1 => {
            let value = const_value(&args[0])?;
            let result = fixed_sqrt(value.as_fixed());
            Some(replacement_number(result.to_f32(), keep_existing_ty))
        }
        "abs" if args.len() == 1 => {
            let value = const_value(&args[0])?;
            if let Some(v) = value.as_int() {
                Some(replacement_int(v.abs(), keep_existing_ty))
            } else {
                let result = value.as_fixed().abs();
                Some(replacement_number(result.to_f32(), keep_existing_ty))
            }
        }
        "floor" if args.len() == 1 => {
            let value = const_value(&args[0])?;
            let result = fixed_floor(value.as_fixed());
            Some(replacement_number(result.to_f32(), keep_existing_ty))
        }
        "ceil" if args.len() == 1 => {
            let value = const_value(&args[0])?;
            let result = fixed_ceil(value.as_fixed());
            Some(replacement_number(result.to_f32(), keep_existing_ty))
        }
        "min" if args.len() == 2 => {
            let left = const_value(&args[0])?;
            let right = const_value(&args[1])?;
            if let (Some(l), Some(r)) = (left.as_int(), right.as_int()) {
                Some(replacement_int(l.min(r), keep_existing_ty))
            } else {
                let result = left.as_fixed().min(right.as_fixed());
                Some(replacement_number(result.to_f32(), keep_existing_ty))
            }
        }
        "max" if args.len() == 2 => {
            let left = const_value(&args[0])?;
            let right = const_value(&args[1])?;
            if let (Some(l), Some(r)) = (left.as_int(), right.as_int()) {
                Some(replacement_int(l.max(r), keep_existing_ty))
            } else {
                let result = left.as_fixed().max(right.as_fixed());
                Some(replacement_number(result.to_f32(), keep_existing_ty))
            }
        }
        "clamp" if args.len() == 3 => {
            let x = const_value(&args[0])?.as_fixed();
            let min_val = const_value(&args[1])?.as_fixed();
            let max_val = const_value(&args[2])?.as_fixed();
            let result = x.clamp(min_val, max_val);
            Some(replacement_number(result.to_f32(), keep_existing_ty))
        }
        "pow" if args.len() == 2 => {
            let base = const_value(&args[0])?.as_fixed();
            let exponent = const_value(&args[1])?.as_fixed();
            let exp_int = exponent.to_i32();
            let result = fixed_pow(base, exp_int);
            Some(replacement_number(result.to_f32(), keep_existing_ty))
        }
        "lerp" | "mix" if args.len() == 3 => {
            let a = const_value(&args[0])?.as_fixed();
            let b = const_value(&args[1])?.as_fixed();
            let t = const_value(&args[2])?.as_fixed();
            let result = fixed_lerp(a, b, t);
            Some(replacement_number(result.to_f32(), keep_existing_ty))
        }
        "saturate" if args.len() == 1 => {
            let value = const_value(&args[0])?.as_fixed();
            let result = fixed_saturate(value);
            Some(replacement_number(result.to_f32(), keep_existing_ty))
        }
        _ => None,
    }
}

fn fold_ternary(condition: &Expr, then_expr: &Expr, else_expr: &Expr) -> Option<FoldReplacement> {
    let cond = const_value(condition)?;
    let selected = if cond.truthy() { then_expr } else { else_expr };
    Some(FoldReplacement::new(
        selected.kind.clone(),
        selected.ty.clone(),
        false,
    ))
}

fn fold_binary_numeric(
    left: &Expr,
    right: &Expr,
    op: fn(f32, f32) -> f32,
    int_op: Option<fn(i32, i32) -> i32>,
    keep_existing_ty: bool,
) -> Option<FoldReplacement> {
    let left_val = const_value(left)?;
    let right_val = const_value(right)?;

    if let (Some(op_int), Some(l), Some(r)) = (int_op, left_val.as_int(), right_val.as_int()) {
        Some(replacement_int(op_int(l, r), keep_existing_ty))
    } else {
        Some(replacement_number(
            op(left_val.as_float(), right_val.as_float()),
            keep_existing_ty,
        ))
    }
}

fn fold_binary_int(
    left: &Expr,
    right: &Expr,
    op: fn(i32, i32) -> i32,
    keep_existing_ty: bool,
) -> Option<FoldReplacement> {
    let left_val = const_value(left)?.as_int()?;
    let right_val = const_value(right)?.as_int()?;
    Some(replacement_int(op(left_val, right_val), keep_existing_ty))
}

fn fold_binary_bool(
    left: &Expr,
    right: &Expr,
    op: fn(bool, bool) -> bool,
) -> Option<FoldReplacement> {
    let left_val = const_value(left)?;
    let right_val = const_value(right)?;
    Some(replacement_bool(op(left_val.truthy(), right_val.truthy())))
}

fn fold_compare(left: &Expr, right: &Expr, cmp: fn(f32, f32) -> bool) -> Option<FoldReplacement> {
    let left_val = const_value(left)?.as_float();
    let right_val = const_value(right)?.as_float();
    Some(replacement_bool(cmp(left_val, right_val)))
}

fn fold_equality(left: &Expr, right: &Expr, expected_equal: bool) -> Option<FoldReplacement> {
    let left_val = const_value(left)?;
    let right_val = const_value(right)?;
    let equal = match (left_val, right_val) {
        (ConstValue::Int(a), ConstValue::Int(b)) => a == b,
        (ConstValue::Float(a), ConstValue::Float(b)) => a == b,
        (a, b) => a.as_float() == b.as_float(),
    };
    Some(replacement_bool(if expected_equal {
        equal
    } else {
        !equal
    }))
}

fn fold_unary_numeric(
    operand: &Expr,
    op: fn(f32) -> f32,
    int_op: Option<fn(i32) -> i32>,
    keep_existing_ty: bool,
) -> Option<FoldReplacement> {
    let operand_val = const_value(operand)?;
    if let (Some(op_int), Some(v)) = (int_op, operand_val.as_int()) {
        Some(replacement_int(op_int(v), keep_existing_ty))
    } else {
        Some(replacement_number(
            op(operand_val.as_float()),
            keep_existing_ty,
        ))
    }
}

fn fold_not(operand: &Expr) -> Option<FoldReplacement> {
    let operand_val = const_value(operand)?;
    Some(replacement_bool(!operand_val.truthy()))
}

fn fold_bitwise_not(operand: &Expr, keep_existing_ty: bool) -> Option<FoldReplacement> {
    let operand_val = const_value(operand)?.as_int()?;
    Some(replacement_int(!operand_val, keep_existing_ty))
}

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
            for arg in args.iter_mut() {
                changed |= fold_constants(arg);
            }
        }
        Swizzle { expr: inner, .. } => {
            changed |= fold_constants(inner.as_mut());
        }
        Number(_) | IntNumber(_) | Variable(_) | PreIncrement(_) | PreDecrement(_)
        | PostIncrement(_) | PostDecrement(_) => {}
    }

    let replacement = match &mut expr.kind {
        Add(left, right) => fold_binary_numeric(
            left.as_ref(),
            right.as_ref(),
            |a, b| a + b,
            Some(|a, b| a + b),
            true,
        ),
        Sub(left, right) => fold_binary_numeric(
            left.as_ref(),
            right.as_ref(),
            |a, b| a - b,
            Some(|a, b| a - b),
            true,
        ),
        Mul(left, right) => fold_binary_numeric(
            left.as_ref(),
            right.as_ref(),
            |a, b| a * b,
            Some(|a, b| a * b),
            true,
        ),
        Div(left, right) => {
            if let Some(denom) = const_value(right.as_ref()) {
                let zero = match denom {
                    ConstValue::Int(v) => v == 0,
                    _ => denom.as_float() == 0.0,
                };
                if zero {
                    None
                } else {
                    fold_binary_numeric(
                        left.as_ref(),
                        right.as_ref(),
                        |a, b| a / b,
                        Some(|a, b| a / b),
                        true,
                    )
                }
            } else {
                None
            }
        }
        Mod(left, right) => {
            let left_val = const_value(left.as_ref());
            let right_val = const_value(right.as_ref());
            match (left_val, right_val) {
                (Some(ConstValue::Int(a)), Some(ConstValue::Int(b))) if b != 0 => {
                    Some(replacement_int(a % b, true))
                }
                (Some(l), Some(r)) => {
                    let divisor = r.as_float();
                    if divisor == 0.0 {
                        None
                    } else {
                        Some(replacement_number(l.as_float() % divisor, true))
                    }
                }
                _ => None,
            }
        }
        BitwiseAnd(left, right) => {
            fold_binary_int(left.as_ref(), right.as_ref(), |a, b| a & b, true)
        }
        BitwiseOr(left, right) => {
            fold_binary_int(left.as_ref(), right.as_ref(), |a, b| a | b, true)
        }
        BitwiseXor(left, right) => {
            fold_binary_int(left.as_ref(), right.as_ref(), |a, b| a ^ b, true)
        }
        LeftShift(left, right) => fold_binary_int(
            left.as_ref(),
            right.as_ref(),
            |a, b| a.wrapping_shl(b as u32),
            true,
        ),
        RightShift(left, right) => fold_binary_int(
            left.as_ref(),
            right.as_ref(),
            |a, b| a.wrapping_shr(b as u32),
            true,
        ),
        Less(left, right) => fold_compare(left.as_ref(), right.as_ref(), |a, b| a < b),
        Greater(left, right) => fold_compare(left.as_ref(), right.as_ref(), |a, b| a > b),
        LessEq(left, right) => fold_compare(left.as_ref(), right.as_ref(), |a, b| a <= b),
        GreaterEq(left, right) => fold_compare(left.as_ref(), right.as_ref(), |a, b| a >= b),
        Eq(left, right) => fold_equality(left.as_ref(), right.as_ref(), true),
        NotEq(left, right) => fold_equality(left.as_ref(), right.as_ref(), false),
        And(left, right) => fold_binary_bool(left.as_ref(), right.as_ref(), |a, b| a && b),
        Or(left, right) => fold_binary_bool(left.as_ref(), right.as_ref(), |a, b| a || b),
        Neg(operand) => fold_unary_numeric(operand.as_ref(), |a| -a, Some(|a| -a), true),
        BitwiseNot(operand) => fold_bitwise_not(operand.as_ref(), true),
        Not(operand) => fold_not(operand.as_ref()),
        Ternary {
            condition,
            true_expr,
            false_expr,
        } => fold_ternary(condition.as_ref(), true_expr.as_ref(), false_expr.as_ref()),
        Assign { .. } => None,
        Call { name, args } => fold_call(name, args.as_mut_slice(), true),
        Vec2Constructor(_) | Vec3Constructor(_) | Vec4Constructor(_) => None,
        Swizzle { .. } => None,
        Number(_) | IntNumber(_) | Variable(_) | PreIncrement(_) | PreDecrement(_)
        | PostIncrement(_) | PostDecrement(_) => None,
    };

    if let Some(replacement) = replacement {
        replacement.apply(expr);
        changed = true;
    }

    changed
}
