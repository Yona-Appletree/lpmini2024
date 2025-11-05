/// Program type checking
extern crate alloc;
use alloc::vec::Vec;

use crate::lpscript::compiler::ast::Program;
use crate::lpscript::compiler::error::{TypeError, TypeErrorKind};
use crate::lpscript::compiler::typechecker::{FunctionTable, SymbolTable, TypeChecker};
use crate::lpscript::shared::Type;

impl TypeChecker {
    /// Type check a program (script mode)
    pub fn check_program(mut program: Program) -> Result<Program, TypeError> {
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

        // Second pass: Type check each function body
        for func in &mut program.functions {
            TypeChecker::check_function(func, &func_table)?;
        }

        // Third pass: Type check top-level statements
        let mut symbols = SymbolTable::new();
        for stmt in &mut program.stmts {
            Self::check_stmt(stmt, &mut symbols, &func_table)?;
        }

        Ok(program)
    }
}

