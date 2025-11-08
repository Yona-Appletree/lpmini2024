/// Type checker for LightPlayer Script
///
/// This module serves as the main entry point for type checking.
/// The actual implementation is distributed across specialized modules:
/// - expr/expr_types.rs: Expression type checking (check_expr, infer_type)
/// - stmt/stmt_types.rs: Statement type checking (check_stmt)
/// - prog/prog_types.rs: Program-level type checking (check_program)
///
/// Each expression and statement type has its own dedicated _types.rs file
/// in the expr/ and stmt/ subdirectories respectively.
use crate::compiler::ast::Expr;
use crate::compiler::error::TypeError;

// Import function-related types from compiler::func
pub(crate) use crate::compiler::func::FunctionTable;
// Import symbol table from compiler::symbol_table
pub(crate) use crate::compiler::symbol_table::SymbolTable;

pub struct TypeChecker;

// Import the implementation modules to bring the impl blocks into scope
#[allow(unused_imports)]
use crate::compiler::expr::expr_types;
#[allow(unused_imports)]
use crate::compiler::prog::prog_types;
#[allow(unused_imports)]
use crate::compiler::stmt::stmt_types;

impl TypeChecker {
    /// Type check an expression (expression mode)
    pub fn check(expr_id: Expr, pool: AstPool) -> Result<(Expr, AstPool), TypeError> {
        let mut pool = pool;
        let mut symbols = SymbolTable::new();
        let func_table = FunctionTable::new(); // Empty for expression mode
        Self::infer_type_id(&mut pool, expr_id, &mut symbols, &func_table)?;
        Ok((expr_id, pool))
    }
}
