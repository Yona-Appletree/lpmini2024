/// Assignment expression module
/// 
/// Groups parsing, code generation, type checking, and tests for assignment expressions.

mod assign_expr_gen;
mod assign_expr_parse;
// TODO: Update assign_expr_types to use pool-based API
// mod assign_expr_types;

#[cfg(test)]
mod assign_expr_tests;

