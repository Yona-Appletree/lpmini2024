/// Binary arithmetic operator tests
#[cfg(test)]
mod tests {
    use crate::lpscript::compiler::expr::expr_test_util::ExprTest;
    use crate::lpscript::compiler::test_ast::*;
    use crate::lpscript::error::Type;
    use crate::lpscript::vm::opcodes::LpsOpCode;
    use crate::math::ToFixed;

    #[test]
    fn test_addition() -> Result<(), String> {
        ExprTest::new("1.0 + 2.0")
            .expect_ast(add(num(1.0), num(2.0), Type::Fixed))
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::AddFixed,
                LpsOpCode::Return,
            ])
            .expect_result_fixed(3.0)
            .run()
    }

    #[test]
    fn test_subtraction() -> Result<(), String> {
        ExprTest::new("5.0 - 2.0")
            .expect_ast(sub(num(5.0), num(2.0), Type::Fixed))
            .expect_result_fixed(3.0)
            .run()
    }

    #[test]
    fn test_multiplication() -> Result<(), String> {
        ExprTest::new("3.0 * 4.0")
            .expect_ast(mul(num(3.0), num(4.0), Type::Fixed))
            .expect_result_fixed(12.0)
            .run()
    }

    #[test]
    fn test_division() -> Result<(), String> {
        ExprTest::new("10.0 / 2.0")
            .expect_ast(div(num(10.0), num(2.0), Type::Fixed))
            .expect_result_fixed(5.0)
            .run()
    }

    #[test]
    fn test_operator_precedence() -> Result<(), String> {
        // 1 + 2 * 3 should be 1 + (2 * 3) = 7
        ExprTest::new("1.0 + 2.0 * 3.0")
            .expect_ast(add(
                num(1.0),
                mul(num(2.0), num(3.0), Type::Fixed),
                Type::Fixed,
            ))
            .expect_result_fixed(7.0)
            .run()
    }

    #[test]
    fn test_parenthesized_expression() -> Result<(), String> {
        // (1 + 2) * 3 should be 9
        ExprTest::new("(1.0 + 2.0) * 3.0")
            .expect_ast(mul(
                add(num(1.0), num(2.0), Type::Fixed),
                num(3.0),
                Type::Fixed,
            ))
            .expect_result_fixed(9.0)
            .run()
    }

    #[test]
    fn test_int_float_promotion() -> Result<(), String> {
        ExprTest::new("1 + 2.0").expect_result_fixed(3.0).run()?;

        ExprTest::new("2.0 + 1").expect_result_fixed(3.0).run()
    }
}
