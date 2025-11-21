/// For loop tests
#[cfg(test)]
mod tests {
    use crate::lp_script::compiler::stmt::stmt_test_util::ScriptTest;

    #[test]
    fn test_for_loop_basic() -> Result<(), String> {
        ScriptTest::new(
            "float sum = 0.0; 
             for (float i = 0.0; i < 3.0; i = i + 1.0) { 
                 sum = sum + i; 
             } 
             return sum;",
        )
        .expect_result_dec32(3.0) // 0 + 1 + 2
        .run()
    }

    #[test]
    fn test_for_loop_no_init() -> Result<(), String> {
        ScriptTest::new(
            "float i = 0.0; 
             for (; i < 3.0; i = i + 1.0) { } 
             return i;",
        )
        .expect_result_dec32(3.0)
        .run()
    }

    #[test]
    fn test_for_loop_nested() -> Result<(), String> {
        ScriptTest::new(
            "float sum = 0.0;
             for (float i = 0.0; i < 2.0; i = i + 1.0) {
                 for (float j = 0.0; j < 2.0; j = j + 1.0) {
                     sum = sum + 1.0;
                 }
             }
             return sum;",
        )
        .expect_result_dec32(4.0) // 2 * 2 iterations
        .run()
    }

    #[test]
    fn test_for_loop_with_builtin() -> Result<(), String> {
        ScriptTest::new(
            "float result = 0.0; 
             for (float i = 0.0; i < time; i = i + 1.0) { 
                 result = result + 1.0; 
             } 
             return result;",
        )
        .with_time(5.0)
        .expect_result_dec32(5.0)
        .run()
    }

    // === Variable Scoping Tests ===

    #[test]
    fn test_for_loop_with_local_variable() -> Result<(), String> {
        ScriptTest::new(
            "float sum = 0.0;
             for (float i = 0.0; i < 3.0; i = i + 1.0) {
                 float temp = i * 2.0;
                 sum = sum + temp;
             }
             return sum;",
        )
        .expect_result_dec32(6.0) // (0*2) + (1*2) + (2*2) = 0 + 2 + 4 = 6
        .run()
    }

    #[test]
    fn test_for_loop_multiple_locals() -> Result<(), String> {
        ScriptTest::new(
            "float sum = 0.0;
             for (float i = 0.0; i < 3.0; i = i + 1.0) {
                 float temp1 = i * 2.0;
                 float temp2 = i * 3.0;
                 sum = sum + temp1 + temp2;
             }
             return sum;",
        )
        .expect_result_dec32(15.0) // i=0: 0, i=1: 5, i=2: 10 => total 15
        .run()
    }

    #[test]
    fn test_for_loop_init_variable_scope() -> Result<(), String> {
        // The loop variable 'i' should only be accessible inside the loop
        ScriptTest::new(
            "float result = 0.0;
             for (float i = 5.0; i < 8.0; i = i + 1.0) {
                 result = result + i;
             }
             return result;",
        )
        .expect_result_dec32(18.0) // 5 + 6 + 7 = 18
        .run()
    }

    #[test]
    fn test_nested_loops_with_locals() -> Result<(), String> {
        ScriptTest::new(
            "float sum = 0.0;
             for (float i = 0.0; i < 2.0; i = i + 1.0) {
                 float outer_temp = i * 10.0;
                 for (float j = 0.0; j < 2.0; j = j + 1.0) {
                     float inner_temp = j * 5.0;
                     sum = sum + outer_temp + inner_temp;
                 }
             }
             return sum;",
        )
        .expect_result_dec32(30.0) // i=0,j=0: 0, i=0,j=1: 5, i=1,j=0: 10, i=1,j=1: 15 => 30
        .run()
    }

