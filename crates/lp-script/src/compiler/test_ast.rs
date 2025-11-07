// Suppress warnings for unused test helper functions
#![allow(dead_code)]

/// Helper functions for building expected AST expressions in tests
///
/// These use a builder pattern with AstPool to create index-based AST nodes.
/// Tests can focus on structure rather than memory management.
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

use crate::compiler::ast::{AstPool, ExprId, ExprKind};
use crate::shared::{Span, Type};

/// Builder for creating test AST nodes with a pool
pub struct AstBuilder {
    pool: AstPool,
}

impl AstBuilder {
    /// Create a new builder with an empty pool
    pub fn new() -> Self {
        Self {
            pool: AstPool::new(),
        }
    }

    /// Consume the builder and return the pool
    pub fn into_pool(self) -> AstPool {
        self.pool
    }

    /// Get a reference to the pool
    pub fn pool(&self) -> &AstPool {
        &self.pool
    }

    /// Get a mutable reference to the pool
    pub fn pool_mut(&mut self) -> &mut AstPool {
        &mut self.pool
    }

    // ============================================================================
    // Leaf nodes (return ExprId with auto-typed)
    // ============================================================================

    /// Create an integer literal expression
    pub fn int32(&mut self, value: i32) -> ExprId {
        let id = self
            .pool
            .alloc_expr(ExprKind::IntNumber(value), Span::EMPTY)
            .unwrap();
        self.pool.expr_mut(id).ty = Some(Type::Int32);
        id
    }

    /// Create a float literal expression
    pub fn num(&mut self, value: f32) -> ExprId {
        let id = self
            .pool
            .alloc_expr(ExprKind::Number(value), Span::EMPTY)
            .unwrap();
        self.pool.expr_mut(id).ty = Some(Type::Fixed);
        id
    }

    /// Create a variable reference expression
    pub fn var(&mut self, name: &str) -> ExprId {
        self.pool
            .alloc_expr(ExprKind::Variable(String::from(name)), Span::EMPTY)
            .unwrap()
    }

    // ============================================================================
    // Comparison operators (return ExprId, auto-typed to Bool)
    // ============================================================================

    /// Less than: left < right
    pub fn less(&mut self, left: ExprId, right: ExprId) -> ExprId {
        let id = self
            .pool
            .alloc_expr(ExprKind::Less(left, right), Span::EMPTY)
            .unwrap();
        self.pool.expr_mut(id).ty = Some(Type::Bool);
        id
    }

    /// Greater than: left > right
    pub fn greater(&mut self, left: ExprId, right: ExprId) -> ExprId {
        let id = self
            .pool
            .alloc_expr(ExprKind::Greater(left, right), Span::EMPTY)
            .unwrap();
        self.pool.expr_mut(id).ty = Some(Type::Bool);
        id
    }

    /// Less than or equal: left <= right
    pub fn less_eq(&mut self, left: ExprId, right: ExprId) -> ExprId {
        let id = self
            .pool
            .alloc_expr(ExprKind::LessEq(left, right), Span::EMPTY)
            .unwrap();
        self.pool.expr_mut(id).ty = Some(Type::Bool);
        id
    }

    /// Greater than or equal: left >= right
    pub fn greater_eq(&mut self, left: ExprId, right: ExprId) -> ExprId {
        let id = self
            .pool
            .alloc_expr(ExprKind::GreaterEq(left, right), Span::EMPTY)
            .unwrap();
        self.pool.expr_mut(id).ty = Some(Type::Bool);
        id
    }

    /// Equal: left == right
    pub fn eq(&mut self, left: ExprId, right: ExprId) -> ExprId {
        let id = self
            .pool
            .alloc_expr(ExprKind::Eq(left, right), Span::EMPTY)
            .unwrap();
        self.pool.expr_mut(id).ty = Some(Type::Bool);
        id
    }

    /// Not equal: left != right
    pub fn not_eq(&mut self, left: ExprId, right: ExprId) -> ExprId {
        let id = self
            .pool
            .alloc_expr(ExprKind::NotEq(left, right), Span::EMPTY)
            .unwrap();
        self.pool.expr_mut(id).ty = Some(Type::Bool);
        id
    }

