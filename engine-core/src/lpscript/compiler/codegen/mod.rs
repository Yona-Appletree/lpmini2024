/// Code generator: converts AST to VM opcodes
extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use crate::lpscript::compiler::ast::{Expr, Program};
use crate::lpscript::LpsOpCode;

mod expr;
pub(crate) mod local_allocator;
mod native_functions;
mod program;
mod stmt;

pub(crate) use local_allocator::LocalAllocator;
pub use native_functions::NativeFunction;

pub struct CodeGenerator<'a> {
    pub(in crate::lpscript) code: &'a mut Vec<LpsOpCode>,
    pub(in crate::lpscript) locals: &'a mut LocalAllocator,
    pub(in crate::lpscript) func_offsets: &'a BTreeMap<String, u32>,
}

impl<'a> CodeGenerator<'a> {
    /// Create a new code generator instance
    pub(crate) fn new(
        code: &'a mut Vec<LpsOpCode>,
        locals: &'a mut LocalAllocator,
        func_offsets: &'a BTreeMap<String, u32>,
    ) -> Self {
        CodeGenerator {
            code,
            locals,
            func_offsets,
        }
    }

    /// Generate opcodes for an expression (expression mode)
    pub fn generate(pool: &crate::lpscript::compiler::ast::AstPool, expr_id: crate::lpscript::compiler::ast::ExprId) -> Vec<LpsOpCode> {
        Self::generate_with_locals(pool, expr_id, Vec::new())
    }

    /// Generate opcodes for an expression with pre-declared local variables
    ///
    /// This is useful for testing assignment expressions which need mutable locals.
    /// The locals should be ordered by index (e.g., [("x", 0), ("y", 1), ...])
    pub fn generate_with_locals(pool: &crate::lpscript::compiler::ast::AstPool, expr_id: crate::lpscript::compiler::ast::ExprId, predeclared: Vec<(String, u32, crate::lpscript::shared::Type)>) -> Vec<LpsOpCode> {
        let mut code = Vec::new();
        let mut locals = LocalAllocator::new();
        let func_offsets = BTreeMap::new(); // Empty for expression mode

        // Pre-allocate declared locals in order with their types
        for (name, expected_index, ty) in predeclared {
            let actual_index = locals.allocate_typed(name, ty);
            // Verify indices match (should always be true if called correctly)
            assert_eq!(
                actual_index, expected_index,
                "Local index mismatch during pre-allocation"
            );
        }

        let mut gen = CodeGenerator::new(&mut code, &mut locals, &func_offsets);
        gen.gen_expr_id(pool, expr_id);
        gen.code.push(LpsOpCode::Return);

        code
    }

    /// Generate opcodes for a program (script mode)
    /// Returns (opcodes, local_count, local_types) tuple
    pub fn generate_program(
        pool: &crate::lpscript::compiler::ast::AstPool,
        program: &Program,
    ) -> (
        Vec<LpsOpCode>,
        u32,
        alloc::collections::BTreeMap<u32, crate::lpscript::shared::Type>,
    ) {
        program::gen_program(pool, program, |pool, stmt_id, code, locals, func_offsets| {
            let mut gen = CodeGenerator::new(code, locals, func_offsets);
            gen.gen_stmt_id(pool, stmt_id);
        })
    }

