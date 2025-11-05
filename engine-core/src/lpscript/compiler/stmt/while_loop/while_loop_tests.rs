/// While loop tests
#[cfg(test)]
mod tests {
    use crate::lpscript::compiler::stmt::stmt_test_util::ScriptTest;

    #[test]
    #[ignore] // TODO: Fix while loop bytecode generation
    fn test_while_loop_counter() -> Result<(), String> {
        ScriptTest::new("float i = 0.0; while (i < 5.0) { i = i + 1.0; } return i;")
            .expect_result_fixed(5.0)
            .run()
    }

    #[test]
    #[ignore] // TODO: Fix while loop bytecode generation
    fn test_while_loop_sum() -> Result<(), String> {
        ScriptTest::new("float sum = 0.0; float i = 1.0; while (i <= 3.0) { sum = sum + i; i = i + 1.0; } return sum;")
            .expect_result_fixed(6.0) // 1 + 2 + 3
            .run()
    }

    #[test]
    #[ignore] // TODO: Fix while loop bytecode generation
    fn test_while_loop_with_break_condition() -> Result<(), String> {
        ScriptTest::new("float x = 0.0; while (x < 10.0) { x = x + 2.0; } return x;")
            .expect_result_fixed(10.0)
            .run()
    }
}
