/// Optimization passes for AST and opcodes
///
/// This module provides configurable optimization passes that can be applied
/// to the AST after type checking and to opcodes after code generation.
///
/// # Optimization Levels
///
/// - **AST Level**: High-level semantic optimizations (constant folding, algebraic simplification, dead code elimination)
/// - **Opcode Level**: Low-level bytecode optimizations (peephole patterns)
///
/// # Safety
///
/// All optimizations preserve program semantics. The optimized code will produce
/// identical results to the unoptimized code.
extern crate alloc;
use alloc::vec::Vec;

use super::ast::{Expr, Program};
use crate::lpscript::vm::opcodes::LpsOpCode;

pub mod ast;
pub mod ops;

#[cfg(test)]
pub mod ast_test_util;
#[cfg(test)]
mod tests;

/// Configuration for optimization passes
#[derive(Debug, Clone, Copy)]
pub struct OptimizeOptions {
    /// Enable constant folding (e.g., `2 + 3` → `5`)
    pub constant_folding: bool,

    /// Enable algebraic simplification (e.g., `x * 1` → `x`)
    pub algebraic_simplification: bool,

    /// Enable dead code elimination
    pub dead_code_elimination: bool,

    /// Enable opcode peephole optimization
    pub peephole_optimization: bool,

    /// Maximum number of AST optimization passes (to reach fixed point)
    pub max_ast_passes: usize,
}

impl OptimizeOptions {
    /// All optimizations enabled (recommended default)
    pub fn all() -> Self {
        Self {
            constant_folding: true,
            algebraic_simplification: true,
            dead_code_elimination: true,
            peephole_optimization: true,
            max_ast_passes: 5,
        }
    }

    /// All optimizations disabled (for debugging)
    pub fn none() -> Self {
        Self {
            constant_folding: false,
            algebraic_simplification: false,
            dead_code_elimination: false,
            peephole_optimization: false,
            max_ast_passes: 0,
        }
    }
}

impl Default for OptimizeOptions {
    fn default() -> Self {
        Self::all()
    }
}

/// Optimize an expression AST
///
/// Applies AST-level optimizations based on the provided options.
/// Runs multiple passes until a fixed point is reached or max iterations exceeded.
pub fn optimize_ast_expr(
    expr_id: crate::lpscript::compiler::ast::ExprId,
    pool: crate::lpscript::compiler::ast::AstPool,
    options: &OptimizeOptions,
) -> (crate::lpscript::compiler::ast::ExprId, crate::lpscript::compiler::ast::AstPool) {
    if options.max_ast_passes == 0 {
        return (expr_id, pool);
    }

    ast::optimize_expr_id(expr_id, pool, options)
}

/// Optimize a program AST (with statements)
///
/// Applies AST-level optimizations to the full program.
pub fn optimize_ast_program(
    program: Program,
    pool: crate::lpscript::compiler::ast::AstPool,
    options: &OptimizeOptions,
) -> (Program, crate::lpscript::compiler::ast::AstPool) {
    if options.max_ast_passes == 0 {
        return (program, pool);
    }

    ast::optimize_program_id(program, pool, options)
}

/// Optimize a sequence of opcodes
///
/// Applies opcode-level peephole optimizations.
pub fn optimize_opcodes(opcodes: Vec<LpsOpCode>, options: &OptimizeOptions) -> Vec<LpsOpCode> {
    if !options.peephole_optimization {
        return opcodes;
    }

    ops::optimize(opcodes)
}
