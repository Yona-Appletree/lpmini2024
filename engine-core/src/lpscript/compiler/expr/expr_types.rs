/// Expression type checking
extern crate alloc;

use crate::lpscript::compiler::ast::{Expr, ExprKind};
use crate::lpscript::compiler::error::TypeError;
use crate::lpscript::compiler::typechecker::{FunctionTable, SymbolTable, TypeChecker};
use crate::lpscript::shared::Type;

impl TypeChecker {
    /// Type check an expression, returning a typed AST
    pub fn check_expr(mut expr: Expr) -> Result<Expr, TypeError> {
        let mut symbols = SymbolTable::new();
        let func_table = FunctionTable::new(); // Empty for expression mode
        Self::infer_type(&mut expr, &mut symbols, &func_table)?;
        Ok(expr)
    }

    /// Type check an expression, returning a typed AST
    /// 
    /// Alias for check_expr for backward compatibility
    pub fn check(expr: Expr) -> Result<Expr, TypeError> {
        Self::check_expr(expr)
    }

    pub(crate) fn infer_type(
        expr: &mut Expr,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
    ) -> Result<(), TypeError> {
        match &mut expr.kind {
            // Literals
            ExprKind::Number(_) => {
                expr.ty = Some(Self::check_number());
            }

            ExprKind::IntNumber(_) => {
                expr.ty = Some(Self::check_int_number());
            }

            ExprKind::Variable(name) => {
                let var_type = Self::check_variable(name, symbols, expr.span)?;
                expr.ty = Some(var_type);
            }

            // Binary arithmetic operations
            ExprKind::Add(left, right)
            | ExprKind::Sub(left, right)
            | ExprKind::Mul(left, right)
            | ExprKind::Div(left, right)
            | ExprKind::Mod(left, right) => {
                let result_ty =
                    Self::check_binary_arithmetic(left, right, symbols, func_table, expr.span)?;
                expr.ty = Some(result_ty);
            }

            // Bitwise operations (Int32 only)
            ExprKind::BitwiseAnd(left, right)
            | ExprKind::BitwiseOr(left, right)
            | ExprKind::BitwiseXor(left, right)
            | ExprKind::LeftShift(left, right)
            | ExprKind::RightShift(left, right) => {
                let result_ty =
                    Self::check_bitwise_binary(left, right, symbols, func_table, expr.span)?;
                expr.ty = Some(result_ty);
            }

            ExprKind::BitwiseNot(operand) => {
                let result_ty = Self::check_bitwise_not(operand, symbols, func_table, expr.span)?;
                expr.ty = Some(result_ty);
            }

            // Comparisons return Fixed (0 or 1) - handled in compiler/expr/compare module
            ExprKind::Less(left, right)
            | ExprKind::Greater(left, right)
            | ExprKind::LessEq(left, right)
            | ExprKind::GreaterEq(left, right)
            | ExprKind::Eq(left, right)
            | ExprKind::NotEq(left, right) => {
                let ty = Self::check_comparison(left, right, symbols, func_table)?;
                expr.ty = Some(ty);
            }

            // Logical operations
            ExprKind::And(left, right) | ExprKind::Or(left, right) => {
                let result_ty = Self::check_logical(left, right, symbols, func_table)?;
                expr.ty = Some(result_ty);
            }

            ExprKind::Not(operand) => {
                Self::infer_type(operand, symbols, func_table)?;
                expr.ty = Some(Type::Bool);
            }

            // Unary negation
            ExprKind::Neg(operand) => {
                Self::infer_type(operand, symbols, func_table)?;
                // Type of negation is same as operand
                expr.ty = operand.ty.clone();
            }

            // Increment/Decrement operators
            ExprKind::PreIncrement(var_name)
            | ExprKind::PreDecrement(var_name)
            | ExprKind::PostIncrement(var_name)
            | ExprKind::PostDecrement(var_name) => {
                let result_ty = Self::check_incdec(var_name, symbols, expr.span)?;
                expr.ty = Some(result_ty);
            }

            // Ternary
            ExprKind::Ternary {
                condition,
                true_expr,
                false_expr,
            } => {
                let result_ty = Self::check_ternary(
                    condition, true_expr, false_expr, symbols, func_table, expr.span,
                )?;
                expr.ty = Some(result_ty);
            }

            // Assignment expression
            ExprKind::Assign { target, value } => {
                let result_ty =
                    Self::check_assign_expr(target, value, symbols, func_table, expr.span)?;
                expr.ty = Some(result_ty);
            }

            // Function calls
            ExprKind::Call { .. } => {
                // Type check the function call (may transform expr via expansion)
                let return_ty =
                    Self::check_function_call(expr, symbols, func_table)?;
                expr.ty = Some(return_ty);
            }

            // Vector constructors
            // In GLSL, these can take mixed vec/scalar args: vec3(vec2, float) is valid
            ExprKind::Vec2Constructor(args) => {
                Self::check_vector_constructor(args, 2, "vec2", expr.span, symbols, func_table)?;
                expr.ty = Some(Type::Vec2);
            }

            ExprKind::Vec3Constructor(args) => {
                Self::check_vector_constructor(args, 3, "vec3", expr.span, symbols, func_table)?;
                expr.ty = Some(Type::Vec3);
            }

            ExprKind::Vec4Constructor(args) => {
                Self::check_vector_constructor(args, 4, "vec4", expr.span, symbols, func_table)?;
                expr.ty = Some(Type::Vec4);
            }

            ExprKind::Swizzle {
                expr: base_expr,
                components,
            } => {
                let result_type =
                    Self::check_swizzle(base_expr, components, symbols, func_table, expr.span)?;
                expr.ty = Some(result_type);
            }
        }

        Ok(())
    }
}

