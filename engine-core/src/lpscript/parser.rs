/// Parser: converts tokens to AST
extern crate alloc;
use alloc::vec::Vec;
use alloc::boxed::Box;

use super::lexer::Token;
use super::ast::Expr;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }
    
    fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }
    
    fn advance(&mut self) {
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
    }
    
    pub fn parse(&mut self) -> Expr {
        self.ternary()
    }
    
    // Ternary: condition ? true_expr : false_expr
    fn ternary(&mut self) -> Expr {
        let mut expr = self.logical_or();
        
        if matches!(self.current(), Token::Question) {
            self.advance(); // consume '?'
            let true_expr = Box::new(self.ternary());
            
            if matches!(self.current(), Token::Colon) {
                self.advance(); // consume ':'
                let false_expr = Box::new(self.ternary());
                
                expr = Expr::Ternary {
                    condition: Box::new(expr),
                    true_expr,
                    false_expr,
                };
            }
        }
        
        expr
    }
    
    // Logical OR: ||
    fn logical_or(&mut self) -> Expr {
        let mut expr = self.logical_and();
        
        while matches!(self.current(), Token::Or) {
            self.advance();
            let right = self.logical_and();
            expr = Expr::Or(Box::new(expr), Box::new(right));
        }
        
        expr
    }
    
    // Logical AND: &&
    fn logical_and(&mut self) -> Expr {
        let mut expr = self.comparison();
        
        while matches!(self.current(), Token::And) {
            self.advance();
            let right = self.comparison();
            expr = Expr::And(Box::new(expr), Box::new(right));
        }
        
        expr
    }
    
    // Comparison: <, >, <=, >=, ==, !=
    fn comparison(&mut self) -> Expr {
        let mut expr = self.additive();
        
        loop {
            match self.current() {
                Token::Less => {
                    self.advance();
                    let right = self.additive();
                    expr = Expr::Less(Box::new(expr), Box::new(right));
                }
                Token::Greater => {
                    self.advance();
                    let right = self.additive();
                    expr = Expr::Greater(Box::new(expr), Box::new(right));
                }
                Token::LessEq => {
                    self.advance();
                    let right = self.additive();
                    expr = Expr::LessEq(Box::new(expr), Box::new(right));
                }
                Token::GreaterEq => {
                    self.advance();
                    let right = self.additive();
                    expr = Expr::GreaterEq(Box::new(expr), Box::new(right));
                }
                Token::EqEq => {
                    self.advance();
                    let right = self.additive();
                    expr = Expr::Eq(Box::new(expr), Box::new(right));
                }
                Token::NotEq => {
                    self.advance();
                    let right = self.additive();
                    expr = Expr::NotEq(Box::new(expr), Box::new(right));
                }
                _ => break,
            }
        }
        
        expr
    }
    
    // Additive: + -
    fn additive(&mut self) -> Expr {
        let mut expr = self.multiplicative();
        
        loop {
            match self.current() {
                Token::Plus => {
                    self.advance();
                    let right = self.multiplicative();
                    expr = Expr::Add(Box::new(expr), Box::new(right));
                }
                Token::Minus => {
                    self.advance();
                    let right = self.multiplicative();
                    expr = Expr::Sub(Box::new(expr), Box::new(right));
                }
                _ => break,
            }
        }
        
        expr
    }
    
    // Multiplicative: * / %
    fn multiplicative(&mut self) -> Expr {
        let mut expr = self.exponential();
        
        loop {
            match self.current() {
                Token::Star => {
                    self.advance();
                    let right = self.exponential();
                    expr = Expr::Mul(Box::new(expr), Box::new(right));
                }
                Token::Slash => {
                    self.advance();
                    let right = self.exponential();
                    expr = Expr::Div(Box::new(expr), Box::new(right));
                }
                Token::Percent => {
                    self.advance();
                    let right = self.exponential();
                    expr = Expr::Mod(Box::new(expr), Box::new(right));
                }
                _ => break,
            }
        }
        
        expr
    }
    
    // Exponential: ^ (right-associative)
    fn exponential(&mut self) -> Expr {
        let mut expr = self.primary();
        
        if matches!(self.current(), Token::Caret) {
            self.advance();
            let right = self.exponential(); // Right-associative
            expr = Expr::Pow(Box::new(expr), Box::new(right));
        }
        
        expr
    }
    
    // Primary: number, variable, function call, or parenthesized expression
    fn primary(&mut self) -> Expr {
        match self.current().clone() {
            Token::Number(n) => {
                self.advance();
                Expr::Number(n)
            }
            Token::Ident(name) => {
                self.advance();
                if matches!(self.current(), Token::LParen) {
                    // Function call
                    self.advance(); // consume '('
                    let args = self.parse_args();
                    if matches!(self.current(), Token::RParen) {
                        self.advance(); // consume ')'
                    }
                    Expr::Call { name, args }
                } else {
                    // Variable
                    Expr::Variable(name)
                }
            }
            Token::LParen => {
                self.advance(); // consume '('
                let expr = self.ternary();
                if matches!(self.current(), Token::RParen) {
                    self.advance(); // consume ')'
                }
                expr
            }
            _ => Expr::Number(0.0), // Error fallback
        }
    }
    
    fn parse_args(&mut self) -> Vec<Expr> {
        let mut args = Vec::new();
        
        if matches!(self.current(), Token::RParen) {
            return args;
        }
        
        loop {
            args.push(self.ternary());
            if matches!(self.current(), Token::Comma) {
                self.advance();
            } else {
                break;
            }
        }
        
        args
    }
}

