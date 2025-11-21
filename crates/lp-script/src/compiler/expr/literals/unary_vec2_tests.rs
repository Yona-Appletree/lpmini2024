/// Unary negation tests for Vec2 type
#[cfg(test)]
mod tests {
    use crate::compiler::expr::expr_test_util::ExprTest;
    use crate::dec32::{ToDec32, Vec2};
    use crate::shared::Type;
    use crate::vm::opcodes::LpsOpCode;

    #[test]
    fn test_negation() -> Result<(), String> {
        ExprTest::new("-vec2(1.0, 2.0)")
            .expect_ast(|b| {
                let arg1 = b.num(1.0);
                let arg2 = b.num(2.0);
                let operand = b.vec2(vec![arg1, arg2]);
                b.neg(operand, Type::Vec2)
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_dec32()),
                LpsOpCode::Push(2.0.to_dec32()),
                LpsOpCode::NegVec2,
                LpsOpCode::Return,
            ])
            .expect_result_vec2(Vec2::new(-1.0.to_dec32(), -2.0.to_dec32()))
            .run()
    }

    #[test]
    fn test_negation_zero() -> Result<(), String> {
        ExprTest::new("-vec2(0.0, 0.0)")
            .expect_result_vec2(Vec2::new(0.0.to_dec32(), 0.0.to_dec32()))
            .run()
    }

    #[test]
    fn test_negation_negative() -> Result<(), String> {
        ExprTest::new("--vec2(1.0, 2.0)")
            .expect_result_vec2(Vec2::new(1.0.to_dec32(), 2.0.to_dec32()))
            .run()
    }

    #[test]
    fn test_negation_with_expression() -> Result<(), String> {
        ExprTest::new("-(vec2(1.0, 2.0) + vec2(3.0, 4.0))")
            .expect_result_vec2(Vec2::new(-4.0.to_dec32(), -6.0.to_dec32()))
            .run()
    }
}
