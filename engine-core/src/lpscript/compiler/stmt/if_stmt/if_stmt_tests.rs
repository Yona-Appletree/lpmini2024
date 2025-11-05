/// If statement tests
#[cfg(test)]
mod tests {
    use crate::lpscript::compiler::stmt::stmt_test_util::ScriptTest;

    #[test]
    #[ignore] // TODO: Fix if statement bytecode generation
    fn test_if_without_else() -> Result<(), String> {
        ScriptTest::new("if (1.0 > 0.5) { return 10.0; } return 0.0;")
            .expect_result_fixed(10.0)
            .run()
    }

    #[test]
    #[ignore] // TODO: Fix if statement bytecode generation
    fn test_if_with_else() -> Result<(), String> {
        ScriptTest::new("if (1.0 > 0.5) { return 10.0; } else { return 20.0; }")
            .expect_result_fixed(10.0)
            .run()
    }

    #[test]
    #[ignore] // TODO: Fix if statement bytecode generation
    fn test_if_with_variable() -> Result<(), String> {
        ScriptTest::new("float x = 0.3; if (x > 0.5) { return 1.0; } else { return 0.0; }")
            .expect_result_fixed(0.0)
            .run()
    }

    #[test]
    #[ignore] // TODO: Fix if statement bytecode generation
    fn test_if_with_builtin() -> Result<(), String> {
        ScriptTest::new("if (time > 5.0) { return 100.0; } else { return -100.0; }")
            .with_time(10.0)
            .expect_result_fixed(100.0)
            .run()
    }
}
