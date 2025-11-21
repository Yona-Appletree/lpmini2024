#[cfg(test)]
mod tests {
    use crate::lp_script::compiler::error::CodegenErrorKind;
    use crate::lp_script::compiler::expr::expr_test_util::ExprTest;

    #[test]
    fn mat3_modulo_is_not_supported() {
        ExprTest::new(
            "mat3(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0) % \
             mat3(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0)",
        )
        .expect_codegen_error_with_message(
            CodegenErrorKind::UnsupportedFeature(String::new()),
            "mat3",
        )
        .run()
        .expect("Mat3 modulo should produce codegen error");
    }

    #[test]
    fn modulo_with_mat3_and_scalar_is_not_supported() {
        ExprTest::new("mat3(2.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 2.0) % 2.0")
            .expect_codegen_error_with_message(
                CodegenErrorKind::UnsupportedFeature(String::new()),
                "mat3",
            )
            .run()
            .expect("Mat3 modulo with scalar should produce codegen error");
    }

    #[test]
    fn modulo_with_scalar_and_mat3_is_not_supported() {
        ExprTest::new("2.0 % mat3(2.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 2.0)")
            .expect_codegen_error_with_message(
                CodegenErrorKind::UnsupportedFeature(String::new()),
                "mat3",
            )
            .run()
            .expect("Scalar modulo with Mat3 should produce codegen error");
    }
}
