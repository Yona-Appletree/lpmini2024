/// Parser: converts tokens to AST with spans
extern crate alloc;
use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::string::String;

use super::lexer::{Token, TokenKind};
use super::ast::{Expr, ExprKind, Stmt, StmtKind, Program, FunctionDef, Parameter};
use crate::lpscript::error::{Span, Type};

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
    
    /// Parse an expression (expression mode)
    pub fn parse(&mut self) -> Expr {
        self.ternary()
    }
    
    /// Parse a full program (script mode)
    pub fn parse_program(&mut self) -> Program {
        let start = self.current().span.start;
        let mut functions = Vec::new();
        let mut stmts = Vec::new();
        
        // Parse function definitions first (must come before statements)
        while self.is_function_definition() {
            functions.push(self.parse_function_def());
        }
        
        // Parse top-level statements
        while !matches!(self.current().kind, TokenKind::Eof) {
            stmts.push(self.parse_stmt());
        }
        
        let end = if !stmts.is_empty() {
            stmts.last().unwrap().span.end
        } else if !functions.is_empty() {
            functions.last().unwrap().span.end
        } else {
            start
        };
        
        Program {
            functions,
            stmts,
            span: Span::new(start, end),
        }
    }
    
    /// Check if the current position is a function definition
    /// Function def: type name(params) { body }
    fn is_function_definition(&mut self) -> bool {
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
    fn parse_function_def(&mut self) -> FunctionDef {
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
    
    /// Parse a type keyword
    fn parse_type(&mut self) -> Type {
        let ty = match &self.current().kind {
            TokenKind::Float => Type::Fixed,
            TokenKind::Int => Type::Int32,
            TokenKind::Vec2 => Type::Vec2,
            TokenKind::Vec3 => Type::Vec3,
            TokenKind::Vec4 => Type::Vec4,
            TokenKind::Void => Type::Void,
            _ => Type::Fixed, // Fallback
        };
        self.advance();
        ty
    }
    
    /// Parse a statement
    fn parse_stmt(&mut self) -> Stmt {
        let start = self.current().span.start;
        
        let stmt = match &self.current().kind {
            TokenKind::If => self.parse_if_stmt(),
            TokenKind::While => self.parse_while_stmt(),
            TokenKind::For => self.parse_for_stmt(),
            TokenKind::Return => self.parse_return_stmt(),
            TokenKind::LBrace => self.parse_block(),
            TokenKind::Float | TokenKind::Int => self.parse_var_decl(),
            TokenKind::Ident(name) => {
                // Could be assignment or expression statement
                let name = name.clone();
                self.advance();
                
                if matches!(self.current().kind, TokenKind::Eq) {
                    // Assignment
                    self.advance(); // consume '='
                    let value = self.ternary();
                    self.consume_semicolon();
                    let end = self.current().span.end;
                    Stmt::new(
                        StmtKind::Assignment { name, value },
                        Span::new(start, end),
                    )
                } else {
                    // Put token back and parse as expression statement
                    self.pos -= 1;
                    self.parse_expr_stmt()
                }
            }
            _ => self.parse_expr_stmt(),
        };
        
        stmt
    }
    
    fn parse_if_stmt(&mut self) -> Stmt {
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
    
    fn parse_while_stmt(&mut self) -> Stmt {
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
    
    fn parse_for_stmt(&mut self) -> Stmt {
        let start = self.current().span.start;
        self.advance(); // consume 'for'
        
        self.expect(TokenKind::LParen);
        
        // Parse init (can be var decl or expression)
        let init = if matches!(self.current().kind, TokenKind::Semicolon) {
            self.advance();
            None
        } else if matches!(self.current().kind, TokenKind::Float | TokenKind::Int) {
            // Parse var decl inline without consuming semicolon
            let decl = self.parse_var_decl_no_semicolon();
            self.expect(TokenKind::Semicolon);
            Some(Box::new(decl))
        } else {
            // Parse expression and consume its semicolon
            let expr = self.ternary();
            self.expect(TokenKind::Semicolon);
            let span = expr.span;
            Some(Box::new(Stmt::new(StmtKind::Expr(expr), span)))
        };
        
        // Parse condition
        let condition = if matches!(self.current().kind, TokenKind::Semicolon) {
            self.advance();
            None
        } else {
            let cond = self.ternary();
            self.expect(TokenKind::Semicolon);
            Some(cond)
        };
        
        // Parse increment (can be assignment expression)
        let increment = if matches!(self.current().kind, TokenKind::RParen) {
            None
        } else {
            Some(self.parse_assignment_expr())
        };
        
        self.expect(TokenKind::RParen);
        
        let body = Box::new(self.parse_stmt());
        let end = body.span.end;
        
        Stmt::new(
            StmtKind::For {
                init,
                condition,
                increment,
                body,
            },
            Span::new(start, end),
        )
    }
    
    fn parse_return_stmt(&mut self) -> Stmt {
        let start = self.current().span.start;
        self.advance(); // consume 'return'
        
        let expr = self.ternary();
        self.consume_semicolon();
        let end = self.current().span.end;
        
        Stmt::new(StmtKind::Return(expr), Span::new(start, end))
    }
    
    fn parse_var_decl(&mut self) -> Stmt {
        let stmt = self.parse_var_decl_no_semicolon();
        self.consume_semicolon();
        stmt
    }
    
    fn parse_var_decl_no_semicolon(&mut self) -> Stmt {
        let start = self.current().span.start;
        
        // Parse type
        let ty = self.parse_type();
        
        // Parse name
        let name = if let TokenKind::Ident(n) = &self.current().kind {
            let name = n.clone();
            self.advance();
            name
        } else {
            String::from("error")
        };
        
        // Parse optional initializer
        let init = if matches!(self.current().kind, TokenKind::Eq) {
            self.advance(); // consume '='
            Some(self.ternary())
        } else {
            None
        };
        
        let end = self.current().span.end;
        
        Stmt::new(
            StmtKind::VarDecl { ty, name, init },
            Span::new(start, end),
        )
    }
    
    fn parse_block(&mut self) -> Stmt {
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
    
    fn parse_expr_stmt(&mut self) -> Stmt {
        let expr = self.ternary();
        let span = expr.span;
        self.consume_semicolon();
        Stmt::new(StmtKind::Expr(expr), span)
    }
    
    fn expect(&mut self, expected: TokenKind) -> bool {
        if core::mem::discriminant(&self.current().kind) == core::mem::discriminant(&expected) {
            self.advance();
            true
        } else {
            // In debug mode, report what we expected vs what we got
            // Note: eprintln! not available in no_std
            false
        }
    }
    
    fn consume_semicolon(&mut self) -> bool {
        if matches!(self.current().kind, TokenKind::Semicolon) {
            self.advance();
            true
        } else {
            // Note: eprintln! not available in no_std
            false
        }
    }
    
    // Assignment expression: lvalue = rvalue (right-associative)
    // This is needed for for-loop increments like: i = i + 1
    fn parse_assignment_expr(&mut self) -> Expr {
        let expr = self.ternary();
        
        // Check if this is an assignment
        if matches!(self.current().kind, TokenKind::Eq) {
            if let ExprKind::Variable(name) = &expr.kind {
                let start = expr.span.start;
                self.advance(); // consume '='
                let value = Box::new(self.parse_assignment_expr()); // right-associative
                let end = value.span.end;
                
                // Return assignment expression (which evaluates to the assigned value)
                return Expr::new(
                    ExprKind::Assign {
                        target: name.clone(),
                        value,
                    },
                    Span::new(start, end),
                );
            }
        }
        
        expr
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
        let mut expr = self.postfix();
        
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
    
    // Postfix: swizzle (.xyzw, .rgba, .stpq)
    fn postfix(&mut self) -> Expr {
        let mut expr = self.primary();
        
        // Handle swizzle operations
        while matches!(self.current().kind, TokenKind::Dot) {
            let start = expr.span.start;
            self.advance(); // consume '.'
            
            // Read the swizzle components
            if let TokenKind::Ident(components) = &self.current().kind {
                let components = components.clone();
                let end = self.current().span.end;
                self.advance();
                
                expr = Expr::new(
                    ExprKind::Swizzle {
                        expr: Box::new(expr),
                        components,
                    },
                    Span::new(start, end),
                );
            } else {
                // Invalid swizzle, just break
                break;
            }
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
            TokenKind::Vec2 | TokenKind::Vec3 | TokenKind::Vec4 => {
                let vec_kind = token.kind.clone();
                let start = token.span.start;
                self.advance();
                
                // Must be followed by '(' for constructor
                self.expect(TokenKind::LParen);
                let args = self.parse_args();
                let end = if matches!(self.current().kind, TokenKind::RParen) {
                    let span = self.current().span;
                    self.advance(); // consume ')'
                    span.end
                } else {
                    self.current().span.end
                };
                
                let kind = match vec_kind {
                    TokenKind::Vec2 => ExprKind::Vec2Constructor(args),
                    TokenKind::Vec3 => ExprKind::Vec3Constructor(args),
                    TokenKind::Vec4 => ExprKind::Vec4Constructor(args),
                    _ => unreachable!(),
                };
                
                Expr::new(kind, Span::new(start, end))
            }
            
            TokenKind::Ident(name) => {
                let name = name.clone();
                let start = token.span.start;
                self.advance();
                
                if matches!(self.current().kind, TokenKind::LParen) {
                    // Function call
                    self.advance(); // consume '('
                    let args = self.parse_args();
                    let end = if matches!(self.current().kind, TokenKind::RParen) {
                        let span = self.current().span;
                        self.advance(); // consume ')'
                        span.end
                    } else {
                        self.current().span.end
                    };
                    
                    let kind = ExprKind::Call { name, args };
                    
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
