/// Expression type checking
extern crate alloc;
use alloc::vec;

use crate::compiler::ast::{Expr, ExprKind};
use crate::compiler::error::{TypeError, TypeErrorKind};
use crate::compiler::typechecker::{FunctionTable, SymbolTable, TypeChecker};
use crate::shared::Type;

impl TypeChecker {
    /// Type check an expression, mutating it in place
    pub(crate) fn infer_type(
        expr: &mut Expr,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
    ) -> Result<(), TypeError> {
        let expr_span = expr.span;

        match &mut expr.kind {
            // Literals
            ExprKind::Number(_) => {
                expr.ty = Some(Self::check_number());
            }

            ExprKind::IntNumber(_) => {
                expr.ty = Some(Self::check_int_number());
            }

            ExprKind::Variable(name) => {
                let var_type =
                    crate::compiler::expr::variable::check_variable(name, symbols, expr_span)?;
                expr.ty = Some(var_type);
            }

            // Binary arithmetic operations
            ExprKind::Add(left, right)
            | ExprKind::Sub(left, right)
            | ExprKind::Mul(left, right)
            | ExprKind::Div(left, right)
            | ExprKind::Mod(left, right) => {
                let result_ty = crate::compiler::expr::binary::check_binary_arithmetic(
                    left.as_mut(),
                    right.as_mut(),
                    symbols,
                    func_table,
                    expr_span,
                )?;
                expr.ty = Some(result_ty);
            }

            // Bitwise operations (Int32 only)
            ExprKind::BitwiseAnd(left, right)
            | ExprKind::BitwiseOr(left, right)
            | ExprKind::BitwiseXor(left, right)
            | ExprKind::LeftShift(left, right)
            | ExprKind::RightShift(left, right) => {
                let result_ty = Self::check_bitwise_binary(
                    left.as_mut(),
                    right.as_mut(),
                    symbols,
                    func_table,
                    expr_span,
                )?;
                expr.ty = Some(result_ty);
            }

            ExprKind::BitwiseNot(operand) => {
                let result_ty =
                    Self::check_bitwise_not(operand.as_mut(), symbols, func_table, expr_span)?;
                expr.ty = Some(result_ty);
            }

            // Comparisons
            ExprKind::Less(left, right)
            | ExprKind::Greater(left, right)
            | ExprKind::LessEq(left, right)
            | ExprKind::GreaterEq(left, right)
            | ExprKind::Eq(left, right)
            | ExprKind::NotEq(left, right) => {
                let ty =
                    Self::check_comparison(left.as_mut(), right.as_mut(), symbols, func_table)?;
                expr.ty = Some(ty);
            }

            // Logical operations
            ExprKind::And(left, right) | ExprKind::Or(left, right) => {
                let result_ty =
                    Self::check_logical(left.as_mut(), right.as_mut(), symbols, func_table)?;
                expr.ty = Some(result_ty);
            }

            ExprKind::Not(operand) => {
                Self::infer_type(operand.as_mut(), symbols, func_table)?;
                expr.ty = Some(Type::Bool);
            }

            // Unary negation
            ExprKind::Neg(operand) => {
                Self::infer_type(operand.as_mut(), symbols, func_table)?;
                let operand_ty = operand.ty.clone();
                expr.ty = operand_ty;
            }

            // Increment/Decrement
            ExprKind::PreIncrement(name)
            | ExprKind::PreDecrement(name)
            | ExprKind::PostIncrement(name)
            | ExprKind::PostDecrement(name) => {
                let ty = crate::compiler::expr::variable::check_incdec(name, symbols, expr_span)?;
                expr.ty = Some(ty);
            }

            // Ternary
            ExprKind::Ternary {
                condition,
                true_expr,
                false_expr,
            } => {
                let ty = Self::check_ternary(
                    condition.as_mut(),
                    true_expr.as_mut(),
                    false_expr.as_mut(),
                    symbols,
                    func_table,
                )?;
                expr.ty = Some(ty);
            }

            // Assignment
            ExprKind::Assign { target, value } => {
                let ty = Self::check_assign(target, value.as_mut(), symbols, func_table)?;
                expr.ty = Some(ty);
            }

            // Function call
            ExprKind::Call { name, args } => {
                let (ty, expanded_expr) = crate::compiler::expr::call::check_call(
                    name, args, symbols, func_table, expr_span,
                )?;

                if let Some(expanded) = expanded_expr {
                    // Replace this call with the expanded component-wise version
                    *expr = expanded;
                } else {
                    expr.ty = Some(ty);
                }
            }

            // Vector constructors
            ExprKind::Vec2Constructor(args) => {
                let ty = Self::check_vec_constructor(args, 2, symbols, func_table, expr_span)?;
                expr.ty = Some(ty);
            }
            ExprKind::Vec3Constructor(args) => {
                let ty = Self::check_vec_constructor(args, 3, symbols, func_table, expr_span)?;
                expr.ty = Some(ty);
            }
            ExprKind::Vec4Constructor(args) => {
                let ty = Self::check_vec_constructor(args, 4, symbols, func_table, expr_span)?;
                expr.ty = Some(ty);
            }

            // Swizzle
            ExprKind::Swizzle {
                expr: swizzle_expr,
                components,
            } => {
                let ty =
                    Self::check_swizzle(swizzle_expr.as_mut(), components, symbols, func_table)?;
                expr.ty = Some(ty);
            }
        }

        Ok(())
    }

