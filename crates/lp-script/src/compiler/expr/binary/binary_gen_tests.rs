#[cfg(test)]
mod tests {
    use crate::compile_expr;
    use crate::compiler::error::{CodegenErrorKind, CompileError};

    fn expect_codegen_error(input: &str) -> CodegenErrorKind {
        match compile_expr(input) {
            Ok(_) => panic!("Expected codegen error for input: {}", input),
            Err(CompileError::Codegen(err)) => err.kind,
            Err(other) => panic!(
                "Expected codegen error for input: {}, got different error: {}",
                input, other
            ),
        }
    }

    #[test]
    fn mat3_modulo_is_not_supported() {
        let kind = expect_codegen_error(
            "mat3(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0) % \
             mat3(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0)",
        );
        if let CodegenErrorKind::UnsupportedFeature(message) = kind {
            assert!(
                message.contains("mat3") && message.contains("%"),
                "Unexpected error message: {}",
                message
            );
        } else {
            panic!("Expected UnsupportedFeature error, got {:?}", kind);
        }
    }

    #[test]
    fn modulo_with_mat3_and_scalar_is_not_supported() {
        let kind = expect_codegen_error("mat3(2.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 2.0) % 2.0");
        if let CodegenErrorKind::UnsupportedFeature(message) = kind {
            assert!(
                message.contains("mat3") && message.contains("%"),
                "Unexpected error message: {}",
                message
            );
        } else {
            panic!("Expected UnsupportedFeature error, got {:?}", kind);
        }
    }

    #[test]
    fn modulo_with_scalar_and_mat3_is_not_supported() {
        let kind = expect_codegen_error("2.0 % mat3(2.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 2.0)");
        if let CodegenErrorKind::UnsupportedFeature(message) = kind {
            assert!(
                message.contains("mat3") && message.contains("%"),
                "Unexpected error message: {}",
                message
            );
        } else {
            panic!("Expected UnsupportedFeature error, got {:?}", kind);
        }
    }
}
