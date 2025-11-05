/// Block statement tests
#[cfg(test)]
mod tests {
    use crate::lpscript::compiler::stmt::stmt_test_ast::*;
    use crate::lpscript::compiler::stmt::stmt_test_util::ScriptTest;
    use crate::lpscript::error::Type;
    use crate::lpscript::vm::opcodes::LpsOpCode;
    use crate::math::ToFixed;

    #[test]
    fn test_simple_block() -> Result<(), String> {
        ScriptTest::new("{ float x = 5.0; return x; }")
            .expect_ast(program(vec![block(vec![
                var_decl(Type::Fixed, "x", Some(num(5.0))),
                return_stmt(typed_var("x", Type::Fixed)),
            ])]))
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
    fn test_nested_blocks() -> Result<(), String> {
        ScriptTest::new("{ float x = 1.0; { float y = 2.0; x = x + y; } return x; }")
            .expect_ast(program(vec![block(vec![
                var_decl(Type::Fixed, "x", Some(num(1.0))),
                block(vec![
                    var_decl(Type::Fixed, "y", Some(num(2.0))),
                    assign_stmt("x", add(typed_var("x", Type::Fixed), typed_var("y", Type::Fixed), Type::Fixed)),
                ]),
                return_stmt(typed_var("x", Type::Fixed)),
            ])]))
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::StoreLocalFixed(0),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::StoreLocalFixed(1),
                LpsOpCode::LoadLocalFixed(0),
                LpsOpCode::LoadLocalFixed(1),
                LpsOpCode::AddFixed,
                LpsOpCode::StoreLocalFixed(0),
                LpsOpCode::LoadLocalFixed(0),
                LpsOpCode::Return,
            ])
            .expect_result_fixed(3.0)
            .run()
    }

    #[test]
    fn test_block_with_multiple_statements() -> Result<(), String> {
        ScriptTest::new("{ float a = 1.0; float b = 2.0; float c = a + b; return c; }")
            .expect_ast(program(vec![block(vec![
                var_decl(Type::Fixed, "a", Some(num(1.0))),
                var_decl(Type::Fixed, "b", Some(num(2.0))),
                var_decl(Type::Fixed, "c", Some(add(typed_var("a", Type::Fixed), typed_var("b", Type::Fixed), Type::Fixed))),
                return_stmt(typed_var("c", Type::Fixed)),
            ])]))
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::StoreLocalFixed(0),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::StoreLocalFixed(1),
                LpsOpCode::LoadLocalFixed(0),
                LpsOpCode::LoadLocalFixed(1),
                LpsOpCode::AddFixed,
                LpsOpCode::StoreLocalFixed(2),
                LpsOpCode::LoadLocalFixed(2),
                LpsOpCode::Return,
            ])
            .expect_result_fixed(3.0)
            .run()
    }

    #[test]
    fn test_empty_block() -> Result<(), String> {
        ScriptTest::new("{ } return 42.0;")
            .expect_ast(program(vec![block(vec![]), return_stmt(num(42.0))]))
            .expect_opcodes(vec![
                LpsOpCode::Push(42.0.to_fixed()),
                LpsOpCode::Return,
            ])
            .expect_result_fixed(42.0)
            .run()
    }
}
