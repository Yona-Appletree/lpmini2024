/// Comparison type checking
extern crate alloc;

use crate::lpscript::ast::Expr;
use crate::lpscript::error::{Type, TypeError};
use crate::lpscript::typechecker::{TypeChecker, SymbolTable, FunctionTable};
use alloc::boxed::Box;

impl TypeChecker {
    /// Type check comparison operators (<, >, <=, >=, ==, !=)
    /// 
    /// All comparisons return Int32 (0 or 1) representing false or true.
    /// Returns the result type (always Type::Int32).
    pub(crate) fn check_comparison(
        left: &mut Box<Expr>,
        right: &mut Box<Expr>,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
    ) -> Result<Type, TypeError> {
        // Type check both operands
        Self::infer_type(left, symbols, func_table)?;
        Self::infer_type(right, symbols, func_table)?;

        // Comparisons always return Int32 (0 or 1)
        Ok(Type::Int32)
    }
}