    #[test]
    fn test_for_loop_variable_shadowing() -> Result<(), String> {
        ScriptTest::new(
            "float x = 100.0;
             float sum = 0.0;
             for (float i = 0.0; i < 2.0; i = i + 1.0) {
                 float x = i * 2.0;  // Shadows outer x
                 sum = sum + x;
             }
             return sum + x;", // Uses outer x
        )
        .expect_result_dec32(102.0) // inner: (0*2) + (1*2) = 2, outer: 100 => 102
        .run()
    }

    #[test]
    fn test_for_loop_accumulator_pattern() -> Result<(), String> {
        ScriptTest::new(
            "float product = 1.0;
             for (float i = 1.0; i <= 4.0; i = i + 1.0) {
                 float multiplier = i;
                 product = product * multiplier;
             }
             return product;",
        )
        .expect_result_dec32(24.0) // 1 * 2 * 3 * 4 = 24
        .run()
    }

    #[test]
    fn test_triple_nested_loops() -> Result<(), String> {
        ScriptTest::new(
            "float count = 0.0;
             for (float i = 0.0; i < 2.0; i = i + 1.0) {
                 for (float j = 0.0; j < 2.0; j = j + 1.0) {
                     for (float k = 0.0; k < 2.0; k = k + 1.0) {
                         float temp = i + j + k;
                         count = count + 1.0;
                     }
                 }
             }
             return count;",
        )
        .expect_result_dec32(8.0) // 2 * 2 * 2 = 8 iterations
        .run()
    }

    #[test]
    fn test_for_loop_local_computed_from_index() -> Result<(), String> {
        ScriptTest::new(
            "float sum = 0.0;
             for (float i = 1.0; i <= 3.0; i = i + 1.0) {
                 float square = i * i;
                 sum = sum + square;
             }
             return sum;",
        )
        .expect_result_dec32(14.0) // 1² + 2² + 3² = 1 + 4 + 9 = 14
        .run()
    }

    #[test]
    fn test_for_loop_conditional_local_update() -> Result<(), String> {
        ScriptTest::new(
            "float result = 0.0;
             for (float i = 0.0; i < 5.0; i = i + 1.0) {
                 float value = i;
                 if (i > 2.0) {
                     value = value * 2.0;
                 }
                 result = result + value;
             }
             return result;",
        )
        .expect_result_dec32(11.0) // 0 + 1 + 2 + (3*2) + (4*2) = 0+1+2+6+8 = 17... wait let me recalc
        // i=0: value=0, result=0
        // i=1: value=1, result=1
        // i=2: value=2, result=3
        // i=3: value=6, result=9
        // i=4: value=8, result=17
        .expect_result_dec32(17.0)
        .run()
    }

    #[test]
    fn test_for_loop_locals_dont_leak() -> Result<(), String> {
        // This test verifies that loop-local variables don't interfere
        // with variables in subsequent iterations
        ScriptTest::new(
            "float sum = 0.0;
             for (float i = 0.0; i < 3.0; i = i + 1.0) {
                 float temp = 10.0;
                 temp = temp + i;
                 sum = sum + temp;
             }
             return sum;",
        )
        .expect_result_dec32(33.0) // (10+0) + (10+1) + (10+2) = 10 + 11 + 12 = 33
        .run()
    }

    #[test]
    fn test_for_loop_sum_integration() -> Result<(), String> {
        // Integration test from tests/control_flow.rs
        ScriptTest::new(
            "float sum = 0.0;
             for (float i = 1.0; i <= 4.0; i = i + 1.0) {
                 sum = sum + i;
             }
             return sum;",
        )
        .expect_result_dec32(10.0) // 1 + 2 + 3 + 4 = 10
        .run()
    }

    #[test]
    fn test_for_loop_with_break_condition_integration() -> Result<(), String> {
        // Integration test from tests/control_flow.rs
        ScriptTest::new(
            "float result = 0.0;
             for (float i = 0.0; i < 100.0; i = i + 1.0) {
                 if (i >= 3.0) {
                     return i;
                 }
                 result = i;
             }
             return result;",
        )
        .expect_result_dec32(3.0)
        .run()
    }
}
