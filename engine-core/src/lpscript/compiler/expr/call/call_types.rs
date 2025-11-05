/// Function call type checking
extern crate alloc;

use crate::lpscript::ast::Expr;
use crate::lpscript::error::{Type, TypeError};
use crate::lpscript::typechecker::{TypeChecker, SymbolTable, FunctionTable};

impl TypeChecker {
    /// Type check function call
    /// 
    /// Infers the return type based on the function signature.
    pub(crate) fn check_function_call(
        name: &str,
        args: &mut [Expr],
        symbols: &mut SymbolTable,
        func_table: &FunctionTable,
    ) -> Result<Type, TypeError> {
        // Type check all arguments first
        for arg in args.iter_mut() {
            Self::infer_type(arg, symbols, func_table)?;
        }

        // Infer function return type (calls existing function_return_type method)
        Self::function_return_type(name, args)
    }
}

