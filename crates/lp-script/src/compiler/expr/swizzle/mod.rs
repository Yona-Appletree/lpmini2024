/// Swizzle operators module
///
/// Groups parsing, code generation, type checking, and tests for swizzle operators.
mod swizzle_gen;
mod swizzle_parse;
// TODO: Update swizzle_types to use pool-based API
// mod swizzle_types;

#[cfg(test)]
mod swizzle_tests;
