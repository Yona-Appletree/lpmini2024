/// Parser: converts tokens to AST with spans
extern crate alloc;
use alloc::vec::Vec;
use alloc::boxed::Box;

use super::lexer::{Token, TokenKind};
use super::ast::{Expr, ExprKind};
use crate::lpscript::error::Span;

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
        
        if matches!(self.current().kind, TokenKind::Question) {
            let start = expr.span.start;
            self.advance(); // consume '?'
            let true_expr = Box::new(self.ternary());
            
            if matches!(self.current().kind, TokenKind::Colon) {
                self.advance(); // consume ':'
                let false_expr = Box::new(self.ternary());
                let end = false_expr.span.end;
                
                expr = Expr::new(
                    ExprKind::Ternary {
                        condition: Box::new(expr),
                        true_expr,
                        false_expr,
                    },
                    Span::new(start, end),
                );
            }
        }
        
        expr
    }
    
    // Logical OR: ||
    fn logical_or(&mut self) -> Expr {
        let mut expr = self.logical_and();
        
        while matches!(self.current().kind, TokenKind::Or) {
            let start = expr.span.start;
            self.advance();
            let right = self.logical_and();
            let end = right.span.end;
            expr = Expr::new(
                ExprKind::Or(Box::new(expr), Box::new(right)),
                Span::new(start, end),
            );
        }
        
        expr
    }
    
    // Logical AND: &&
    fn logical_and(&mut self) -> Expr {
        let mut expr = self.comparison();
        
        while matches!(self.current().kind, TokenKind::And) {
            let start = expr.span.start;
            self.advance();
            let right = self.comparison();
            let end = right.span.end;
            expr = Expr::new(
                ExprKind::And(Box::new(expr), Box::new(right)),
                Span::new(start, end),
            );
        }
        
        expr
    }
    
    // Comparison: <, >, <=, >=, ==, !=
    fn comparison(&mut self) -> Expr {
        let mut expr = self.additive();
        
        loop {
            let start = expr.span.start;
            match &self.current().kind {
                TokenKind::Less => {
                    self.advance();
                    let right = self.additive();
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::Less(Box::new(expr), Box::new(right)),
                        Span::new(start, end),
                    );
                }
                TokenKind::Greater => {
                    self.advance();
                    let right = self.additive();
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::Greater(Box::new(expr), Box::new(right)),
                        Span::new(start, end),
                    );
                }
                TokenKind::LessEq => {
                    self.advance();
                    let right = self.additive();
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::LessEq(Box::new(expr), Box::new(right)),
                        Span::new(start, end),
                    );
                }
                TokenKind::GreaterEq => {
                    self.advance();
                    let right = self.additive();
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::GreaterEq(Box::new(expr), Box::new(right)),
                        Span::new(start, end),
                    );
                }
                TokenKind::EqEq => {
                    self.advance();
                    let right = self.additive();
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::Eq(Box::new(expr), Box::new(right)),
                        Span::new(start, end),
                    );
                }
                TokenKind::NotEq => {
                    self.advance();
                    let right = self.additive();
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::NotEq(Box::new(expr), Box::new(right)),
                        Span::new(start, end),
                    );
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
            let start = expr.span.start;
            match &self.current().kind {
                TokenKind::Plus => {
                    self.advance();
                    let right = self.multiplicative();
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::Add(Box::new(expr), Box::new(right)),
                        Span::new(start, end),
                    );
                }
                TokenKind::Minus => {
                    self.advance();
                    let right = self.multiplicative();
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::Sub(Box::new(expr), Box::new(right)),
                        Span::new(start, end),
                    );
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
            let start = expr.span.start;
            match &self.current().kind {
                TokenKind::Star => {
                    self.advance();
                    let right = self.exponential();
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::Mul(Box::new(expr), Box::new(right)),
                        Span::new(start, end),
                    );
                }
                TokenKind::Slash => {
                    self.advance();
                    let right = self.exponential();
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::Div(Box::new(expr), Box::new(right)),
                        Span::new(start, end),
                    );
                }
                TokenKind::Percent => {
                    self.advance();
                    let right = self.exponential();
                    let end = right.span.end;
                    expr = Expr::new(
                        ExprKind::Mod(Box::new(expr), Box::new(right)),
                        Span::new(start, end),
                    );
                }
                _ => break,
            }
        }
        
        expr
    }
    
    // Exponential: ^ (right-associative)
    fn exponential(&mut self) -> Expr {
        let mut expr = self.primary();
        
        if matches!(self.current().kind, TokenKind::Caret) {
            let start = expr.span.start;
            self.advance();
            let right = self.exponential(); // Right-associative
            let end = right.span.end;
            expr = Expr::new(
                ExprKind::Pow(Box::new(expr), Box::new(right)),
                Span::new(start, end),
            );
        }
        
        expr
    }
    
    // Primary: number, variable, function call, or parenthesized expression
    fn primary(&mut self) -> Expr {
        let token = self.current().clone();
        
        match &token.kind {
            TokenKind::FloatLiteral(n) => {
                self.advance();
                Expr::new(ExprKind::Number(*n), token.span)
            }
            TokenKind::IntLiteral(n) => {
                self.advance();
                Expr::new(ExprKind::IntNumber(*n), token.span)
            }
            TokenKind::Ident(name) => {
                let name = name.clone();
                let start = token.span.start;
                self.advance();
                
                if matches!(self.current().kind, TokenKind::LParen) {
                    // Function call or vector constructor
                    self.advance(); // consume '('
                    let args = self.parse_args();
                    let end = if matches!(self.current().kind, TokenKind::RParen) {
                        let span = self.current().span;
                        self.advance(); // consume ')'
                        span.end
                    } else {
                        self.current().span.end
                    };
                    
                    // Check for vector constructors
                    let kind = match name.as_str() {
                        "vec2" if args.len() == 2 => {
                            ExprKind::Vec2Constructor(
                                Box::new(args[0].clone()),
                                Box::new(args[1].clone()),
                            )
                        }
                        "vec3" if args.len() == 3 => {
                            ExprKind::Vec3Constructor(
                                Box::new(args[0].clone()),
                                Box::new(args[1].clone()),
                                Box::new(args[2].clone()),
                            )
                        }
                        "vec4" if args.len() == 4 => {
                            ExprKind::Vec4Constructor(
                                Box::new(args[0].clone()),
                                Box::new(args[1].clone()),
                                Box::new(args[2].clone()),
                                Box::new(args[3].clone()),
                            )
                        }
                        _ => ExprKind::Call { name, args },
                    };
                    
                    Expr::new(kind, Span::new(start, end))
                } else {
                    // Variable
                    Expr::new(ExprKind::Variable(name), token.span)
                }
            }
            TokenKind::LParen => {
                self.advance(); // consume '('
                let expr = self.ternary();
                if matches!(self.current().kind, TokenKind::RParen) {
                    self.advance(); // consume ')'
                }
                expr
            }
            _ => {
                // Error fallback
                Expr::new(ExprKind::Number(0.0), token.span)
            }
        }
    }
    
    fn parse_args(&mut self) -> Vec<Expr> {
        let mut args = Vec::new();
        
        if matches!(self.current().kind, TokenKind::RParen) {
            return args;
        }
        
        loop {
            args.push(self.ternary());
            if matches!(self.current().kind, TokenKind::Comma) {
                self.advance();
            } else {
                break;
            }
        }
        
        args
    }
}
