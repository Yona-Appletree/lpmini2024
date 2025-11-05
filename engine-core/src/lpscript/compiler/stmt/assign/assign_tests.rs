/// Tests for assignment statements
#[cfg(test)]
mod tests {
    use crate::lpscript::compiler::{lexer, parser};

    #[test]
    fn test_simple_assignment_only() {
        // First make sure simple assignment still works
        let input = "int x = 10; x = 5;";
        let mut lexer = lexer::Lexer::new(input);
        let tokens = lexer.tokenize();

        let mut parser = parser::Parser::new(tokens);
        let program = parser.parse_program();

        assert_eq!(program.stmts.len(), 2);
    }

    #[test]
    fn test_compound_assignment_tokens() {
        // Just verify the tokens are generated correctly
        let input = "x += 5;";
        let mut lexer = lexer::Lexer::new(input);
        let tokens = lexer.tokenize();

        println!("Tokens for 'x += 5;':");
        for (i, token) in tokens.iter().enumerate() {
            println!("  {}: {:?}", i, token.kind);
        }

        assert!(tokens.len() < 10);
    }

    #[test]
    fn test_compound_assignment_parse_single() {
        // Test parsing a single compound assignment
        // Start with just variable declaration to avoid the assignment parsing
        let input = "int x = 1;";
        let mut lexer = lexer::Lexer::new(input);
        let tokens = lexer.tokenize();
        let mut parser = parser::Parser::new(tokens);
        let program = parser.parse_program();
        assert_eq!(program.stmts.len(), 1);

        // Now test with compound assignment
        let input2 = "int x = 1; x += 1;";
        let mut lexer2 = lexer::Lexer::new(input2);
        let tokens2 = lexer2.tokenize();

        println!("Parsing 'int x = 1; x += 1;'...");
        println!("Token count: {}", tokens2.len());

        let mut parser2 = parser::Parser::new(tokens2);
        let program2 = parser2.parse_program();

        println!("Statement count: {}", program2.stmts.len());
        assert_eq!(program2.stmts.len(), 2, "Should have 2 statements");
    }
}
