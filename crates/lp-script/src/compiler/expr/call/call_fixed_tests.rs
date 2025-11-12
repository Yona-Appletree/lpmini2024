/// Function call tests for Fixed (scalar) type
#[cfg(test)]
mod tests {
    use crate::compiler::expr::expr_test_util::ExprTest;
    use crate::fixed::ToFixed;
    use crate::shared::Type;
    use crate::vm::opcodes::LpsOpCode;

    #[test]
    fn test_function_call_sin() -> Result<(), String> {
        ExprTest::new("sin(0.0)")
            .expect_ast(|b| {
                let arg = b.num(0.0);
                b.call("sin", vec![arg], Type::Fixed)
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(0.0.to_fixed()),
                LpsOpCode::SinFixed,
                LpsOpCode::Return,
            ])
            .expect_result_fixed(0.0)
            .run()
    }

    #[test]
    fn test_function_call_cos() -> Result<(), String> {
        ExprTest::new("cos(0.0)")
            .expect_ast(|b| {
                let arg = b.num(0.0);
                b.call("cos", vec![arg], Type::Fixed)
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(0.0.to_fixed()),
                LpsOpCode::CosFixed,
                LpsOpCode::Return,
            ])
            .expect_result_fixed(1.0)
            .run()
    }

    #[test]
    fn test_function_call_min() -> Result<(), String> {
        ExprTest::new("min(1.0, 2.0)")
            .expect_ast(|b| {
                let arg1 = b.num(1.0);
                let arg2 = b.num(2.0);
                b.call("min", vec![arg1, arg2], Type::Fixed)
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::MinFixed,
                LpsOpCode::Return,
            ])
            .expect_result_fixed(1.0)
            .run()?;

        ExprTest::new("min(5.0, 3.0)")
            .expect_result_fixed(3.0)
            .run()
    }

    #[test]
    fn test_function_call_max() -> Result<(), String> {
        ExprTest::new("max(1.0, 2.0)")
            .expect_ast(|b| {
                let arg1 = b.num(1.0);
                let arg2 = b.num(2.0);
                b.call("max", vec![arg1, arg2], Type::Fixed)
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::MaxFixed,
                LpsOpCode::Return,
            ])
            .expect_result_fixed(2.0)
            .run()?;

        ExprTest::new("max(5.0, 3.0)")
            .expect_result_fixed(5.0)
            .run()
    }

    #[test]
    fn test_function_call_abs() -> Result<(), String> {
        ExprTest::new("abs(-5.0)")
            .expect_ast(|b| {
                let arg = b.num(-5.0);
                b.call("abs", vec![arg], Type::Fixed)
            })
            .expect_opcodes(vec![
                LpsOpCode::Push((-5.0).to_fixed()),
                LpsOpCode::AbsFixed,
                LpsOpCode::Return,
            ])
            .expect_result_fixed(5.0)
            .run()?;

        ExprTest::new("abs(3.0)").expect_result_fixed(3.0).run()
    }

    #[test]
    fn test_function_call_floor() -> Result<(), String> {
        ExprTest::new("floor(2.7)")
            .expect_ast(|b| {
                let arg = b.num(2.7);
                b.call("floor", vec![arg], Type::Fixed)
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(2.7.to_fixed()),
                LpsOpCode::FloorFixed,
                LpsOpCode::Return,
            ])
            .expect_result_fixed(2.0)
            .run()
    }

    #[test]
    fn test_function_call_ceil() -> Result<(), String> {
        ExprTest::new("ceil(2.3)")
            .expect_ast(|b| {
                let arg = b.num(2.3);
                b.call("ceil", vec![arg], Type::Fixed)
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(2.3.to_fixed()),
                LpsOpCode::CeilFixed,
                LpsOpCode::Return,
            ])
            .expect_result_fixed(3.0)
            .run()
    }

    #[test]
    fn test_function_call_nested() -> Result<(), String> {
        // Test function calls with expressions as arguments
        ExprTest::new("sin(1.0 + 2.0)")
            .expect_result_fixed((3.0f32).sin())
            .run()?;

        // Nested function calls
        ExprTest::new("abs(sin(0.0))")
            .expect_result_fixed(0.0)
            .run()
    }

    #[test]
    fn test_function_call_typecheck() -> Result<(), String> {
        ExprTest::new("sin(time)")
            .with_time(0.0)
            .expect_result_fixed(0.0)
            .run()
    }
}
