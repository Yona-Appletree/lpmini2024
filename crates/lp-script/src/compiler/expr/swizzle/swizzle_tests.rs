/// Swizzle operation tests
#[cfg(test)]
mod tests {
    use crate::compiler::expr::expr_test_util::ExprTest;
    use crate::dec32::{ToDec32, Vec2};
    use crate::shared::Type;
    use crate::vm::opcodes::LpsOpCode;

    #[test]
    fn test_swizzle_single_component_x() -> Result<(), String> {
        ExprTest::new("vec2(1.0, 2.0).x")
            .expect_ast(|b| {
                let arg1 = b.num(1.0);
                let arg2 = b.num(2.0);
                let vec = b.vec2(vec![arg1, arg2]);
                b.swizzle(vec, "x", Some(Type::Dec32))
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_dec32()),
                LpsOpCode::Push(2.0.to_dec32()),
                LpsOpCode::Drop1, // Drop y, keep x
                LpsOpCode::Return,
            ])
            .expect_result_dec32(1.0)
            .run()
    }

    #[test]
    fn test_swizzle_single_component_y() -> Result<(), String> {
        ExprTest::new("vec2(1.0, 2.0).y")
            .expect_ast(|b| {
                let arg1 = b.num(1.0);
                let arg2 = b.num(2.0);
                let vec = b.vec2(vec![arg1, arg2]);
                b.swizzle(vec, "y", Some(Type::Dec32))
            })
            .expect_result_dec32(2.0)
            .run()
    }

    #[test]
    fn test_swizzle_two_components() -> Result<(), String> {
        ExprTest::new("vec3(1.0, 2.0, 3.0).xy")
            .expect_ast(|b| {
                let arg1 = b.num(1.0);
                let arg2 = b.num(2.0);
                let arg3 = b.num(3.0);
                let vec = b.vec3(vec![arg1, arg2, arg3]);
                b.swizzle(vec, "xy", Some(Type::Vec2))
            })
            .expect_result_vec2(Vec2 {
                x: 1.0.to_dec32(),
                y: 2.0.to_dec32(),
            })
            .run()?;

        ExprTest::new("vec3(1.0, 2.0, 3.0).yz")
            .expect_result_vec2(Vec2 {
                x: 2.0.to_dec32(),
                y: 3.0.to_dec32(),
            })
            .run()
    }

    #[test]
    fn test_swizzle_reorder() -> Result<(), String> {
        ExprTest::new("vec2(1.0, 2.0).yx")
            .expect_ast(|b| {
                let arg1 = b.num(1.0);
                let arg2 = b.num(2.0);
                let vec = b.vec2(vec![arg1, arg2]);
                b.swizzle(vec, "yx", Some(Type::Vec2))
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_dec32()),
                LpsOpCode::Push(2.0.to_dec32()),
                LpsOpCode::Swap, // Swap x and y
                LpsOpCode::Return,
            ])
            .expect_result_vec2(Vec2 {
                x: 2.0.to_dec32(),
                y: 1.0.to_dec32(),
            })
            .run()
    }

    #[test]
    fn test_swizzle_duplicate() -> Result<(), String> {
        ExprTest::new("vec2(1.0, 2.0).xx")
            .expect_ast(|b| {
                let arg1 = b.num(1.0);
                let arg2 = b.num(2.0);
                let vec = b.vec2(vec![arg1, arg2]);
                b.swizzle(vec, "xx", Some(Type::Vec2))
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_dec32()),
                LpsOpCode::Push(2.0.to_dec32()),
                LpsOpCode::Drop1, // Drop y
                LpsOpCode::Dup1,  // Duplicate x
                LpsOpCode::Return,
            ])
            .expect_result_vec2(Vec2 {
                x: 1.0.to_dec32(),
                y: 1.0.to_dec32(),
            })
            .run()?;

        ExprTest::new("vec2(1.0, 2.0).yy")
            .expect_result_vec2(Vec2 {
                x: 2.0.to_dec32(),
                y: 2.0.to_dec32(),
            })
            .run()
    }

    #[test]
    fn test_swizzle_rgba() -> Result<(), String> {
        ExprTest::new("vec2(1.0, 2.0).gr")
            .expect_ast(|b| {
                let arg1 = b.num(1.0);
                let arg2 = b.num(2.0);
                let vec = b.vec2(vec![arg1, arg2]);
                b.swizzle(vec, "gr", Some(Type::Vec2))
            })
            .expect_result_vec2(Vec2 {
                x: 2.0.to_dec32(),
                y: 1.0.to_dec32(),
            })
            .run()
    }

    #[test]
    fn test_swizzle_builtin_variable() -> Result<(), String> {
        ExprTest::new("uv.x")
            .with_x(0.7)
            .expect_result_dec32(0.7)
            .run()?;

        ExprTest::new("uv.yx")
            .with_x(0.3)
            .with_y(0.7)
            .expect_result_vec2(Vec2 {
                x: 0.7.to_dec32(),
                y: 0.3.to_dec32(),
            })
            .run()
    }

    // Type checking tests (using ExprTest validates types automatically)
    // These tests already exist above and validate type checking through execution
}
