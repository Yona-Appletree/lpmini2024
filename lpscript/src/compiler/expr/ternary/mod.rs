/// Ternary conditional operator module
/// 
/// Groups parsing, code generation, type checking, and tests for ternary operator.

mod ternary_gen;
mod ternary_parse;
// mod ternary_types; // Old Box-based type checker (tests disabled - use ternary_tests.rs)

#[cfg(test)]
mod ternary_tests;

