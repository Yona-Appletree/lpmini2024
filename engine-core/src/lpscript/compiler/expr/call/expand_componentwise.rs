/// Component-wise function expansion
/// 
/// Transforms function calls on vectors into component-wise scalar calls.
/// For example: `sin(vec2(a, b))` becomes `vec2(sin(vec2(a, b).x), sin(vec2(a, b).y))`
extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use crate::lpscript::compiler::ast::{Expr, ExprKind};
use crate::lpscript::shared::{Span, Type};

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
pub(crate) fn expand_componentwise_call(
    name: &str,
    args: &[Expr],
    span: Span,
) -> Option<Expr> {
    // Check if any argument is a vector type
    let has_vector_arg = args.iter().any(|arg| {
        matches!(arg.ty.as_ref(), Some(Type::Vec2 | Type::Vec3 | Type::Vec4))
    });

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

    let vec_type = match result_vec_type {
        2 => Type::Vec2,
        3 => Type::Vec3,
        4 => Type::Vec4,
        _ => return None,
    };

    // Generate component-wise calls
    let component_chars = match result_vec_type {
        2 => vec!['x', 'y'],
        3 => vec!['x', 'y', 'z'],
        4 => vec!['x', 'y', 'z', 'w'],
        _ => return None,
    };

    let mut component_calls = Vec::new();

    for comp in component_chars {
        // For each component, create argument list with swizzles where needed
        let comp_args: Vec<Expr> = args
            .iter()
            .map(|arg| {
                match arg.ty.as_ref() {
                    Some(Type::Vec2 | Type::Vec3 | Type::Vec4) => {
                        // Swizzle the vector arg to get the component
                        let comp_str = alloc::format!("{}", comp);
                        Expr {
                            kind: ExprKind::Swizzle {
                                expr: Box::new(arg.clone()),
                                components: comp_str,
                            },
                            span: arg.span,
                            ty: Some(Type::Fixed), // Swizzled single component is Fixed
                        }
                    }
                    _ => {
                        // Scalar arg - use as-is (broadcast to all components)
                        arg.clone()
                    }
                }
            })
            .collect();

        // Create the scalar function call for this component
        let comp_call = Expr {
            kind: ExprKind::Call {
                name: String::from(name),
                args: comp_args,
            },
            span,
            ty: Some(Type::Fixed), // Result of scalar call
        };

        component_calls.push(comp_call);
    }

    // Wrap in vector constructor
    let expanded = Expr {
        kind: match vec_type {
            Type::Vec2 => ExprKind::Vec2Constructor(component_calls),
            Type::Vec3 => ExprKind::Vec3Constructor(component_calls),
            Type::Vec4 => ExprKind::Vec4Constructor(component_calls),
            _ => return None,
        },
        span,
        ty: Some(vec_type),
    };

    Some(expanded)
}

