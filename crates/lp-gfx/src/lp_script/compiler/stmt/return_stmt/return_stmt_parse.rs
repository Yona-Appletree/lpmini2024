/// Return statement parsing
use crate::lp_script::compiler::ast::{Stmt, StmtKind};
use crate::lp_script::compiler::error::ParseError;
use crate::lp_script::compiler::parser::Parser;
use crate::lp_script::shared::Span;

impl Parser {
    pub(crate) fn parse_return_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.enter_recursion()?;
        let start = self.current().span.start;
        self.advance(); // consume 'return'

        let expr = self.ternary()?;
        self.consume_semicolon();
        let end = self.current().span.end;

        let result = Ok(Stmt::new(StmtKind::Return(expr), Span::new(start, end)));

        self.exit_recursion();
        result
    }
}
