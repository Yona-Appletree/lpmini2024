/// Assignment statement tests
#[cfg(test)]
mod tests {
    use crate::lpscript::compiler::stmt::stmt_test_ast::*;
    use crate::lpscript::compiler::stmt::stmt_test_util::ScriptTest;
    use crate::lpscript::error::Type;
    use crate::lpscript::vm::opcodes::LpsOpCode;
    use crate::math::ToFixed;

    #[test]
    fn test_simple_assignment() -> Result<(), String> {
        ScriptTest::new("float x = 1.0; x = 5.0; return x;")
            .expect_ast(program(vec![
                var_decl(Type::Fixed, "x", Some(num(1.0))),
                assign_stmt("x", num(5.0)),
                return_stmt(typed_var("x", Type::Fixed)),
            ]))
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::StoreLocalFixed(0),
                LpsOpCode::Push(5.0.to_fixed()),
                LpsOpCode::StoreLocalFixed(0),
                LpsOpCode::LoadLocalFixed(0),
                LpsOpCode::Return,
            ])
            .expect_result_fixed(5.0)
            .run()
    }

    #[test]
    fn test_assignment_with_expression() -> Result<(), String> {
        ScriptTest::new("float x = 1.0; x = x + 2.0; return x;")
            .expect_ast(program(vec![
                var_decl(Type::Fixed, "x", Some(num(1.0))),
                assign_stmt("x", add(typed_var("x", Type::Fixed), num(2.0), Type::Fixed)),
                return_stmt(typed_var("x", Type::Fixed)),
            ]))
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::StoreLocalFixed(0),
                LpsOpCode::LoadLocalFixed(0),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::AddFixed,
                LpsOpCode::StoreLocalFixed(0),
                LpsOpCode::LoadLocalFixed(0),
                LpsOpCode::Return,
            ])
            .expect_result_fixed(3.0)
            .run()
    }

    #[test]
    fn test_multiple_assignments() -> Result<(), String> {
        ScriptTest::new("float x = 1.0; x = 2.0; x = 3.0; return x;")
            .expect_ast(program(vec![
                var_decl(Type::Fixed, "x", Some(num(1.0))),
                assign_stmt("x", num(2.0)),
                assign_stmt("x", num(3.0)),
                return_stmt(typed_var("x", Type::Fixed)),
            ]))
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::StoreLocalFixed(0),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::StoreLocalFixed(0),
                LpsOpCode::Push(3.0.to_fixed()),
                LpsOpCode::StoreLocalFixed(0),
                LpsOpCode::LoadLocalFixed(0),
                LpsOpCode::Return,
            ])
            .expect_result_fixed(3.0)
            .run()
    }

    #[test]
    fn test_assignment_with_builtin() -> Result<(), String> {
        ScriptTest::new("float x = time; x = x * 2.0; return x;")
            .expect_ast(program(vec![
                var_decl(Type::Fixed, "x", Some(typed_var("time", Type::Fixed))),
                assign_stmt("x", mul(typed_var("x", Type::Fixed), num(2.0), Type::Fixed)),
                return_stmt(typed_var("x", Type::Fixed)),
            ]))
            .expect_opcodes(vec![
                LpsOpCode::Load(crate::test_engine::LoadSource::Time),
                LpsOpCode::StoreLocalFixed(0),
                LpsOpCode::LoadLocalFixed(0),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::MulFixed,
                LpsOpCode::StoreLocalFixed(0),
                LpsOpCode::LoadLocalFixed(0),
                LpsOpCode::Return,
            ])
            .with_time(5.0)
            .expect_result_fixed(10.0)
            .run()
    }
}
