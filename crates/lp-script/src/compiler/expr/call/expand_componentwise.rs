/// Component-wise function expansion
///
/// Transforms function calls on vectors into component-wise scalar calls.
/// For example: `sin(vec2(a, b))` becomes `vec2(sin(vec2(a, b).x), sin(vec2(a, b).y))`
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

use lp_pool::LpBox;

use crate::compiler::ast::{Expr, ExprKind};
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
/// Returns Some(expanded_expr) if expansion was performed, None otherwise
pub(crate) fn expand_componentwise_call(name: &str, args: &[Expr], span: Span) -> Option<Expr> {
    // Check if any argument is a vector type
    let has_vector_arg = args
        .iter()
        .any(|arg| matches!(arg.ty.as_ref(), Some(Type::Vec2 | Type::Vec3 | Type::Vec4)));

    if !has_vector_arg {
        return None; // No expansion needed
    }

    // Determine the result vector type (largest vector among args)
    let result_vec_type = args
        .iter()
        .filter_map(|arg| arg.ty.as_ref())
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

    for &component in components.iter().take(result_vec_type) {
        // Build args for this component
        let mut component_call_args = Vec::new();
        for arg in args {
            let arg_ty = arg.ty.as_ref();

            let component_arg = match arg_ty {
                Some(Type::Vec2 | Type::Vec3 | Type::Vec4) => {
                    // Extract component using swizzle
                    // Clone the arg for the swizzle
                    Expr::new(
                        ExprKind::Swizzle {
                            expr: LpBox::try_new(arg.clone()).ok()?,
                            components: String::from(component),
                        },
                        span,
                    )
                }
                _ => {
                    // Scalar - clone and use as-is
                    arg.clone()
                }
            };

            component_call_args.push(component_arg);
        }

        // Build the function call for this component
        let component_call = Expr::new(
            ExprKind::Call {
                name: String::from(name),
                args: component_call_args,
            },
            span,
        );

        expanded_args.push(component_call);
    }

    // Build the vector constructor with the expanded args
    let constructor = match result_type {
        Type::Vec2 => ExprKind::Vec2Constructor(expanded_args),
        Type::Vec3 => ExprKind::Vec3Constructor(expanded_args),
        Type::Vec4 => ExprKind::Vec4Constructor(expanded_args),
        _ => return None,
    };

    Some(Expr::new(constructor, span))
}
