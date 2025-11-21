/// Literal expression tests
#[cfg(test)]
mod tests {
    use crate::compiler::expr::expr_test_util::ExprTest;
    use crate::dec32::ToDec32;
    use crate::shared::Type;
    use crate::vm::opcodes::LpsOpCode;

    #[test]
    fn test_float_literal() -> Result<(), String> {
        ExprTest::new("42.5")
            .expect_ast(|b| b.num(42.5))
            .expect_opcodes(vec![LpsOpCode::Push(42.5.to_dec32()), LpsOpCode::Return])
            .expect_result_dec32(42.5)
            .run()
    }

    #[test]
    fn test_int_literal() -> Result<(), String> {
        ExprTest::new("42")
            .expect_ast(|b| b.int32(42))
            .expect_opcodes(vec![LpsOpCode::PushInt32(42), LpsOpCode::Return])
            .expect_result_int(42)
            .run()
    }

    #[test]
    fn test_zero() -> Result<(), String> {
        ExprTest::new("0.0")
            .expect_ast(|b| b.num(0.0))
            .expect_opcodes(vec![LpsOpCode::Push(0.0.to_dec32()), LpsOpCode::Return])
            .expect_result_dec32(0.0)
            .run()
    }

    #[test]
    fn test_one() -> Result<(), String> {
        ExprTest::new("1.0")
            .expect_ast(|b| b.num(1.0))
            .expect_opcodes(vec![LpsOpCode::Push(1.0.to_dec32()), LpsOpCode::Return])
            .expect_result_dec32(1.0)
            .run()
    }

    #[test]
    fn test_parenthesized_expression() -> Result<(), String> {
        ExprTest::new("(1.0 + 2.0)")
            .expect_ast(|b| {
                let left = b.num(1.0);
                let right = b.num(2.0);
                b.add(left, right, Type::Dec32)
            })
            .expect_result_dec32(3.0)
            .run()
    }

    #[test]
    fn test_negative_literal() -> Result<(), String> {
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
    fn test_fractional_literal() -> Result<(), String> {
        ExprTest::new("0.5")
            .expect_ast(|b| b.num(0.5))
            .expect_result_dec32(0.5)
            .run()
    }

    // Type checking tests (using ExprTest validates types automatically)
    #[test]
    fn test_simple_number_typecheck() -> Result<(), String> {
        ExprTest::new("42.0").expect_result_dec32(42.0).run()
    }

    #[test]
    fn test_int_number_typecheck() -> Result<(), String> {
        ExprTest::new("42").expect_result_int(42).run()
    }
}
