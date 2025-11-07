/// Variable expressions module
/// 
/// Groups parsing, code generation, type checking, and tests for variable expressions.

mod variable_gen;
mod variable_parse;
mod variable_types;
pub(in crate::compiler) use variable_types::{check_variable, check_incdec};

#[cfg(test)]
mod variable_tests;

