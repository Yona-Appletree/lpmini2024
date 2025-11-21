/// Variable expressions module
///
/// Groups parsing, code generation, type checking, and tests for variable expressions.
mod variable_gen;
mod variable_parse;
mod variable_types;
pub(crate) use variable_types::{check_incdec, check_variable};

#[cfg(test)]
mod variable_tests;
