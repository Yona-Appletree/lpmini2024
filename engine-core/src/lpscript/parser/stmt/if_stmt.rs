/// If statement parsing
use super::Parser;
use crate::lpscript::ast::{Stmt, StmtKind};
use crate::lpscript::error::Span;
use crate::lpscript::lexer::TokenKind;
use alloc::boxed::Box;

impl Parser {
    pub(in crate::lpscript::parser) fn parse_if_stmt(&mut self) -> Stmt {
        let start = self.current().span.start;
        self.advance(); // consume 'if'
        
        self.expect(TokenKind::LParen);
        let condition = self.ternary();
        self.expect(TokenKind::RParen);
        
        let then_stmt = Box::new(self.parse_stmt());
        
        let else_stmt = if matches!(self.current().kind, TokenKind::Else) {
            self.advance(); // consume 'else'
            Some(Box::new(self.parse_stmt()))
        } else {
            None
        };
        
        let end = else_stmt.as_ref()
            .map(|s| s.span.end)
            .unwrap_or(then_stmt.span.end);
        
        Stmt::new(
            StmtKind::If {
                condition,
                then_stmt,
                else_stmt,
            },
            Span::new(start, end),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::lpscript::lexer::Lexer;
    use crate::lpscript::parser::Parser;

    #[test]
    fn test_parse_if_structure() {
        let mut lexer = Lexer::new("if (x > 0.5) { return 1.0; }");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program();
        
        assert_eq!(program.stmts.len(), 1);
        if let crate::lpscript::ast::StmtKind::If { condition: _, then_stmt: _, else_stmt } = &program.stmts[0].kind {
            assert!(else_stmt.is_none());
        } else {
            panic!("Expected If statement");
        }
    }

    #[test]
    fn test_parse_if_with_else() {
        let mut lexer = Lexer::new("if (x > 0.5) { return 1.0; } else { return 0.0; }");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program();
        
        assert_eq!(program.stmts.len(), 1);
        if let crate::lpscript::ast::StmtKind::If { condition: _, then_stmt: _, else_stmt } = &program.stmts[0].kind {
            assert!(else_stmt.is_some());
        } else {
            panic!("Expected If statement");
        }
    }
}

