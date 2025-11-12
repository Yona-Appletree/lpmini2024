/// Binary operation code generation
extern crate alloc;

use crate::compiler::ast::Expr;
use crate::compiler::codegen::CodeGenerator;
use crate::shared::Type;
use crate::vm::opcodes::LpsOpCode;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_add(&mut self, left: &Expr, right: &Expr, ty: &Type) {
        self.gen_expr(left);
        self.gen_expr(right);
        self.code.push(match ty {
            Type::Fixed => LpsOpCode::AddFixed,
            Type::Int32 => LpsOpCode::AddInt32,
            Type::Vec2 => LpsOpCode::AddVec2,
            Type::Vec3 => LpsOpCode::AddVec3,
            Type::Vec4 => LpsOpCode::AddVec4,
            Type::Mat3 => LpsOpCode::AddMat3,
            _ => LpsOpCode::AddFixed,
        });
    }

    pub(crate) fn gen_sub(&mut self, left: &Expr, right: &Expr, ty: &Type) {
        self.gen_expr(left);
        self.gen_expr(right);
        self.code.push(match ty {
            Type::Fixed => LpsOpCode::SubFixed,
            Type::Int32 => LpsOpCode::SubInt32,
            Type::Vec2 => LpsOpCode::SubVec2,
            Type::Vec3 => LpsOpCode::SubVec3,
            Type::Vec4 => LpsOpCode::SubVec4,
            Type::Mat3 => LpsOpCode::SubMat3,
            _ => LpsOpCode::SubFixed,
        });
    }

    pub(crate) fn gen_mul(&mut self, left: &Expr, right: &Expr, ty: &Type) {
        let left_ty = left.ty.as_ref().unwrap();
        let right_ty = right.ty.as_ref().unwrap();

        // For scalar-vector/matrix operations, generate in reverse order to get correct stack layout
        let is_scalar_vector = matches!(
            (left_ty, right_ty),
            (
                Type::Fixed | Type::Int32,
                Type::Vec2 | Type::Vec3 | Type::Vec4 | Type::Mat3
            )
        );

        if is_scalar_vector {
            // Generate: scalar * vector -> [vec_components..., scalar]
            self.gen_expr(right); // Vector first
            self.gen_expr(left); // Scalar on top
        } else {
            // Normal order
            self.gen_expr(left);
            self.gen_expr(right);
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

            // Matrix-Matrix operations (matrix multiplication)
            (Type::Mat3, Type::Mat3, Type::Mat3) => LpsOpCode::MulMat3,

            // Vector-Scalar operations
            (Type::Vec2, Type::Fixed | Type::Int32, Type::Vec2) => LpsOpCode::MulVec2Scalar,
            (Type::Vec3, Type::Fixed | Type::Int32, Type::Vec3) => LpsOpCode::MulVec3Scalar,
            (Type::Vec4, Type::Fixed | Type::Int32, Type::Vec4) => LpsOpCode::MulVec4Scalar,

            // Matrix-Scalar operations
            (Type::Mat3, Type::Fixed | Type::Int32, Type::Mat3) => LpsOpCode::MulMat3Scalar,

            // Scalar-Vector operations (already generated in correct order)
            (Type::Fixed | Type::Int32, Type::Vec2, Type::Vec2) => LpsOpCode::MulVec2Scalar,
            (Type::Fixed | Type::Int32, Type::Vec3, Type::Vec3) => LpsOpCode::MulVec3Scalar,
            (Type::Fixed | Type::Int32, Type::Vec4, Type::Vec4) => LpsOpCode::MulVec4Scalar,

            // Scalar-Matrix operations (already generated in correct order)
            (Type::Fixed | Type::Int32, Type::Mat3, Type::Mat3) => LpsOpCode::MulMat3Scalar,

            _ => LpsOpCode::MulFixed, // Fallback
        };

        self.code.push(opcode);
    }

    pub(crate) fn gen_div(&mut self, left: &Expr, right: &Expr, ty: &Type) {
        let left_ty = left.ty.as_ref().unwrap();
        let right_ty = right.ty.as_ref().unwrap();

        self.gen_expr(left);
        self.gen_expr(right);

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

            // Matrix-Scalar operations (mat / scalar)
            (Type::Mat3, Type::Fixed | Type::Int32, Type::Mat3) => LpsOpCode::DivMat3Scalar,

            _ => LpsOpCode::DivFixed, // Fallback
        };

        self.code.push(opcode);
    }

    pub(crate) fn gen_mod(&mut self, left: &Expr, right: &Expr, ty: &Type) {
        self.gen_expr(left);
        self.gen_expr(right);
        self.code.push(match ty {
            Type::Fixed => LpsOpCode::ModFixed,
            Type::Int32 => LpsOpCode::ModInt32,
            Type::Vec2 => LpsOpCode::ModVec2,
            Type::Vec3 => LpsOpCode::ModVec3,
            Type::Vec4 => LpsOpCode::ModVec4,
            // Note: Mat3 modulo not supported (no ModMat3 opcode exists)
            _ => LpsOpCode::ModFixed,
        });
    }
}
