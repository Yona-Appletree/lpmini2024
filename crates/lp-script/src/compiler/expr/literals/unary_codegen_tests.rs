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
    fn bool_negation_is_not_supported() {
        let kind = expect_codegen_error("-(xNorm > 0.5)");
        if let CodegenErrorKind::UnsupportedFeature(message) = kind {
            assert!(
                message.contains("negation") || message.contains("unary"),
                "Unexpected error message: {}",
                message
            );
        } else {
            panic!("Expected UnsupportedFeature error, got {:?}", kind);
        }
    }
}
