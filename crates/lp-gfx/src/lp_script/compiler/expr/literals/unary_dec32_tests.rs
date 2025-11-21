/// Unary negation tests for Dec32 (scalar) type
#[cfg(test)]
mod tests {
    use lp_math::dec32::ToDec32;

    use crate::lp_script::compiler::expr::expr_test_util::ExprTest;
    use crate::lp_script::shared::Type;
    use crate::lp_script::vm::opcodes::LpsOpCode;

    #[test]
    fn test_negation() -> Result<(), String> {
        ExprTest::new("-5.0")
            .expect_ast(|b| {
                let operand = b.num(5.0);
                b.neg(operand, Type::Dec32)
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(5.0.to_dec32()),
                LpsOpCode::NegDec32,
                LpsOpCode::Return,
            ])
            .expect_result_dec32(-5.0)
            .run()
    }

    #[test]
    fn test_negation_zero() -> Result<(), String> {
        ExprTest::new("-0.0").expect_result_dec32(0.0).run()
    }

    #[test]
    fn test_negation_negative() -> Result<(), String> {
        ExprTest::new("--3.0").expect_result_dec32(3.0).run()
    }

    #[test]
    fn test_negation_with_expression() -> Result<(), String> {
        ExprTest::new("-(1.0 + 2.0)")
            .expect_result_dec32(-3.0)
            .run()
    }
}
