/// Block statement parsing
use super::Parser;
use crate::lpscript::ast::{Stmt, StmtKind};
use crate::lpscript::error::Span;
use crate::lpscript::lexer::TokenKind;
use alloc::vec::Vec;

impl Parser {
    pub(in crate::lpscript::parser) fn parse_block(&mut self) -> Stmt {
        let start = self.current().span.start;
        self.advance(); // consume '{'
        
        let mut stmts = Vec::new();
        while !matches!(self.current().kind, TokenKind::RBrace | TokenKind::Eof) {
            stmts.push(self.parse_stmt());
        }
        
        let end = self.current().span.end;
        self.expect(TokenKind::RBrace);
        
        Stmt::new(StmtKind::Block(stmts), Span::new(start, end))
    }
}

#[cfg(test)]
mod tests {
    use crate::lpscript::lexer::Lexer;
    use crate::lpscript::parser::Parser;

    #[test]
    fn test_parse_block_structure() {
        let mut lexer = Lexer::new("{ float x = 1.0; }");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program();
        
        assert_eq!(program.stmts.len(), 1);
        assert!(matches!(program.stmts[0].kind, crate::lpscript::ast::StmtKind::Block(_)));
    }

    #[test]
    fn test_parse_multiple_statements() {
        let mut lexer = Lexer::new("float a = 1.0; float b = 2.0; return a + b;");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program();
        
        assert_eq!(program.stmts.len(), 3);
    }
}

