/// Comparison operator tests for Int32 (scalar) type
#[cfg(test)]
mod tests {
    use crate::compiler::expr::expr_test_util::ExprTest;
    use crate::vm::opcodes::LpsOpCode;

    #[test]
    fn test_less_than() -> Result<(), String> {
        ExprTest::new("1 < 2")
            .expect_ast(|b| {
                let left = b.int32(1);
                let right = b.int32(2);
                b.less(left, right)
            })
            .expect_opcodes(vec![
                LpsOpCode::PushInt32(1),
                LpsOpCode::PushInt32(2),
                LpsOpCode::LessInt32,
                LpsOpCode::Return,
            ])
            .expect_result_bool(true)
            .run()?;

        ExprTest::new("5 < 3").expect_result_bool(false).run()
    }

    #[test]
    fn test_greater_than() -> Result<(), String> {
        ExprTest::new("5 > 3")
            .expect_ast(|b| {
                let left = b.int32(5);
                let right = b.int32(3);
                b.greater(left, right)
            })
            .expect_opcodes(vec![
                LpsOpCode::PushInt32(5),
                LpsOpCode::PushInt32(3),
                LpsOpCode::GreaterInt32,
                LpsOpCode::Return,
            ])
            .expect_result_bool(true)
            .run()?;

        ExprTest::new("2 > 5").expect_result_bool(false).run()
    }

    #[test]
    fn test_less_equal() -> Result<(), String> {
        ExprTest::new("2 <= 3")
            .expect_ast(|b| {
                let left = b.int32(2);
                let right = b.int32(3);
                b.less_eq(left, right)
            })
            .expect_opcodes(vec![
                LpsOpCode::PushInt32(2),
                LpsOpCode::PushInt32(3),
                LpsOpCode::LessEqInt32,
                LpsOpCode::Return,
            ])
            .expect_result_bool(true)
            .run()?;

        // Equal case
        ExprTest::new("5 <= 5").expect_result_bool(true).run()?;

        // Less case
        ExprTest::new("3 <= 5").expect_result_bool(true).run()?;

        // Greater case
        ExprTest::new("7 <= 5").expect_result_bool(false).run()
    }

    #[test]
    fn test_greater_equal() -> Result<(), String> {
        ExprTest::new("5 >= 3")
            .expect_ast(|b| {
                let left = b.int32(5);
                let right = b.int32(3);
                b.greater_eq(left, right)
            })
            .expect_opcodes(vec![
                LpsOpCode::PushInt32(5),
                LpsOpCode::PushInt32(3),
                LpsOpCode::GreaterEqInt32,
                LpsOpCode::Return,
            ])
            .expect_result_bool(true)
            .run()?;

        // Equal case
        ExprTest::new("5 >= 5").expect_result_bool(true).run()?;

        // Greater case
        ExprTest::new("7 >= 5").expect_result_bool(true).run()?;

        // Less case
        ExprTest::new("3 >= 5").expect_result_bool(false).run()
    }

    #[test]
    fn test_equal() -> Result<(), String> {
        ExprTest::new("2 == 2")
            .expect_ast(|b| {
                let left = b.int32(2);
                let right = b.int32(2);
                b.eq(left, right)
            })
            .expect_opcodes(vec![
                LpsOpCode::PushInt32(2),
                LpsOpCode::PushInt32(2),
                LpsOpCode::EqInt32,
                LpsOpCode::Return,
            ])
            .expect_result_bool(true)
            .run()?;

        ExprTest::new("2 == 3").expect_result_bool(false).run()
    }

    #[test]
    fn test_not_equal() -> Result<(), String> {
        ExprTest::new("2 != 3")
            .expect_ast(|b| {
                let left = b.int32(2);
                let right = b.int32(3);
                b.not_eq(left, right)
            })
            .expect_opcodes(vec![
                LpsOpCode::PushInt32(2),
                LpsOpCode::PushInt32(3),
                LpsOpCode::NotEqInt32,
                LpsOpCode::Return,
            ])
            .expect_result_bool(true)
            .run()?;

        ExprTest::new("2 != 2").expect_result_bool(false).run()
    }

    #[test]
    fn test_edge_cases() -> Result<(), String> {
        // Zero
        ExprTest::new("0 < 1").expect_result_bool(true).run()?;

        ExprTest::new("0 == 0").expect_result_bool(true).run()?;

        // Negative numbers
        ExprTest::new("-3 < -1").expect_result_bool(true).run()?;

        ExprTest::new("-1 > -3").expect_result_bool(true).run()?;

        // Large numbers
        ExprTest::new("1000 > 500").expect_result_bool(true).run()
    }
}
