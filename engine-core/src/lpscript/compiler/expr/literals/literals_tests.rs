/// Literal expression tests
#[cfg(test)]
mod tests {
    use crate::lpscript::compiler::expr::expr_test_util::ExprTest;
    
    use crate::lpscript::shared::Type;
    use crate::lpscript::vm::opcodes::LpsOpCode;
    use crate::math::ToFixed;

    #[test]
    fn test_float_literal() -> Result<(), String> {
        ExprTest::new("42.5")
            .expect_ast(|b| b.num(42.5))
            .expect_opcodes(vec![LpsOpCode::Push(42.5.to_fixed()), LpsOpCode::Return])
            .expect_result_fixed(42.5)
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
            .expect_opcodes(vec![LpsOpCode::Push(0.0.to_fixed()), LpsOpCode::Return])
            .expect_result_fixed(0.0)
            .run()
    }

    #[test]
    fn test_one() -> Result<(), String> {
        ExprTest::new("1.0")
            .expect_ast(|b| b.num(1.0))
            .expect_opcodes(vec![LpsOpCode::Push(1.0.to_fixed()), LpsOpCode::Return])
            .expect_result_fixed(1.0)
            .run()
    }

    #[test]
    fn test_parenthesized_expression() -> Result<(), String> {
        ExprTest::new("(1.0 + 2.0)")
            .expect_ast(|b| {
                let left = b.num(1.0);
                let right = b.num(2.0);
                b.add(left, right, Type::Fixed)
            })
            .expect_result_fixed(3.0)
            .run()
    }

    #[test]
    fn test_negative_literal() -> Result<(), String> {
        ExprTest::new("-5.0")
            .expect_ast(|b| b.num(-5.0))
            .expect_opcodes(vec![LpsOpCode::Push((-5.0).to_fixed()), LpsOpCode::Return])
            .expect_result_fixed(-5.0)
            .run()
    }

    #[test]
    fn test_fractional_literal() -> Result<(), String> {
        ExprTest::new("0.5")
            .expect_ast(|b| b.num(0.5))
            .expect_result_fixed(0.5)
            .run()
    }

    // Type checking tests (using ExprTest validates types automatically)
    #[test]
    fn test_simple_number_typecheck() -> Result<(), String> {
        ExprTest::new("42.0").expect_result_fixed(42.0).run()
    }

    #[test]
    fn test_int_number_typecheck() -> Result<(), String> {
        ExprTest::new("42").expect_result_int(42).run()
    }
}