    // Helper methods for leaf node type checking
    fn check_number() -> Type {
        Type::Fixed
    }

    fn check_int_number() -> Type {
        Type::Int32
    }

    // Helper methods delegated to specific modules:
    // check_variable - delegated to variable/variable_types.rs
    // check_incdec - delegated to variable/variable_types.rs
    // check_binary_arithmetic - delegated to binary/binary_types.rs

    fn check_bitwise_binary(
        left: &mut Expr,
        right: &mut Expr,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
        _span: crate::shared::Span,
    ) -> Result<Type, TypeError> {
        Self::infer_type(left, symbols, func_table)?;
        Self::infer_type(right, symbols, func_table)?;
        Ok(Type::Int32)
    }

    fn check_bitwise_not(
        operand: &mut Expr,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
        _span: crate::shared::Span,
    ) -> Result<Type, TypeError> {
        Self::infer_type(operand, symbols, func_table)?;
        Ok(Type::Int32)
    }

    fn check_comparison(
        left: &mut Expr,
        right: &mut Expr,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
    ) -> Result<Type, TypeError> {
        Self::infer_type(left, symbols, func_table)?;
        Self::infer_type(right, symbols, func_table)?;
        Ok(Type::Bool)
    }

    fn check_logical(
        left: &mut Expr,
        right: &mut Expr,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
    ) -> Result<Type, TypeError> {
        Self::infer_type(left, symbols, func_table)?;
        Self::infer_type(right, symbols, func_table)?;
        Ok(Type::Bool)
    }

    fn check_ternary(
        condition: &mut Expr,
        true_expr: &mut Expr,
        false_expr: &mut Expr,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
    ) -> Result<Type, TypeError> {
        Self::infer_type(condition, symbols, func_table)?;
        Self::infer_type(true_expr, symbols, func_table)?;
        Self::infer_type(false_expr, symbols, func_table)?;

        let true_ty = true_expr.ty.clone().unwrap_or(Type::Fixed);
        Ok(true_ty)
    }

    fn check_assign(
        target: &str,
        value: &mut Expr,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
    ) -> Result<Type, TypeError> {
        use alloc::string::ToString;
        Self::infer_type(value, symbols, func_table)?;
        let value_ty = value.ty.clone().unwrap_or(Type::Fixed);

        // Update symbol table
        symbols.set(target.to_string(), value_ty.clone());
        Ok(value_ty)
    }

    // check_call - delegated to call/call_types.rs

    fn check_vec_constructor(
        args: &mut [Expr],
        _dim: usize,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
        _span: crate::shared::Span,
    ) -> Result<Type, TypeError> {
        for arg in args.iter_mut() {
            Self::infer_type(arg, symbols, func_table)?;
        }

        // Return appropriate vector type based on dimension
        Ok(match _dim {
            2 => Type::Vec2,
            3 => Type::Vec3,
            4 => Type::Vec4,
            _ => Type::Fixed,
        })
    }

    fn check_swizzle(
        swizzle_expr: &mut Expr,
        components: &str,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
    ) -> Result<Type, TypeError> {
        Self::infer_type(swizzle_expr, symbols, func_table)?;

        let base_ty = swizzle_expr.ty.as_ref().unwrap();
        let span = swizzle_expr.span;

        // Validate that base is a vector
        let base_size = match base_ty {
            Type::Vec2 => 2,
            Type::Vec3 => 3,
            Type::Vec4 => 4,
            _ => {
                return Err(TypeError {
                    kind: TypeErrorKind::InvalidOperation {
                        op: alloc::string::String::from(
                            "Swizzle can only be applied to vector types",
                        ),
                        types: vec![base_ty.clone()],
                    },
                    span,
                })
            }
        };

        // Validate components
        for c in components.chars() {
            let idx = match c {
                'x' | 'r' | 's' => 0,
                'y' | 'g' | 't' => 1,
                'z' | 'b' | 'p' => 2,
                'w' | 'a' | 'q' => 3,
                _ => {
                    return Err(TypeError {
                        kind: TypeErrorKind::InvalidOperation {
                            op: alloc::format!("Invalid swizzle component: {}", c),
                            types: vec![base_ty.clone()],
                        },
                        span,
                    })
                }
            };

            if idx >= base_size {
                return Err(TypeError {
                    kind: TypeErrorKind::InvalidOperation {
                        op: alloc::format!(
                            "Component {} out of range for type {}",
                            c,
                            type_to_string(base_ty)
                        ),
                        types: vec![base_ty.clone()],
                    },
                    span,
                });
            }
        }

        // Result type based on component count
        Ok(match components.len() {
            1 => Type::Fixed,
            2 => Type::Vec2,
            3 => Type::Vec3,
            4 => Type::Vec4,
            _ => {
                return Err(TypeError {
                    kind: TypeErrorKind::InvalidOperation {
                        op: alloc::string::String::from("Swizzle must have 1-4 components"),
                        types: vec![base_ty.clone()],
                    },
                    span,
                })
            }
        })
    }
}

fn type_to_string(ty: &Type) -> &str {
    match ty {
        Type::Fixed => "float",
        Type::Int32 => "int",
        Type::Bool => "bool",
        Type::Vec2 => "vec2",
        Type::Vec3 => "vec3",
        Type::Vec4 => "vec4",
        Type::Void => "void",
    }
}
