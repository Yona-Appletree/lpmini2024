/// Return statement tests
#[cfg(test)]
mod tests {
    use crate::lpscript::compiler::stmt::stmt_test_ast::*;
    use crate::lpscript::compiler::stmt::stmt_test_util::ScriptTest;
    use crate::lpscript::error::Type;
    use crate::lpscript::vm::opcodes::LpsOpCode;
    use crate::math::ToFixed;
    use crate::test_engine::LoadSource;

    #[test]
    fn test_return_literal() -> Result<(), String> {
        ScriptTest::new("return 42.0;")
            .expect_ast(program(vec![return_stmt(num(42.0))]))
            .expect_opcodes(vec![
                LpsOpCode::Push(42.0.to_fixed()),
                LpsOpCode::Return,
            ])
            .expect_result_fixed(42.0)
            .run()
    }

    #[test]
    fn test_return_expression() -> Result<(), String> {
        ScriptTest::new("return 1.0 + 2.0 * 3.0;")
            .expect_ast(program(vec![return_stmt(add(
                num(1.0),
                mul(num(2.0), num(3.0), Type::Fixed),
                Type::Fixed,
            ))]))
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::Push(3.0.to_fixed()),
                LpsOpCode::MulFixed,
                LpsOpCode::AddFixed,
                LpsOpCode::Return,
            ])
            .expect_result_fixed(7.0)
            .run()
    }

    #[test]
    fn test_return_variable() -> Result<(), String> {
        ScriptTest::new("float x = 10.0; return x;")
            .expect_ast(program(vec![
                var_decl(Type::Fixed, "x", Some(num(10.0))),
                return_stmt(typed_var("x", Type::Fixed)),
            ]))
            .expect_opcodes(vec![
                LpsOpCode::Push(10.0.to_fixed()),
                LpsOpCode::StoreLocalFixed(0),
                LpsOpCode::LoadLocalFixed(0),
                LpsOpCode::Return,
            ])
            .expect_result_fixed(10.0)
            .run()
    }

    #[test]
    fn test_return_builtin() -> Result<(), String> {
        ScriptTest::new("return time;")
            .expect_ast(program(vec![return_stmt(typed_var("time", Type::Fixed))]))
            .expect_opcodes(vec![LpsOpCode::Load(LoadSource::Time), LpsOpCode::Return])
            .with_time(5.0)
            .expect_result_fixed(5.0)
            .run()
    }

    #[test]
    fn test_return_function_call() -> Result<(), String> {
        ScriptTest::new("return sin(0.0);")
            .expect_ast(program(vec![return_stmt(call("sin", vec![num(0.0)], Type::Fixed))]))
            .expect_opcodes(vec![
                LpsOpCode::Push(0.0.to_fixed()),
                LpsOpCode::SinFixed,
                LpsOpCode::Return,
            ])
            .expect_result_fixed(0.0)
            .run()
    }

    #[test]
    fn test_return_after_statements() -> Result<(), String> {
        ScriptTest::new("float a = 5.0; float b = 3.0; return a - b;")
            .expect_ast(program(vec![
                var_decl(Type::Fixed, "a", Some(num(5.0))),
                var_decl(Type::Fixed, "b", Some(num(3.0))),
                return_stmt(sub(typed_var("a", Type::Fixed), typed_var("b", Type::Fixed), Type::Fixed)),
            ]))
            .expect_opcodes(vec![
                LpsOpCode::Push(5.0.to_fixed()),
                LpsOpCode::StoreLocalFixed(0),
                LpsOpCode::Push(3.0.to_fixed()),
                LpsOpCode::StoreLocalFixed(1),
                LpsOpCode::LoadLocalFixed(0),
                LpsOpCode::LoadLocalFixed(1),
                LpsOpCode::SubFixed,
                LpsOpCode::Return,
            ])
            .expect_result_fixed(2.0)
            .run()
    }
}
