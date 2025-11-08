/// Component-wise function expansion
///
/// Transforms function calls on vectors into component-wise scalar calls.
/// For example: `sin(vec2(a, b))` becomes `vec2(sin(vec2(a, b).x), sin(vec2(a, b).y))`
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

use crate::compiler::ast::{AstPool, ExprId, ExprKind};
use crate::shared::{Span, Type};

/// Check if a function supports component-wise expansion
pub(crate) fn is_componentwise_function(name: &str) -> bool {
    matches!(
        name,
        // Single-arg component-wise functions
        "sin" | "cos" | "tan" | "atan" | "asin" | "acos" |
        "abs" | "floor" | "ceil" | "sqrt" | "sign" | "fract" | "saturate" |
        "exp" | "log" | "exp2" | "log2" | "inversesqrt" |
        "radians" | "degrees" | "trunc" | "round" |
        // Multi-arg component-wise functions
        "min" | "max" | "mod" | "pow" | 
        "clamp" | "step" | "mix" | "smoothstep"
    )
}

/// Expand a component-wise function call
///
/// Returns Some(expanded_expr_id) if expansion was performed, None otherwise
pub(crate) fn expand_componentwise_call(
    pool: &mut AstPool,
    name: &str,
    args: &[ExprId],
    span: Span,
) -> Option<ExprId> {
    // Check if any argument is a vector type
    let has_vector_arg = args.iter().any(|&arg_id| {
        matches!(
            pool.expr(arg_id).ty.as_ref(),
            Some(Type::Vec2 | Type::Vec3 | Type::Vec4)
        )
    });

    if !has_vector_arg {
        return None; // No expansion needed
    }

    // Determine the result vector type (largest vector among args)
    let result_vec_type = args
        .iter()
        .filter_map(|&arg_id| pool.expr(arg_id).ty.as_ref())
        .filter_map(|ty| match ty {
            Type::Vec2 => Some(2),
            Type::Vec3 => Some(3),
            Type::Vec4 => Some(4),
            _ => None,
        })
        .max()?;

    let result_type = match result_vec_type {
        2 => Type::Vec2,
        3 => Type::Vec3,
        4 => Type::Vec4,
        _ => return None,
    };

    // Build component-wise calls
    let components = ["x", "y", "z", "w"];
    let mut expanded_args = Vec::new();

    for i in 0..result_vec_type {
        let component = components[i];

        // Build args for this component
        let mut component_call_args = Vec::new();
        for &arg_id in args {
            let arg_ty = pool.expr(arg_id).ty.as_ref();

            let component_arg = match arg_ty {
                Some(Type::Vec2 | Type::Vec3 | Type::Vec4) => {
                    // Extract component using swizzle
                    pool.alloc_expr(
                        ExprKind::Swizzle {
                            expr: arg_id,
                            components: String::from(component),
                        },
                        span,
                    )
                    .ok()?
                }
                _ => {
                    // Scalar - use as-is
                    arg_id
                }
            };

            component_call_args.push(component_arg);
        }

        // Build the function call for this component
        let component_call = pool.alloc_expr(
            ExprKind::Call {
                name: String::from(name),
                args: component_call_args,
            },
            span,
        );

        if let Ok(id) = component_call {
            expanded_args.push(id);
        } else {
            return None; // Pool exhausted
        }
    }

    // Build the vector constructor with the expanded args
    let constructor = match result_type {
        Type::Vec2 => ExprKind::Vec2Constructor(expanded_args),
        Type::Vec3 => ExprKind::Vec3Constructor(expanded_args),
        Type::Vec4 => ExprKind::Vec4Constructor(expanded_args),
        _ => return None,
    };

    pool.alloc_expr(constructor, span).ok()
}
