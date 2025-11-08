/// Expression statement parsing
use crate::compiler::ast::{StmtId, StmtKind};
use crate::compiler::error::ParseError;
use crate::compiler::parser::Parser;

impl Parser {
    pub(crate) fn parse_expr_stmt(&mut self) -> Result<StmtId, ParseError> {
        self.enter_recursion()?;
        let expr_id = self.parse()?; // Use parse() to handle assignments, not just ternary()
        let span = self.pool.expr(expr_id).span;
        self.consume_semicolon();

        let result = self
            .pool
            .alloc_stmt(StmtKind::Expr(expr_id), span)
            .map_err(|e| self.pool_error_to_parse_error(e));

        self.exit_recursion();
        result
    }
}
