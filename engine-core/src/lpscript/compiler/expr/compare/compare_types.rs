/// Comparison type checking
extern crate alloc;

use crate::lpscript::compiler::ast::Expr;
use crate::lpscript::compiler::typechecker::{FunctionTable, SymbolTable, TypeChecker};
use crate::lpscript::compiler::error::TypeError;
use crate::lpscript::shared::Type;
use alloc::boxed::Box;

impl TypeChecker {
    /// Type check comparison operators (<, >, <=, >=, ==, !=)
    ///
    /// All comparisons return Bool (true or false).
    /// Returns the result type (always Type::Bool).
    pub(crate) fn check_comparison(
        left: &mut Box<Expr>,
        right: &mut Box<Expr>,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
    ) -> Result<Type, TypeError> {
        // Type check both operands
        Self::infer_type(left, symbols, func_table)?;
        Self::infer_type(right, symbols, func_table)?;

        // Comparisons always return Bool
        Ok(Type::Bool)
    }
}