    // ============================================================================
    // Arithmetic operators (return ExprId, type parameter required)
    // ============================================================================

    /// Addition: left + right
    pub fn add(&mut self, left: ExprId, right: ExprId, ty: Type) -> ExprId {
        let id = self
            .pool
            .alloc_expr(ExprKind::Add(left, right), Span::EMPTY)
            .unwrap();
        self.pool.expr_mut(id).ty = Some(ty);
        id
    }

    /// Subtraction: left - right
    pub fn sub(&mut self, left: ExprId, right: ExprId, ty: Type) -> ExprId {
        let id = self
            .pool
            .alloc_expr(ExprKind::Sub(left, right), Span::EMPTY)
            .unwrap();
        self.pool.expr_mut(id).ty = Some(ty);
        id
    }

    /// Multiplication: left * right
    pub fn mul(&mut self, left: ExprId, right: ExprId, ty: Type) -> ExprId {
        let id = self
            .pool
            .alloc_expr(ExprKind::Mul(left, right), Span::EMPTY)
            .unwrap();
        self.pool.expr_mut(id).ty = Some(ty);
        id
    }

    /// Division: left / right
    pub fn div(&mut self, left: ExprId, right: ExprId, ty: Type) -> ExprId {
        let id = self
            .pool
            .alloc_expr(ExprKind::Div(left, right), Span::EMPTY)
            .unwrap();
        self.pool.expr_mut(id).ty = Some(ty);
        id
    }

    /// Modulo: left % right
    pub fn modulo(&mut self, left: ExprId, right: ExprId, ty: Type) -> ExprId {
        let id = self
            .pool
            .alloc_expr(ExprKind::Mod(left, right), Span::EMPTY)
            .unwrap();
        self.pool.expr_mut(id).ty = Some(ty);
        id
    }

    // ============================================================================
    // Bitwise operators (Int32 only, auto-typed)
    // ============================================================================

    /// Bitwise AND: left & right
    pub fn bitwise_and(&mut self, left: ExprId, right: ExprId, ty: Type) -> ExprId {
        let id = self
            .pool
            .alloc_expr(ExprKind::BitwiseAnd(left, right), Span::EMPTY)
            .unwrap();
        self.pool.expr_mut(id).ty = Some(ty);
        id
    }

    /// Bitwise OR: left | right
    pub fn bitwise_or(&mut self, left: ExprId, right: ExprId, ty: Type) -> ExprId {
        let id = self
            .pool
            .alloc_expr(ExprKind::BitwiseOr(left, right), Span::EMPTY)
            .unwrap();
        self.pool.expr_mut(id).ty = Some(ty);
        id
    }

    /// Bitwise XOR: left ^ right
    pub fn bitwise_xor(&mut self, left: ExprId, right: ExprId, ty: Type) -> ExprId {
        let id = self
            .pool
            .alloc_expr(ExprKind::BitwiseXor(left, right), Span::EMPTY)
            .unwrap();
        self.pool.expr_mut(id).ty = Some(ty);
        id
    }

    /// Bitwise NOT: ~operand
    pub fn bitwise_not(&mut self, operand: ExprId, ty: Type) -> ExprId {
        let id = self
            .pool
            .alloc_expr(ExprKind::BitwiseNot(operand), Span::EMPTY)
            .unwrap();
        self.pool.expr_mut(id).ty = Some(ty);
        id
    }

    /// Left shift: left << right
    pub fn left_shift(&mut self, left: ExprId, right: ExprId, ty: Type) -> ExprId {
        let id = self
            .pool
            .alloc_expr(ExprKind::LeftShift(left, right), Span::EMPTY)
            .unwrap();
        self.pool.expr_mut(id).ty = Some(ty);
        id
    }

    /// Right shift: left >> right
    pub fn right_shift(&mut self, left: ExprId, right: ExprId, ty: Type) -> ExprId {
        let id = self
            .pool
            .alloc_expr(ExprKind::RightShift(left, right), Span::EMPTY)
            .unwrap();
        self.pool.expr_mut(id).ty = Some(ty);
        id
    }

    // ============================================================================
    // Logical operators (Bool, auto-typed)
    // ============================================================================

