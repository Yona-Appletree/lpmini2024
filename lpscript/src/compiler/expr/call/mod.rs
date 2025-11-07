/// Function call module
/// 
/// Groups parsing, code generation, type checking, and tests for function calls.

mod call_gen;
mod call_parse;
mod call_types;
mod expand_componentwise;

pub(in crate::compiler) use call_types::check_call_id;

#[cfg(test)]
mod call_tests;

