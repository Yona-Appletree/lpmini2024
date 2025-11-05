/// For loop parsing
use crate::lpscript::ast::{Stmt, StmtKind};
use crate::lpscript::error::Span;
use crate::lpscript::lexer::TokenKind;
use crate::lpscript::compiler::parser::Parser;
use alloc::boxed::Box;

impl Parser {
    pub(in crate::lpscript) fn parse_for_stmt(&mut self) -> Stmt {
        let start = self.current().span.start;
        self.advance(); // consume 'for'
        
        self.expect(TokenKind::LParen);
        
        // Parse init (can be var decl or expression)
        let init = if matches!(self.current().kind, TokenKind::Semicolon) {
            self.advance();
            None
        } else if matches!(self.current().kind, TokenKind::Float | TokenKind::Int) {
            // Parse var decl inline without consuming semicolon
            let decl = self.parse_var_decl_no_semicolon();
            self.expect(TokenKind::Semicolon);
            Some(Box::new(decl))
        } else {
            // Parse expression and consume its semicolon
            let expr = self.ternary();
            self.expect(TokenKind::Semicolon);
            let span = expr.span;
            Some(Box::new(Stmt::new(StmtKind::Expr(expr), span)))
        };
        
        // Parse condition
        let condition = if matches!(self.current().kind, TokenKind::Semicolon) {
            self.advance();
            None
        } else {
            let cond = self.ternary();
            self.expect(TokenKind::Semicolon);
            Some(cond)
        };
        
        // Parse increment (can be assignment expression)
        let increment = if matches!(self.current().kind, TokenKind::RParen) {
            None
        } else {
            Some(self.parse_assignment_expr())
        };
        
        self.expect(TokenKind::RParen);
        
        let body = Box::new(self.parse_stmt());
        let end = body.span.end;
        
        Stmt::new(
            StmtKind::For {
                init,
                condition,
                increment,
                body,
            },
            Span::new(start, end),
        )
    }
}
