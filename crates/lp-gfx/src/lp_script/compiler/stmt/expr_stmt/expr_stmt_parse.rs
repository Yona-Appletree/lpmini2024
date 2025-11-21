/// Expression statement parsing
use crate::lp_script::compiler::ast::{Stmt, StmtKind};
use crate::lp_script::compiler::error::ParseError;
use crate::lp_script::compiler::parser::Parser;

impl Parser {
    pub(crate) fn parse_expr_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.enter_recursion()?;
        let expr = self.parse()?; // Use parse() to handle assignments, not just ternary()
        let span = expr.span;
        self.consume_semicolon();

        let result = Ok(Stmt::new(StmtKind::Expr(expr), span));

        self.exit_recursion();
        result
    }
}