    // Expression code generation by ID - main dispatcher
    pub(in crate::lpscript) fn gen_expr_id(
        &mut self,
        pool: &crate::lpscript::compiler::ast::AstPool,
        expr_id: crate::lpscript::compiler::ast::ExprId,
    ) {
        use crate::lpscript::compiler::ast::ExprKind;
        
        let expr = pool.expr(expr_id);
        let expr_ty = expr.ty.as_ref();
        
        match &expr.kind {
            ExprKind::Number(n) => self.gen_number(*n),
            ExprKind::IntNumber(n) => {
                self.gen_int_number(*n);
                // If Int32 was promoted to Fixed, emit conversion
                if expr_ty == Some(&crate::lpscript::shared::Type::Fixed) {
                    self.code.push(LpsOpCode::Int32ToFixed);
                }
            }
            ExprKind::Variable(name) => {
                if let Some(ty) = expr_ty {
                    self.gen_variable(name, ty);
                }
            }

            ExprKind::Add(left, right) => {
                if let Some(ty) = expr_ty {
                    self.gen_add_id(pool, *left, *right, ty);
                }
            }
            ExprKind::Sub(left, right) => {
                if let Some(ty) = expr_ty {
                    self.gen_sub_id(pool, *left, *right, ty);
                }
            }
            ExprKind::Mul(left, right) => {
                if let Some(ty) = expr_ty {
                    self.gen_mul_id(pool, *left, *right, ty);
                }
            }
            ExprKind::Div(left, right) => {
                if let Some(ty) = expr_ty {
                    self.gen_div_id(pool, *left, *right, ty);
                }
            }
            ExprKind::Mod(left, right) => {
                if let Some(ty) = expr_ty {
                    self.gen_mod_id(pool, *left, *right, ty);
                }
            }

            ExprKind::BitwiseAnd(left, right) => self.gen_bitwise_and_id(pool, *left, *right),
            ExprKind::BitwiseOr(left, right) => self.gen_bitwise_or_id(pool, *left, *right),
            ExprKind::BitwiseXor(left, right) => self.gen_bitwise_xor_id(pool, *left, *right),
            ExprKind::BitwiseNot(operand) => self.gen_bitwise_not_id(pool, *operand),
            ExprKind::LeftShift(left, right) => self.gen_left_shift_id(pool, *left, *right),
            ExprKind::RightShift(left, right) => self.gen_right_shift_id(pool, *left, *right),

            ExprKind::Less(left, right) => self.gen_less_id(pool, *left, *right),
            ExprKind::Greater(left, right) => self.gen_greater_id(pool, *left, *right),
            ExprKind::LessEq(left, right) => self.gen_less_eq_id(pool, *left, *right),
            ExprKind::GreaterEq(left, right) => self.gen_greater_eq_id(pool, *left, *right),
            ExprKind::Eq(left, right) => self.gen_eq_id(pool, *left, *right),
            ExprKind::NotEq(left, right) => self.gen_not_eq_id(pool, *left, *right),

            ExprKind::And(left, right) => self.gen_and_id(pool, *left, *right),
            ExprKind::Or(left, right) => self.gen_or_id(pool, *left, *right),
            ExprKind::Not(operand) => self.gen_not_id(pool, *operand),

            ExprKind::Neg(operand) => self.gen_neg_id(pool, *operand),

            ExprKind::PreIncrement(var_name) => {
                if let Some(ty) = expr_ty {
                    self.gen_pre_increment(var_name, ty);
                }
            }
            ExprKind::PreDecrement(var_name) => {
                if let Some(ty) = expr_ty {
                    self.gen_pre_decrement(var_name, ty);
                }
            }
            ExprKind::PostIncrement(var_name) => {
                if let Some(ty) = expr_ty {
                    self.gen_post_increment(var_name, ty);
                }
            }
            ExprKind::PostDecrement(var_name) => {
                if let Some(ty) = expr_ty {
                    self.gen_post_decrement(var_name, ty);
                }
            }

            ExprKind::Ternary {
                condition,
                true_expr,
                false_expr,
            } => self.gen_ternary_id(pool, *condition, *true_expr, *false_expr),

            ExprKind::Assign { target, value } => self.gen_assign_expr_id(pool, target, *value),

            ExprKind::Call { name, args } => self.gen_function_call_id(pool, name, args),

            ExprKind::Vec2Constructor(args)
            | ExprKind::Vec3Constructor(args)
            | ExprKind::Vec4Constructor(args) => self.gen_vec_constructor_id(pool, args),

            ExprKind::Swizzle { expr, components } => self.gen_swizzle_id(pool, *expr, components),
        }
    }

    // Statement code generation by ID - main dispatcher
    pub(in crate::lpscript) fn gen_stmt_id(
        &mut self,
        pool: &crate::lpscript::compiler::ast::AstPool,
        stmt_id: crate::lpscript::compiler::ast::StmtId,
    ) {
        use crate::lpscript::compiler::ast::StmtKind;
        
        let stmt = pool.stmt(stmt_id);
        
        match &stmt.kind {
            StmtKind::VarDecl { ty, name, init } => {
                self.gen_var_decl_id(pool, ty, name, init);
            }
            StmtKind::Return(expr_id) => {
                self.gen_expr_id(pool, *expr_id);
                self.code.push(LpsOpCode::Return);
            }
            StmtKind::Expr(expr_id) => {
                self.gen_expr_id(pool, *expr_id);
                // Expression statements discard their result
                self.code.push(LpsOpCode::Drop1);
            }
            StmtKind::Block(stmts) => {
                self.gen_block_id(pool, stmts);
            }
            StmtKind::If {
                condition,
                then_stmt,
                else_stmt,
            } => {
                self.gen_if_stmt_id(pool, *condition, *then_stmt, *else_stmt);
            }
            StmtKind::While { condition, body } => {
                self.gen_while_stmt_id(pool, *condition, *body);
            }
            StmtKind::For {
                init,
                condition,
                increment,
                body,
            } => {
                self.gen_for_stmt_id(pool, init, condition, increment, *body);
            }
        }
    }

    // Stub implementations for ID-based codegen methods
    // These will be properly implemented by updating the individual codegen modules
    
