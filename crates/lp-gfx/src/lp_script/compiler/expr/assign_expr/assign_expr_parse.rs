use alloc::boxed::Box;

/// Assignment expression parsing
use crate::lp_script::compiler::ast::{Expr, ExprKind};
use crate::lp_script::compiler::error::ParseError;
use crate::lp_script::compiler::lexer::TokenKind;
use crate::lp_script::compiler::parser::Parser;
use crate::lp_script::shared::Span;

impl Parser {
    // Assignment expression: lvalue = rvalue (right-associative)
    // Supports simple assignment (=) and compound assignments (+=, -=, *=, /=, %=, &=, |=, ^=, <<=, >>=)
    pub(crate) fn parse_assignment_expr(&mut self) -> Result<Expr, ParseError> {
        self.enter_recursion()?;
        let expr = self.ternary()?;

        // Check if this is an assignment (simple or compound)
        let op_token = self.current().kind.clone();

        let result = match op_token {
            // Simple assignment
            TokenKind::Eq => {
                if let ExprKind::Variable(name) = &expr.kind {
                    let name = name.clone();
                    let start = expr.span.start;
                    self.advance(); // consume '='
                    let value = self.parse_assignment_expr()?; // right-associative
                    let end = value.span.end;

                    Ok(Expr::new(
                        ExprKind::Assign {
                            target: name,
                            value: Box::new(value),
                        },
                        Span::new(start, end),
                    ))
                } else {
                    Ok(expr)
                }
            }

            // Compound assignments - desugar to simple assignment with binary operation
            // e.g., x += 5 becomes x = x + 5
            TokenKind::PlusEq => self.parse_compound_assignment(expr, ExprKind::Add),
            TokenKind::MinusEq => self.parse_compound_assignment(expr, ExprKind::Sub),
            TokenKind::StarEq => self.parse_compound_assignment(expr, ExprKind::Mul),
            TokenKind::SlashEq => self.parse_compound_assignment(expr, ExprKind::Div),
            TokenKind::PercentEq => self.parse_compound_assignment(expr, ExprKind::Mod),
            TokenKind::AmpersandEq => self.parse_compound_assignment(expr, ExprKind::BitwiseAnd),
            TokenKind::PipeEq => self.parse_compound_assignment(expr, ExprKind::BitwiseOr),
            TokenKind::CaretEq => self.parse_compound_assignment(expr, ExprKind::BitwiseXor),
            TokenKind::LShiftEq => self.parse_compound_assignment(expr, ExprKind::LeftShift),
            TokenKind::RShiftEq => self.parse_compound_assignment(expr, ExprKind::RightShift),

            _ => Ok(expr),
        };

        self.exit_recursion();
        result
    }

    /// Helper for compound assignment operators
    fn parse_compound_assignment<F>(&mut self, expr: Expr, make_op: F) -> Result<Expr, ParseError>
    where
        F: FnOnce(Box<Expr>, Box<Expr>) -> ExprKind,
    {
        if let ExprKind::Variable(name) = &expr.kind {
            let name = name.clone();
            let start = expr.span.start;
            self.advance(); // consume compound operator
            let rhs = self.ternary()?; // Use ternary to avoid infinite recursion

            // Create a reference to the variable for the left side of the operation
            let var_ref = Expr::new(ExprKind::Variable(name.clone()), Span::new(start, start));

            // Create the binary operation
            let end = rhs.span.end;
            let op = Expr::new(
                make_op(Box::new(var_ref), Box::new(rhs)),
                Span::new(start, end),
            );

            // Create the assignment
            Ok(Expr::new(
                ExprKind::Assign {
                    target: name,
                    value: Box::new(op),
                },
                Span::new(start, end),
            ))
        } else {
            Ok(expr)
        }
    }
}
