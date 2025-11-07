/// Return statement parsing
use crate::compiler::ast::{StmtId, StmtKind};
use crate::compiler::error::ParseError;
use crate::compiler::parser::Parser;
use crate::shared::Span;

impl Parser {
    pub(crate) fn parse_return_stmt(&mut self) -> Result<StmtId, ParseError> {
        self.enter_recursion()?;
        let start = self.current().span.start;
        self.advance(); // consume 'return'

        let expr_id = self.ternary()?;
        self.consume_semicolon();
        let end = self.current().span.end;

        let result = self
            .pool
            .alloc_stmt(StmtKind::Return(expr_id), Span::new(start, end))
            .map_err(|e| self.pool_error_to_parse_error(e));

        self.exit_recursion();
        result
    }
}
