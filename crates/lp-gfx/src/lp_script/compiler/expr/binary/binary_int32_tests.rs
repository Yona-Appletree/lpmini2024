/// Binary arithmetic operator tests for Int32 (scalar) type
#[cfg(test)]
mod tests {
    use crate::lp_script::compiler::expr::expr_test_util::ExprTest;
    use crate::lp_script::shared::Type;
    use crate::lp_script::vm::opcodes::LpsOpCode;

    #[test]
    fn test_addition() -> Result<(), String> {
        ExprTest::new("1 + 2")
            .expect_ast(|b| {
                let left = b.int32(1);
                let right = b.int32(2);
                b.add(left, right, Type::Int32)
            })
            .expect_opcodes(vec![
                LpsOpCode::PushInt32(1),
                LpsOpCode::PushInt32(2),
                LpsOpCode::AddInt32,
                LpsOpCode::Return,
            ])
            .expect_result_int(3)
            .run()
    }

    #[test]
    fn test_subtraction() -> Result<(), String> {
        ExprTest::new("5 - 2")
            .expect_ast(|b| {
                let left = b.int32(5);
                let right = b.int32(2);
                b.sub(left, right, Type::Int32)
            })
            .expect_opcodes(vec![
                LpsOpCode::PushInt32(5),
                LpsOpCode::PushInt32(2),
                LpsOpCode::SubInt32,
                LpsOpCode::Return,
            ])
            .expect_result_int(3)
            .run()
    }

    #[test]
    fn test_multiplication() -> Result<(), String> {
        ExprTest::new("3 * 4")
            .expect_ast(|b| {
                let left = b.int32(3);
                let right = b.int32(4);
                b.mul(left, right, Type::Int32)
            })
            .expect_opcodes(vec![
                LpsOpCode::PushInt32(3),
                LpsOpCode::PushInt32(4),
                LpsOpCode::MulInt32,
                LpsOpCode::Return,
            ])
            .expect_result_int(12)
            .run()
    }

    #[test]
    fn test_division() -> Result<(), String> {
        ExprTest::new("10 / 2")
            .expect_ast(|b| {
                let left = b.int32(10);
                let right = b.int32(2);
                b.div(left, right, Type::Int32)
            })
            .expect_opcodes(vec![
                LpsOpCode::PushInt32(10),
                LpsOpCode::PushInt32(2),
                LpsOpCode::DivInt32,
                LpsOpCode::Return,
            ])
            .expect_result_int(5)
            .run()?;

        // Integer division truncates
        ExprTest::new("7 / 2").expect_result_int(3).run()
    }

    #[test]
    fn test_modulo() -> Result<(), String> {
        ExprTest::new("10 % 3")
            .expect_ast(|b| {
                let left = b.int32(10);
                let right = b.int32(3);
                b.modulo(left, right, Type::Int32)
            })
            .expect_opcodes(vec![
                LpsOpCode::PushInt32(10),
                LpsOpCode::PushInt32(3),
                LpsOpCode::ModInt32,
                LpsOpCode::Return,
            ])
            .expect_result_int(1)
            .run()?;

        ExprTest::new("15 % 4").expect_result_int(3).run()
    }

    #[test]
    fn test_int_float_promotion() -> Result<(), String> {
        // Int + Dec32 should promote Int to Dec32
        ExprTest::new("1 + 2.0").expect_result_dec32(3.0).run()?;

        // Dec32 + Int should promote Int to Dec32
        ExprTest::new("2.0 + 1").expect_result_dec32(3.0).run()?;

        // Verify promotion in multiplication
        ExprTest::new("3 * 2.0").expect_result_dec32(6.0).run()?;

        ExprTest::new("2.0 * 3").expect_result_dec32(6.0).run()
    }

    #[test]
    fn test_operator_precedence() -> Result<(), String> {
        // 1 + 2 * 3 should be 1 + (2 * 3) = 7
        ExprTest::new("1 + 2 * 3").expect_result_int(7).run()?;

        // (1 + 2) * 3 should be 9
        ExprTest::new("(1 + 2) * 3").expect_result_int(9).run()
    }

    #[test]
    fn test_edge_cases() -> Result<(), String> {
        // Zero
        ExprTest::new("0 + 5").expect_result_int(5).run()?;

        ExprTest::new("5 * 0").expect_result_int(0).run()?;

        // Negative numbers
        ExprTest::new("-3 + 5").expect_result_int(2).run()?;

        ExprTest::new("-2 * -3").expect_result_int(6).run()?;

        // Large numbers
        ExprTest::new("1000 + 2000").expect_result_int(3000).run()
    }
}
