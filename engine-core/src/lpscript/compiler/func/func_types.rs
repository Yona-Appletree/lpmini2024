/// Function type checking
extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::lpscript::ast::{FunctionDef, Stmt, StmtKind};
use crate::lpscript::error::{Type, TypeError, TypeErrorKind};
use crate::lpscript::typechecker::{TypeChecker, SymbolTable};

/// Function signature for user-defined functions
#[derive(Debug, Clone)]
pub(crate) struct FunctionSignature {
    pub(crate) params: Vec<Type>,
    pub(crate) return_type: Type,
}

/// Function table for tracking user-defined functions
#[derive(Debug, Clone)]
pub(crate) struct FunctionTable {
    functions: BTreeMap<String, FunctionSignature>,
}

impl FunctionTable {
    pub(crate) fn new() -> Self {
        FunctionTable {
            functions: BTreeMap::new(),
        }
    }

    pub(crate) fn declare(
        &mut self,
        name: String,
        params: Vec<Type>,
        return_type: Type,
    ) -> Result<(), String> {
        if self.functions.contains_key(&name) {
            return Err(format!("Function '{}' already declared", name));
        }
        self.functions.insert(
            name,
            FunctionSignature {
                params,
                return_type,
            },
        );
        Ok(())
    }

    pub(crate) fn lookup(&self, name: &str) -> Option<&FunctionSignature> {
        self.functions.get(name)
    }
}

impl TypeChecker {
    /// Type check a function definition
    pub(crate) fn check_function(
        func: &mut FunctionDef,
        func_table: &FunctionTable,
    ) -> Result<(), TypeError> {
        let mut symbols = SymbolTable::new();

        // Add parameters to symbol table
        for param in &func.params {
            symbols
                .declare(param.name.clone(), param.ty.clone())
                .map_err(|msg| TypeError {
                    kind: TypeErrorKind::UndefinedVariable(msg),
                    span: func.span,
                })?;
        }

        // Type check function body
        for stmt in &mut func.body {
            Self::check_stmt(stmt, &mut symbols, func_table)?;
        }

        // Verify all code paths return a value (if return_type != Void)
        if func.return_type != Type::Void {
            if !Self::all_paths_return(&func.body) {
                return Err(TypeError {
                    kind: TypeErrorKind::MissingReturn(func.name.clone()),
                    span: func.span,
                });
            }
        }

        Ok(())
    }

    /// Check if all code paths in a statement list return
    fn all_paths_return(stmts: &[Stmt]) -> bool {
        stmts.iter().any(|stmt| Self::stmt_always_returns(stmt))
    }

    /// Check if a statement always returns (guarantees return on all code paths)
    fn stmt_always_returns(stmt: &Stmt) -> bool {
        match &stmt.kind {
            StmtKind::Return(_) => true,

            StmtKind::Block(stmts) => {
                // A block always returns if any statement in it always returns
                Self::all_paths_return(stmts)
            }

            StmtKind::If {
                then_stmt,
                else_stmt,
                ..
            } => {
                // An if statement always returns only if:
                // 1. Both branches exist
                // 2. Both branches always return
                if let Some(else_s) = else_stmt {
                    Self::stmt_always_returns(then_stmt) && Self::stmt_always_returns(else_s)
                } else {
                    false
                }
            }

            // Loops don't guarantee returns (they might not execute)
            StmtKind::While { .. } | StmtKind::For { .. } => false,

            // Other statements don't return
            StmtKind::VarDecl { .. }
            | StmtKind::Assignment { .. }
            | StmtKind::Expr(_) => false,
        }
    }
}

