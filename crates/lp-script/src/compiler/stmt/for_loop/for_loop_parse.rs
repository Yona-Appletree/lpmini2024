/// For loop parsing
use crate::compiler::ast::{StmtId, StmtKind};
use crate::compiler::error::ParseError;
use crate::compiler::lexer::TokenKind;
use crate::compiler::parser::Parser;
use crate::shared::Span;


impl Parser {
    pub(in crate) fn parse_for_stmt(&mut self) -> Result<StmtId, ParseError> {
        self.enter_recursion()?;
        let start = self.current().span.start;
        self.advance(); // consume 'for'

        self.expect(TokenKind::LParen);

        // Parse init (can be var decl or expression)
        let init = if matches!(self.current().kind, TokenKind::Semicolon) {
            self.advance();
            None
        } else if matches!(self.current().kind, TokenKind::Float | TokenKind::Int) {
            // Parse var decl inline without consuming semicolon
            let decl_id = self.parse_var_decl_no_semicolon()?;
            self.expect(TokenKind::Semicolon);
            Some(decl_id)
        } else {
            // Parse expression and consume its semicolon
            let expr_id = self.ternary()?;
            self.expect(TokenKind::Semicolon);
            let span = self.pool.expr(expr_id).span;
            let stmt_id = self
                .pool
                .alloc_stmt(StmtKind::Expr(expr_id), span)
                .map_err(|e| self.pool_error_to_parse_error(e))?;
            Some(stmt_id)
        };

        // Parse condition
        let condition = if matches!(self.current().kind, TokenKind::Semicolon) {
            self.advance();
            None
        } else {
            let cond = self.ternary()?;
            self.expect(TokenKind::Semicolon);
            Some(cond)
        };

        // Parse increment (can be assignment expression)
        let increment = if matches!(self.current().kind, TokenKind::RParen) {
            None
        } else {
            Some(self.parse_assignment_expr()?)
        };

        self.expect(TokenKind::RParen);

        let body = self.parse_stmt()?;
        let end = self.pool.stmt(body).span.end;

        let result = self
            .pool
            .alloc_stmt(
                StmtKind::For {
                    init,
                    condition,
                    increment,
                    body,
                },
                Span::new(start, end),
            )
            .map_err(|e| self.pool_error_to_parse_error(e));

        self.exit_recursion();
        result
    }
}
