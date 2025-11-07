/// Literal expressions module
/// 
/// Groups parsing, code generation, type checking, and tests for literal expressions.

mod literals_gen;
mod literals_parse;
// TODO: Update literals_types to use pool-based API
// mod literals_types;
mod unary_gen;

#[cfg(test)]
mod literals_tests;

