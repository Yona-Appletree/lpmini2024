/// Code generator: converts AST to VM opcodes
extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use crate::lpscript::compiler::ast::Program;
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
    pub fn generate(
        pool: &crate::lpscript::compiler::ast::AstPool,
        expr_id: crate::lpscript::compiler::ast::ExprId,
    ) -> Vec<LpsOpCode> {
        Self::generate_with_locals(pool, expr_id, Vec::new())
    }

    /// Generate opcodes for an expression with pre-declared local variables
    ///
    /// This is useful for testing assignment expressions which need mutable locals.
    /// The locals should be ordered by index (e.g., [("x", 0), ("y", 1), ...])
    pub fn generate_with_locals(
        pool: &crate::lpscript::compiler::ast::AstPool,
        expr_id: crate::lpscript::compiler::ast::ExprId,
        predeclared: Vec<(String, u32, crate::lpscript::shared::Type)>,
    ) -> Vec<LpsOpCode> {
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

    /// Generate functions for a program (new API with FunctionTable)
    pub fn generate_program_with_functions(
        pool: &crate::lpscript::compiler::ast::AstPool,
        program: &Program,
        func_table: &crate::lpscript::compiler::func::FunctionTable,
    ) -> Vec<crate::lpscript::vm::FunctionDef> {
        program::gen_program_with_functions(
            pool,
            program,
            func_table,
            |pool, stmt_id, code, locals, func_offsets| {
                let mut gen = CodeGenerator::new(code, locals, func_offsets);
                gen.gen_stmt_id(pool, stmt_id);
            },
        )
    }

    /// Generate opcodes for a program (script mode) - Legacy API
    /// Returns (opcodes, local_count, local_types) tuple
    #[cfg(test)]
    pub fn generate_program(
        pool: &crate::lpscript::compiler::ast::AstPool,
        program: &Program,
    ) -> (
        Vec<LpsOpCode>,
        u32,
        alloc::collections::BTreeMap<u32, crate::lpscript::shared::Type>,
    ) {
        program::gen_program(
            pool,
            program,
            |pool, stmt_id, code, locals, func_offsets| {
                let mut gen = CodeGenerator::new(code, locals, func_offsets);
                gen.gen_stmt_id(pool, stmt_id);
            },
        )
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
                self.gen_return_id(pool, *expr_id);
            }
            StmtKind::Expr(expr_id) => {
                self.gen_expr_stmt_id(pool, *expr_id);
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

    // ID-based codegen methods are now implemented in their respective organized modules
    // (expr/*/binary_gen.rs, expr/*/ternary_gen.rs, etc.)

    // All codegen methods are now implemented in their respective organized modules
    // Expression generators: expr/*/...
    // Statement generators: stmt/*/...
}
