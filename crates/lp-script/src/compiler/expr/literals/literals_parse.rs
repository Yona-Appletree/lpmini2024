/// Literal parsing (numbers, parenthesized expressions)
use crate::compiler::ast::{Expr, ExprKind};
use crate::compiler::error::ParseError;
use crate::compiler::lexer::TokenKind;
use crate::compiler::parser::Parser;
use crate::shared::Span;
use lp_pool::LpBox;

impl Parser {
    // Unary: - ! ~ ++ -- (between exponential and postfix in precedence)
    pub(crate) fn unary(&mut self) -> Result<Expr, ParseError> {
        self.enter_recursion()?;
        let token = self.current().clone();

        let result = match &token.kind {
            TokenKind::PlusPlus => {
                self.advance();
                // Prefix increment must be followed by a variable
                if let TokenKind::Ident(name) = &self.current().kind {
                    let name = name.clone();
                    let end = self.current().span.end;
                    self.advance();
                    Ok(Expr::new(
                        ExprKind::PreIncrement(name),
                        Span::new(token.span.start, end),
                    ))
                } else {
                    // Error: prefix increment requires an l-value
                    // For now, create a dummy expression (will be caught by type checker)
                    Ok(Expr::new(
                        ExprKind::Number(0.0),
                        Span::new(token.span.start, self.current().span.end),
                    ))
                }
            }
            TokenKind::MinusMinus => {
                self.advance();
                // Prefix decrement must be followed by a variable
                if let TokenKind::Ident(name) = &self.current().kind {
                    let name = name.clone();
                    let end = self.current().span.end;
                    self.advance();
                    Ok(Expr::new(
                        ExprKind::PreDecrement(name),
                        Span::new(token.span.start, end),
                    ))
                } else {
                    // Error: prefix decrement requires an l-value
                    // For now, create a dummy expression (will be caught by type checker)
                    Ok(Expr::new(
                        ExprKind::Number(0.0),
                        Span::new(token.span.start, self.current().span.end),
                    ))
                }
            }
            TokenKind::Minus => {
                self.advance();
                let operand = self.unary()?; // Right-associative (can stack: --x)
                let end = operand.span.end;

                // Optimization: if operand is a literal, fold the negation
                match &operand.kind {
                    ExprKind::Number(n) => Ok(Expr::new(
                        ExprKind::Number(-n),
                        Span::new(token.span.start, end),
                    )),
                    ExprKind::IntNumber(n) => Ok(Expr::new(
                        ExprKind::IntNumber(-n),
                        Span::new(token.span.start, end),
                    )),
                    _ => Ok(Expr::new(
                        ExprKind::Neg(LpBox::try_new(operand)?),
                        Span::new(token.span.start, end),
                    )),
                }
            }
            TokenKind::Bang => {
                self.advance();
                let operand = self.unary()?;
                let end = operand.span.end;
                Ok(Expr::new(
                    ExprKind::Not(LpBox::try_new(operand)?),
                    Span::new(token.span.start, end),
                ))
            }
            TokenKind::Tilde => {
                self.advance();
                let operand = self.unary()?;
                let end = operand.span.end;
                Ok(Expr::new(
                    ExprKind::BitwiseNot(LpBox::try_new(operand)?),
                    Span::new(token.span.start, end),
                ))
            }
            _ => self.postfix(),
        };

        self.exit_recursion();
        result
    }

    // Primary: number, variable, function call, constructor, or parenthesized expression
    pub(crate) fn primary(&mut self) -> Result<Expr, ParseError> {
        self.enter_recursion()?;
        let token = self.current().clone();

        let result = match &token.kind {
            TokenKind::FloatLiteral(n) => {
                self.advance();
                Ok(Expr::new(ExprKind::Number(*n), token.span))
            }
            TokenKind::IntLiteral(n) => {
                self.advance();
                Ok(Expr::new(ExprKind::IntNumber(*n), token.span))
            }
            TokenKind::LParen => {
                self.advance(); // consume '('
                let expr = self.parse_assignment_expr()?;
                if matches!(self.current().kind, TokenKind::RParen) {
                    self.advance(); // consume ')'
                }
                Ok(expr)
            }
            TokenKind::Vec2 | TokenKind::Vec3 | TokenKind::Vec4 => self.parse_vec_constructor(),
            TokenKind::Ident(_) => self.parse_ident(),
            _ => {
                // Error fallback
                Ok(Expr::new(ExprKind::Number(0.0), token.span))
            }
        };

        self.exit_recursion();
        result
    }
}
