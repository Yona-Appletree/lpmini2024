/// Return statement parsing
use crate::lpscript::compiler::ast::{StmtId, StmtKind};
use crate::lpscript::compiler::error::ParseError;
use crate::lpscript::compiler::parser::Parser;
use crate::lpscript::shared::Span;


impl Parser {
    pub(in crate::lpscript) fn parse_return_stmt(&mut self) -> Result<StmtId, ParseError> {
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
