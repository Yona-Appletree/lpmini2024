use alloc::boxed::Box;

/// For loop parsing
use crate::compiler::ast::{Stmt, StmtKind};
use crate::compiler::error::ParseError;
use crate::compiler::lexer::TokenKind;
use crate::compiler::parser::Parser;
use crate::shared::Span;

impl Parser {
    pub(crate) fn parse_for_stmt(&mut self) -> Result<Stmt, ParseError> {
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
            let decl = self.parse_var_decl_no_semicolon()?;
            self.expect(TokenKind::Semicolon);
            Some(Box::new(decl))
        } else {
            // Parse expression and consume its semicolon
            let expr = self.ternary()?;
            self.expect(TokenKind::Semicolon);
            let span = expr.span;
            let stmt = Stmt::new(StmtKind::Expr(expr), span);
            Some(Box::new(stmt))
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
        let end = body.span.end;

        let result = Ok(Stmt::new(
            StmtKind::For {
                init,
                condition,
                increment,
                body: Box::new(body),
            },
            Span::new(start, end),
        ));

        self.exit_recursion();
        result
    }
}
