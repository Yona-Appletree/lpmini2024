/// Constant folding optimization (pool-based API)
///
/// Evaluates expressions with constant operands at compile time.
extern crate alloc;

use crate::compiler::ast::{Expr, ExprKind};

// Import fixed-point fixed for compile-time constant evaluation
use crate::fixed::{ceil, cos, floor, saturate, sin, sqrt, tan, Fixed};

// Import libm for pow (not yet implemented in fixed-point)
use libm::powf;

/// Fold constants in an expression tree
pub fn fold_constants(expr_id: Expr, pool: AstPool) -> (Expr, AstPool) {
    let expr = pool.expr(expr_id);
    let span = expr.span;
    let ty = expr.ty.clone();
    let kind = expr.kind.clone();

    match kind {
        // Binary arithmetic - fold if both operands are constants
        ExprKind::Add(left_id, right_id) => {
            let (new_left, pool2) = fold_constants(left_id, pool);
            let (new_right, mut pool3) = fold_constants(right_id, pool2);

            match (&pool3.expr(new_left).kind, &pool3.expr(new_right).kind) {
                (ExprKind::Number(a), ExprKind::Number(b)) => {
                    if let Ok(id) = pool3.alloc_expr(ExprKind::Number(a + b), span) {
                        pool3.expr_mut(id).ty = ty.clone();
                        (id, pool3)
                    } else {
                        (expr_id, pool3)
                    }
                }
                (ExprKind::IntNumber(a), ExprKind::IntNumber(b)) => {
                    if let Ok(id) = pool3.alloc_expr(ExprKind::IntNumber(a + b), span) {
                        pool3.expr_mut(id).ty = ty.clone();
                        (id, pool3)
                    } else {
                        (expr_id, pool3)
                    }
                }
                _ => {
                    // Update the Add node with potentially optimized children
                    pool3.expr_mut(expr_id).kind = ExprKind::Add(new_left, new_right);
                    (expr_id, pool3)
                }
            }
        }

        ExprKind::Sub(left_id, right_id) => {
            let (new_left, pool2) = fold_constants(left_id, pool);
            let (new_right, mut pool3) = fold_constants(right_id, pool2);

            match (&pool3.expr(new_left).kind, &pool3.expr(new_right).kind) {
                (ExprKind::Number(a), ExprKind::Number(b)) => {
                    if let Ok(id) = pool3.alloc_expr(ExprKind::Number(a - b), span) {
                        pool3.expr_mut(id).ty = ty.clone();
                        (id, pool3)
                    } else {
                        (expr_id, pool3)
                    }
                }
                _ => {
                    pool3.expr_mut(expr_id).kind = ExprKind::Sub(new_left, new_right);
                    (expr_id, pool3)
                }
            }
        }

        ExprKind::Mul(left_id, right_id) => {
            let (new_left, pool2) = fold_constants(left_id, pool);
            let (new_right, mut pool3) = fold_constants(right_id, pool2);

            match (&pool3.expr(new_left).kind, &pool3.expr(new_right).kind) {
                (ExprKind::Number(a), ExprKind::Number(b)) => {
                    if let Ok(id) = pool3.alloc_expr(ExprKind::Number(a * b), span) {
                        pool3.expr_mut(id).ty = ty.clone();
                        (id, pool3)
                    } else {
                        (expr_id, pool3)
                    }
                }
                (ExprKind::IntNumber(a), ExprKind::IntNumber(b)) => {
                    if let Ok(id) = pool3.alloc_expr(ExprKind::IntNumber(a * b), span) {
                        pool3.expr_mut(id).ty = ty.clone();
                        (id, pool3)
                    } else {
                        (expr_id, pool3)
                    }
                }
                _ => {
                    pool3.expr_mut(expr_id).kind = ExprKind::Mul(new_left, new_right);
                    (expr_id, pool3)
                }
            }
        }

        ExprKind::Div(left_id, right_id) => {
            let (new_left, pool2) = fold_constants(left_id, pool);
            let (new_right, mut pool3) = fold_constants(right_id, pool2);

            match (&pool3.expr(new_left).kind, &pool3.expr(new_right).kind) {
                (ExprKind::Number(a), ExprKind::Number(b)) if *b != 0.0 => {
                    if let Ok(id) = pool3.alloc_expr(ExprKind::Number(a / b), span) {
                        pool3.expr_mut(id).ty = ty.clone();
                        (id, pool3)
                    } else {
                        (expr_id, pool3)
                    }
                }
                _ => {
                    pool3.expr_mut(expr_id).kind = ExprKind::Div(new_left, new_right);
                    (expr_id, pool3)
                }
            }
        }

        // Unary negation
        ExprKind::Neg(operand_id) => {
            let (new_operand, mut pool2) = fold_constants(operand_id, pool);
            match &pool2.expr(new_operand).kind {
                ExprKind::Number(n) => {
                    if let Ok(id) = pool2.alloc_expr(ExprKind::Number(-n), span) {
                        (id, pool2)
                    } else {
                        (expr_id, pool2)
                    }
                }
                _ => {
                    pool2.expr_mut(expr_id).kind = ExprKind::Neg(new_operand);
                    (expr_id, pool2)
                }
            }
        }

        // Comparisons
        ExprKind::Greater(left_id, right_id) => {
            let (new_left, pool2) = fold_constants(left_id, pool);
            let (new_right, mut pool3) = fold_constants(right_id, pool2);

            match (&pool3.expr(new_left).kind, &pool3.expr(new_right).kind) {
                (ExprKind::Number(a), ExprKind::Number(b)) => {
                    let result_val = if a > b { 1.0 } else { 0.0 };
                    if let Ok(id) = pool3.alloc_expr(ExprKind::Number(result_val), span) {
                        pool3.expr_mut(id).ty = ty.clone();
                        (id, pool3)
                    } else {
                        (expr_id, pool3)
                    }
                }
                _ => {
                    pool3.expr_mut(expr_id).kind = ExprKind::Greater(new_left, new_right);
                    (expr_id, pool3)
                }
            }
        }

        ExprKind::Less(left_id, right_id) => {
            let (new_left, pool2) = fold_constants(left_id, pool);
            let (new_right, mut pool3) = fold_constants(right_id, pool2);

            match (&pool3.expr(new_left).kind, &pool3.expr(new_right).kind) {
                (ExprKind::Number(a), ExprKind::Number(b)) => {
                    let result_val = if a < b { 1.0 } else { 0.0 };
                    if let Ok(id) = pool3.alloc_expr(ExprKind::Number(result_val), span) {
                        pool3.expr_mut(id).ty = ty.clone();
                        (id, pool3)
                    } else {
                        (expr_id, pool3)
                    }
                }
                _ => {
                    pool3.expr_mut(expr_id).kind = ExprKind::Less(new_left, new_right);
                    (expr_id, pool3)
                }
            }
        }

        // Ternary: condition ? true_expr : false_expr
        ExprKind::Ternary {
            condition,
            true_expr,
            false_expr,
        } => {
            let (new_cond, pool2) = fold_constants(condition, pool);
            let (new_true, pool3) = fold_constants(true_expr, pool2);
            let (new_false, mut pool4) = fold_constants(false_expr, pool3);

            // If condition is constant, return the appropriate branch
            match &pool4.expr(new_cond).kind {
                ExprKind::Number(n) if *n != 0.0 => (new_true, pool4), // true
                ExprKind::Number(_) => (new_false, pool4),             // false (0.0)
                _ => {
                    pool4.expr_mut(expr_id).kind = ExprKind::Ternary {
                        condition: new_cond,
                        true_expr: new_true,
                        false_expr: new_false,
                    };
                    (expr_id, pool4)
                }
            }
        }

        // Logical And: a && b
        ExprKind::And(left_id, right_id) => {
            let (new_left, pool2) = fold_constants(left_id, pool);
            let (new_right, mut pool3) = fold_constants(right_id, pool2);

            match (&pool3.expr(new_left).kind, &pool3.expr(new_right).kind) {
                (ExprKind::Number(a), ExprKind::Number(b)) => {
                    let result_val = if *a != 0.0 && *b != 0.0 { 1.0 } else { 0.0 };
                    if let Ok(id) = pool3.alloc_expr(ExprKind::Number(result_val), span) {
                        pool3.expr_mut(id).ty = ty.clone();
                        (id, pool3)
                    } else {
                        (expr_id, pool3)
                    }
                }
                (ExprKind::Number(a), _) if *a == 0.0 => {
                    // false && x = false
                    (new_left, pool3)
                }
                (_, ExprKind::Number(b)) if *b == 0.0 => {
                    // x && false = false
                    (new_right, pool3)
                }
                _ => {
                    pool3.expr_mut(expr_id).kind = ExprKind::And(new_left, new_right);
                    (expr_id, pool3)
                }
            }
        }

        // Logical Or: a || b
        ExprKind::Or(left_id, right_id) => {
            let (new_left, pool2) = fold_constants(left_id, pool);
            let (new_right, mut pool3) = fold_constants(right_id, pool2);

            match (&pool3.expr(new_left).kind, &pool3.expr(new_right).kind) {
                (ExprKind::Number(a), ExprKind::Number(b)) => {
                    let result_val = if *a != 0.0 || *b != 0.0 { 1.0 } else { 0.0 };
                    if let Ok(id) = pool3.alloc_expr(ExprKind::Number(result_val), span) {
                        pool3.expr_mut(id).ty = ty.clone();
                        (id, pool3)
                    } else {
                        (expr_id, pool3)
                    }
                }
                (ExprKind::Number(a), _) if *a != 0.0 => {
                    // true || x = true
                    (new_left, pool3)
                }
                (_, ExprKind::Number(b)) if *b != 0.0 => {
                    // x || true = true
                    (new_right, pool3)
                }
                _ => {
                    pool3.expr_mut(expr_id).kind = ExprKind::Or(new_left, new_right);
                    (expr_id, pool3)
                }
            }
        }

        // Logical Not: !a
        ExprKind::Not(operand_id) => {
            let (new_operand, mut pool2) = fold_constants(operand_id, pool);
            match &pool2.expr(new_operand).kind {
                ExprKind::Number(n) => {
                    let result_val = if *n == 0.0 { 1.0 } else { 0.0 };
                    if let Ok(id) = pool2.alloc_expr(ExprKind::Number(result_val), span) {
                        (id, pool2)
                    } else {
                        (expr_id, pool2)
                    }
                }
                _ => {
                    pool2.expr_mut(expr_id).kind = ExprKind::Not(new_operand);
                    (expr_id, pool2)
                }
            }
        }

        // Binary fixed functions (min, max, pow, mod)
        ExprKind::Call { ref name, ref args } if args.len() == 2 => {
            let arg1_id = args[0];
            let arg2_id = args[1];
            let (new_arg1, pool2) = fold_constants(arg1_id, pool);
            let (new_arg2, mut pool3) = fold_constants(arg2_id, pool2);

            match (&pool3.expr(new_arg1).kind, &pool3.expr(new_arg2).kind) {
                (ExprKind::Number(a), ExprKind::Number(b)) => {
                    let result_val = match name.as_str() {
                        "min" => a.min(*b),
                        "max" => a.max(*b),
                        "pow" => powf(*a, *b),
                        "mod" => {
                            if *b != 0.0 {
                                a % b
                            } else {
                                pool3.expr_mut(expr_id).kind = ExprKind::Call {
                                    name: name.clone(),
                                    args: alloc::vec![new_arg1, new_arg2],
                                };
                                return (expr_id, pool3);
                            }
                        }
                        _ => {
                            pool3.expr_mut(expr_id).kind = ExprKind::Call {
                                name: name.clone(),
                                args: alloc::vec![new_arg1, new_arg2],
                            };
                            return (expr_id, pool3);
                        }
                    };

                    if let Ok(id) = pool3.alloc_expr(ExprKind::Number(result_val), span) {
                        pool3.expr_mut(id).ty = ty.clone();
                        (id, pool3)
                    } else {
                        (expr_id, pool3)
                    }
                }
                _ => {
                    pool3.expr_mut(expr_id).kind = ExprKind::Call {
                        name: name.clone(),
                        args: alloc::vec![new_arg1, new_arg2],
                    };
                    (expr_id, pool3)
                }
            }
        }

        // Ternary fixed functions (clamp, lerp, smoothstep)
        ExprKind::Call { ref name, ref args } if args.len() == 3 => {
            let arg1_id = args[0];
            let arg2_id = args[1];
            let arg3_id = args[2];
            let (new_arg1, pool2) = fold_constants(arg1_id, pool);
            let (new_arg2, pool3) = fold_constants(arg2_id, pool2);
            let (new_arg3, mut pool4) = fold_constants(arg3_id, pool3);

            match (
                &pool4.expr(new_arg1).kind,
                &pool4.expr(new_arg2).kind,
                &pool4.expr(new_arg3).kind,
            ) {
                (ExprKind::Number(a), ExprKind::Number(b), ExprKind::Number(c)) => {
                    let result_val = match name.as_str() {
                        "clamp" => a.max(*b).min(*c),      // clamp(x, min, max)
                        "lerp" | "mix" => a + (c - a) * b, // lerp(a, b, c) = a + (c - a) * b
                        "smoothstep" => {
                            // smoothstep(edge0, edge1, x) - complex, skip for now
                            pool4.expr_mut(expr_id).kind = ExprKind::Call {
                                name: name.clone(),
                                args: alloc::vec![new_arg1, new_arg2, new_arg3],
                            };
                            return (expr_id, pool4);
                        }
                        _ => {
                            pool4.expr_mut(expr_id).kind = ExprKind::Call {
                                name: name.clone(),
                                args: alloc::vec![new_arg1, new_arg2, new_arg3],
                            };
                            return (expr_id, pool4);
                        }
                    };

                    if let Ok(id) = pool4.alloc_expr(ExprKind::Number(result_val), span) {
                        (id, pool4)
                    } else {
                        (expr_id, pool4)
                    }
                }
                _ => {
                    pool4.expr_mut(expr_id).kind = ExprKind::Call {
                        name: name.clone(),
                        args: alloc::vec![new_arg1, new_arg2, new_arg3],
                    };
                    (expr_id, pool4)
                }
            }
        }

        // Unary fixed functions (sin, cos, etc.)
        ExprKind::Call { ref name, ref args } if args.len() == 1 => {
            let arg_id = args[0];
            let (new_arg, mut pool2) = fold_constants(arg_id, pool);

            if let ExprKind::Number(n) = &pool2.expr(new_arg).kind {
                // Convert to fixed-point for fixed operations
                let fixed_arg = Fixed::from_f32(*n);
                let result_fixed = match name.as_str() {
                    "sin" => sin(fixed_arg),
                    "cos" => cos(fixed_arg),
                    "tan" => tan(fixed_arg),
                    "sqrt" => sqrt(fixed_arg),
                    "abs" => fixed_arg.abs(),
                    "floor" => floor(fixed_arg),
                    "ceil" => ceil(fixed_arg),
                    "saturate" => saturate(fixed_arg),
                    _ => {
                        pool2.expr_mut(expr_id).kind = ExprKind::Call {
                            name: name.clone(),
                            args: alloc::vec![new_arg],
                        };
                        return (expr_id, pool2);
                    }
                };
                // Convert back to f32 for AST storage
                let result_val = result_fixed.to_f32();

                if let Ok(id) = pool2.alloc_expr(ExprKind::Number(result_val), span) {
                    (id, pool2)
                } else {
                    (expr_id, pool2)
                }
            } else {
                pool2.expr_mut(expr_id).kind = ExprKind::Call {
                    name: name.clone(),
                    args: alloc::vec![new_arg],
                };
                (expr_id, pool2)
            }
        }

        // Recursively fold vector constructors
        ExprKind::Vec2Constructor(ref args) => {
            let mut new_args = alloc::vec::Vec::new();
            let mut current_pool = pool;

            for &arg_id in args {
                let (new_arg, next_pool) = fold_constants(arg_id, current_pool);
                new_args.push(new_arg);
                current_pool = next_pool;
            }

            current_pool.expr_mut(expr_id).kind = ExprKind::Vec2Constructor(new_args);
            (expr_id, current_pool)
        }

        ExprKind::Vec3Constructor(ref args) => {
            let mut new_args = alloc::vec::Vec::new();
            let mut current_pool = pool;

            for &arg_id in args {
                let (new_arg, next_pool) = fold_constants(arg_id, current_pool);
                new_args.push(new_arg);
                current_pool = next_pool;
            }

            current_pool.expr_mut(expr_id).kind = ExprKind::Vec3Constructor(new_args);
            (expr_id, current_pool)
        }

        ExprKind::Vec4Constructor(ref args) => {
            let mut new_args = alloc::vec::Vec::new();
            let mut current_pool = pool;

            for &arg_id in args {
                let (new_arg, next_pool) = fold_constants(arg_id, current_pool);
                new_args.push(new_arg);
                current_pool = next_pool;
            }

            current_pool.expr_mut(expr_id).kind = ExprKind::Vec4Constructor(new_args);
            (expr_id, current_pool)
        }

        // All other expressions - return as-is
        _ => (expr_id, pool),
    }
}
