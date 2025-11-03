/// Lexer for expression language
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Number(f32),
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
    
    fn read_number(&mut self) -> f32 {
        let mut num = String::new();
        while let Some(ch) = self.current() {
            if ch.is_numeric() || ch == '.' {
                num.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        num.parse().unwrap_or(0.0)
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
            let is_eof = tok == Token::Eof;
            tokens.push(tok);
            if is_eof {
                break;
            }
        }
        tokens
    }
    
    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        
        match self.current() {
            None => Token::Eof,
            Some(ch) => match ch {
                '+' => { self.advance(); Token::Plus }
                '-' => { self.advance(); Token::Minus }
                '*' => { self.advance(); Token::Star }
                '/' => { self.advance(); Token::Slash }
                '%' => { self.advance(); Token::Percent }
                '^' => { self.advance(); Token::Caret }
                '(' => { self.advance(); Token::LParen }
                ')' => { self.advance(); Token::RParen }
                ',' => { self.advance(); Token::Comma }
                '?' => { self.advance(); Token::Question }
                ':' => { self.advance(); Token::Colon }
                '<' => {
                    self.advance();
                    if self.current() == Some('=') {
                        self.advance();
                        Token::LessEq
                    } else {
                        Token::Less
                    }
                }
                '>' => {
                    self.advance();
                    if self.current() == Some('=') {
                        self.advance();
                        Token::GreaterEq
                    } else {
                        Token::Greater
                    }
                }
                '=' => {
                    self.advance();
                    if self.current() == Some('=') {
                        self.advance();
                        Token::EqEq
                    } else {
                        Token::Eof // Single = not supported
                    }
                }
                '!' => {
                    self.advance();
                    if self.current() == Some('=') {
                        self.advance();
                        Token::NotEq
                    } else {
                        Token::Eof // Single ! not supported yet
                    }
                }
                '&' => {
                    self.advance();
                    if self.current() == Some('&') {
                        self.advance();
                        Token::And
                    } else {
                        Token::Eof
                    }
                }
                '|' => {
                    self.advance();
                    if self.current() == Some('|') {
                        self.advance();
                        Token::Or
                    } else {
                        Token::Eof
                    }
                }
                '0'..='9' | '.' => Token::Number(self.read_number()),
                'a'..='z' | 'A'..='Z' | '_' => Token::Ident(self.read_ident()),
                _ => { self.advance(); Token::Eof }
            }
        }
    }
}

