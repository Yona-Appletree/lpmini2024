/// Logical operation type checking
extern crate alloc;

use crate::lpscript::ast::Expr;
use crate::lpscript::error::{Type, TypeError};
use crate::lpscript::typechecker::{TypeChecker, SymbolTable, FunctionTable};
use alloc::boxed::Box;

impl TypeChecker {
    /// Type check logical operators (&&, ||)
    /// 
    /// Both operands are evaluated, result is always Bool.
    pub(crate) fn check_logical(
        left: &mut Box<Expr>,
        right: &mut Box<Expr>,
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
    ) -> Result<Type, TypeError> {
        Self::infer_type(left, symbols, func_table)?;
        Self::infer_type(right, symbols, func_table)?;

        // Logical operations always return Bool
        Ok(Type::Bool)
    }
}

