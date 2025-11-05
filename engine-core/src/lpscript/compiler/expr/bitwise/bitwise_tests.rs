/// Tests for bitwise operators
#[cfg(test)]
mod tests {
    use crate::lpscript::compiler::expr::expr_test_util::ExprTest;
    use crate::lpscript::compiler::test_ast::*;
    use crate::lpscript::shared::Type;
    use crate::lpscript::vm::opcodes::LpsOpCode;

    #[test]
    fn test_bitwise_and() -> Result<(), String> {
        ExprTest::new("12 & 10")
            .expect_ast(bitwise_and(int32(12), int32(10), Type::Int32))
            .expect_opcodes(vec![
                LpsOpCode::PushInt32(12),
                LpsOpCode::PushInt32(10),
                LpsOpCode::BitwiseAndInt32,
                LpsOpCode::Return,
            ])
            .expect_result_int(8) // 12 & 10 = 8
            .run()
    }

    #[test]
    fn test_bitwise_or() -> Result<(), String> {
        ExprTest::new("12 | 10")
            .expect_ast(bitwise_or(int32(12), int32(10), Type::Int32))
            .expect_opcodes(vec![
                LpsOpCode::PushInt32(12),
                LpsOpCode::PushInt32(10),
                LpsOpCode::BitwiseOrInt32,
                LpsOpCode::Return,
            ])
            .expect_result_int(14) // 12 | 10 = 14
            .run()
    }

    #[test]
    fn test_bitwise_xor() -> Result<(), String> {
        ExprTest::new("12 ^ 10")
            .expect_ast(bitwise_xor(int32(12), int32(10), Type::Int32))
            .expect_opcodes(vec![
                LpsOpCode::PushInt32(12),
                LpsOpCode::PushInt32(10),
                LpsOpCode::BitwiseXorInt32,
                LpsOpCode::Return,
            ])
            .expect_result_int(6) // 12 ^ 10 = 6
            .run()
    }

    #[test]
    fn test_bitwise_not() -> Result<(), String> {
        ExprTest::new("~5")
            .expect_ast(bitwise_not(int32(5), Type::Int32))
            .expect_opcodes(vec![
                LpsOpCode::PushInt32(5),
                LpsOpCode::BitwiseNotInt32,
                LpsOpCode::Return,
            ])
            .expect_result_int(-6) // ~5 = -6
            .run()
    }

    #[test]
    fn test_left_shift() -> Result<(), String> {
        ExprTest::new("5 << 2")
            .expect_ast(left_shift(int32(5), int32(2), Type::Int32))
            .expect_opcodes(vec![
                LpsOpCode::PushInt32(5),
                LpsOpCode::PushInt32(2),
                LpsOpCode::LeftShiftInt32,
                LpsOpCode::Return,
            ])
            .expect_result_int(20) // 5 << 2 = 20
            .run()
    }

    #[test]
    fn test_right_shift() -> Result<(), String> {
        ExprTest::new("20 >> 2")
            .expect_ast(right_shift(int32(20), int32(2), Type::Int32))
            .expect_opcodes(vec![
                LpsOpCode::PushInt32(20),
                LpsOpCode::PushInt32(2),
                LpsOpCode::RightShiftInt32,
                LpsOpCode::Return,
            ])
            .expect_result_int(5) // 20 >> 2 = 5
            .run()
    }

    #[test]
    fn test_bitwise_precedence() -> Result<(), String> {
        // Test that & has lower precedence than <<
        // Should be 8 & (4 << 1) = 8 & 8 = 8
        ExprTest::new("8 & 4 << 1")
            .expect_ast(bitwise_and(
                int32(8),
                left_shift(int32(4), int32(1), Type::Int32),
                Type::Int32,
            ))
            .expect_result_int(8)
            .run()
    }

    #[test]
    fn test_bitwise_complex() -> Result<(), String> {
        // Complex expression with multiple bitwise operators
        // (12 & 10) | (5 ^ 3) = 8 | 6 = 14
        ExprTest::new("(12 & 10) | (5 ^ 3)")
            .expect_ast(bitwise_or(
                bitwise_and(int32(12), int32(10), Type::Int32),
                bitwise_xor(int32(5), int32(3), Type::Int32),
                Type::Int32,
            ))
            .expect_result_int(14)
            .run()
    }
}
