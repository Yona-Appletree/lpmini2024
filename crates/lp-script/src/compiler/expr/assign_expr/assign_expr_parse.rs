/// Assignment expression parsing
use crate::compiler::ast::{Expr, ExprKind};
use crate::compiler::error::ParseError;
use crate::compiler::lexer::TokenKind;
use crate::compiler::parser::Parser;
use crate::shared::Span;
use lp_pool::LpBox;

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
                            value: LpBox::try_new(value)?,
                        },
                        Span::new(start, end),
                    ))
                } else {
                    Ok(expr)
                }
            }

            // Compound assignments - desugar to simple assignment with binary operation
            // e.g., x += 5 becomes x = x + 5
            TokenKind::PlusEq => {
                self.parse_compound_assignment(expr, |left, right| ExprKind::Add(left, right))
            }
            TokenKind::MinusEq => {
                self.parse_compound_assignment(expr, |left, right| ExprKind::Sub(left, right))
            }
            TokenKind::StarEq => {
                self.parse_compound_assignment(expr, |left, right| ExprKind::Mul(left, right))
            }
            TokenKind::SlashEq => {
                self.parse_compound_assignment(expr, |left, right| ExprKind::Div(left, right))
            }
            TokenKind::PercentEq => {
                self.parse_compound_assignment(expr, |left, right| ExprKind::Mod(left, right))
            }
            TokenKind::AmpersandEq => self
                .parse_compound_assignment(expr, |left, right| ExprKind::BitwiseAnd(left, right)),
            TokenKind::PipeEq => {
                self.parse_compound_assignment(expr, |left, right| ExprKind::BitwiseOr(left, right))
            }
            TokenKind::CaretEq => self
                .parse_compound_assignment(expr, |left, right| ExprKind::BitwiseXor(left, right)),
            TokenKind::LShiftEq => {
                self.parse_compound_assignment(expr, |left, right| ExprKind::LeftShift(left, right))
            }
            TokenKind::RShiftEq => self
                .parse_compound_assignment(expr, |left, right| ExprKind::RightShift(left, right)),

            _ => Ok(expr),
        };

        self.exit_recursion();
        result
    }

    /// Helper for compound assignment operators
    fn parse_compound_assignment<F>(&mut self, expr: Expr, make_op: F) -> Result<Expr, ParseError>
    where
        F: FnOnce(LpBox<Expr>, LpBox<Expr>) -> ExprKind,
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
                make_op(LpBox::try_new(var_ref)?, LpBox::try_new(rhs)?),
                Span::new(start, end),
            );

            // Create the assignment
            Ok(Expr::new(
                ExprKind::Assign {
                    target: name,
                    value: LpBox::try_new(op)?,
                },
                Span::new(start, end),
            ))
        } else {
            Ok(expr)
        }
    }
}
