/// Program type checking
extern crate alloc;
use alloc::vec::Vec;

use crate::lpscript::compiler::ast::{AstPool, Program};
use crate::lpscript::compiler::error::{TypeError, TypeErrorKind};
use crate::lpscript::compiler::typechecker::{FunctionTable, SymbolTable, TypeChecker};
use crate::lpscript::shared::Type;

impl TypeChecker {
    /// Type check a program (script mode)
    pub fn check_program(program: Program, mut pool: AstPool) -> Result<(Program, AstPool), TypeError> {
        let mut func_table = FunctionTable::new();

        // First pass: Register all function signatures
        for func in &program.functions {
            let param_types: Vec<Type> = func.params.iter().map(|p| p.ty.clone()).collect();
            func_table
                .declare(func.name.clone(), param_types, func.return_type.clone())
                .map_err(|msg| TypeError {
                    kind: TypeErrorKind::UndefinedFunction(msg),
                    span: func.span,
                })?;
        }

        // Second pass: Type check each function body (functions are mutable in Program)
        // Note: We need mutable access to functions to type check them
        // For now, skip function type checking - functions contain StmtId which needs pool access
        // TODO: Properly implement function type checking with pool
        
        // Third pass: Type check top-level statements
        let mut symbols = SymbolTable::new();
        for stmt_id in &program.stmts {
            Self::check_stmt_id(&mut pool, *stmt_id, &mut symbols, &func_table)?;
        }

        Ok((program, pool))
    }
}
