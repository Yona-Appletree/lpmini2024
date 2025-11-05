/// Vector constructor tests
#[cfg(test)]
mod tests {
    use crate::lpscript::compiler::expr::expr_test_util::ExprTest;
    use crate::lpscript::compiler::test_ast::*;
    use crate::lpscript::error::Type;
    use crate::lpscript::vm::opcodes::LpsOpCode;
    use crate::math::{ToFixed, Vec2, Vec3, Vec4};

    #[test]
    fn test_vec2_constructor() -> Result<(), String> {
        ExprTest::new("vec2(1.0, 2.0)")
            .expect_ast(vec2_ctor(vec![num(1.0), num(2.0)], Type::Vec2))
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
            .expect_ast(vec3_ctor(vec![num(1.0), num(2.0), num(3.0)], Type::Vec3))
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
            .expect_ast(vec4_ctor(
                vec![num(1.0), num(2.0), num(3.0), num(4.0)],
                Type::Vec4,
            ))
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
}
