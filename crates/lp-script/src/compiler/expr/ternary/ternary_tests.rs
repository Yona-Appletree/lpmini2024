/// Ternary expression tests
#[cfg(test)]
mod tests {
    use crate::compiler::expr::expr_test_util::ExprTest;
    use crate::dec32::ToDec32;
    use crate::shared::Type;
    use crate::vm::opcodes::LpsOpCode;

    #[test]
    fn test_ternary_basic() -> Result<(), String> {
        ExprTest::new("1.0 > 0.5 ? 1.0 : 0.0")
            .expect_ast(|b| {
                let left = b.num(1.0);
                let right = b.num(0.5);
                let condition = b.greater(left, right);
                let true_expr = b.num(1.0);
                let false_expr = b.num(0.0);
                b.ternary(condition, true_expr, false_expr, Type::Dec32)
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_dec32()),
                LpsOpCode::Push(0.5.to_dec32()),
                LpsOpCode::GreaterDec32,
                LpsOpCode::JumpIfZero(2), // If false, jump over true branch
                LpsOpCode::Push(1.0.to_dec32()), // True value
                LpsOpCode::Jump(1),       // Jump over false branch
                LpsOpCode::Push(0.0.to_dec32()), // False value
                LpsOpCode::Return,
            ])
            .expect_result_dec32(1.0) // true ? 1.0 : 0.0 = 1.0
            .run()?;

        // Test false condition
        ExprTest::new("1.0 < 0.5 ? 1.0 : 0.0")
            .expect_result_dec32(0.0) // false ? 1.0 : 0.0 = 0.0
            .run()
    }

    #[test]
    fn test_ternary_with_variables() -> Result<(), String> {
        ExprTest::new("x > 0.5 ? 10.0 : -10.0")
            .with_x(0.7)
            .expect_result_dec32(10.0)
            .run()?;

        ExprTest::new("x > 0.5 ? 10.0 : -10.0")
            .with_x(0.3)
            .expect_result_dec32(-10.0)
            .run()
    }

    #[test]
    fn test_ternary_nested() -> Result<(), String> {
        ExprTest::new("1.0 > 0.5 ? (2.0 > 1.0 ? 1.0 : 2.0) : 3.0")
            .expect_result_dec32(1.0) // true ? (true ? 1.0 : 2.0) : 3.0 = 1.0
            .run()?;

        ExprTest::new("0.0 > 0.5 ? 10.0 : (1.0 > 0.5 ? 20.0 : 30.0)")
            .expect_result_dec32(20.0) // false ? 10.0 : (true ? 20.0 : 30.0) = 20.0
            .run()
    }

    #[test]
    fn test_ternary_with_comparison() -> Result<(), String> {
        ExprTest::new("(1.0 == 1.0) ? 10.0 : 20.0")
            .expect_result_dec32(10.0)
            .run()?;

        ExprTest::new("(1.0 != 1.0) ? 10.0 : 20.0")
            .expect_result_dec32(20.0)
            .run()
    }

    // ========================================================================
    // Vector Ternary Tests - NOT SUPPORTED
    // ========================================================================
    // Note: Ternary operator with vector results is currently not supported.
    // The Select opcode only handles single stack values, not multi-component vectors.
    // This is a known limitation documented in TODO.md.
    //
    // GLSL: `condition ? vec2(1,0) : vec2(0,1)` ✓
    // LPS:  Currently not implemented
    //
    // These tests are commented out until Select opcode is extended to support vectors.

    // #[test]
    // fn test_ternary_vec2_result_true() -> Result<(), String> {
    //     use crate::dec32::Vec2;
    //     ExprTest::new("x > 0.5 ? vec2(1.0, 0.0) : vec2(0.0, 1.0)")
    //         .with_x(0.7)
    //         .expect_result_vec2(Vec2::new(1.0.to_dec32(), 0.0.to_dec32()))
    //         .run()
    // }
}
