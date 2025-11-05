/// Variable declaration tests
#[cfg(test)]
mod tests {
    use crate::lpscript::compiler::stmt::stmt_test_ast::*;
    use crate::lpscript::compiler::stmt::stmt_test_util::ScriptTest;
    use crate::lpscript::error::Type;
    use crate::lpscript::vm::opcodes::LpsOpCode;
    use crate::math::ToFixed;

    #[test]
    fn test_var_decl_with_init() -> Result<(), String> {
        ScriptTest::new("float x = 5.0; return x;")
            .expect_ast(program(vec![
                var_decl(Type::Fixed, "x", Some(num(5.0))),
                return_stmt(typed_var("x", Type::Fixed)),
            ]))
            .expect_opcodes(vec![
                LpsOpCode::Push(5.0.to_fixed()),
                LpsOpCode::StoreLocalFixed(0),
                LpsOpCode::LoadLocalFixed(0),
                LpsOpCode::Return,
            ])
            .expect_result_fixed(5.0)
            .run()
    }

    #[test]
    fn test_var_decl_without_init() -> Result<(), String> {
        ScriptTest::new("float x; x = 10.0; return x;")
            .expect_ast(program(vec![
                var_decl(Type::Fixed, "x", None),
                assign_stmt("x", num(10.0)),
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
    fn test_multiple_var_decls() -> Result<(), String> {
        ScriptTest::new("float x = 1.0; float y = 2.0; return x + y;")
            .expect_ast(program(vec![
                var_decl(Type::Fixed, "x", Some(num(1.0))),
                var_decl(Type::Fixed, "y", Some(num(2.0))),
                return_stmt(add(
                    typed_var("x", Type::Fixed),
                    typed_var("y", Type::Fixed),
                    Type::Fixed,
                )),
            ]))
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::StoreLocalFixed(0),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::StoreLocalFixed(1),
                LpsOpCode::LoadLocalFixed(0),
                LpsOpCode::LoadLocalFixed(1),
                LpsOpCode::AddFixed,
                LpsOpCode::Return,
            ])
            .expect_result_fixed(3.0)
            .run()
    }

    #[test]
    fn test_var_decl_with_expression() -> Result<(), String> {
        ScriptTest::new("float x = 1.0 + 2.0; return x;")
            .expect_ast(program(vec![
                var_decl(Type::Fixed, "x", Some(add(num(1.0), num(2.0), Type::Fixed))),
                return_stmt(typed_var("x", Type::Fixed)),
            ]))
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_fixed()),
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
    #[ignore] // Vec2 local variables need more VM work
    fn test_var_decl_vec2() -> Result<(), String> {
        ScriptTest::new("vec2 v = vec2(1.0, 2.0); return v.x;")
            .expect_result_fixed(1.0)
            .run()
    }

    #[test]
    fn test_var_decl_from_builtin() -> Result<(), String> {
        ScriptTest::new("float t = time; return t * 2.0;")
            .expect_ast(program(vec![
                var_decl(Type::Fixed, "t", Some(typed_var("time", Type::Fixed))),
                return_stmt(mul(typed_var("t", Type::Fixed), num(2.0), Type::Fixed)),
            ]))
            .expect_opcodes(vec![
                LpsOpCode::Load(crate::test_engine::LoadSource::Time),
                LpsOpCode::StoreLocalFixed(0),
                LpsOpCode::LoadLocalFixed(0),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::MulFixed,
                LpsOpCode::Return,
            ])
            .with_time(3.0)
            .expect_result_fixed(6.0)
            .run()
    }
}