    /// Logical AND: left && right
    pub fn logical_and(&mut self, left: ExprId, right: ExprId) -> ExprId {
        let id = self
            .pool
            .alloc_expr(ExprKind::And(left, right), Span::EMPTY)
            .unwrap();
        self.pool.expr_mut(id).ty = Some(Type::Bool);
        id
    }

    /// Logical OR: left || right
    pub fn logical_or(&mut self, left: ExprId, right: ExprId) -> ExprId {
        let id = self
            .pool
            .alloc_expr(ExprKind::Or(left, right), Span::EMPTY)
            .unwrap();
        self.pool.expr_mut(id).ty = Some(Type::Bool);
        id
    }

    /// Logical NOT: !operand
    pub fn logical_not(&mut self, operand: ExprId) -> ExprId {
        let id = self
            .pool
            .alloc_expr(ExprKind::Not(operand), Span::EMPTY)
            .unwrap();
        self.pool.expr_mut(id).ty = Some(Type::Bool);
        id
    }

    // ============================================================================
    // Unary operators
    // ============================================================================

    /// Negation: -operand
    pub fn neg(&mut self, operand: ExprId, ty: Type) -> ExprId {
        let id = self
            .pool
            .alloc_expr(ExprKind::Neg(operand), Span::EMPTY)
            .unwrap();
        self.pool.expr_mut(id).ty = Some(ty);
        id
    }

    // ============================================================================
    // Other expressions
    // ============================================================================

    /// Ternary: condition ? true_expr : false_expr
    pub fn ternary(
        &mut self,
        condition: ExprId,
        true_expr: ExprId,
        false_expr: ExprId,
        ty: Type,
    ) -> ExprId {
        let id = self
            .pool
            .alloc_expr(
                ExprKind::Ternary {
                    condition,
                    true_expr,
                    false_expr,
                },
                Span::EMPTY,
            )
            .unwrap();
        self.pool.expr_mut(id).ty = Some(ty);
        id
    }

    /// Assignment: target = value
    pub fn assign(&mut self, target: &str, value: ExprId, ty: Type) -> ExprId {
        let id = self
            .pool
            .alloc_expr(
                ExprKind::Assign {
                    target: String::from(target),
                    value,
                },
                Span::EMPTY,
            )
            .unwrap();
        self.pool.expr_mut(id).ty = Some(ty);
        id
    }

    /// Function call: name(args...)
    pub fn call(&mut self, name: &str, args: Vec<ExprId>, ty: Type) -> ExprId {
        let id = self
            .pool
            .alloc_expr(
                ExprKind::Call {
                    name: String::from(name),
                    args,
                },
                Span::EMPTY,
            )
            .unwrap();
        self.pool.expr_mut(id).ty = Some(ty);
        id
    }

    /// Vector constructor: vec2(args...)
    pub fn vec2(&mut self, args: Vec<ExprId>) -> ExprId {
        let id = self
            .pool
            .alloc_expr(ExprKind::Vec2Constructor(args), Span::EMPTY)
            .unwrap();
        self.pool.expr_mut(id).ty = Some(Type::Vec2);
        id
    }

    /// Vector constructor: vec3(args...)
    pub fn vec3(&mut self, args: Vec<ExprId>) -> ExprId {
        let id = self
            .pool
            .alloc_expr(ExprKind::Vec3Constructor(args), Span::EMPTY)
            .unwrap();
        self.pool.expr_mut(id).ty = Some(Type::Vec3);
        id
    }

    /// Vector constructor: vec4(args...)
    pub fn vec4(&mut self, args: Vec<ExprId>) -> ExprId {
        let id = self
            .pool
            .alloc_expr(ExprKind::Vec4Constructor(args), Span::EMPTY)
            .unwrap();
        self.pool.expr_mut(id).ty = Some(Type::Vec4);
        id
    }

    /// Swizzle: expr.components
    pub fn swizzle(&mut self, expr: ExprId, components: &str, ty: Type) -> ExprId {
        let id = self
            .pool
            .alloc_expr(
                ExprKind::Swizzle {
                    expr,
                    components: String::from(components),
                },
                Span::EMPTY,
            )
            .unwrap();
        self.pool.expr_mut(id).ty = Some(ty);
        id
    }
}

impl Default for AstBuilder {
    fn default() -> Self {
        Self::new()
    }
}