    fn gen_add_id(&mut self, pool: &crate::lpscript::compiler::ast::AstPool, left: crate::lpscript::compiler::ast::ExprId, right: crate::lpscript::compiler::ast::ExprId, ty: &crate::lpscript::shared::Type) {
        self.gen_expr_id(pool, left);
        self.gen_expr_id(pool, right);
        self.code.push(match ty {
            crate::lpscript::shared::Type::Fixed => LpsOpCode::AddFixed,
            crate::lpscript::shared::Type::Int32 => LpsOpCode::AddInt32,
            crate::lpscript::shared::Type::Vec2 => LpsOpCode::AddVec2,
            crate::lpscript::shared::Type::Vec3 => LpsOpCode::AddVec3,
            crate::lpscript::shared::Type::Vec4 => LpsOpCode::AddVec4,
            _ => LpsOpCode::AddFixed,
        });
    }

    fn gen_sub_id(&mut self, pool: &crate::lpscript::compiler::ast::AstPool, left: crate::lpscript::compiler::ast::ExprId, right: crate::lpscript::compiler::ast::ExprId, ty: &crate::lpscript::shared::Type) {
        self.gen_expr_id(pool, left);
        self.gen_expr_id(pool, right);
        self.code.push(match ty {
            crate::lpscript::shared::Type::Fixed => LpsOpCode::SubFixed,
            crate::lpscript::shared::Type::Int32 => LpsOpCode::SubInt32,
            crate::lpscript::shared::Type::Vec2 => LpsOpCode::SubVec2,
            crate::lpscript::shared::Type::Vec3 => LpsOpCode::SubVec3,
            crate::lpscript::shared::Type::Vec4 => LpsOpCode::SubVec4,
            _ => LpsOpCode::SubFixed,
        });
    }

