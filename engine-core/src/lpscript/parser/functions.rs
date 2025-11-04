/// Function definition parsing methods
use super::Parser;
use crate::lpscript::ast::{FunctionDef, Parameter};
use crate::lpscript::error::Span;
use crate::lpscript::lexer::TokenKind;
use alloc::string::String;
use alloc::vec::Vec;

impl Parser {
    /// Check if the current position is a function definition
    /// Function def: type name(params) { body }
    pub(super) fn is_function_definition(&mut self) -> bool {
        // Look ahead: type + identifier + (
        matches!(
            self.current().kind,
            TokenKind::Float | TokenKind::Int | TokenKind::Vec2 | TokenKind::Vec3 | TokenKind::Vec4 | TokenKind::Void
        ) && {
            // Look ahead 2 tokens
            let saved_pos = self.pos;
            self.advance(); // Skip type
            let is_ident = matches!(self.current().kind, TokenKind::Ident(_));
            if is_ident {
                self.advance(); // Skip name
                let has_paren = matches!(self.current().kind, TokenKind::LParen);
                self.pos = saved_pos; // Restore position
                has_paren
            } else {
                self.pos = saved_pos;
                false
            }
        }
    }
    
    /// Parse a function definition
    pub(super) fn parse_function_def(&mut self) -> FunctionDef {
        let start = self.current().span.start;
        
        // Parse return type
        let return_type = self.parse_type();
        
        // Parse function name
        let name = if let TokenKind::Ident(n) = &self.current().kind {
            let name = n.clone();
            self.advance();
            name
        } else {
            String::from("error")
        };
        
        // Parse parameter list
        self.expect(TokenKind::LParen);
        let params = self.parse_parameters();
        self.expect(TokenKind::RParen);
        
        // Parse body
        self.expect(TokenKind::LBrace);
        let mut body = Vec::new();
        while !matches!(self.current().kind, TokenKind::RBrace | TokenKind::Eof) {
            body.push(self.parse_stmt());
        }
        let end = self.current().span.end;
        self.expect(TokenKind::RBrace);
        
        FunctionDef {
            name,
            params,
            return_type,
            body,
            span: Span::new(start, end),
        }
    }
    
    /// Parse function parameters
    fn parse_parameters(&mut self) -> Vec<Parameter> {
        let mut params = Vec::new();
        
        // Empty parameter list
        if matches!(self.current().kind, TokenKind::RParen) {
            return params;
        }
        
        loop {
            // Parse parameter type
            let ty = self.parse_type();
            
            // Parse parameter name
            let name = if let TokenKind::Ident(n) = &self.current().kind {
                let name = n.clone();
                self.advance();
                name
            } else {
                String::from("error")
            };
            
            params.push(Parameter { name, ty });
            
            // Check for more parameters
            if matches!(self.current().kind, TokenKind::Comma) {
                self.advance(); // consume ','
            } else {
                break;
            }
        }
        
        params
    }
}

#[cfg(test)]
mod tests {
    use crate::lpscript::lexer::Lexer;
    use crate::lpscript::parser::Parser;

    #[test]
    fn test_parse_function_no_params() {
        let mut lexer = Lexer::new("float getPi() { return 3.14; }");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program();
        
        assert_eq!(program.functions.len(), 1);
        assert_eq!(program.functions[0].name, "getPi");
        assert_eq!(program.functions[0].params.len(), 0);
    }

    #[test]
    fn test_parse_function_with_params() {
        let mut lexer = Lexer::new("float add(float a, float b) { return a + b; }");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program();
        
        assert_eq!(program.functions.len(), 1);
        assert_eq!(program.functions[0].params.len(), 2);
        assert_eq!(program.functions[0].params[0].name, "a");
        assert_eq!(program.functions[0].params[1].name, "b");
    }

    #[test]
    fn test_parse_function_body() {
        let mut lexer = Lexer::new("float double(float x) { return x * 2.0; }");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program();
        
        assert_eq!(program.functions.len(), 1);
        assert!(!program.functions[0].body.is_empty());
    }

    #[test]
    fn test_parse_multiple_functions() {
        let mut lexer = Lexer::new("
            float add(float a, float b) { return a + b; }
            float sub(float a, float b) { return a - b; }
        ");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program();
        
        assert_eq!(program.functions.len(), 2);
        assert_eq!(program.functions[0].name, "add");
        assert_eq!(program.functions[1].name, "sub");
    }
}

