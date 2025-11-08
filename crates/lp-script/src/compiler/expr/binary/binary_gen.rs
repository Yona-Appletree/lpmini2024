/// Binary operation code generation
extern crate alloc;

use crate::compiler::ast::{AstPool, ExprId};
use crate::compiler::codegen::CodeGenerator;
use crate::shared::Type;
use crate::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_add_id(&mut self, pool: &AstPool, left: ExprId, right: ExprId, ty: &Type) {
        self.gen_expr_id(pool, left);
        self.gen_expr_id(pool, right);
        self.code.push(match ty {
            Type::Fixed => LpsOpCode::AddFixed,
            Type::Int32 => LpsOpCode::AddInt32,
            Type::Vec2 => LpsOpCode::AddVec2,
            Type::Vec3 => LpsOpCode::AddVec3,
            Type::Vec4 => LpsOpCode::AddVec4,
            _ => LpsOpCode::AddFixed,
        });
    }

    pub(crate) fn gen_sub_id(&mut self, pool: &AstPool, left: ExprId, right: ExprId, ty: &Type) {
        self.gen_expr_id(pool, left);
        self.gen_expr_id(pool, right);
        self.code.push(match ty {
            Type::Fixed => LpsOpCode::SubFixed,
            Type::Int32 => LpsOpCode::SubInt32,
            Type::Vec2 => LpsOpCode::SubVec2,
            Type::Vec3 => LpsOpCode::SubVec3,
            Type::Vec4 => LpsOpCode::SubVec4,
            _ => LpsOpCode::SubFixed,
        });
    }

    pub(crate) fn gen_mul_id(&mut self, pool: &AstPool, left: ExprId, right: ExprId, ty: &Type) {
        let left_ty = pool.expr(left).ty.as_ref().unwrap();
        let right_ty = pool.expr(right).ty.as_ref().unwrap();

        // For scalar-vector operations, generate in reverse order to get correct stack layout
        let is_scalar_vector = matches!(
            (left_ty, right_ty),
            (
                Type::Fixed | Type::Int32,
                Type::Vec2 | Type::Vec3 | Type::Vec4
            )
        );

        if is_scalar_vector {
            // Generate: scalar * vector -> [vec_components..., scalar]
            self.gen_expr_id(pool, right); // Vector first
            self.gen_expr_id(pool, left); // Scalar on top
        } else {
            // Normal order
            self.gen_expr_id(pool, left);
            self.gen_expr_id(pool, right);
        }

        // Emit appropriate opcode
        let opcode = match (left_ty, right_ty, ty) {
            // Scalar operations
            (Type::Fixed, Type::Fixed, Type::Fixed) => LpsOpCode::MulFixed,
            (Type::Int32, Type::Int32, Type::Int32) => LpsOpCode::MulInt32,

            // Vector-Vector operations
            (Type::Vec2, Type::Vec2, Type::Vec2) => LpsOpCode::MulVec2,
            (Type::Vec3, Type::Vec3, Type::Vec3) => LpsOpCode::MulVec3,
            (Type::Vec4, Type::Vec4, Type::Vec4) => LpsOpCode::MulVec4,

            // Vector-Scalar operations
            (Type::Vec2, Type::Fixed | Type::Int32, Type::Vec2) => LpsOpCode::MulVec2Scalar,
            (Type::Vec3, Type::Fixed | Type::Int32, Type::Vec3) => LpsOpCode::MulVec3Scalar,
            (Type::Vec4, Type::Fixed | Type::Int32, Type::Vec4) => LpsOpCode::MulVec4Scalar,

            // Scalar-Vector operations (already generated in correct order)
            (Type::Fixed | Type::Int32, Type::Vec2, Type::Vec2) => LpsOpCode::MulVec2Scalar,
            (Type::Fixed | Type::Int32, Type::Vec3, Type::Vec3) => LpsOpCode::MulVec3Scalar,
            (Type::Fixed | Type::Int32, Type::Vec4, Type::Vec4) => LpsOpCode::MulVec4Scalar,

            _ => LpsOpCode::MulFixed, // Fallback
        };

        self.code.push(opcode);
    }

    pub(crate) fn gen_div_id(&mut self, pool: &AstPool, left: ExprId, right: ExprId, ty: &Type) {
        let left_ty = pool.expr(left).ty.as_ref().unwrap();
        let right_ty = pool.expr(right).ty.as_ref().unwrap();

        self.gen_expr_id(pool, left);
        self.gen_expr_id(pool, right);

        // Emit appropriate opcode
        let opcode = match (left_ty, right_ty, ty) {
            // Scalar operations
            (Type::Fixed, Type::Fixed, Type::Fixed) => LpsOpCode::DivFixed,
            (Type::Int32, Type::Int32, Type::Int32) => LpsOpCode::DivInt32,

            // Vector-Vector operations
            (Type::Vec2, Type::Vec2, Type::Vec2) => LpsOpCode::DivVec2,
            (Type::Vec3, Type::Vec3, Type::Vec3) => LpsOpCode::DivVec3,
            (Type::Vec4, Type::Vec4, Type::Vec4) => LpsOpCode::DivVec4,

            // Vector-Scalar operations (vec / scalar)
            (Type::Vec2, Type::Fixed | Type::Int32, Type::Vec2) => LpsOpCode::DivVec2Scalar,
            (Type::Vec3, Type::Fixed | Type::Int32, Type::Vec3) => LpsOpCode::DivVec3Scalar,
            (Type::Vec4, Type::Fixed | Type::Int32, Type::Vec4) => LpsOpCode::DivVec4Scalar,

            _ => LpsOpCode::DivFixed, // Fallback
        };

        self.code.push(opcode);
    }

    pub(crate) fn gen_mod_id(&mut self, pool: &AstPool, left: ExprId, right: ExprId, ty: &Type) {
        self.gen_expr_id(pool, left);
        self.gen_expr_id(pool, right);
        self.code.push(match ty {
            Type::Fixed => LpsOpCode::ModFixed,
            Type::Int32 => LpsOpCode::ModInt32,
            Type::Vec2 => LpsOpCode::ModVec2,
            Type::Vec3 => LpsOpCode::ModVec3,
            Type::Vec4 => LpsOpCode::ModVec4,
            _ => LpsOpCode::ModFixed,
        });
    }
}
