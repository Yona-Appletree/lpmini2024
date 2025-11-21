#[cfg(test)]
mod tests {
    use crate::lp_script::compiler::error::CodegenErrorKind;
    use crate::lp_script::compiler::expr::expr_test_util::ExprTest;

    #[test]
    fn bool_negation_is_not_supported() {
        ExprTest::new("-(xNorm > 0.5)")
            .expect_codegen_error_with_message(
                CodegenErrorKind::UnsupportedFeature(String::new()),
                "negation",
            )
            .run()
            .expect("Bool negation should produce codegen error");
    }
}
