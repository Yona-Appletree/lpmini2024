/// Binary arithmetic operators module
///
/// Groups parsing, code generation, type checking, and tests for binary operators.
mod binary_gen;
mod binary_parse;
mod binary_types;
pub(in crate::compiler) use binary_types::check_binary_arithmetic;

#[cfg(test)]
mod binary_fixed_tests;
#[cfg(test)]
mod binary_gen_tests;
#[cfg(test)]
mod binary_int32_tests;
#[cfg(test)]
mod binary_mat3_tests;
#[cfg(test)]
mod binary_vec2_tests;
#[cfg(test)]
mod binary_vec3_tests;
#[cfg(test)]
mod binary_vec4_tests;
