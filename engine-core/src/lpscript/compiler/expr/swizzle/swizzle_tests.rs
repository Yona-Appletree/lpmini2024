/// Swizzle operation tests
#[cfg(test)]
mod tests {
    use crate::lpscript::compiler::expr::expr_test_util::ExprTest;
    use crate::lpscript::compiler::test_ast::*;
    use crate::lpscript::shared::Type;
    use crate::lpscript::vm::opcodes::LpsOpCode;
    use crate::math::{ToFixed, Vec2};

    #[test]
    fn test_swizzle_single_component_x() -> Result<(), String> {
        ExprTest::new("vec2(1.0, 2.0).x")
            .expect_ast(|b| {
                let arg1 = b.num(1.0);
                let arg2 = b.num(2.0);
                let vec = b.vec2(vec![arg1, arg2]);
                b.swizzle(vec, "x", Type::Fixed)
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::Drop1, // Drop y, keep x
                LpsOpCode::Return,
            ])
            .expect_result_fixed(1.0)
            .run()
    }

    #[test]
    fn test_swizzle_single_component_y() -> Result<(), String> {
        ExprTest::new("vec2(1.0, 2.0).y")
            .expect_ast(|b| {
                let arg1 = b.num(1.0);
                let arg2 = b.num(2.0);
                let vec = b.vec2(vec![arg1, arg2]);
                b.swizzle(vec, "y", Type::Fixed)
            })
            .expect_result_fixed(2.0)
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
                b.swizzle(vec, "xy", Type::Vec2)
            })
            .expect_result_vec2(Vec2 {
                x: 1.0.to_fixed(),
                y: 2.0.to_fixed(),
            })
            .run()?;

        ExprTest::new("vec3(1.0, 2.0, 3.0).yz")
            .expect_result_vec2(Vec2 {
                x: 2.0.to_fixed(),
                y: 3.0.to_fixed(),
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
                b.swizzle(vec, "yx", Type::Vec2)
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::Swap, // Swap x and y
                LpsOpCode::Return,
            ])
            .expect_result_vec2(Vec2 {
                x: 2.0.to_fixed(),
                y: 1.0.to_fixed(),
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
                b.swizzle(vec, "xx", Type::Vec2)
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::Drop1, // Drop y
                LpsOpCode::Dup1,  // Duplicate x
                LpsOpCode::Return,
            ])
            .expect_result_vec2(Vec2 {
                x: 1.0.to_fixed(),
                y: 1.0.to_fixed(),
            })
            .run()?;

        ExprTest::new("vec2(1.0, 2.0).yy")
            .expect_result_vec2(Vec2 {
                x: 2.0.to_fixed(),
                y: 2.0.to_fixed(),
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
                b.swizzle(vec, "gr", Type::Vec2)
            })
            .expect_result_vec2(Vec2 {
                x: 2.0.to_fixed(),
                y: 1.0.to_fixed(),
            })
            .run()
    }

    #[test]
    fn test_swizzle_builtin_variable() -> Result<(), String> {
        ExprTest::new("uv.x")
            .with_x(0.7)
            .expect_result_fixed(0.7)
            .run()?;

        ExprTest::new("uv.yx")
            .with_x(0.3)
            .with_y(0.7)
            .expect_result_vec2(Vec2 {
                x: 0.7.to_fixed(),
                y: 0.3.to_fixed(),
            })
            .run()
    }

    // Type checking tests (using ExprTest validates types automatically)
    // These tests already exist above and validate type checking through execution
}
