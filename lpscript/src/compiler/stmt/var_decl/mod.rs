/// Variable declaration statement module
/// 
/// Groups parsing, code generation, type checking, and tests for variable declarations.

mod var_decl_gen;
mod var_decl_parse;
// TODO: Update var_decl_types to use pool-based API
// mod var_decl_types;

#[cfg(test)]
mod var_decl_tests;

