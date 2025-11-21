/// Function call tests for Vec2 type
#[cfg(test)]
mod tests {
    use lp_math::dec32::ToDec32;

    use crate::lp_script::compiler::expr::expr_test_util::ExprTest;
    use crate::lp_script::vm::opcodes::LpsOpCode;

    #[test]
    fn test_length() -> Result<(), String> {
        ExprTest::new("length(vec2(3.0, 4.0))")
            .expect_opcodes(vec![
                LpsOpCode::Push(3.0.to_dec32()),
                LpsOpCode::Push(4.0.to_dec32()),
                LpsOpCode::Length2,
                LpsOpCode::Return,
            ])
            .expect_result_dec32(5.0)
            .run()
    }

    #[test]
    fn test_normalize() -> Result<(), String> {
        ExprTest::new("normalize(vec2(3.0, 4.0)).x")
            .expect_result_dec32(0.6)
            .run()?;

        ExprTest::new("normalize(vec2(3.0, 4.0)).y")
            .expect_result_dec32(0.8)
            .run()
    }

    #[test]
    fn test_dot() -> Result<(), String> {
        ExprTest::new("dot(vec2(1.0, 2.0), vec2(3.0, 4.0))")
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_dec32()),
                LpsOpCode::Push(2.0.to_dec32()),
                LpsOpCode::Push(3.0.to_dec32()),
                LpsOpCode::Push(4.0.to_dec32()),
                LpsOpCode::Dot2,
                LpsOpCode::Return,
            ])
            .expect_result_dec32(11.0) // 1*3 + 2*4 = 11
            .run()
    }

    #[test]
    fn test_distance() -> Result<(), String> {
        ExprTest::new("distance(vec2(0.0, 0.0), vec2(3.0, 4.0))")
            .expect_opcodes(vec![
                LpsOpCode::Push(0.0.to_dec32()),
                LpsOpCode::Push(0.0.to_dec32()),
                LpsOpCode::Push(3.0.to_dec32()),
                LpsOpCode::Push(4.0.to_dec32()),
                LpsOpCode::Distance2,
                LpsOpCode::Return,
            ])
            .expect_result_dec32(5.0)
            .run()
    }
}
