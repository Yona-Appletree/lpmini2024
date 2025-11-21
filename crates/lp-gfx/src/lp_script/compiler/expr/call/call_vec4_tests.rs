/// Function call tests for Vec4 type
#[cfg(test)]
mod tests {
    use lp_math::dec32::ToDec32;

    use crate::lp_script::compiler::expr::expr_test_util::ExprTest;
    use crate::lp_script::vm::opcodes::LpsOpCode;

    #[test]
    fn test_length() -> Result<(), String> {
        ExprTest::new("length(vec4(2.0, 3.0, 6.0, 0.0))")
            .expect_opcodes(vec![
                LpsOpCode::Push(2.0.to_dec32()),
                LpsOpCode::Push(3.0.to_dec32()),
                LpsOpCode::Push(6.0.to_dec32()),
                LpsOpCode::Push(0.0.to_dec32()),
                LpsOpCode::Length4,
                LpsOpCode::Return,
            ])
            .expect_result_dec32(7.0) // sqrt(4 + 9 + 36 + 0) = 7
            .run()
    }

    #[test]
    fn test_normalize() -> Result<(), String> {
        ExprTest::new("normalize(vec4(2.0, 0.0, 0.0, 0.0)).x")
            .expect_result_dec32(1.0)
            .run()?;

        ExprTest::new("normalize(vec4(2.0, 0.0, 0.0, 0.0)).y")
            .expect_result_dec32(0.0)
            .run()
    }

    #[test]
    fn test_dot() -> Result<(), String> {
        ExprTest::new("dot(vec4(1.0, 2.0, 3.0, 4.0), vec4(5.0, 6.0, 7.0, 8.0))")
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_dec32()),
                LpsOpCode::Push(2.0.to_dec32()),
                LpsOpCode::Push(3.0.to_dec32()),
                LpsOpCode::Push(4.0.to_dec32()),
                LpsOpCode::Push(5.0.to_dec32()),
                LpsOpCode::Push(6.0.to_dec32()),
                LpsOpCode::Push(7.0.to_dec32()),
                LpsOpCode::Push(8.0.to_dec32()),
                LpsOpCode::Dot4,
                LpsOpCode::Return,
            ])
            .expect_result_dec32(70.0) // 1*5 + 2*6 + 3*7 + 4*8 = 70
            .run()
    }

    #[test]
    fn test_distance() -> Result<(), String> {
        ExprTest::new("distance(vec4(0.0, 0.0, 0.0, 0.0), vec4(2.0, 3.0, 6.0, 0.0))")
            .expect_opcodes(vec![
                LpsOpCode::Push(0.0.to_dec32()),
                LpsOpCode::Push(0.0.to_dec32()),
                LpsOpCode::Push(0.0.to_dec32()),
                LpsOpCode::Push(0.0.to_dec32()),
                LpsOpCode::Push(2.0.to_dec32()),
                LpsOpCode::Push(3.0.to_dec32()),
                LpsOpCode::Push(6.0.to_dec32()),
                LpsOpCode::Push(0.0.to_dec32()),
                LpsOpCode::Distance4,
                LpsOpCode::Return,
            ])
            .expect_result_dec32(7.0)
            .run()
    }
}