    fn gen_mul_id(&mut self, pool: &crate::lpscript::compiler::ast::AstPool, left: crate::lpscript::compiler::ast::ExprId, right: crate::lpscript::compiler::ast::ExprId, ty: &crate::lpscript::shared::Type) {
        use crate::lpscript::shared::Type;
        
        let left_ty = pool.expr(left).ty.as_ref().unwrap();
        let right_ty = pool.expr(right).ty.as_ref().unwrap();
        
        // For scalar-vector operations, generate in reverse order to get correct stack layout
        let is_scalar_vector = matches!((left_ty, right_ty), 
            (Type::Fixed | Type::Int32, Type::Vec2 | Type::Vec3 | Type::Vec4));
        
        if is_scalar_vector {
            // Generate: scalar * vector -> [vec_components..., scalar]
            self.gen_expr_id(pool, right); // Vector first
            self.gen_expr_id(pool, left);  // Scalar on top
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

    fn gen_div_id(&mut self, pool: &crate::lpscript::compiler::ast::AstPool, left: crate::lpscript::compiler::ast::ExprId, right: crate::lpscript::compiler::ast::ExprId, ty: &crate::lpscript::shared::Type) {
        use crate::lpscript::shared::Type;
        
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

    fn gen_mod_id(&mut self, pool: &crate::lpscript::compiler::ast::AstPool, left: crate::lpscript::compiler::ast::ExprId, right: crate::lpscript::compiler::ast::ExprId, ty: &crate::lpscript::shared::Type) {
        self.gen_expr_id(pool, left);
        self.gen_expr_id(pool, right);
        self.code.push(match ty {
            crate::lpscript::shared::Type::Fixed => LpsOpCode::ModFixed,
            crate::lpscript::shared::Type::Int32 => LpsOpCode::ModInt32,
            crate::lpscript::shared::Type::Vec2 => LpsOpCode::ModVec2,
            crate::lpscript::shared::Type::Vec3 => LpsOpCode::ModVec3,
            crate::lpscript::shared::Type::Vec4 => LpsOpCode::ModVec4,
            _ => LpsOpCode::ModFixed,
        });
    }

    fn gen_bitwise_and_id(&mut self, pool: &crate::lpscript::compiler::ast::AstPool, left: crate::lpscript::compiler::ast::ExprId, right: crate::lpscript::compiler::ast::ExprId) {
        self.gen_expr_id(pool, left);
        self.gen_expr_id(pool, right);
        self.code.push(LpsOpCode::BitwiseAndInt32);
    }

    fn gen_bitwise_or_id(&mut self, pool: &crate::lpscript::compiler::ast::AstPool, left: crate::lpscript::compiler::ast::ExprId, right: crate::lpscript::compiler::ast::ExprId) {
        self.gen_expr_id(pool, left);
        self.gen_expr_id(pool, right);
        self.code.push(LpsOpCode::BitwiseOrInt32);
    }

    fn gen_bitwise_xor_id(&mut self, pool: &crate::lpscript::compiler::ast::AstPool, left: crate::lpscript::compiler::ast::ExprId, right: crate::lpscript::compiler::ast::ExprId) {
        self.gen_expr_id(pool, left);
        self.gen_expr_id(pool, right);
        self.code.push(LpsOpCode::BitwiseXorInt32);
    }

    fn gen_bitwise_not_id(&mut self, pool: &crate::lpscript::compiler::ast::AstPool, operand: crate::lpscript::compiler::ast::ExprId) {
        self.gen_expr_id(pool, operand);
        self.code.push(LpsOpCode::BitwiseNotInt32);
    }

    fn gen_left_shift_id(&mut self, pool: &crate::lpscript::compiler::ast::AstPool, left: crate::lpscript::compiler::ast::ExprId, right: crate::lpscript::compiler::ast::ExprId) {
        self.gen_expr_id(pool, left);
        self.gen_expr_id(pool, right);
        self.code.push(LpsOpCode::LeftShiftInt32);
    }

    fn gen_right_shift_id(&mut self, pool: &crate::lpscript::compiler::ast::AstPool, left: crate::lpscript::compiler::ast::ExprId, right: crate::lpscript::compiler::ast::ExprId) {
        self.gen_expr_id(pool, left);
        self.gen_expr_id(pool, right);
        self.code.push(LpsOpCode::RightShiftInt32);
    }

    fn gen_less_id(&mut self, pool: &crate::lpscript::compiler::ast::AstPool, left: crate::lpscript::compiler::ast::ExprId, right: crate::lpscript::compiler::ast::ExprId) {
        self.gen_expr_id(pool, left);
        self.gen_expr_id(pool, right);
        self.code.push(LpsOpCode::LessFixed);
    }

    fn gen_greater_id(&mut self, pool: &crate::lpscript::compiler::ast::AstPool, left: crate::lpscript::compiler::ast::ExprId, right: crate::lpscript::compiler::ast::ExprId) {
        self.gen_expr_id(pool, left);
        self.gen_expr_id(pool, right);
        self.code.push(LpsOpCode::GreaterFixed);
    }

    fn gen_less_eq_id(&mut self, pool: &crate::lpscript::compiler::ast::AstPool, left: crate::lpscript::compiler::ast::ExprId, right: crate::lpscript::compiler::ast::ExprId) {
        self.gen_expr_id(pool, left);
        self.gen_expr_id(pool, right);
        self.code.push(LpsOpCode::LessEqFixed);
    }

    fn gen_greater_eq_id(&mut self, pool: &crate::lpscript::compiler::ast::AstPool, left: crate::lpscript::compiler::ast::ExprId, right: crate::lpscript::compiler::ast::ExprId) {
        self.gen_expr_id(pool, left);
        self.gen_expr_id(pool, right);
        self.code.push(LpsOpCode::GreaterEqFixed);
    }

    fn gen_eq_id(&mut self, pool: &crate::lpscript::compiler::ast::AstPool, left: crate::lpscript::compiler::ast::ExprId, right: crate::lpscript::compiler::ast::ExprId) {
        self.gen_expr_id(pool, left);
        self.gen_expr_id(pool, right);
        self.code.push(LpsOpCode::EqFixed);
    }

    fn gen_not_eq_id(&mut self, pool: &crate::lpscript::compiler::ast::AstPool, left: crate::lpscript::compiler::ast::ExprId, right: crate::lpscript::compiler::ast::ExprId) {
        self.gen_expr_id(pool, left);
        self.gen_expr_id(pool, right);
        self.code.push(LpsOpCode::NotEqFixed);
    }

    fn gen_and_id(&mut self, pool: &crate::lpscript::compiler::ast::AstPool, left: crate::lpscript::compiler::ast::ExprId, right: crate::lpscript::compiler::ast::ExprId) {
        self.gen_expr_id(pool, left);
        self.gen_expr_id(pool, right);
        self.code.push(LpsOpCode::AndFixed);
    }

    fn gen_or_id(&mut self, pool: &crate::lpscript::compiler::ast::AstPool, left: crate::lpscript::compiler::ast::ExprId, right: crate::lpscript::compiler::ast::ExprId) {
        self.gen_expr_id(pool, left);
        self.gen_expr_id(pool, right);
        self.code.push(LpsOpCode::OrFixed);
    }

    fn gen_not_id(&mut self, pool: &crate::lpscript::compiler::ast::AstPool, operand: crate::lpscript::compiler::ast::ExprId) {
        self.gen_expr_id(pool, operand);
        self.code.push(LpsOpCode::NotFixed);
    }

    fn gen_neg_id(&mut self, pool: &crate::lpscript::compiler::ast::AstPool, operand: crate::lpscript::compiler::ast::ExprId) {
        self.gen_expr_id(pool, operand);
        let operand_ty = pool.expr(operand).ty.as_ref();
        self.code.push(match operand_ty {
            Some(crate::lpscript::shared::Type::Int32) => LpsOpCode::NegInt32,
            _ => LpsOpCode::NegFixed,
        });
    }

    fn gen_ternary_id(&mut self, pool: &crate::lpscript::compiler::ast::AstPool, condition: crate::lpscript::compiler::ast::ExprId, true_expr: crate::lpscript::compiler::ast::ExprId, false_expr: crate::lpscript::compiler::ast::ExprId) {
        // Generate condition
        self.gen_expr_id(pool, condition);
        
        // JumpIfZero to false branch
        let jump_to_false = self.code.len();
        self.code.push(LpsOpCode::JumpIfZero(0)); // Placeholder
        
        // True branch
        self.gen_expr_id(pool, true_expr);
        let jump_to_end = self.code.len();
        self.code.push(LpsOpCode::Jump(0)); // Placeholder
        
        // Patch jump to false
        let false_start = self.code.len();
        if let LpsOpCode::JumpIfZero(ref mut offset) = self.code[jump_to_false] {
            *offset = (false_start as i32) - (jump_to_false as i32) - 1;
        }
        
        // False branch
        self.gen_expr_id(pool, false_expr);
        
        // Patch jump to end
        let end = self.code.len();
        if let LpsOpCode::Jump(ref mut offset) = self.code[jump_to_end] {
            *offset = (end as i32) - (jump_to_end as i32) - 1;
        }
    }

    fn gen_assign_expr_id(&mut self, pool: &crate::lpscript::compiler::ast::AstPool, target: &str, value: crate::lpscript::compiler::ast::ExprId) {
        use crate::lpscript::shared::Type;
        
        self.gen_expr_id(pool, value);
        
        if let Some(local_idx) = self.locals.get(target) {
            let var_type = self.locals.get_type(local_idx).unwrap_or(&Type::Fixed);
            
            // Duplicate value based on type (assignment returns the assigned value)
            match var_type {
                Type::Vec2 => self.code.push(LpsOpCode::Dup2),
                Type::Vec3 => self.code.push(LpsOpCode::Dup3),
                Type::Vec4 => self.code.push(LpsOpCode::Dup4),
                _ => self.code.push(LpsOpCode::Dup1),
            }
            
            // Store using type-specific opcode
            self.code.push(match var_type {
                Type::Fixed | Type::Bool => LpsOpCode::StoreLocalFixed(local_idx),
                Type::Int32 => LpsOpCode::StoreLocalInt32(local_idx),
                Type::Vec2 => LpsOpCode::StoreLocalVec2(local_idx),
                Type::Vec3 => LpsOpCode::StoreLocalVec3(local_idx),
                Type::Vec4 => LpsOpCode::StoreLocalVec4(local_idx),
                _ => LpsOpCode::StoreLocalFixed(local_idx),
            });
        }
    }

    fn gen_function_call_id(&mut self, pool: &crate::lpscript::compiler::ast::AstPool, name: &str, args: &[crate::lpscript::compiler::ast::ExprId]) {
        use crate::lpscript::compiler::ast::ExprKind;
        
        // Check if it's a user-defined function
        if let Some(&offset) = self.func_offsets.get(name) {
            // Generate code for arguments (push onto stack)
            for arg_id in args {
                self.gen_expr_id(pool, *arg_id);
            }
            // Emit Call opcode with function offset
            self.code.push(LpsOpCode::Call(offset));
            return;
        }

        // Special case: perlin3(vec3) or perlin3(vec3, octaves)
        // Octaves is embedded in opcode, not pushed to stack
        if name == "perlin3" {
            // First arg is vec3, generate code to push its 3 components
            self.gen_expr_id(pool, args[0]);

            // Extract octaves from 2nd arg or use default
            let octaves = if args.len() >= 2 {
                let arg_expr = pool.expr(args[1]);
                match &arg_expr.kind {
                    ExprKind::Number(n) => *n as u8,
                    ExprKind::IntNumber(n) => *n as u8,
                    _ => 3,
                }
            } else {
                3
            };

            self.code.push(LpsOpCode::Perlin3(octaves));
            return;
        }

        // For all other functions, generate code for all arguments first
        for arg_id in args {
            self.gen_expr_id(pool, *arg_id);
        }

        // Emit the appropriate instruction
        self.gen_builtin_function_id(pool, name, args);
    }

    fn gen_builtin_function_id(&mut self, pool: &crate::lpscript::compiler::ast::AstPool, name: &str, args: &[crate::lpscript::compiler::ast::ExprId]) {
        use crate::lpscript::shared::Type;
        
        match name {
            "sin" => self.code.push(LpsOpCode::SinFixed),
            "cos" => self.code.push(LpsOpCode::CosFixed),
            "frac" | "fract" => self.code.push(LpsOpCode::FractFixed),

            // Math functions - use explicit opcodes
            "min" => self.code.push(LpsOpCode::MinFixed),
            "max" => self.code.push(LpsOpCode::MaxFixed),
            "abs" => self.code.push(LpsOpCode::AbsFixed),
            "floor" => self.code.push(LpsOpCode::FloorFixed),
            "ceil" => self.code.push(LpsOpCode::CeilFixed),
            "sqrt" => self.code.push(LpsOpCode::SqrtFixed),
            "tan" => self.code.push(LpsOpCode::TanFixed),
            "pow" => self.code.push(LpsOpCode::PowFixed),
            "sign" => self.code.push(LpsOpCode::SignFixed),
            "mod" => self.code.push(LpsOpCode::ModFixed),
            "atan" => {
                if args.len() == 2 {
                    self.code.push(LpsOpCode::Atan2Fixed);
                } else {
                    self.code.push(LpsOpCode::AtanFixed);
                }
            }

            // Clamping/interpolation
            "clamp" => self.code.push(LpsOpCode::ClampFixed),
            "saturate" => self.code.push(LpsOpCode::SaturateFixed),
            "step" => self.code.push(LpsOpCode::StepFixed),
            "lerp" | "mix" => self.code.push(LpsOpCode::LerpFixed),
            "smoothstep" => self.code.push(LpsOpCode::SmoothstepFixed),

            // Vector functions - use typed opcodes based on argument type
            "length" => {
                if !args.is_empty() {
                    let arg_ty = pool.expr(args[0]).ty.as_ref().unwrap();
                    match arg_ty {
                        Type::Vec2 => self.code.push(LpsOpCode::Length2),
                        Type::Vec3 => self.code.push(LpsOpCode::Length3),
                        Type::Vec4 => self.code.push(LpsOpCode::Length4),
                        _ => {}
                    }
                }
            }
            "normalize" => {
                if !args.is_empty() {
                    let arg_ty = pool.expr(args[0]).ty.as_ref().unwrap();
                    match arg_ty {
                        Type::Vec2 => self.code.push(LpsOpCode::Normalize2),
                        Type::Vec3 => self.code.push(LpsOpCode::Normalize3),
                        Type::Vec4 => self.code.push(LpsOpCode::Normalize4),
                        _ => {}
                    }
                }
            }
            "dot" => {
                if !args.is_empty() {
                    let arg_ty = pool.expr(args[0]).ty.as_ref().unwrap();
                    match arg_ty {
                        Type::Vec2 => self.code.push(LpsOpCode::Dot2),
                        Type::Vec3 => self.code.push(LpsOpCode::Dot3),
                        Type::Vec4 => self.code.push(LpsOpCode::Dot4),
                        _ => {}
                    }
                }
            }
            "distance" => {
                if !args.is_empty() {
                    let arg_ty = pool.expr(args[0]).ty.as_ref().unwrap();
                    match arg_ty {
                        Type::Vec2 => self.code.push(LpsOpCode::Distance2),
                        Type::Vec3 => self.code.push(LpsOpCode::Distance3),
                        Type::Vec4 => self.code.push(LpsOpCode::Distance4),
                        _ => {}
                    }
                }
            }
            "cross" => {
                // Always vec3
                self.code.push(LpsOpCode::Cross3);
            }

            _ => {} // Unknown function - ignore
        }
    }

    fn gen_vec_constructor_id(&mut self, pool: &crate::lpscript::compiler::ast::AstPool, args: &[crate::lpscript::compiler::ast::ExprId]) {
        // Generate code for each argument (leaves values on stack in order)
        for arg_id in args {
            self.gen_expr_id(pool, *arg_id);
        }
        // Vector constructors don't need a special opcode - args are already on stack
        // Vec2(x, y) leaves x, y on stack (that IS a vec2)
    }

    fn gen_swizzle_id(&mut self, pool: &crate::lpscript::compiler::ast::AstPool, expr: crate::lpscript::compiler::ast::ExprId, components: &str) {
        // Generate the base expression (leaves vector components on stack)
        self.gen_expr_id(pool, expr);
        
        // Generate swizzle opcodes based on component string
        let expr_obj = pool.expr(expr);
        let source_type = expr_obj.ty.as_ref().unwrap();
        let source_size = match source_type {
            crate::lpscript::shared::Type::Vec2 => 2,
            crate::lpscript::shared::Type::Vec3 => 3,
            crate::lpscript::shared::Type::Vec4 => 4,
            _ => 1,
        };
        
        // Call the helper function
        gen_swizzle_opcodes(components, source_size, &mut self.code);
    }

    fn gen_var_decl_id(&mut self, pool: &crate::lpscript::compiler::ast::AstPool, ty: &crate::lpscript::shared::Type, name: &str, init: &Option<crate::lpscript::compiler::ast::ExprId>) {
        use alloc::string::ToString;
        use crate::lpscript::shared::Type;
        
        let local_idx = self.locals.allocate_typed(name.to_string(), ty.clone());
        
        if let Some(init_id) = init {
            self.gen_expr_id(pool, *init_id);
            // Use type-specific StoreLocal opcode
            self.code.push(match ty {
                Type::Fixed | Type::Bool => LpsOpCode::StoreLocalFixed(local_idx),
                Type::Int32 => LpsOpCode::StoreLocalInt32(local_idx),
                Type::Vec2 => LpsOpCode::StoreLocalVec2(local_idx),
                Type::Vec3 => LpsOpCode::StoreLocalVec3(local_idx),
                Type::Vec4 => LpsOpCode::StoreLocalVec4(local_idx),
                _ => LpsOpCode::StoreLocalFixed(local_idx), // Fallback
            });
        }
    }

    fn gen_block_id(&mut self, pool: &crate::lpscript::compiler::ast::AstPool, stmts: &[crate::lpscript::compiler::ast::StmtId]) {
        self.locals.push_scope();
        for &stmt_id in stmts {
            self.gen_stmt_id(pool, stmt_id);
        }
        self.locals.pop_scope();
    }

    fn gen_if_stmt_id(&mut self, pool: &crate::lpscript::compiler::ast::AstPool, condition: crate::lpscript::compiler::ast::ExprId, then_stmt: crate::lpscript::compiler::ast::StmtId, else_stmt: Option<crate::lpscript::compiler::ast::StmtId>) {
        // Generate condition
        self.gen_expr_id(pool, condition);
        
        // JumpIfZero to else/end
        let jump_to_else = self.code.len();
        self.code.push(LpsOpCode::JumpIfZero(0)); // Placeholder
        
        // Then branch
        self.gen_stmt_id(pool, then_stmt);
        
        if let Some(else_id) = else_stmt {
            // Jump over else
            let jump_to_end = self.code.len();
            self.code.push(LpsOpCode::Jump(0)); // Placeholder
            
            // Patch jump to else
            let else_start = self.code.len();
            if let LpsOpCode::JumpIfZero(ref mut offset) = self.code[jump_to_else] {
                *offset = (else_start as i32) - (jump_to_else as i32) - 1;
            }
            
            // Else branch
            self.gen_stmt_id(pool, else_id);
            
            // Patch jump to end
            let end = self.code.len();
            if let LpsOpCode::Jump(ref mut offset) = self.code[jump_to_end] {
                *offset = (end as i32) - (jump_to_end as i32) - 1;
            }
        } else {
            // No else, patch jump to end
            let end = self.code.len();
            if let LpsOpCode::JumpIfZero(ref mut offset) = self.code[jump_to_else] {
                *offset = (end as i32) - (jump_to_else as i32) - 1;
            }
        }
    }

    fn gen_while_stmt_id(&mut self, pool: &crate::lpscript::compiler::ast::AstPool, condition: crate::lpscript::compiler::ast::ExprId, body: crate::lpscript::compiler::ast::StmtId) {
        let loop_start = self.code.len();
        
        // Generate condition
        self.gen_expr_id(pool, condition);
        
        // JumpIfZero to end
        let jump_to_end = self.code.len();
        self.code.push(LpsOpCode::JumpIfZero(0)); // Placeholder
        
        // Body
        self.gen_stmt_id(pool, body);
        
        // Jump back to loop start
        let jump_back_idx = self.code.len();
        self.code.push(LpsOpCode::Jump(
            (loop_start as i32) - (jump_back_idx as i32) - 1,
        ));
        
        // Patch jump to end
        let end = self.code.len();
        if let LpsOpCode::JumpIfZero(ref mut offset) = self.code[jump_to_end] {
            *offset = (end as i32) - (jump_to_end as i32) - 1;
        }
    }

    fn gen_for_stmt_id(&mut self, pool: &crate::lpscript::compiler::ast::AstPool, init: &Option<crate::lpscript::compiler::ast::StmtId>, condition: &Option<crate::lpscript::compiler::ast::ExprId>, increment: &Option<crate::lpscript::compiler::ast::ExprId>, body: crate::lpscript::compiler::ast::StmtId) {
        self.locals.push_scope();
        
        // Init
        if let Some(init_id) = init {
            self.gen_stmt_id(pool, *init_id);
        }
        
        let loop_start = self.code.len();
        
        // Condition (defaults to true if omitted)
        let jump_to_end = if let Some(cond_id) = condition {
            self.gen_expr_id(pool, *cond_id);
            let jump_idx = self.code.len();
            self.code.push(LpsOpCode::JumpIfZero(0)); // Placeholder
            Some(jump_idx)
        } else {
            None
        };
        
        // Body
        self.gen_stmt_id(pool, body);
        
        // Increment
        if let Some(inc_id) = increment {
            self.gen_expr_id(pool, *inc_id);
            self.code.push(LpsOpCode::Drop1); // Discard result
        }
        
        // Jump back to loop start
        let jump_back_idx = self.code.len();
        self.code.push(LpsOpCode::Jump(
            (loop_start as i32) - (jump_back_idx as i32) - 1,
        ));
        
        // Patch jump to end
        if let Some(jump_idx) = jump_to_end {
            let end = self.code.len();
            if let LpsOpCode::JumpIfZero(ref mut offset) = self.code[jump_idx] {
                *offset = (end as i32) - (jump_idx as i32) - 1;
            }
        }
        
        self.locals.pop_scope();
    }
}

/// Generate opcodes for swizzling
/// Stack layout: components are pushed in order, so for vec2(x,y), stack is [x, y] with y on top
fn gen_swizzle_opcodes(components: &str, source_size: usize, code: &mut Vec<LpsOpCode>) {
    use alloc::vec::Vec as AllocVec;
    
    // Map component characters to indices
    let indices: AllocVec<usize> = components
        .chars()
        .map(|c| match c {
            'x' | 'r' | 's' => 0,
            'y' | 'g' | 't' => 1,
            'z' | 'b' | 'p' => 2,
            'w' | 'a' | 'q' => 3,
            _ => unreachable!("Type checker should validate swizzle components"),
        })
        .collect();

    // Strategy: Pop all source components into temporary positions,
    // then push back the desired components in the right order

    // We'll use a simple approach: generate Dup/Swap/Drop operations
    // This is not the most efficient but is correct and simple

    if components.len() == 1 {
        // Single component extraction
        let idx = indices[0];
        // Stack has [c0, c1, ..., c(n-1)] with c(n-1) on top
        // We want to keep component at index idx
        // Index 0 is at bottom, index (n-1) is at top
        let drop_count = source_size - 1 - idx;
        for _ in 0..drop_count {
            code.push(LpsOpCode::Drop1);
        }
        // Now we have [c0, c1, ..., c(idx)]
        // We want just c(idx), so drop everything below
        for _ in 0..idx {
            code.push(LpsOpCode::Swap); // Bring bottom to top
            code.push(LpsOpCode::Drop1); // Drop it
        }
    } else {
        // Multi-component swizzle
        // Check if it's an identity swizzle first (optimization)
        let is_identity = indices.iter().enumerate().all(|(i, &idx)| i == idx);
        if is_identity && indices.len() == source_size {
            // Identity swizzle, no-op
            return;
        }

        // For vec2 specifically, handle common cases efficiently
        if source_size == 2 {
            match components {
                "yx" | "gr" | "ts" => code.push(LpsOpCode::Swap),
                "xx" | "rr" | "ss" => {
                    // [x, y] -> [x, x]
                    code.push(LpsOpCode::Drop1); // [x]
                    code.push(LpsOpCode::Dup1); // [x, x]
                }
                "yy" | "gg" | "tt" => {
                    // [x, y] -> [y, y]
                    code.push(LpsOpCode::Swap); // [y, x]
                    code.push(LpsOpCode::Drop1); // [y]
                    code.push(LpsOpCode::Dup1); // [y, y]
                }
                _ => {
                    // General case for vec2: Handle by reconstruction
                    // This is rare but possible (e.g., if type checker allows it)
                }
            }
        } else if components.len() == 2 && source_size == 3 {
            // vec3 -> vec2 swizzle
            code.push(LpsOpCode::Swizzle3to2(indices[0] as u8, indices[1] as u8));
        } else if components.len() == 3 && source_size == 3 {
            // vec3 -> vec3 swizzle
            code.push(LpsOpCode::Swizzle3to3(
                indices[0] as u8,
                indices[1] as u8,
                indices[2] as u8,
            ));
        } else if components.len() == 2 && source_size == 4 {
            // vec4 -> vec2 swizzle
            code.push(LpsOpCode::Swizzle4to2(indices[0] as u8, indices[1] as u8));
        } else if components.len() == 3 && source_size == 4 {
            // vec4 -> vec3 swizzle
            code.push(LpsOpCode::Swizzle4to3(
                indices[0] as u8,
                indices[1] as u8,
                indices[2] as u8,
            ));
        } else if components.len() == 4 && source_size == 4 {
            // vec4 -> vec4 swizzle
            code.push(LpsOpCode::Swizzle4to4(
                indices[0] as u8,
                indices[1] as u8,
                indices[2] as u8,
                indices[3] as u8,
            ));
        }
    }
}
