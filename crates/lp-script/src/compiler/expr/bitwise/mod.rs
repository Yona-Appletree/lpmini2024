/// Bitwise operators (&, |, ^, ~, <<, >>)
///
/// These operators only work on Int32 types in GLSL style.
mod bitwise_parse;
// TODO: Update bitwise_types to use pool-based API
// mod bitwise_types;
mod bitwise_gen;

#[cfg(test)]
mod bitwise_tests;
