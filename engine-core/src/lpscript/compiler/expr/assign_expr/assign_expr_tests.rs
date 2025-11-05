/// Assignment expression tests using full compilation pipeline

#[cfg(test)]
mod tests {
    use crate::lpscript::compiler::expr::expr_test_util::ExprTest;
    use crate::math::{ToFixed, Vec2, Vec3, Vec4};

    #[test]
    fn test_simple_assignment() {
        // Assignment expression should return the assigned value
        ExprTest::new("x = 5.0")
            .local_fixed(0, "x", 0.0.to_fixed())
            .expect_result_fixed(5.0)
            .expect_local_fixed("x", 5.0)
            .run()
            .expect("x = 5.0 should assign and return 5.0");
    }

    #[test]
    fn test_chained_assignment() {
        // Chained assignment should be right-associative
        ExprTest::new("z = y = x = 7.0")
            .local_fixed(0, "x", 0.0.to_fixed())
            .local_fixed(1, "y", 0.0.to_fixed())
            .local_fixed(2, "z", 0.0.to_fixed())
            .expect_result_fixed(7.0)
            .expect_local_fixed("x", 7.0)
            .expect_local_fixed("y", 7.0)
            .expect_local_fixed("z", 7.0)
            .run()
            .expect("Chained assignments should all equal 7.0");
    }

    #[test]
    fn test_assignment_with_expression() {
        // Assignment should evaluate the RHS expression
        ExprTest::new("x = y + 2.0")
            .local_fixed(0, "x", 0.0.to_fixed())
            .local_fixed(1, "y", 3.0.to_fixed())
            .expect_result_fixed(5.0)
            .expect_local_fixed("x", 5.0)
            .run()
            .expect("x = y + 2.0 should assign 5.0 to x");
    }

    #[test]
    fn test_assignment_in_expression() {
        // Assignment expression can be used within larger expressions
        ExprTest::new("(x = 3.0) + (y = 4.0)")
            .local_fixed(0, "x", 0.0.to_fixed())
            .local_fixed(1, "y", 0.0.to_fixed())
            .expect_result_fixed(7.0)
            .expect_local_fixed("x", 3.0)
            .expect_local_fixed("y", 4.0)
            .run()
            .expect("(x = 3.0) + (y = 4.0) should equal 7.0");
    }

    #[test]
    fn test_vec2_assignment() {
        // Assignment of vec2 values
        ExprTest::new("v = vec2(1.0, 2.0)")
            .local_vec2(0, "v", Vec2::new(0.0.to_fixed(), 0.0.to_fixed()))
            .expect_result_vec2(Vec2::new(1.0.to_fixed(), 2.0.to_fixed()))
            .run()
            .expect("vec2 assignment should work");
    }

    #[test]
    fn test_vec3_assignment() {
        // Assignment of vec3 values
        ExprTest::new("v = vec3(1.0, 2.0, 3.0)")
            .local_vec3(
                0,
                "v",
                Vec3::new(0.0.to_fixed(), 0.0.to_fixed(), 0.0.to_fixed()),
            )
            .expect_result_vec3(Vec3::new(1.0.to_fixed(), 2.0.to_fixed(), 3.0.to_fixed()))
            .run()
            .expect("vec3 assignment should work");
    }

    #[test]
    fn test_vec4_assignment() {
        // Assignment of vec4 values
        ExprTest::new("v = vec4(1.0, 2.0, 3.0, 4.0)")
            .local_vec4(
                0,
                "v",
                Vec4::new(
                    0.0.to_fixed(),
                    0.0.to_fixed(),
                    0.0.to_fixed(),
                    0.0.to_fixed(),
                ),
            )
            .expect_result_vec4(Vec4::new(
                1.0.to_fixed(),
                2.0.to_fixed(),
                3.0.to_fixed(),
                4.0.to_fixed(),
            ))
            .run()
            .expect("vec4 assignment should work");
    }

    #[test]
    fn test_vec2_chained_assignment() {
        // Chained vec2 assignment
        ExprTest::new("b = a = vec2(5.0, 6.0)")
            .local_vec2(0, "a", Vec2::new(0.0.to_fixed(), 0.0.to_fixed()))
            .local_vec2(1, "b", Vec2::new(0.0.to_fixed(), 0.0.to_fixed()))
            .expect_result_vec2(Vec2::new(5.0.to_fixed(), 6.0.to_fixed()))
            .run()
            .expect("Chained vec2 assignment should work");
    }
}
