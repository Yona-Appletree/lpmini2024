/// Unary negation tests for Int32 (scalar) type
#[cfg(test)]
mod tests {
    use crate::lp_script::compiler::expr::expr_test_util::ExprTest;
    use crate::lp_script::shared::Type;
    use crate::lp_script::vm::opcodes::LpsOpCode;

    #[test]
    fn test_negation() -> Result<(), String> {
        ExprTest::new("-5")
            .expect_ast(|b| {
                let operand = b.int32(5);
                b.neg(operand, Type::Int32)
            })
            .expect_opcodes(vec![
                LpsOpCode::PushInt32(5),
                LpsOpCode::NegInt32,
                LpsOpCode::Return,
            ])
            .expect_result_int(-5)
            .run()
    }

    #[test]
    fn test_negation_zero() -> Result<(), String> {
        ExprTest::new("-0").expect_result_int(0).run()
    }

    #[test]
    fn test_negation_negative() -> Result<(), String> {
        ExprTest::new("--3").expect_result_int(3).run()
    }

    #[test]
    fn test_negation_with_expression() -> Result<(), String> {
        ExprTest::new("-(1 + 2)").expect_result_int(-3).run()
    }

    #[test]
    fn test_negation_large_number() -> Result<(), String> {
        ExprTest::new("-1000").expect_result_int(-1000).run()
    }
}
