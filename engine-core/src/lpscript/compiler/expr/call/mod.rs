/// Function call module
/// 
/// Groups parsing, code generation, type checking, and tests for function calls.

mod call_gen;
mod call_parse;
mod call_types;
pub(crate) mod expand_componentwise;

#[cfg(test)]
mod call_tests;

