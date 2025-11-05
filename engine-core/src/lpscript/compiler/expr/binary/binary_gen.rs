/// Binary operation code generation
extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::lpscript::compiler::ast::Expr;
use crate::lpscript::compiler::codegen::CodeGenerator;
use crate::lpscript::shared::Type;
use crate::lpscript::vm::opcodes::LpsOpCode;

/// Binary operation types
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_add(&mut self, left: &Box<Expr>, right: &Box<Expr>, result_ty: &Type) {
        self.gen_expr(left);
        self.gen_expr(right);
        gen_binary_op(
            BinaryOp::Add,
            left.ty.as_ref().unwrap(),
            right.ty.as_ref().unwrap(),
            result_ty,
            self.code,
        );
    }

    pub(crate) fn gen_sub(&mut self, left: &Box<Expr>, right: &Box<Expr>, result_ty: &Type) {
        self.gen_expr(left);
        self.gen_expr(right);
        gen_binary_op(
            BinaryOp::Sub,
            left.ty.as_ref().unwrap(),
            right.ty.as_ref().unwrap(),
            result_ty,
            self.code,
        );
    }

    pub(crate) fn gen_mul(&mut self, left: &Box<Expr>, right: &Box<Expr>, result_ty: &Type) {
        self.gen_expr(left);
        self.gen_expr(right);
        gen_binary_op(
            BinaryOp::Mul,
            left.ty.as_ref().unwrap(),
            right.ty.as_ref().unwrap(),
            result_ty,
            self.code,
        );
    }

    pub(crate) fn gen_div(&mut self, left: &Box<Expr>, right: &Box<Expr>, result_ty: &Type) {
        self.gen_expr(left);
        self.gen_expr(right);
        gen_binary_op(
            BinaryOp::Div,
            left.ty.as_ref().unwrap(),
            right.ty.as_ref().unwrap(),
            result_ty,
            self.code,
        );
    }

    pub(crate) fn gen_mod(&mut self, left: &Box<Expr>, right: &Box<Expr>, result_ty: &Type) {
        self.gen_expr(left);
        self.gen_expr(right);
        gen_binary_op(
            BinaryOp::Mod,
            left.ty.as_ref().unwrap(),
            right.ty.as_ref().unwrap(),
            result_ty,
            self.code,
        );
    }

    pub(crate) fn gen_pow(&mut self, left: &Box<Expr>, right: &Box<Expr>) {
        self.gen_expr(left);
        self.gen_expr(right);
        // Pow is always scalar for now (pow only works on Fixed types)
        self.code.push(LpsOpCode::PowFixed);
    }
}

/// Generate typed binary operation based on operand and result types
fn gen_binary_op(
    op: BinaryOp,
    _left_ty: &Type,
    right_ty: &Type,
    result_ty: &Type,
    code: &mut Vec<LpsOpCode>,
) {
    let _ = _left_ty; // Silence unused variable warning
    match (op, result_ty) {
        // Fixed operations
        (BinaryOp::Add, Type::Fixed | Type::Int32) => code.push(LpsOpCode::AddFixed),
        (BinaryOp::Sub, Type::Fixed | Type::Int32) => code.push(LpsOpCode::SubFixed),
        (BinaryOp::Mul, Type::Fixed | Type::Int32) => code.push(LpsOpCode::MulFixed),
        (BinaryOp::Div, Type::Fixed | Type::Int32) => code.push(LpsOpCode::DivFixed),
        (BinaryOp::Mod, Type::Fixed | Type::Int32) => {
            // Modulo operation for fixed-point numbers
            code.push(LpsOpCode::ModFixed);
        }

        // Vec2 operations
        (BinaryOp::Add, Type::Vec2) => code.push(LpsOpCode::AddVec2),
        (BinaryOp::Sub, Type::Vec2) => code.push(LpsOpCode::SubVec2),
        (BinaryOp::Mul, Type::Vec2) => {
            // Check if it's vec * scalar or vec * vec
            if matches!(right_ty, Type::Fixed | Type::Int32) {
                code.push(LpsOpCode::MulVec2Scalar);
            } else {
                code.push(LpsOpCode::MulVec2);
            }
        }
        (BinaryOp::Div, Type::Vec2) => {
            if matches!(right_ty, Type::Fixed | Type::Int32) {
                code.push(LpsOpCode::DivVec2Scalar);
            } else {
                code.push(LpsOpCode::DivVec2);
            }
        }
        (BinaryOp::Mod, Type::Vec2) => {
            // Vec2 modulo not commonly supported, but we can implement component-wise
            // For now, this is a placeholder - proper implementation would need ModVec2 opcode
            code.push(LpsOpCode::MulVec2);
        }

        // Vec3 operations
        (BinaryOp::Add, Type::Vec3) => code.push(LpsOpCode::AddVec3),
        (BinaryOp::Sub, Type::Vec3) => code.push(LpsOpCode::SubVec3),
        (BinaryOp::Mul, Type::Vec3) => {
            if matches!(right_ty, Type::Fixed | Type::Int32) {
                code.push(LpsOpCode::MulVec3Scalar);
            } else {
                code.push(LpsOpCode::MulVec3);
            }
        }
        (BinaryOp::Div, Type::Vec3) => {
            if matches!(right_ty, Type::Fixed | Type::Int32) {
                code.push(LpsOpCode::DivVec3Scalar);
            } else {
                code.push(LpsOpCode::DivVec3);
            }
        }
        (BinaryOp::Mod, Type::Vec3) => {
            // Vec3 modulo not commonly supported, but we can implement component-wise
            // For now, this is a placeholder - proper implementation would need ModVec3 opcode
            code.push(LpsOpCode::MulVec3);
        }

        // Vec4 operations
        (BinaryOp::Add, Type::Vec4) => code.push(LpsOpCode::AddVec4),
        (BinaryOp::Sub, Type::Vec4) => code.push(LpsOpCode::SubVec4),
        (BinaryOp::Mul, Type::Vec4) => {
            if matches!(right_ty, Type::Fixed | Type::Int32) {
                code.push(LpsOpCode::MulVec4Scalar);
            } else {
                code.push(LpsOpCode::MulVec4);
            }
        }
        (BinaryOp::Div, Type::Vec4) => {
            if matches!(right_ty, Type::Fixed | Type::Int32) {
                code.push(LpsOpCode::DivVec4Scalar);
            } else {
                code.push(LpsOpCode::DivVec4);
            }
        }
        (BinaryOp::Mod, Type::Vec4) => {
            // Vec4 modulo not commonly supported, but we can implement component-wise
            // For now, this is a placeholder - proper implementation would need ModVec4 opcode
            code.push(LpsOpCode::MulVec4);
        }

        _ => {} // Void or unsupported
    }
}
