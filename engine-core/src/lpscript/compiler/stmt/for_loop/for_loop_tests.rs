/// For loop tests
#[cfg(test)]
mod tests {
    use crate::lpscript::compiler::stmt::stmt_test_util::ScriptTest;

    #[test]
    #[ignore] // TODO: Fix for loop bytecode generation
    fn test_for_loop_basic() -> Result<(), String> {
        ScriptTest::new("float sum = 0.0; for (float i = 0.0; i < 3.0; i = i + 1.0) { sum = sum + i; } return sum;")
            .expect_result_fixed(3.0) // 0 + 1 + 2
            .run()
    }

    #[test]
    #[ignore] // TODO: Fix for loop bytecode generation
    fn test_for_loop_no_init() -> Result<(), String> {
        ScriptTest::new("float i = 0.0; for (; i < 3.0; i = i + 1.0) { } return i;")
            .expect_result_fixed(3.0)
            .run()
    }

    #[test]
    #[ignore] // TODO: Fix for loop bytecode generation
    fn test_for_loop_nested() -> Result<(), String> {
        ScriptTest::new("
            float sum = 0.0;
            for (float i = 0.0; i < 2.0; i = i + 1.0) {
                for (float j = 0.0; j < 2.0; j = j + 1.0) {
                    sum = sum + 1.0;
                }
            }
            return sum;
        ")
            .expect_result_fixed(4.0) // 2 * 2 iterations
            .run()
    }

    #[test]
    #[ignore] // TODO: Fix for loop bytecode generation
    fn test_for_loop_with_builtin() -> Result<(), String> {
        ScriptTest::new("float result = 0.0; for (float i = 0.0; i < time; i = i + 1.0) { result = result + 1.0; } return result;")
            .with_time(5.0)
            .expect_result_fixed(5.0)
            .run()
    }
}
