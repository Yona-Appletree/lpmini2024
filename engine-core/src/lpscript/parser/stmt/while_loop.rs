/// While loop parsing
use super::Parser;
use crate::lpscript::ast::{Stmt, StmtKind};
use crate::lpscript::error::Span;
use crate::lpscript::lexer::TokenKind;
use alloc::boxed::Box;

impl Parser {
    pub(in crate::lpscript::parser) fn parse_while_stmt(&mut self) -> Stmt {
        let start = self.current().span.start;
        self.advance(); // consume 'while'
        
        self.expect(TokenKind::LParen);
        let condition = self.ternary();
        self.expect(TokenKind::RParen);
        
        let body = Box::new(self.parse_stmt());
        let end = body.span.end;
        
        Stmt::new(
            StmtKind::While { condition, body },
            Span::new(start, end),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::lpscript::lexer::Lexer;
    use crate::lpscript::parser::Parser;

    #[test]
    fn test_parse_while_structure() {
        let mut lexer = Lexer::new("while (i < 5.0) { i = i + 1.0; }");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program();
        
        assert_eq!(program.stmts.len(), 1);
        assert!(matches!(program.stmts[0].kind, crate::lpscript::ast::StmtKind::While { .. }));
    }
}

