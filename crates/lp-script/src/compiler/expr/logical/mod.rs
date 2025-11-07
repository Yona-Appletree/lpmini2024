/// Logical operators module
///
/// Groups parsing, code generation, type checking, and tests for logical operators.
mod logical_gen;
mod logical_parse;
// TODO: Update logical_types to use pool-based API
// mod logical_types;

#[cfg(test)]
mod logical_tests;
