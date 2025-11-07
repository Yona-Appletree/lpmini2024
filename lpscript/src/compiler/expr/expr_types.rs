/// Expression type checking
extern crate alloc;
use alloc::vec;

use crate::compiler::ast::{AstPool, ExprId, ExprKind};
use crate::compiler::error::{TypeError, TypeErrorKind};
use crate::compiler::typechecker::{FunctionTable, SymbolTable, TypeChecker};
use crate::shared::Type;

impl TypeChecker {
    /// Type check an expression by ID, mutating the pool
    pub(crate) fn infer_type_id(
        pool: &mut AstPool,
        expr_id: ExprId,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
    ) -> Result<(), TypeError> {
        // Clone the expression kind to avoid borrow issues
        let expr_kind = pool.expr(expr_id).kind.clone();
        let expr_span = pool.expr(expr_id).span;
        
        match &expr_kind {
            // Literals
            ExprKind::Number(_) => {
                pool.expr_mut(expr_id).ty = Some(Self::check_number());
            }

            ExprKind::IntNumber(_) => {
                pool.expr_mut(expr_id).ty = Some(Self::check_int_number());
            }

            ExprKind::Variable(name) => {
                let var_type = crate::compiler::expr::variable::check_variable(name, symbols, expr_span)?;
                pool.expr_mut(expr_id).ty = Some(var_type);
            }

            // Binary arithmetic operations
            ExprKind::Add(left_id, right_id)
            | ExprKind::Sub(left_id, right_id)
            | ExprKind::Mul(left_id, right_id)
            | ExprKind::Div(left_id, right_id)
            | ExprKind::Mod(left_id, right_id) => {
                let result_ty = crate::compiler::expr::binary::check_binary_arithmetic_id(
                    pool, *left_id, *right_id, symbols, func_table, expr_span,
                )?;
                pool.expr_mut(expr_id).ty = Some(result_ty);
            }

            // Bitwise operations (Int32 only)
            ExprKind::BitwiseAnd(left_id, right_id)
            | ExprKind::BitwiseOr(left_id, right_id)
            | ExprKind::BitwiseXor(left_id, right_id)
            | ExprKind::LeftShift(left_id, right_id)
            | ExprKind::RightShift(left_id, right_id) => {
                let result_ty = Self::check_bitwise_binary_id(
                    pool, *left_id, *right_id, symbols, func_table, expr_span,
                )?;
                pool.expr_mut(expr_id).ty = Some(result_ty);
            }

            ExprKind::BitwiseNot(operand_id) => {
                let result_ty =
                    Self::check_bitwise_not_id(pool, *operand_id, symbols, func_table, expr_span)?;
                pool.expr_mut(expr_id).ty = Some(result_ty);
            }

            // Comparisons
            ExprKind::Less(left_id, right_id)
            | ExprKind::Greater(left_id, right_id)
            | ExprKind::LessEq(left_id, right_id)
            | ExprKind::GreaterEq(left_id, right_id)
            | ExprKind::Eq(left_id, right_id)
            | ExprKind::NotEq(left_id, right_id) => {
                let ty = Self::check_comparison_id(pool, *left_id, *right_id, symbols, func_table)?;
                pool.expr_mut(expr_id).ty = Some(ty);
            }

            // Logical operations
            ExprKind::And(left_id, right_id) | ExprKind::Or(left_id, right_id) => {
                let result_ty = Self::check_logical_id(pool, *left_id, *right_id, symbols, func_table)?;
                pool.expr_mut(expr_id).ty = Some(result_ty);
            }

            ExprKind::Not(operand_id) => {
                Self::infer_type_id(pool, *operand_id, symbols, func_table)?;
                pool.expr_mut(expr_id).ty = Some(Type::Bool);
            }

            // Unary negation
            ExprKind::Neg(operand_id) => {
                Self::infer_type_id(pool, *operand_id, symbols, func_table)?;
                let operand_ty = pool.expr(*operand_id).ty.clone();
                pool.expr_mut(expr_id).ty = operand_ty;
            }

            // Increment/Decrement
            ExprKind::PreIncrement(name)
            | ExprKind::PreDecrement(name)
            | ExprKind::PostIncrement(name)
            | ExprKind::PostDecrement(name) => {
                let ty = crate::compiler::expr::variable::check_incdec(name, symbols, expr_span)?;
                pool.expr_mut(expr_id).ty = Some(ty);
            }

            // Ternary
            ExprKind::Ternary {
                condition,
                true_expr,
                false_expr,
            } => {
                let ty = Self::check_ternary_id(
                    pool, *condition, *true_expr, *false_expr, symbols, func_table,
                )?;
                pool.expr_mut(expr_id).ty = Some(ty);
            }

            // Assignment
            ExprKind::Assign { target, value } => {
                let ty = Self::check_assign_id(pool, target, *value, symbols, func_table)?;
                pool.expr_mut(expr_id).ty = Some(ty);
            }

            // Function call
            ExprKind::Call { name, args } => {
                let (ty, expanded_id) = crate::compiler::expr::call::check_call_id(pool, name, args, symbols, func_table, expr_span)?;
                
                if let Some(expanded) = expanded_id {
                    // Replace this call with the expanded component-wise version
                    // Copy the expanded expression to this location
                    let expanded_kind = pool.expr(expanded).kind.clone();
                    let expanded_ty = pool.expr(expanded).ty.clone();
                    pool.expr_mut(expr_id).kind = expanded_kind;
                    pool.expr_mut(expr_id).ty = expanded_ty;
                } else {
                    pool.expr_mut(expr_id).ty = Some(ty);
                }
            }

            // Vector constructors
            ExprKind::Vec2Constructor(args) => {
                let ty = Self::check_vec_constructor_id(pool, args, 2, symbols, func_table, expr_span)?;
                pool.expr_mut(expr_id).ty = Some(ty);
            }
            ExprKind::Vec3Constructor(args) => {
                let ty = Self::check_vec_constructor_id(pool, args, 3, symbols, func_table, expr_span)?;
                pool.expr_mut(expr_id).ty = Some(ty);
            }
            ExprKind::Vec4Constructor(args) => {
                let ty = Self::check_vec_constructor_id(pool, args, 4, symbols, func_table, expr_span)?;
                pool.expr_mut(expr_id).ty = Some(ty);
            }

            // Swizzle
            ExprKind::Swizzle { expr: swizzle_expr, components } => {
                let ty = Self::check_swizzle_id(pool, *swizzle_expr, components, symbols, func_table)?;
                pool.expr_mut(expr_id).ty = Some(ty);
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
    // Helper methods that work with IDs instead of references
    // check_binary_arithmetic_id - delegated to binary/binary_types.rs

    fn check_bitwise_binary_id(
        pool: &mut AstPool,
        left_id: ExprId,
        right_id: ExprId,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
        _span: crate::shared::Span,
    ) -> Result<Type, TypeError> {
        Self::infer_type_id(pool, left_id, symbols, func_table)?;
        Self::infer_type_id(pool, right_id, symbols, func_table)?;
        Ok(Type::Int32)
    }

    fn check_bitwise_not_id(
        pool: &mut AstPool,
        operand_id: ExprId,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
        _span: crate::shared::Span,
    ) -> Result<Type, TypeError> {
        Self::infer_type_id(pool, operand_id, symbols, func_table)?;
        Ok(Type::Int32)
    }

    fn check_comparison_id(
        pool: &mut AstPool,
        left_id: ExprId,
        right_id: ExprId,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
    ) -> Result<Type, TypeError> {
        Self::infer_type_id(pool, left_id, symbols, func_table)?;
        Self::infer_type_id(pool, right_id, symbols, func_table)?;
        Ok(Type::Bool)
    }

    fn check_logical_id(
        pool: &mut AstPool,
        left_id: ExprId,
        right_id: ExprId,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
    ) -> Result<Type, TypeError> {
        Self::infer_type_id(pool, left_id, symbols, func_table)?;
        Self::infer_type_id(pool, right_id, symbols, func_table)?;
        Ok(Type::Bool)
    }

    fn check_ternary_id(
        pool: &mut AstPool,
        condition_id: ExprId,
        true_id: ExprId,
        false_id: ExprId,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
    ) -> Result<Type, TypeError> {
        Self::infer_type_id(pool, condition_id, symbols, func_table)?;
        Self::infer_type_id(pool, true_id, symbols, func_table)?;
        Self::infer_type_id(pool, false_id, symbols, func_table)?;
        
        let true_ty = pool.expr(true_id).ty.clone().unwrap_or(Type::Fixed);
        Ok(true_ty)
    }

    fn check_assign_id(
        pool: &mut AstPool,
        target: &str,
        value_id: ExprId,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
    ) -> Result<Type, TypeError> {
        use alloc::string::ToString;
        Self::infer_type_id(pool, value_id, symbols, func_table)?;
        let value_ty = pool.expr(value_id).ty.clone().unwrap_or(Type::Fixed);
        
        // Update symbol table
        symbols.set(target.to_string(), value_ty.clone());
        Ok(value_ty)
    }

    // check_call_id - delegated to call/call_types.rs

    fn check_vec_constructor_id(
        pool: &mut AstPool,
        args: &[ExprId],
        _dim: usize,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
        _span: crate::shared::Span,
    ) -> Result<Type, TypeError> {
        for &arg_id in args {
            Self::infer_type_id(pool, arg_id, symbols, func_table)?;
        }
        
        // Return appropriate vector type based on dimension
        Ok(match _dim {
            2 => Type::Vec2,
            3 => Type::Vec3,
            4 => Type::Vec4,
            _ => Type::Fixed,
        })
    }

    fn check_swizzle_id(
        pool: &mut AstPool,
        expr_id: ExprId,
        components: &str,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
    ) -> Result<Type, TypeError> {
        Self::infer_type_id(pool, expr_id, symbols, func_table)?;
        
        let base_expr = pool.expr(expr_id);
        let base_ty = base_expr.ty.as_ref().unwrap();
        let span = base_expr.span;

        // Validate that base is a vector
        let base_size = match base_ty {
            Type::Vec2 => 2,
            Type::Vec3 => 3,
            Type::Vec4 => 4,
            _ => {
                return Err(TypeError {
                    kind: TypeErrorKind::InvalidOperation {
                        op: alloc::string::String::from("Swizzle can only be applied to vector types"),
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
