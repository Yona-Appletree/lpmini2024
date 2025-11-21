/// Unary negation tests for Vec4 type
#[cfg(test)]
mod tests {
    use crate::compiler::expr::expr_test_util::ExprTest;
    use crate::dec32::{ToDec32, Vec4};
    use crate::shared::Type;
    use crate::vm::opcodes::LpsOpCode;

    #[test]
    fn test_negation() -> Result<(), String> {
        ExprTest::new("-vec4(1.0, 2.0, 3.0, 4.0)")
            .expect_ast(|b| {
                let arg1 = b.num(1.0);
                let arg2 = b.num(2.0);
                let arg3 = b.num(3.0);
                let arg4 = b.num(4.0);
                let operand = b.vec4(vec![arg1, arg2, arg3, arg4]);
                b.neg(operand, Type::Vec4)
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_dec32()),
                LpsOpCode::Push(2.0.to_dec32()),
                LpsOpCode::Push(3.0.to_dec32()),
                LpsOpCode::Push(4.0.to_dec32()),
                LpsOpCode::NegVec4,
                LpsOpCode::Return,
            ])
            .expect_result_vec4(Vec4::new(
                -1.0.to_dec32(),
                -2.0.to_dec32(),
                -3.0.to_dec32(),
                -4.0.to_dec32(),
            ))
            .run()
    }

    #[test]
    fn test_negation_zero() -> Result<(), String> {
        ExprTest::new("-vec4(0.0, 0.0, 0.0, 0.0)")
            .expect_result_vec4(Vec4::new(
                0.0.to_dec32(),
                0.0.to_dec32(),
                0.0.to_dec32(),
                0.0.to_dec32(),
            ))
            .run()
    }

    #[test]
    fn test_negation_negative() -> Result<(), String> {
        ExprTest::new("--vec4(1.0, 2.0, 3.0, 4.0)")
            .expect_result_vec4(Vec4::new(
                1.0.to_dec32(),
                2.0.to_dec32(),
                3.0.to_dec32(),
                4.0.to_dec32(),
            ))
            .run()
    }

    #[test]
    fn test_negation_with_expression() -> Result<(), String> {
        ExprTest::new("-(vec4(1.0, 2.0, 3.0, 4.0) + vec4(1.0, 1.0, 1.0, 1.0))")
            .expect_result_vec4(Vec4::new(
                -2.0.to_dec32(),
                -3.0.to_dec32(),
                -4.0.to_dec32(),
                -5.0.to_dec32(),
            ))
            .run()
    }
}
