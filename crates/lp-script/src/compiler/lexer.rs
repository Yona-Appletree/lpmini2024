/// Lexer for expression language with span tracking
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

use crate::shared::Span;

/// Token with span information
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Literals
    FloatLiteral(f32),
    IntLiteral(i32),
    Ident(String),

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,    // Modulo
    Caret,      // Bitwise XOR
    PlusPlus,   // Increment
    MinusMinus, // Decrement

    // Comparisons
    Less,
    Greater,
    LessEq,
    GreaterEq,
    EqEq,
    NotEq,

    // Logical
    And,  // &&
    Or,   // ||
    Bang, // Logical not !

    // Bitwise
    Ampersand, // &
    Pipe,      // |
    Tilde,     // ~
    LShift,    // <<
    RShift,    // >>

    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Semicolon,
    Question, // Ternary ?
    Colon,    // Ternary :
    Dot,      // Member access / swizzle
    Eq,       // Assignment =

    // Compound assignments
    PlusEq,      // +=
    MinusEq,     // -=
    StarEq,      // *=
    SlashEq,     // /=
    PercentEq,   // %=
    AmpersandEq, // &=
    PipeEq,      // |=
    CaretEq,     // ^=
    LShiftEq,    // <<=
    RShiftEq,    // >>=

    // Keywords
    If,
    Else,
    While,
    For,
    Return,
    Float,
    Int,
    Vec2,
    Vec3,
    Vec4,
    Void,

    Eof,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    fn current(&self) -> Option<char> {
        if self.pos < self.input.len() {
            Some(self.input[self.pos])
        } else {
            None
        }
    }

    #[allow(dead_code)]
    fn peek(&self, offset: usize) -> Option<char> {
        let idx = self.pos + offset;
        if idx < self.input.len() {
            Some(self.input[idx])
        } else {
            None
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_line_comment(&mut self) {
        // Skip the //
        self.advance();
        self.advance();

        // Skip until newline or EOF
        while let Some(ch) = self.current() {
            if ch == '\n' {
                self.advance();
                break;
            }
            self.advance();
        }
    }

    fn skip_block_comment(&mut self) {
        // Skip the /*
        self.advance();
        self.advance();

        // Skip until */ or EOF
        while let Some(ch) = self.current() {
            if ch == '*' {
                if let Some('/') = self.peek(1) {
                    self.advance(); // Skip *
                    self.advance(); // Skip /
                    break;
                }
            }
            self.advance();
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            self.skip_whitespace();

            if let Some('/') = self.current() {
                match self.peek(1) {
                    Some('/') => self.skip_line_comment(),
                    Some('*') => self.skip_block_comment(),
                    _ => break,
                }
            } else {
                break;
            }
        }
    }

    fn read_number(&mut self) -> (String, bool) {
        let mut num = String::new();
        let mut is_float = false;

        while let Some(ch) = self.current() {
            if ch.is_numeric() || ch == '.' {
                if ch == '.' {
                    is_float = true;
                }
                num.push(ch);
                self.advance();
            } else if ch == 'e' || ch == 'E' {
                // Scientific notation
                is_float = true;
                num.push(ch);
                self.advance();
                if let Some(sign) = self.current() {
                    if sign == '+' || sign == '-' {
                        num.push(sign);
                        self.advance();
                    }
                }
            } else if ch == 'f' || ch == 'F' {
                // Float suffix
                is_float = true;
                self.advance();
                break;
            } else if ch == 'x' || ch == 'X' {
                // Hex literal
                if num == "0" {
                    num.push(ch);
                    self.advance();
                    while let Some(hex_ch) = self.current() {
                        if hex_ch.is_ascii_hexdigit() {
                            num.push(hex_ch);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
                break;
            } else {
                break;
            }
        }
        (num, is_float)
    }

    fn read_ident(&mut self) -> String {
        let mut ident = String::new();
        while let Some(ch) = self.current() {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        ident
    }

    fn keyword_or_ident(ident: String) -> TokenKind {
        match ident.as_str() {
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,
            "for" => TokenKind::For,
            "return" => TokenKind::Return,
            "float" => TokenKind::Float,
            "int" => TokenKind::Int,
            "vec2" => TokenKind::Vec2,
            "vec3" => TokenKind::Vec3,
            "vec4" => TokenKind::Vec4,
            "void" => TokenKind::Void,
            _ => TokenKind::Ident(ident),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            let is_eof = matches!(tok.kind, TokenKind::Eof);
            tokens.push(tok);
            if is_eof {
                break;
            }
        }
        tokens
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace_and_comments();

        let start = self.pos;

        match self.current() {
            None => Token {
                kind: TokenKind::Eof,
                span: Span::new(start, start),
            },
            Some(ch) => {
                let kind = match ch {
                    '+' => {
                        self.advance();
                        if self.current() == Some('+') {
                            self.advance();
                            TokenKind::PlusPlus
                        } else if self.current() == Some('=') {
                            self.advance();
                            TokenKind::PlusEq
                        } else {
                            TokenKind::Plus
                        }
                    }
                    '-' => {
                        self.advance();
                        if self.current() == Some('-') {
                            self.advance();
                            TokenKind::MinusMinus
                        } else if self.current() == Some('=') {
                            self.advance();
                            TokenKind::MinusEq
                        } else {
                            TokenKind::Minus
                        }
                    }
                    '*' => {
                        self.advance();
                        if self.current() == Some('=') {
                            self.advance();
                            TokenKind::StarEq
                        } else {
                            TokenKind::Star
                        }
                    }
                    '/' => {
                        self.advance();
                        if self.current() == Some('=') {
                            self.advance();
                            TokenKind::SlashEq
                        } else {
                            TokenKind::Slash
                        }
                    }
                    '%' => {
                        self.advance();
                        if self.current() == Some('=') {
                            self.advance();
                            TokenKind::PercentEq
                        } else {
                            TokenKind::Percent
                        }
                    }
                    '^' => {
                        self.advance();
                        if self.current() == Some('=') {
                            self.advance();
                            TokenKind::CaretEq
                        } else {
                            TokenKind::Caret
                        }
                    }
                    '(' => {
                        self.advance();
                        TokenKind::LParen
                    }
                    ')' => {
                        self.advance();
                        TokenKind::RParen
                    }
                    '{' => {
                        self.advance();
                        TokenKind::LBrace
                    }
                    '}' => {
                        self.advance();
                        TokenKind::RBrace
                    }
                    ',' => {
                        self.advance();
                        TokenKind::Comma
                    }
                    ';' => {
                        self.advance();
                        TokenKind::Semicolon
                    }
                    '?' => {
                        self.advance();
                        TokenKind::Question
                    }
                    ':' => {
                        self.advance();
                        TokenKind::Colon
                    }
                    '.' => {
                        // Check if it's a number like .5 or a dot operator
                        if let Some(next_ch) = self.peek(1) {
                            if next_ch.is_numeric() {
                                // It's a number like .5
                                let (num_str, is_float) = self.read_number();
                                if is_float {
                                    TokenKind::FloatLiteral(num_str.parse().unwrap_or(0.0))
                                } else {
                                    TokenKind::FloatLiteral(num_str.parse().unwrap_or(0.0))
                                }
                            } else {
                                // It's a dot operator
                                self.advance();
                                TokenKind::Dot
                            }
                        } else {
                            // End of input, treat as dot
                            self.advance();
                            TokenKind::Dot
                        }
                    }
                    '<' => {
                        self.advance();
                        if self.current() == Some('=') {
                            self.advance();
                            TokenKind::LessEq
                        } else if self.current() == Some('<') {
                            self.advance();
                            if self.current() == Some('=') {
                                self.advance();
                                TokenKind::LShiftEq
                            } else {
                                TokenKind::LShift
                            }
                        } else {
                            TokenKind::Less
                        }
                    }
                    '>' => {
                        self.advance();
                        if self.current() == Some('=') {
                            self.advance();
                            TokenKind::GreaterEq
                        } else if self.current() == Some('>') {
                            self.advance();
                            if self.current() == Some('=') {
                                self.advance();
                                TokenKind::RShiftEq
                            } else {
                                TokenKind::RShift
                            }
                        } else {
                            TokenKind::Greater
                        }
                    }
                    '=' => {
                        self.advance();
                        if self.current() == Some('=') {
                            self.advance();
                            TokenKind::EqEq
                        } else {
                            TokenKind::Eq // Assignment
                        }
                    }
                    '!' => {
                        self.advance();
                        if self.current() == Some('=') {
                            self.advance();
                            TokenKind::NotEq
                        } else {
                            TokenKind::Bang
                        }
                    }
                    '&' => {
                        self.advance();
                        if self.current() == Some('&') {
                            self.advance();
                            TokenKind::And
                        } else if self.current() == Some('=') {
                            self.advance();
                            TokenKind::AmpersandEq
                        } else {
                            TokenKind::Ampersand
                        }
                    }
                    '|' => {
                        self.advance();
                        if self.current() == Some('|') {
                            self.advance();
                            TokenKind::Or
                        } else if self.current() == Some('=') {
                            self.advance();
                            TokenKind::PipeEq
                        } else {
                            TokenKind::Pipe
                        }
                    }
                    '~' => {
                        self.advance();
                        TokenKind::Tilde
                    }
                    '0'..='9' => {
                        let (num_str, is_float) = self.read_number();
                        if num_str.starts_with("0x") || num_str.starts_with("0X") {
                            // Hex number
                            let hex_str = &num_str[2..];
                            if let Ok(val) = i32::from_str_radix(hex_str, 16) {
                                TokenKind::IntLiteral(val)
                            } else {
                                TokenKind::IntLiteral(0)
                            }
                        } else if is_float {
                            TokenKind::FloatLiteral(num_str.parse().unwrap_or(0.0))
                        } else {
                            // Try to parse as int, fallback to float
                            if let Ok(val) = num_str.parse::<i32>() {
                                TokenKind::IntLiteral(val)
                            } else {
                                TokenKind::FloatLiteral(num_str.parse().unwrap_or(0.0))
                            }
                        }
                    }
                    'a'..='z' | 'A'..='Z' | '_' => Self::keyword_or_ident(self.read_ident()),
                    _ => {
                        self.advance();
                        TokenKind::Eof
                    }
                };

                Token {
                    kind,
                    span: Span::new(start, self.pos),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tokenize(input: &str) -> Vec<TokenKind> {
        let mut lexer = Lexer::new(input);
        lexer.tokenize().into_iter().map(|t| t.kind).collect()
    }

    #[test]
    fn test_basic_operators() {
        assert_eq!(
            tokenize("+ - * / % ^"),
            vec![
                TokenKind::Plus,
                TokenKind::Minus,
                TokenKind::Star,
                TokenKind::Slash,
                TokenKind::Percent,
                TokenKind::Caret,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_comparison_operators() {
        assert_eq!(
            tokenize("< > <= >= == !="),
            vec![
                TokenKind::Less,
                TokenKind::Greater,
                TokenKind::LessEq,
                TokenKind::GreaterEq,
                TokenKind::EqEq,
                TokenKind::NotEq,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_logical_operators() {
        assert_eq!(
            tokenize("&& ||"),
            vec![TokenKind::And, TokenKind::Or, TokenKind::Eof]
        );
    }

    #[test]
    fn test_delimiters() {
        assert_eq!(
            tokenize("( ) { } , ; ? : . ="),
            vec![
                TokenKind::LParen,
                TokenKind::RParen,
                TokenKind::LBrace,
                TokenKind::RBrace,
                TokenKind::Comma,
                TokenKind::Semicolon,
                TokenKind::Question,
                TokenKind::Colon,
                TokenKind::Dot,
                TokenKind::Eq,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_keywords() {
        assert_eq!(
            tokenize("if else while for return"),
            vec![
                TokenKind::If,
                TokenKind::Else,
                TokenKind::While,
                TokenKind::For,
                TokenKind::Return,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_type_keywords() {
        assert_eq!(
            tokenize("float int vec2 vec3 vec4 void"),
            vec![
                TokenKind::Float,
                TokenKind::Int,
                TokenKind::Vec2,
                TokenKind::Vec3,
                TokenKind::Vec4,
                TokenKind::Void,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_identifiers() {
        let tokens = tokenize("foo bar baz_123 _underscore camelCase");
        assert!(matches!(tokens[0], TokenKind::Ident(ref s) if s == "foo"));
        assert!(matches!(tokens[1], TokenKind::Ident(ref s) if s == "bar"));
        assert!(matches!(tokens[2], TokenKind::Ident(ref s) if s == "baz_123"));
        assert!(matches!(tokens[3], TokenKind::Ident(ref s) if s == "_underscore"));
        assert!(matches!(tokens[4], TokenKind::Ident(ref s) if s == "camelCase"));
        assert!(matches!(tokens[5], TokenKind::Eof));
    }

    #[test]
    fn test_integer_literals() {
        let tokens = tokenize("0 1 42 999");
        assert_eq!(tokens[0], TokenKind::IntLiteral(0));
        assert_eq!(tokens[1], TokenKind::IntLiteral(1));
        assert_eq!(tokens[2], TokenKind::IntLiteral(42));
        assert_eq!(tokens[3], TokenKind::IntLiteral(999));
        assert!(matches!(tokens[4], TokenKind::Eof));
    }

    #[test]
    fn test_float_literals() {
        let tokens = tokenize("0.0 1.5 3.14159 .5 0.25");
        assert_eq!(tokens[0], TokenKind::FloatLiteral(0.0));
        assert_eq!(tokens[1], TokenKind::FloatLiteral(1.5));
        assert_eq!(tokens[2], TokenKind::FloatLiteral(3.14159));
        assert_eq!(tokens[3], TokenKind::FloatLiteral(0.5));
        assert_eq!(tokens[4], TokenKind::FloatLiteral(0.25));
        assert!(matches!(tokens[5], TokenKind::Eof));
    }

    #[test]
    fn test_float_suffix() {
        let tokens = tokenize("1f 2.5f 0F 3.0F");
        assert_eq!(tokens[0], TokenKind::FloatLiteral(1.0));
        assert_eq!(tokens[1], TokenKind::FloatLiteral(2.5));
        assert_eq!(tokens[2], TokenKind::FloatLiteral(0.0));
        assert_eq!(tokens[3], TokenKind::FloatLiteral(3.0));
        assert!(matches!(tokens[4], TokenKind::Eof));
    }

    #[test]
    fn test_scientific_notation() {
        let tokens = tokenize("1e5 2.5e-3 1.0E+2 3e10");
        assert_eq!(tokens[0], TokenKind::FloatLiteral(1e5));
        assert_eq!(tokens[1], TokenKind::FloatLiteral(2.5e-3));
        assert_eq!(tokens[2], TokenKind::FloatLiteral(1.0e2));
        assert_eq!(tokens[3], TokenKind::FloatLiteral(3e10));
        assert!(matches!(tokens[4], TokenKind::Eof));
    }

    #[test]
    fn test_hex_literals() {
        let tokens = tokenize("0x0 0xFF 0x10 0xABCD");
        assert_eq!(tokens[0], TokenKind::IntLiteral(0));
        assert_eq!(tokens[1], TokenKind::IntLiteral(255));
        assert_eq!(tokens[2], TokenKind::IntLiteral(16));
        assert_eq!(tokens[3], TokenKind::IntLiteral(0xABCD));
        assert!(matches!(tokens[4], TokenKind::Eof));
    }

    #[test]
    fn test_hex_case_insensitive() {
        let tokens = tokenize("0xFF 0Xff 0xfF 0xff");
        assert_eq!(tokens[0], TokenKind::IntLiteral(255));
        assert_eq!(tokens[1], TokenKind::IntLiteral(255));
        assert_eq!(tokens[2], TokenKind::IntLiteral(255));
        assert_eq!(tokens[3], TokenKind::IntLiteral(255));
        assert!(matches!(tokens[4], TokenKind::Eof));
    }

    #[test]
    fn test_whitespace_handling() {
        assert_eq!(
            tokenize("  \t\n  42  \t  +  \n  foo  "),
            vec![
                TokenKind::IntLiteral(42),
                TokenKind::Plus,
                TokenKind::Ident("foo".into()),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_dot_vs_float() {
        let tokens = tokenize("foo.bar .5 1.5");
        assert!(matches!(tokens[0], TokenKind::Ident(ref s) if s == "foo"));
        assert_eq!(tokens[1], TokenKind::Dot);
        assert!(matches!(tokens[2], TokenKind::Ident(ref s) if s == "bar"));
        assert_eq!(tokens[3], TokenKind::FloatLiteral(0.5));
        assert_eq!(tokens[4], TokenKind::FloatLiteral(1.5));
        assert!(matches!(tokens[5], TokenKind::Eof));
    }

    #[test]
    fn test_expression() {
        let tokens = tokenize("x + y * 2.5");
        assert!(matches!(tokens[0], TokenKind::Ident(ref s) if s == "x"));
        assert_eq!(tokens[1], TokenKind::Plus);
        assert!(matches!(tokens[2], TokenKind::Ident(ref s) if s == "y"));
        assert_eq!(tokens[3], TokenKind::Star);
        assert_eq!(tokens[4], TokenKind::FloatLiteral(2.5));
        assert!(matches!(tokens[5], TokenKind::Eof));
    }

    #[test]
    fn test_function_call() {
        let tokens = tokenize("sin(x)");
        assert!(matches!(tokens[0], TokenKind::Ident(ref s) if s == "sin"));
        assert_eq!(tokens[1], TokenKind::LParen);
        assert!(matches!(tokens[2], TokenKind::Ident(ref s) if s == "x"));
        assert_eq!(tokens[3], TokenKind::RParen);
        assert!(matches!(tokens[4], TokenKind::Eof));
    }

    #[test]
    fn test_ternary_operator() {
        let tokens = tokenize("x > 0 ? 1 : -1");
        assert!(matches!(tokens[0], TokenKind::Ident(ref s) if s == "x"));
        assert_eq!(tokens[1], TokenKind::Greater);
        assert_eq!(tokens[2], TokenKind::IntLiteral(0));
        assert_eq!(tokens[3], TokenKind::Question);
        assert_eq!(tokens[4], TokenKind::IntLiteral(1));
        assert_eq!(tokens[5], TokenKind::Colon);
        assert_eq!(tokens[6], TokenKind::Minus);
        assert_eq!(tokens[7], TokenKind::IntLiteral(1));
        assert!(matches!(tokens[8], TokenKind::Eof));
    }

    #[test]
    fn test_variable_declaration() {
        let tokens = tokenize("float x = 3.14;");
        assert_eq!(tokens[0], TokenKind::Float);
        assert!(matches!(tokens[1], TokenKind::Ident(ref s) if s == "x"));
        assert_eq!(tokens[2], TokenKind::Eq);
        assert_eq!(tokens[3], TokenKind::FloatLiteral(3.14));
        assert_eq!(tokens[4], TokenKind::Semicolon);
        assert!(matches!(tokens[5], TokenKind::Eof));
    }

    #[test]
    fn test_swizzle() {
        let tokens = tokenize("pos.xy");
        assert!(matches!(tokens[0], TokenKind::Ident(ref s) if s == "pos"));
        assert_eq!(tokens[1], TokenKind::Dot);
        assert!(matches!(tokens[2], TokenKind::Ident(ref s) if s == "xy"));
        assert!(matches!(tokens[3], TokenKind::Eof));
    }

    #[test]
    fn test_multi_digit_numbers() {
        let tokens = tokenize("123456789");
        assert_eq!(tokens[0], TokenKind::IntLiteral(123456789));
        assert!(matches!(tokens[1], TokenKind::Eof));
    }

    #[test]
    fn test_negative_numbers() {
        // Note: negative numbers are typically parsed as unary minus + number
        let tokens = tokenize("-42");
        assert_eq!(tokens[0], TokenKind::Minus);
        assert_eq!(tokens[1], TokenKind::IntLiteral(42));
        assert!(matches!(tokens[2], TokenKind::Eof));
    }

    #[test]
    fn test_complex_expression() {
        let tokens = tokenize("(a + b) * c / d - e % f");
        assert_eq!(tokens[0], TokenKind::LParen);
        assert!(matches!(tokens[1], TokenKind::Ident(ref s) if s == "a"));
        assert_eq!(tokens[2], TokenKind::Plus);
        assert!(matches!(tokens[3], TokenKind::Ident(ref s) if s == "b"));
        assert_eq!(tokens[4], TokenKind::RParen);
        assert_eq!(tokens[5], TokenKind::Star);
        assert!(matches!(tokens[6], TokenKind::Ident(ref s) if s == "c"));
        assert_eq!(tokens[7], TokenKind::Slash);
        assert!(matches!(tokens[8], TokenKind::Ident(ref s) if s == "d"));
        assert_eq!(tokens[9], TokenKind::Minus);
        assert!(matches!(tokens[10], TokenKind::Ident(ref s) if s == "e"));
        assert_eq!(tokens[11], TokenKind::Percent);
        assert!(matches!(tokens[12], TokenKind::Ident(ref s) if s == "f"));
        assert!(matches!(tokens[13], TokenKind::Eof));
    }

    #[test]
    fn test_span_tracking() {
        let mut lexer = Lexer::new("42 + x");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0].span, Span::new(0, 2)); // "42"
        assert_eq!(tokens[1].span, Span::new(3, 4)); // "+"
        assert_eq!(tokens[2].span, Span::new(5, 6)); // "x"
    }

    #[test]
    fn test_empty_input() {
        let tokens = tokenize("");
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0], TokenKind::Eof));
    }

    #[test]
    fn test_only_whitespace() {
        let tokens = tokenize("   \t\n  ");
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0], TokenKind::Eof));
    }

    #[test]
    fn test_line_comment() {
        let tokens = tokenize("42 // this is a comment\n+ 1");
        assert_eq!(tokens[0], TokenKind::IntLiteral(42));
        assert_eq!(tokens[1], TokenKind::Plus);
        assert_eq!(tokens[2], TokenKind::IntLiteral(1));
        assert!(matches!(tokens[3], TokenKind::Eof));
    }

    #[test]
    fn test_line_comment_at_end() {
        let tokens = tokenize("42 + 1 // comment at end");
        assert_eq!(tokens[0], TokenKind::IntLiteral(42));
        assert_eq!(tokens[1], TokenKind::Plus);
        assert_eq!(tokens[2], TokenKind::IntLiteral(1));
        assert!(matches!(tokens[3], TokenKind::Eof));
    }

    #[test]
    fn test_line_comment_at_start() {
        let tokens = tokenize("// comment at start\n42");
        assert_eq!(tokens[0], TokenKind::IntLiteral(42));
        assert!(matches!(tokens[1], TokenKind::Eof));
    }

    #[test]
    fn test_multiple_line_comments() {
        let tokens = tokenize("// first comment\n42 // second comment\n+ // third\n1");
        assert_eq!(tokens[0], TokenKind::IntLiteral(42));
        assert_eq!(tokens[1], TokenKind::Plus);
        assert_eq!(tokens[2], TokenKind::IntLiteral(1));
        assert!(matches!(tokens[3], TokenKind::Eof));
    }

    #[test]
    fn test_block_comment() {
        let tokens = tokenize("42 /* this is a block comment */ + 1");
        assert_eq!(tokens[0], TokenKind::IntLiteral(42));
        assert_eq!(tokens[1], TokenKind::Plus);
        assert_eq!(tokens[2], TokenKind::IntLiteral(1));
        assert!(matches!(tokens[3], TokenKind::Eof));
    }

    #[test]
    fn test_multiline_block_comment() {
        let tokens = tokenize("42 /* this is\n a multiline\n block comment */ + 1");
        assert_eq!(tokens[0], TokenKind::IntLiteral(42));
        assert_eq!(tokens[1], TokenKind::Plus);
        assert_eq!(tokens[2], TokenKind::IntLiteral(1));
        assert!(matches!(tokens[3], TokenKind::Eof));
    }

    #[test]
    fn test_nested_block_comment_syntax() {
        // Note: This lexer doesn't support nested block comments,
        // so /* /* */ will end at the first */
        let tokens = tokenize("42 /* outer /* inner */ still in comment */ + 1");
        // After the first */, "still in comment */ + 1" is parsed
        assert_eq!(tokens[0], TokenKind::IntLiteral(42));
        assert!(matches!(tokens[1], TokenKind::Ident(ref s) if s == "still"));
    }

    #[test]
    fn test_block_comment_with_asterisks() {
        let tokens = tokenize("42 /* *** comment *** */ + 1");
        assert_eq!(tokens[0], TokenKind::IntLiteral(42));
        assert_eq!(tokens[1], TokenKind::Plus);
        assert_eq!(tokens[2], TokenKind::IntLiteral(1));
        assert!(matches!(tokens[3], TokenKind::Eof));
    }

    #[test]
    fn test_comment_only() {
        let tokens = tokenize("// just a comment");
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0], TokenKind::Eof));
    }

    #[test]
    fn test_block_comment_only() {
        let tokens = tokenize("/* just a block comment */");
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0], TokenKind::Eof));
    }

    #[test]
    fn test_division_vs_comment() {
        // Test that division operator is distinct from comment start
        let tokens = tokenize("42 / 2");
        assert_eq!(tokens[0], TokenKind::IntLiteral(42));
        assert_eq!(tokens[1], TokenKind::Slash);
        assert_eq!(tokens[2], TokenKind::IntLiteral(2));
        assert!(matches!(tokens[3], TokenKind::Eof));
    }

    #[test]
    fn test_mixed_comments() {
        let tokens = tokenize(
            "float x = 42; // line comment\n/* block\ncomment */\nfloat y = 10; // another",
        );
        assert_eq!(tokens[0], TokenKind::Float);
        assert!(matches!(tokens[1], TokenKind::Ident(ref s) if s == "x"));
        assert_eq!(tokens[2], TokenKind::Eq);
        assert_eq!(tokens[3], TokenKind::IntLiteral(42));
        assert_eq!(tokens[4], TokenKind::Semicolon);
        assert_eq!(tokens[5], TokenKind::Float);
        assert!(matches!(tokens[6], TokenKind::Ident(ref s) if s == "y"));
        assert_eq!(tokens[7], TokenKind::Eq);
        assert_eq!(tokens[8], TokenKind::IntLiteral(10));
        assert_eq!(tokens[9], TokenKind::Semicolon);
        assert!(matches!(tokens[10], TokenKind::Eof));
    }

    #[test]
    fn test_comment_between_tokens() {
        let tokens = tokenize("foo/*comment*/bar");
        assert!(matches!(tokens[0], TokenKind::Ident(ref s) if s == "foo"));
        assert!(matches!(tokens[1], TokenKind::Ident(ref s) if s == "bar"));
        assert!(matches!(tokens[2], TokenKind::Eof));
    }

    #[test]
    fn test_unterminated_line_comment() {
        // Line comment at EOF (no newline)
        let tokens = tokenize("42 // comment");
        assert_eq!(tokens[0], TokenKind::IntLiteral(42));
        assert!(matches!(tokens[1], TokenKind::Eof));
    }

    #[test]
    fn test_unterminated_block_comment() {
        // Block comment at EOF (no closing */)
        let tokens = tokenize("42 /* unterminated");
        assert_eq!(tokens[0], TokenKind::IntLiteral(42));
        assert!(matches!(tokens[1], TokenKind::Eof));
    }

    #[test]
    fn test_comment_with_special_chars() {
        let tokens = tokenize("42 // comment with !@#$%^&*()+-={}[]|\\:;\"'<>?,./\n+ 1");
        assert_eq!(tokens[0], TokenKind::IntLiteral(42));
        assert_eq!(tokens[1], TokenKind::Plus);
        assert_eq!(tokens[2], TokenKind::IntLiteral(1));
        assert!(matches!(tokens[3], TokenKind::Eof));
    }
}
