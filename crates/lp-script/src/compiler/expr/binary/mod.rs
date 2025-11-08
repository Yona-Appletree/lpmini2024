/// Binary arithmetic operators module
///
/// Groups parsing, code generation, type checking, and tests for binary operators.
mod binary_gen;
mod binary_parse;
mod binary_types;
pub(in crate::compiler) use binary_types::check_binary_arithmetic_id;

#[cfg(test)]
mod binary_tests;
