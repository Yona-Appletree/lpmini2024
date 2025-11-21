/// Function call module
///
/// Groups parsing, code generation, type checking, and tests for function calls.
mod call_gen;
mod call_parse;
mod call_types;
mod expand_componentwise;

pub(in crate::compiler) use call_types::check_call;

#[cfg(test)]
mod call_dec32_tests;
#[cfg(test)]
mod call_mat3_tests;
#[cfg(test)]
mod call_vec2_tests;
#[cfg(test)]
mod call_vec3_tests;
#[cfg(test)]
mod call_vec4_tests;
