/// Lexer for expression language with span tracking
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

use crate::lpscript::error::Span;

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
    Percent,  // Modulo
    Caret,    // Exponentiation
    
    // Comparisons
    Less,
    Greater,
    LessEq,
    GreaterEq,
    EqEq,
    NotEq,
    
    // Logical
    And,
    Or,
    
    // Delimiters
    LParen,
    RParen,
    Comma,
    Question,  // Ternary ?
    Colon,     // Ternary :
    
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
        self.skip_whitespace();
        
        let start = self.pos;
        
        match self.current() {
            None => Token {
                kind: TokenKind::Eof,
                span: Span::new(start, start),
            },
            Some(ch) => {
                let kind = match ch {
                    '+' => { self.advance(); TokenKind::Plus }
                    '-' => { self.advance(); TokenKind::Minus }
                    '*' => { self.advance(); TokenKind::Star }
                    '/' => { self.advance(); TokenKind::Slash }
                    '%' => { self.advance(); TokenKind::Percent }
                    '^' => { self.advance(); TokenKind::Caret }
                    '(' => { self.advance(); TokenKind::LParen }
                    ')' => { self.advance(); TokenKind::RParen }
                    ',' => { self.advance(); TokenKind::Comma }
                    '?' => { self.advance(); TokenKind::Question }
                    ':' => { self.advance(); TokenKind::Colon }
                    '<' => {
                        self.advance();
                        if self.current() == Some('=') {
                            self.advance();
                            TokenKind::LessEq
                        } else {
                            TokenKind::Less
                        }
                    }
                    '>' => {
                        self.advance();
                        if self.current() == Some('=') {
                            self.advance();
                            TokenKind::GreaterEq
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
                            TokenKind::Eof // Single = not supported
                        }
                    }
                    '!' => {
                        self.advance();
                        if self.current() == Some('=') {
                            self.advance();
                            TokenKind::NotEq
                        } else {
                            TokenKind::Eof // Single ! not supported yet
                        }
                    }
                    '&' => {
                        self.advance();
                        if self.current() == Some('&') {
                            self.advance();
                            TokenKind::And
                        } else {
                            TokenKind::Eof
                        }
                    }
                    '|' => {
                        self.advance();
                        if self.current() == Some('|') {
                            self.advance();
                            TokenKind::Or
                        } else {
                            TokenKind::Eof
                        }
                    }
                    '0'..='9' | '.' => {
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
                    'a'..='z' | 'A'..='Z' | '_' => TokenKind::Ident(self.read_ident()),
                    _ => { self.advance(); TokenKind::Eof }
                };
                
                Token {
                    kind,
                    span: Span::new(start, self.pos),
                }
            }
        }
    }
}
