/// Unary negation tests for Fixed (scalar) type
#[cfg(test)]
mod tests {
    use crate::compiler::expr::expr_test_util::ExprTest;
    use crate::fixed::ToFixed;
    use crate::shared::Type;
    use crate::vm::opcodes::LpsOpCode;

    #[test]
    fn test_negation() -> Result<(), String> {
        ExprTest::new("-5.0")
            .expect_ast(|b| {
                let operand = b.num(5.0);
                b.neg(operand, Type::Fixed)
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(5.0.to_fixed()),
                LpsOpCode::NegFixed,
                LpsOpCode::Return,
            ])
            .expect_result_fixed(-5.0)
            .run()
    }

    #[test]
    fn test_negation_zero() -> Result<(), String> {
        ExprTest::new("-0.0").expect_result_fixed(0.0).run()
    }

    #[test]
    fn test_negation_negative() -> Result<(), String> {
        ExprTest::new("--3.0").expect_result_fixed(3.0).run()
    }

    #[test]
    fn test_negation_with_expression() -> Result<(), String> {
        ExprTest::new("-(1.0 + 2.0)")
            .expect_result_fixed(-3.0)
            .run()
    }
}
