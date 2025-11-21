/// Variable expression tests
#[cfg(test)]
mod tests {
    use crate::lp_script::compiler::expr::expr_test_util::ExprTest;
    use crate::lp_script::vm::opcodes::load::LoadSource;
    use crate::lp_script::vm::opcodes::LpsOpCode;

    #[test]
    fn test_builtin_variable_x() -> Result<(), String> {
        ExprTest::new("x")
            .expect_opcodes(vec![LpsOpCode::Load(LoadSource::XNorm), LpsOpCode::Return])
            .with_x(0.7)
            .expect_result_dec32(0.7)
            .run()
    }

    #[test]
    fn test_builtin_variable_y() -> Result<(), String> {
        ExprTest::new("y")
            .with_y(0.3)
            .expect_result_dec32(0.3)
            .run()
    }

    #[test]
    fn test_builtin_variable_time() -> Result<(), String> {
        ExprTest::new("time")
            .expect_opcodes(vec![LpsOpCode::Load(LoadSource::Time), LpsOpCode::Return])
            .with_time(5.0)
            .expect_result_dec32(5.0)
            .run()
    }

    #[test]
    fn test_legacy_variable_xnorm() -> Result<(), String> {
        ExprTest::new("xNorm")
            .with_x(0.3)
            .expect_result_dec32(0.3)
            .run()
    }

    #[test]
    fn test_legacy_variable_ynorm() -> Result<(), String> {
        ExprTest::new("yNorm")
            .with_y(0.8)
            .expect_result_dec32(0.8)
            .run()
    }

    #[test]
    fn test_builtin_variable_arithmetic() -> Result<(), String> {
        ExprTest::new("x + y")
            .with_x(0.3)
            .with_y(0.7)
            .expect_result_dec32(1.0)
            .run()?;

        ExprTest::new("x * y")
            .with_x(2.0)
            .with_y(3.0)
            .expect_result_dec32(6.0)
            .run()
    }

    #[test]
    fn test_uv_swizzle() -> Result<(), String> {
        // uv is a built-in vec2, so uv.x should work
        ExprTest::new("uv.x")
            .with_x(0.7)
            .expect_result_dec32(0.7)
            .run()?;

        ExprTest::new("uv.y")
            .with_y(0.4)
            .expect_result_dec32(0.4)
            .run()
    }

    // Type checking tests (using ExprTest validates types automatically)
    #[test]
    fn test_variable_typecheck() -> Result<(), String> {
        ExprTest::new("xNorm")
            .with_x(0.5)
            .expect_result_dec32(0.5)
            .run()
    }

    #[test]
    fn test_uv_variable_typecheck() -> Result<(), String> {
        ExprTest::new("uv.x + uv.y")
            .with_x(0.3)
            .with_y(0.7)
            .expect_result_dec32(1.0)
            .run()
    }

    #[test]
    fn test_coord_variable_typecheck() -> Result<(), String> {
        // Note: coord.x loads pixel coordinates (XInt) which aren't available in ExprTest
        // Use uv.x for normalized coordinates instead
        ExprTest::new("uv.x")
            .with_x(0.5)
            .expect_result_dec32(0.5)
            .run()
    }
}
