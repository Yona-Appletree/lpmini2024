/// Vector constructor tests
#[cfg(test)]
mod tests {
    use crate::compiler::expr::expr_test_util::ExprTest;
    
    
    use crate::vm::opcodes::LpsOpCode;
    use crate::fixed::{ToFixed, Vec2, Vec3, Vec4};

    #[test]
    fn test_vec2_constructor() -> Result<(), String> {
        ExprTest::new("vec2(1.0, 2.0)")
            .expect_ast(|b| {
                let arg1 = b.num(1.0);
                let arg2 = b.num(2.0);
                b.vec2(vec![arg1, arg2])
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::Return,
            ])
            .expect_result_vec2(Vec2 {
                x: 1.0.to_fixed(),
                y: 2.0.to_fixed(),
            })
            .run()
    }

    #[test]
    fn test_vec3_constructor() -> Result<(), String> {
        ExprTest::new("vec3(1.0, 2.0, 3.0)")
            .expect_ast(|b| {
                let arg1 = b.num(1.0);
                let arg2 = b.num(2.0);
                let arg3 = b.num(3.0);
                b.vec3(vec![arg1, arg2, arg3])
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::Push(3.0.to_fixed()),
                LpsOpCode::Return,
            ])
            .expect_result_vec3(Vec3 {
                x: 1.0.to_fixed(),
                y: 2.0.to_fixed(),
                z: 3.0.to_fixed(),
            })
            .run()
    }

    #[test]
    fn test_vec4_constructor() -> Result<(), String> {
        ExprTest::new("vec4(1.0, 2.0, 3.0, 4.0)")
            .expect_ast(|b| {
                let arg1 = b.num(1.0);
                let arg2 = b.num(2.0);
                let arg3 = b.num(3.0);
                let arg4 = b.num(4.0);
                b.vec4(vec![arg1, arg2, arg3, arg4])
            })
            .expect_opcodes(vec![
                LpsOpCode::Push(1.0.to_fixed()),
                LpsOpCode::Push(2.0.to_fixed()),
                LpsOpCode::Push(3.0.to_fixed()),
                LpsOpCode::Push(4.0.to_fixed()),
                LpsOpCode::Return,
            ])
            .expect_result_vec4(Vec4 {
                x: 1.0.to_fixed(),
                y: 2.0.to_fixed(),
                z: 3.0.to_fixed(),
                w: 4.0.to_fixed(),
            })
            .run()
    }

    #[test]
    fn test_vec3_from_vec2_and_scalar() -> Result<(), String> {
        ExprTest::new("vec3(vec2(1.0, 2.0), 3.0)")
            .expect_result_vec3(Vec3 {
                x: 1.0.to_fixed(),
                y: 2.0.to_fixed(),
                z: 3.0.to_fixed(),
            })
            .run()
    }

    #[test]
    fn test_vec4_from_vec3_and_scalar() -> Result<(), String> {
        ExprTest::new("vec4(vec3(1.0, 2.0, 3.0), 4.0)")
            .expect_result_vec4(Vec4 {
                x: 1.0.to_fixed(),
                y: 2.0.to_fixed(),
                z: 3.0.to_fixed(),
                w: 4.0.to_fixed(),
            })
            .run()
    }

    #[test]
    fn test_vec4_from_two_vec2() -> Result<(), String> {
        ExprTest::new("vec4(vec2(1.0, 2.0), vec2(3.0, 4.0))")
            .expect_result_vec4(Vec4 {
                x: 1.0.to_fixed(),
                y: 2.0.to_fixed(),
                z: 3.0.to_fixed(),
                w: 4.0.to_fixed(),
            })
            .run()
    }

    #[test]
    fn test_vec2_from_single_scalar_each() -> Result<(), String> {
        ExprTest::new("vec2(1.0, 2.0)")
            .expect_result_vec2(Vec2 {
                x: 1.0.to_fixed(),
                y: 2.0.to_fixed(),
            })
            .run()
    }

    // Type checking tests (using ExprTest validates types automatically)
    // These tests already exist above and validate type checking through execution
}
