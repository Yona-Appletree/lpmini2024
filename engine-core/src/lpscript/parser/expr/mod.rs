/// Expression parsing
use super::Parser;
use crate::lpscript::ast::Expr;

mod assign_expr;
mod binary;
mod call;
mod constructors;
mod literals;
mod logical;
mod swizzle;
mod ternary;
mod variable;

// Comparison parsing is now in compiler module
use crate::lpscript::compiler::expr::compare;

impl Parser {
    /// Parse the entry point for expressions (from parse method)
    pub fn parse(&mut self) -> Expr {
        self.ternary()
    }
}
